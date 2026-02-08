import { promises as fs } from 'fs';
import { createHash } from 'crypto';
import path from 'path';
import { RegistryEntry, RegistryEntrySchema } from './schema.js';

export class Registry {
  private registryPath: string;

  constructor(registryPath: string = 'runs/registry.jsonl') {
    this.registryPath = registryPath;
  }

  async registerRun(entry: Omit<RegistryEntry, 'timestamp'>): Promise<void> {
    const fullEntry: RegistryEntry = {
      ...entry,
      timestamp: new Date().toISOString(),
    };

    // Validate the entry
    RegistryEntrySchema.parse(fullEntry);

    // Append to the registry file
    const line = JSON.stringify(fullEntry) + '\n';
    await fs.appendFile(this.registryPath, line);
  }

  async getAllEntries(): Promise<RegistryEntry[]> {
    try {
      const content = await fs.readFile(this.registryPath, 'utf-8');
      const lines = content.trim().split('\n');
      return lines.map(line => RegistryEntrySchema.parse(JSON.parse(line)));
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code === 'ENOENT') {
        return [];
      }
      throw error;
    }
  }
}

export async function computeScenarioHash(scenarioPath: string): Promise<string> {
  const content = await fs.readFile(scenarioPath, 'utf-8');
  return createHash('sha256').update(content).digest('hex');
}