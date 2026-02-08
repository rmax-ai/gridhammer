import { z } from 'zod';

export const RegistryEntrySchema = z.object({
  runId: z.string(),
  scenarioHash: z.string(), // SHA256 of scenario content
  seed: z.number(),
  coreVersion: z.string(), // e.g., git commit or version
  artifactPaths: z.object({
    ocppTrace: z.string(),
    evseStates: z.string(),
    powerSeries: z.string().optional(),
    assertions: z.string(),
    runMeta: z.string(),
  }),
  timestamp: z.string(), // ISO string
});

export type RegistryEntry = z.infer<typeof RegistryEntrySchema>;