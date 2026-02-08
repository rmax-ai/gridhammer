import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { promises as fs } from 'fs';
import { createHash } from 'crypto';
import { Registry, computeScenarioHash } from './registry.js';
import { RegistryEntrySchema } from './schema.js';

describe('Registry', () => {
  const testRegistryPath = 'test-registry.jsonl';

  beforeEach(async () => {
    // Clean up before each test
    try {
      await fs.unlink(testRegistryPath);
    } catch {
      // Ignore if file doesn't exist
    }
  });

  afterEach(async () => {
    // Clean up after each test
    try {
      await fs.unlink(testRegistryPath);
    } catch {
      // Ignore if file doesn't exist
    }
  });

  it('registers a run correctly', async () => {
    const registry = new Registry(testRegistryPath);

    const entry = {
      runId: 'run-123456789',
      scenarioHash: 'abc123',
      seed: 42,
      coreVersion: 'v1.0.0',
      artifactPaths: {
        ocppTrace: 'runs/run-123456789-ocpp_trace.jsonl',
        evseStates: 'runs/run-123456789-evse_states.jsonl',
        powerSeries: 'runs/run-123456789-power_series.csv',
        assertions: 'runs/run-123456789-assertions.json',
        runMeta: 'runs/run-123456789-run_meta.json',
      },
    };

    await registry.registerRun(entry);

    const entries = await registry.getAllEntries();
    expect(entries).toHaveLength(1);

    const registeredEntry = entries[0];
    expect(registeredEntry.runId).toBe(entry.runId);
    expect(registeredEntry.scenarioHash).toBe(entry.scenarioHash);
    expect(registeredEntry.seed).toBe(entry.seed);
    expect(registeredEntry.coreVersion).toBe(entry.coreVersion);
    expect(registeredEntry.artifactPaths).toEqual(entry.artifactPaths);
    expect(registeredEntry.timestamp).toBeDefined();
    expect(new Date(registeredEntry.timestamp).getTime()).toBeLessThanOrEqual(Date.now());
  });

  it('validates entry schema', async () => {
    const registry = new Registry(testRegistryPath);

    const invalidEntry = {
      runId: 'run-123',
      scenarioHash: 'abc123',
      seed: 'not-a-number', // Invalid: should be number
      coreVersion: 'v1.0.0',
      artifactPaths: {
        ocppTrace: 'path',
        evseStates: 'path',
        assertions: 'path',
        runMeta: 'path',
      },
    };

    await expect(registry.registerRun(invalidEntry as any)).rejects.toThrow();
  });

  it('appends multiple entries', async () => {
    const registry = new Registry(testRegistryPath);

    const entry1 = {
      runId: 'run-1',
      scenarioHash: 'hash1',
      seed: 1,
      coreVersion: 'v1.0.0',
      artifactPaths: {
        ocppTrace: 'runs/run-1-ocpp_trace.jsonl',
        evseStates: 'runs/run-1-evse_states.jsonl',
        assertions: 'runs/run-1-assertions.json',
        runMeta: 'runs/run-1-run_meta.json',
      },
    };

    const entry2 = {
      runId: 'run-2',
      scenarioHash: 'hash2',
      seed: 2,
      coreVersion: 'v1.0.1',
      artifactPaths: {
        ocppTrace: 'runs/run-2-ocpp_trace.jsonl',
        evseStates: 'runs/run-2-evse_states.jsonl',
        assertions: 'runs/run-2-assertions.json',
        runMeta: 'runs/run-2-run_meta.json',
      },
    };

    await registry.registerRun(entry1);
    await registry.registerRun(entry2);

    const entries = await registry.getAllEntries();
    expect(entries).toHaveLength(2);
    expect(entries[0].runId).toBe('run-1');
    expect(entries[1].runId).toBe('run-2');
  });

  it('returns empty array when registry file does not exist', async () => {
    const registry = new Registry('nonexistent.jsonl');
    const entries = await registry.getAllEntries();
    expect(entries).toEqual([]);
  });
});

describe('computeScenarioHash', () => {
  it('computes SHA256 hash of file content', async () => {
    // Create a temporary file
    const tempFile = 'temp-scenario.yaml';
    const content = 'metadata:\n  version: "1.0"\n  name: "Test"\nsite:\n  transformer_capacity_kw: 100\n  evses: []\nevents:\n  arrivals: []\n  power_requests: []\n  faults: []';
    await fs.writeFile(tempFile, content);

    try {
      const hash = await computeScenarioHash(tempFile);
      expect(hash).toMatch(/^[a-f0-9]{64}$/); // SHA256 is 64 hex chars

      // Compute expected hash
      const expectedHash = createHash('sha256').update(content).digest('hex');
      expect(hash).toBe(expectedHash);
    } finally {
      await fs.unlink(tempFile);
    }
  });
});