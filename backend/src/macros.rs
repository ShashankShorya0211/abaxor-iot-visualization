//! Makros zur Konvertierung, Berechnung oder Ausgabe von Meldetexten
use colored::Colorize;
use log::{info, error};

/**
Makro zur Konvertierung eines Strings in ein Array fester Größe (1-32).

# Author: abaxor engineering GmbH

# Parameters
 - $name - generiert Funktionsname
 - $size - größe des zurückgelieferten Arrays

# Returns
 - Array mit fester Größe

# Example
```
str_arr_func!(str2dimarray_20, 20);
let array = str2dimarray_20("string....");

```
*/
#[macro_export]
macro_rules! str_arr_func {
  ($name:ident, $size:expr) => {
    pub fn $name(string: &str) -> [u8; $size] {
      let mut array = [0; $size];
      for (&x, p) in string.as_bytes().iter().zip(array.iter_mut()) {
          *p = x;
      }
      array
    }
  };
}

/**
Makro zur Konvertierung des ADC-Wertebereich sint16 zu SPS-Wertebereich 0..4095.

| SPS  |  ADC   |
|------|--------|
|    0 | -32768 |
| 2048 |      0 |
| 4095 |  32767 |

# Author: abaxor engineering GmbH

# Parameters
 - $input - Wertebereich sint16

# Returns
 - Konvertierter Wert 0-4095

# Example
```
convert_to_sps_range!(-1232);

```
*/
#[macro_export]
macro_rules! convert_to_sps_range {
  ($input:expr) => {

    (((($input as i16) as i32) + 32768i32 ) >> 4)as u32
    /*
    if ($input as i16 & 0x8000) > 0 {
      (($input as i32 - 0x7FFF) >> 4) as u32
    } else {
      ($input as i32 >> 4) as u32
    }*/

    //((($input as i32 & 0x8000 *(-1i32)) * ($input as i32 & 0x7FFF) + 32768i32) >> 4i32) as u32
  };
}

/**
Print line für Debug2-Ausgabe.

Nur aktiviert, wenn PRINT_LEVEL >= 5. Ausgabe in Syslog oder Terminal möglich, 
wenn PRINT_IN_SYSLOG entsprechend gesetzt.

# Author: abaxor engineering GmbH

# Parameters
 - $arg - String mit Argumenten

# Returns
 - no

# Example
```
debug2_println!("text... {}", var);

```
*/
#[macro_export]
macro_rules! debug2_println {
  () => ({
    if settings::PRINT_LEVEL >= 5 {
      if settings::PRINT_IN_SYSLOG == true {
        debug!("DEBUG 2: {}", std::format_args!($($arg)*));
      } else {
        print!("\n")
      }
    }
  });

  ($($arg:tt)*) => ({
    if settings::PRINT_LEVEL >= 5 {
      if settings::PRINT_IN_SYSLOG == true {
        debug!("DEBUG 2: {}", std::format_args!($($arg)*));
      } else {
        print!("DEBUG 2: {}", std::format_args!($($arg)*));
        print!("\n");
      }
    }
  });
}

/**
Print line für Debug-Ausgabe.

Nur aktiviert, wenn PRINT_LEVEL >= 4. Ausgabe in Syslog oder Terminal möglich, 
wenn PRINT_IN_SYSLOG entsprechend gesetzt.

# Author: abaxor engineering GmbH

# Parameters
 - $arg - String mit Argumenten

# Returns
 - no

# Example
```
debug_println!("text... {}", var);

```
*/
#[macro_export]
macro_rules! debug_println {
  () => ({
    if settings::PRINT_LEVEL >= 4 {
      if settings::PRINT_IN_SYSLOG == true {
        debug!("DEBUG : {}", std::format_args!($($arg)*));
      } else {
        print!("\n")
      }
    }
  });

  ($($arg:tt)*) => ({
    if settings::PRINT_LEVEL >= 4 {
      if settings::PRINT_IN_SYSLOG == true {
        debug!("DEBUG : {}", std::format_args!($($arg)*));
      } else {
        print!("DEBUG 2: {}", std::format_args!($($arg)*));
        print!("\n");
      }
    }
  });
}

/**
Print line für Info-Ausgabe.

Nur aktiviert, wenn PRINT_LEVEL >= 3. Ausgabe in Syslog oder Terminal möglich, 
wenn PRINT_IN_SYSLOG entsprechend gesetzt.

# Author: abaxor engineering GmbH

# Parameters
 - $arg - String mit Argumenten

# Returns
 - no

# Example
```
info_println!("text... {}", var);

```
*/
#[macro_export]
macro_rules! info_println {
  () => (
    if settings::PRINT_LEVEL >= 3 {
      if settings::PRINT_IN_SYSLOG == true {
        info!("INFO: {}", std::format_args!($($arg)*));
      } else {
        print!("\n")
      }
    }
  );

  ($($arg:tt)*) => (
    if settings::PRINT_LEVEL >= 3 {
      if settings::PRINT_IN_SYSLOG == true {
        info!("INFO: {}", std::format_args!($($arg)*));
      } else {
        print!("INFO: {}", std::format_args!($($arg)*));
        print!("\n");
      }
    }
  );
}

/**
Print line für Warn-Ausgabe.

Nur aktiviert, wenn PRINT_LEVEL >= 2. Ausgabe in Syslog oder Terminal möglich, 
wenn PRINT_IN_SYSLOG entsprechend gesetzt.

# Author: abaxor engineering GmbH

# Parameters
 - $arg - String mit Argumenten

# Returns
 - no

# Example
```
warn_println!("text... {}", var);

```
*/
#[macro_export]
macro_rules! warn_println {
  () => (
    if settings::PRINT_LEVEL >= 2 {
      if settings::PRINT_IN_SYSLOG == true {
        warn!("WARN: {}", std::format_args!($($arg)*));
      } else {
        print!("\n")
      }
    }
  );

  ($($arg:tt)*) => ({
    if settings::PRINT_LEVEL >= 2 {
      if settings::PRINT_IN_SYSLOG == true {
        warn!("WARN: {}", std::format_args!($($arg)*));
      } else {
        print!(" {} ", "WARN:".yellow());
        print!("{}", std::format_args!($($arg)*));
        print!("\n");
      }
    }
  })
}

/**
Print line für Error-Ausgabe.

Nur aktiviert, wenn PRINT_LEVEL >= 1. Ausgabe in Syslog oder Terminal möglich, 
wenn PRINT_IN_SYSLOG entsprechend gesetzt.

# Author: abaxor engineering GmbH

# Parameters
 - $arg - String mit Argumenten

# Returns
 - no

# Example
```
error_println!("text... {}", var);

```
*/
#[macro_export]
macro_rules! error_println {
  () => (
    if settings::PRINT_LEVEL >= 1 {
      if settings::PRINT_IN_SYSLOG == true {
        error!("ERROR: {}", std::format_args!($($arg)*));
      } else {
        print!("\n")
      }
    }
  );

  ($($arg:tt)*) => ({
    if settings::PRINT_LEVEL >= 1 {
      if settings::PRINT_IN_SYSLOG == true {
        error!("ERROR: {}", std::format_args!($($arg)*));
      } else {
        print!(" {} ", "ERROR:".red());
        print!("{}", std::format_args!($($arg)*));
        print!("\n");
      }
    }
  })
}

/**
Sofortige Ausgabe eines Debug-Strings.

Nur aktiviert, wenn PRINT_LEVEL >= 4.

# Author: abaxor engineering GmbH

# Parameters
 - $arg - String mit Argumenten

# Returns
 - no

# Example
```
debug_print!("text... {}", var);

```
*/
#[macro_export]
macro_rules! debug_print {
  ($($arg:tt)*) => (
    if settings::PRINT_LEVEL >= 4 {
      eprint!("{}", std::format_args!($($arg)*));
    }
  );
}

/**
Sofortige Ausgabe eines Info-Strings.

Nur aktiviert, wenn PRINT_LEVEL >= 3.

# Author: abaxor engineering GmbH

# Parameters
 - $arg - String mit Argumenten

# Returns
 - no

# Example
```
info_print!("text... {}", var);

```
*/
#[macro_export]
macro_rules! info_print {
  ($($arg:tt)*) => (
    if settings::PRINT_LEVEL >= 3 {
      eprint!("{}", std::format_args!($($arg)*));
    }
  );
}
