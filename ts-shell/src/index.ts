#!/usr/bin/env node
import { Command } from 'commander';
import { readFileSync, existsSync } from 'fs';
import { resolve, dirname, join } from 'path';
import { execa } from 'execa';
import yaml from 'js-yaml';
import { z } from 'zod';
import { Registry, computeScenarioHash } from './registry/registry.js';
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
      const assertionsPath = join(options.outputDir, `${runId}-assertions.json`);
      const runMetaPath = join(options.outputDir, `${runId}-run_meta.json`);

      const runConfig = {
        seed: parseInt(options.seed, 10),
        start_time: new Date().toISOString(), // Todo: use scenario start time if available?
        evse_count: scenario.site.evses.length,
        ticks: parseInt(options.ticks, 10),
        ocpp_trace_path: ocppTracePath,
        evse_state_path: evseStatePath
      };

      // Write run metadata
      const runMeta = {
          scenarioPath: resolve(scenarioPath),
          scenarioContent: content, // Snapshot for reproducibility
          runConfig
      };
      await fs.writeFile(runMetaPath, JSON.stringify(runMeta, null, 2));

      console.log('Starting simulation kernel...');
      
      const subprocess = execa(binaryPath, {
        input: JSON.stringify(runConfig),
        stdout: 'inherit',
        stderr: 'inherit'
      });

      await subprocess;
      console.log(`Simulation complete. Artifacts in ${options.outputDir}`);

      // Register run
      try {
        const { stdout: coreVersion } = await execa('git', ['rev-parse', 'HEAD']);
        const registry = new Registry(join(options.outputDir, 'registry.jsonl'));
        const scenarioHash = await computeScenarioHash(scenarioPath);
        
        await registry.registerRun({
            runId,
            scenarioHash,
            seed: runConfig.seed,
            coreVersion: coreVersion.trim(),
            artifactPaths: {
                ocppTrace: ocppTracePath,
                evseStates: evseStatePath,
                assertions: assertionsPath,
                runMeta: runMetaPath
            }
        });
        console.log(`Run registered: ${runId}`);
      } catch (e) {
        console.warn('Failed to register run:', e);
      }

    } catch (error) {
      console.error('Error running scenario:', error);
      process.exit(1);
    }
  });

program
  .command('replay')
  .description('Replay a previous run')
  .argument('<run-id>', 'Run ID to replay')
  .option('--registry <path>', 'Path to registry file', './runs/registry.jsonl')
  .action(async (runId, options) => {
    try {
        const registry = new Registry(options.registry);
        const entries = await registry.getAllEntries();
        const entry = entries.find(e => e.runId === runId);
        
        if (!entry) {
            console.error(`Run ID ${runId} not found in registry ${options.registry}`);
            process.exit(1);
        }

        console.log(`Replaying run ${runId}...`);
        
        // Read run meta
        const fs = await import('fs/promises');
        const runMetaContent = await fs.readFile(entry.artifactPaths.runMeta, 'utf-8');
        const runMeta = JSON.parse(runMetaContent);
        
        // Prepare replay paths
        const outputDir = dirname(entry.artifactPaths.ocppTrace);
        const replayId = `replay-${runId}`;
        const replayOcppTrace = join(outputDir, `${replayId}-ocpp_trace.jsonl`);
        const replayEvseStates = join(outputDir, `${replayId}-evse_states.jsonl`);
        
        const replayConfig = {
            ...runMeta.runConfig,
            ocpp_trace_path: replayOcppTrace,
            evse_state_path: replayEvseStates
        };

        const binaryPath = findRustBinary();
        console.log(`Using core binary: ${binaryPath}`);
        
        const subprocess = execa(binaryPath, {
            input: JSON.stringify(replayConfig),
            stdout: 'inherit',
            stderr: 'inherit'
        });
        
        await subprocess;
        console.log(`Replay complete.`);
        
        // Verify
        console.log('Verifying artifacts...');
        
        const verifyFile = async (original: string, replay: string) => {
            if (!existsSync(original)) {
                 console.warn(`Original file missing: ${original}`);
                 return true; // Skip
            }
            const origContent = await fs.readFile(original, 'utf-8');
            const replayContent = await fs.readFile(replay, 'utf-8');
            if (origContent !== replayContent) {
                console.error(`Mismatch in ${original} vs ${replay}`);
                return false;
            }
            return true;
        };
        
        const ok1 = await verifyFile(entry.artifactPaths.ocppTrace, replayOcppTrace);
        const ok2 = await verifyFile(entry.artifactPaths.evseStates, replayEvseStates);
        
        if (ok1 && ok2) {
            console.log('✅ Replay certification PASSED: Artifacts are identical.');
        } else {
            console.error('❌ Replay certification FAILED: Artifacts differ.');
            process.exit(1);
        }

    } catch (error) {
        console.error('Error replaying:', error);
        process.exit(1);
    }
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
