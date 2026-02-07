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
}
