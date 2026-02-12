# GridHammer Lens

A read-only web UI for visualizing GridHammer simulation artifacts.

## Features

- **EVSE Timelines**: Interactive visualization of state transitions for all EVSEs.
- **Power Curves**: Graphical representation of power usage and limits over time.
- **Profile Viewer**: Detailed history of received Charging Profiles and enforced limits.
- **Comparison Mode**: Side-by-side comparison of two different runs, including:
    - Power curve overlays.
    - Synchronized timeline diffs.
    - Charging Profile semantic diff (Added/Removed/Modified).

## Usage

### Development
```bash
npm run dev
```

### Production Build
```bash
npm run build
npm run preview
```

### Loading Data
Simply drag and drop the following files from your simulation output directory:
- `evse_states.jsonl`
- `power_series.csv`
- `ocpp_trace.jsonl`

In Comparison Mode, you can upload two sets of these files to compare them.

## Technical Details

- **Tech Stack**: React + Vite + TypeScript.
- **Visualizations**: Custom high-performance SVG components (interactive tooltips, zooming).
- **Architecture**: Purely client-side, no backend required. Strictly read-only.
