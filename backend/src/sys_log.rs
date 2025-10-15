//! System-Logging

use syslog::{Facility, Error};
use log::{debug, error};

/**
System-Logging initialisieren

Author: abaxor engineering GmbH

Parameter:
 - no

Return:
 - Results (OK, Error)
*/
pub fn init() -> Result<(), Error> {
  syslog::init(
                Facility::LOG_USER,
                log::LevelFilter::Debug,
                Some("test_fifostream_12ch"))?;
  debug!("Debug-Level Logging initialisiert");
  error!("Error-Level Logging initialisiert");
  Ok(())
}
