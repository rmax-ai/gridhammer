#!/usr/bin/env node
import { Command } from 'commander';
import { readFileSync, existsSync } from 'fs';
import { resolve, dirname, join } from 'path';
import { execa } from 'execa';
import yaml from 'js-yaml';
import { z } from 'zod';
import { ScenarioSchema } from './scenario/schema.js';

import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const program = new Command();

program
  .name('gridhammer')
  .description('GridHammer OCPP 2.1 Simulation Platform')
  .version('0.1.0');

// Helper to find Rust binary
function findRustBinary(): string {
  // Look in typical cargo target locations relative to this script
  const candidates = [
    resolve(__dirname, '../../target/debug/rust-core'),
    resolve(__dirname, '../../target/release/rust-core'),
    resolve(__dirname, '../../../target/debug/rust-core'), // In case we are in dist/
    resolve(process.cwd(), 'target/debug/rust-core'), // Fallback to CWD
  ];

  for (const path of candidates) {
    if (existsSync(path)) {
      return path;
    }
  }
  throw new Error(`Could not find rust-core binary. Checked: ${candidates.join(', ')}`);
}

program
  .command('run')
  .description('Run a simulation scenario')
  .argument('<scenario-path>', 'Path to the scenario file (JSON or YAML)')
  .option('--seed <number>', 'Random seed', '0')
  .option('--ticks <number>', 'Number of ticks to run', '100')
  .option('--output-dir <path>', 'Directory for output artifacts', './runs')
  .action(async (scenarioPath, options) => {
    try {
      console.log(`Loading scenario from ${scenarioPath}...`);
      const content = readFileSync(scenarioPath, 'utf-8');
      
      let rawData;
      if (scenarioPath.endsWith('.yaml') || scenarioPath.endsWith('.yml')) {
        rawData = yaml.load(content);
      } else {
        rawData = JSON.parse(content);
      }

      const scenario = ScenarioSchema.parse(rawData);
      console.log(`Scenario '${scenario.metadata.name}' loaded successfully.`);
      console.log(`Site has ${scenario.site.evses.length} EVSEs.`);

      const binaryPath = findRustBinary();
      console.log(`Using core binary: ${binaryPath}`);

      // Prepare RunConfig for Rust Core
      // Ensure output directory exists
      // For now, we just pass paths. Rust core expects directory creation or we do it?
      // Rust core: File::create(path)? will fail if dir doesn't exist.
      // So we should mkdir -p
      
      const fs = await import('fs/promises');
      await fs.mkdir(options.outputDir, { recursive: true });

      const runId = `run-${Date.now()}`;
      const ocppTracePath = join(options.outputDir, `${runId}-ocpp_trace.jsonl`);
      const evseStatePath = join(options.outputDir, `${runId}-evse_states.jsonl`);

      const runConfig = {
        seed: parseInt(options.seed, 10),
        start_time: new Date().toISOString(), // Todo: use scenario start time if available?
        evse_count: scenario.site.evses.length,
        ticks: parseInt(options.ticks, 10),
        ocpp_trace_path: ocppTracePath,
        evse_state_path: evseStatePath
      };

      console.log('Starting simulation kernel...');
      
      const subprocess = execa(binaryPath, {
        input: JSON.stringify(runConfig),
        stdout: 'inherit',
        stderr: 'inherit'
      });

      await subprocess;
      console.log(`Simulation complete. Artifacts in ${options.outputDir}`);

    } catch (error) {
      console.error('Error running scenario:', error);
      process.exit(1);
    }
  });

program
  .command('replay')
  .description('Replay a previous run (Not implemented)')
  .argument('<run-id>', 'Run ID to replay')
  .action((runId) => {
    console.log('Replay not implemented yet');
  });

program
  .command('validate')
  .description('Validate a scenario file')
  .argument('<scenario-path>', 'Path to the scenario file')
  .action((scenarioPath) => {
    try {
      const content = readFileSync(scenarioPath, 'utf-8');
      let rawData;
      if (scenarioPath.endsWith('.yaml') || scenarioPath.endsWith('.yml')) {
        rawData = yaml.load(content);
      } else {
        rawData = JSON.parse(content);
      }
      ScenarioSchema.parse(rawData);
      console.log('Scenario is valid.');
    } catch (error) {
      console.error('Validation failed:', error);
      process.exit(1);
    }
  });

program.parse();
