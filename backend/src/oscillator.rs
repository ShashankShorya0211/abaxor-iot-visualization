//! Oszillator initialisieren und steuern.

use std::os::unix::prelude::*;

use super::*;

use log::info;

/**
Oszillator initialisieren.

# Author: abaxor engineering GmbH

# Parameters
 - config - Config-Struktur für Konfigurationsdaten
 - reg - File Descriptor für Register-Bank
 - channel_num - Nummer der Messstelle

# Returns
 - Results (OK, Error)
*/
pub fn init(reg: RawFd, channel_num: usize) -> Result<(), ()> {
  // Periodendauer des Oszillator in der Registerbank setzen
  let period = settings::MESSUNG_OSZILLATOR_PERIODEN_DAUER;
  info_println!(" -> Oszillator: CH{0:02} Periodendauer von {1:} ns in der Registerbank setzen", channel_num, period);

  // Oszillator mit 1 Mhz betreiben
  reg_bank::set_osc_period_0(reg, period); // 1/(50*20ns) = 1MHz

  info_println!(" -> Oszillator: CH{0:02} aktivieren", channel_num);
  enable(reg);          // Oszillator einschalten

  Ok(())
}

/**
Aktivieren des Oszillators.

# Author: abaxor engineering GmbH

# Parameters
 - reg - File Descriptor für Register-Bank

# Returns
 - Results (OK, Error)
*/
pub fn enable(reg: RawFd) -> Result<(), ()> {
  let cmd = reg_bank::get_command(reg);
  reg_bank::set_command(reg, cmd | flags::OSCILLATOR_EN);

  Ok(())
}

