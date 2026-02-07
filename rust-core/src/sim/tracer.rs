use serde::Serialize;
use std::io::Write;

use crate::evse::state::EvseState;

#[derive(Debug, Serialize)]
pub struct EvseStateRecord {
    pub tick: u64,
    pub timestamp: String, // ISO 8601
    pub evse_id: u32,
    pub state: EvseState,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    Inbound,
    Outbound,
}

#[derive(Debug, Serialize)]
pub struct OcppTraceRecord {
    pub tick: u64,
    pub timestamp: String, // ISO 8601
    pub direction: Direction,
    pub action: String, // e.g. "BootNotification"
    pub payload: serde_json::Value,
}

pub struct Tracer<W: Write> {
    writer: W,
}

impl<W: Write> Tracer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn trace_evse_state(&mut self, record: &EvseStateRecord) -> std::io::Result<()> {
        serde_json::to_writer(&mut self.writer, record)?;
        self.writer.write_all(b"\n")?;
        Ok(())
    }

    pub fn trace_ocpp_message(&mut self, record: &OcppTraceRecord) -> std::io::Result<()> {
        serde_json::to_writer(&mut self.writer, record)?;
        self.writer.write_all(b"\n")?;
        Ok(())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_trace_evse_state() {
        let mut buffer = Vec::new();
        let mut tracer = Tracer::new(&mut buffer);

        let record = EvseStateRecord {
            tick: 1,
            timestamp: Utc::now().to_rfc3339(),
            evse_id: 101,
            state: EvseState::Charging,
        };

        tracer.trace_evse_state(&record).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("\"evse_id\":101"));
        assert!(output.contains("\"state\":\"Charging\""));
        assert!(output.ends_with('\n'));
    }

    #[test]
    fn test_trace_ocpp_message() {
        let mut buffer = Vec::new();
        let mut tracer = Tracer::new(&mut buffer);

        let payload = serde_json::json!({
            "reason": "PowerUp",
            "chargingStation": {
                "model": "TestModel",
                "vendorName": "TestVendor"
            }
        });

        let record = OcppTraceRecord {
            tick: 42,
            timestamp: Utc::now().to_rfc3339(),
            direction: Direction::Outbound,
            action: "BootNotification".to_string(),
            payload,
        };

        tracer.trace_ocpp_message(&record).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("\"tick\":42"));
        assert!(output.contains("\"direction\":\"outbound\""));
        assert!(output.contains("\"action\":\"BootNotification\""));
        assert!(output.contains("\"reason\":\"PowerUp\""));
    }
}
