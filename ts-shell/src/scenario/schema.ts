import { z } from 'zod';

const metadataSchema = z.object({
  version: z.string(),
  name: z.string(),
  description: z.string().optional(),
});

const evseSchema = z.object({
  id: z.string(),
  max_power_kw: z.number().positive(),
  connector_type: z.string(),
});

const siteTopologySchema = z.object({
  transformer_capacity_kw: z.number().positive(),
  evses: z.array(evseSchema),
});

const arrivalSchema = z.object({
  tick: z.number().int().min(0),
  evse_id: z.string(),
  vehicle_id: z.string(),
  requested_energy_kwh: z.number().positive(),
});

const powerRequestSchema = z.object({
  tick: z.number().int().min(0),
  evse_id: z.string(),
  power_kw: z.number().min(0),
});

const faultSchema = z.object({
  tick: z.number().int().min(0),
  evse_id: z.string(),
  fault_type: z.string(),
});

const eventsSchema = z.object({
  arrivals: z.array(arrivalSchema),
  power_requests: z.array(powerRequestSchema),
  faults: z.array(faultSchema),
});

export const ScenarioSchema = z.object({
  metadata: metadataSchema,
  site: siteTopologySchema,
  events: eventsSchema,
});

export type Scenario = z.infer<typeof ScenarioSchema>;