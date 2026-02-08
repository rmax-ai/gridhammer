import { describe, it, expect } from 'vitest';
import { ScenarioSchema } from './schema.js';

describe('ScenarioSchema', () => {
  it('validates a sample scenario', () => {
    const sample = {
      metadata: {
        version: '1.0',
        name: 'Test Scenario',
        description: 'A test scenario for validation',
      },
      site: {
        transformer_capacity_kw: 100,
        evses: [
          {
            id: 'evse1',
            max_power_kw: 50,
            connector_type: 'Type2',
          },
        ],
      },
      events: {
        arrivals: [
          {
            tick: 0,
            evse_id: 'evse1',
            vehicle_id: 'veh1',
            requested_energy_kwh: 20,
          },
        ],
        power_requests: [],
        faults: [],
      },
    };

    expect(ScenarioSchema.parse(sample)).toEqual(sample);
  });

  it('rejects invalid scenario with negative capacity', () => {
    const invalid = {
      metadata: { version: '1.0', name: 'Test' },
      site: { transformer_capacity_kw: -10, evses: [] },
      events: { arrivals: [], power_requests: [], faults: [] },
    };

    expect(() => ScenarioSchema.parse(invalid)).toThrow();
  });
});