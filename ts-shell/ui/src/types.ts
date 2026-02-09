export type EvseState = 
  | "Available"
  | "Preparing"
  | "Charging"
  | "SuspendedEV"
  | "SuspendedEVSE"
  | "Faulted"
  | "Unavailable"
  | "Reserved";

export type EnforcementReason = 
  | "ProfileLimit"
  | "StationLimit"
  | "InternalConstraint";

export type ChargingRateUnit = 
  | "Watts"
  | "Amperes";

export interface EnforcedLimit {
  limit: number;
  unit: ChargingRateUnit;
  reason: EnforcementReason;
  should_suspend: boolean;
}

export interface EvseStateRecord {
  tick: number;
  timestamp: string; // ISO 8601
  evse_id: number;
  state: EvseState;
  active_transaction_id?: string;
  enforced_limit?: EnforcedLimit;
}

export type Direction = "inbound" | "outbound";

export interface OcppTraceRecord {
  tick: number;
  timestamp: string;
  direction: Direction;
  action: string;
  payload: unknown;
}

export interface PowerSeriesRecord {
  tick: number;
  timestamp: string;
  evse_id: number;
  power_w: number;
  limit_w?: number;
}
