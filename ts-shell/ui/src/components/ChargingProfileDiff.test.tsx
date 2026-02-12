import { render, screen, fireEvent } from '@testing-library/react';
import { ChargingProfileDiff } from './ChargingProfileDiff';
import type { OcppTraceRecord } from '../types';
import { describe, it, expect } from 'vitest';

describe('ChargingProfileDiff', () => {
  const createProfile = (tick: number, id: number, purpose: string, limit: number): OcppTraceRecord => ({
    tick,
    timestamp: new Date().toISOString(),
    direction: 'inbound',
    action: 'SetChargingProfile',
    payload: {
      evseId: 1,
      chargingProfile: {
        id,
        stackLevel: 1,
        chargingProfilePurpose: purpose,
        chargingSchedule: {
          chargingRateUnit: 'W',
          chargingSchedulePeriod: [{ startPeriod: 0, limit }]
        }
      }
    }
  });

  it('renders "No differences" when traces are identical', () => {
    const p1 = createProfile(100, 1, 'TxProfile', 50000);
    render(<ChargingProfileDiff baseTrace={[p1]} compTrace={[p1]} baseLabel="Base" compLabel="Comp" />);
    expect(screen.getByText('No differences in charging profiles found.')).toBeInTheDocument();
  });

  it('detects added profiles', () => {
    const p1 = createProfile(100, 1, 'TxProfile', 50000);
    render(<ChargingProfileDiff baseTrace={[]} compTrace={[p1]} baseLabel="Base" compLabel="Comp" />);
    expect(screen.getByText('Added in Comp')).toBeInTheDocument();
    expect(screen.getByText('Tick 100 (ID 1)')).toBeInTheDocument();
  });

  it('detects removed profiles', () => {
    const p1 = createProfile(100, 1, 'TxProfile', 50000);
    render(<ChargingProfileDiff baseTrace={[p1]} compTrace={[]} baseLabel="Base" compLabel="Comp" />);
    expect(screen.getByText('Removed in Comp')).toBeInTheDocument();
    expect(screen.getByText('Tick 100 (ID 1)')).toBeInTheDocument();
  });

  it('detects modified profiles', () => {
    const p1 = createProfile(100, 1, 'TxProfile', 50000);
    const p2 = createProfile(100, 1, 'TxProfile', 60000); // Changed limit
    render(<ChargingProfileDiff baseTrace={[p1]} compTrace={[p2]} baseLabel="Base" compLabel="Comp" />);
    expect(screen.getByText('Modified Profiles')).toBeInTheDocument();
    expect(screen.getByText('Tick 100 (ID 1)')).toBeInTheDocument();
    // Should see both values
    expect(screen.getByText(/50000/)).toBeInTheDocument();
    expect(screen.getByText(/60000/)).toBeInTheDocument();
  });
  
  it('shows unchanged profiles when checkbox is clicked', () => {
    const p1 = createProfile(100, 1, 'TxProfile', 50000);
    render(<ChargingProfileDiff baseTrace={[p1]} compTrace={[p1]} baseLabel="Base" compLabel="Comp" />);
    
    expect(screen.queryByText(/T100: ID1/)).not.toBeInTheDocument();
    
    const checkbox = screen.getByLabelText(/Show Unchanged/);
    fireEvent.click(checkbox);
    
    expect(screen.getByText(/T100: ID1/)).toBeInTheDocument();
  });
});
