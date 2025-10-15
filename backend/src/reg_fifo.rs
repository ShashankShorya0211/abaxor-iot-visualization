//! Registerbank lesen und schreiben. Autogeneriertes Modul!
//! Generiert mit convert2c-1.0 !

#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused)]

use nix::{ioctl_read_buf, ioctl_write_buf};
use std::os::unix::prelude::*;
use libc::{c_int};
use std::ffi::CString;
use super::*;
use log::{info, error};


//##################### register bank ioctl declarations #####################

// Makro für Lesezugriff auf Register "axi_str_isr"
ioctl_read_buf!(read_axi_str_isr, 0, 1, u8);                                    // ID: 001 : Interrupt Status Register (ISR)

// Makro für Schreibzugriff auf Register "axi_str_ier"
ioctl_write_buf!(write_axi_str_ier, 0, 2, u8);                                  // ID: 002 : Interrupt Enable Register (IER)

// Makro für Lesezugriff auf Register "axi_str_ier"
ioctl_read_buf!(read_axi_str_ier, 0, 2, u8);                                    // ID: 002 : Interrupt Enable Register (IER)

// Makro für Schreibzugriff auf Register "axi_str_tdfr"
ioctl_write_buf!(write_axi_str_tdfr, 0, 3, u8);                                 // ID: 003 : Transmit Data FIFO Reset (TDFR)

// Makro für Schreibzugriff auf Register "axi_str_tdfv"
ioctl_write_buf!(write_axi_str_tdfv, 0, 4, u8);                                 // ID: 004 : Transmit Data FIFO Vacancy (TDFV)

// Makro für Lesezugriff auf Register "axi_str_tdfv"
ioctl_read_buf!(read_axi_str_tdfv, 0, 4, u8);                                   // ID: 004 : Transmit Data FIFO Vacancy (TDFV)

// Makro für Schreibzugriff auf Register "axi4_str_tdfd"
ioctl_write_buf!(write_axi4_str_tdfd, 0, 5, u8);                                // ID: 005 : Transmit Data FIFO 32-bit Wide Data Write Port (TDFD)

// Makro für Schreibzugriff auf Register "axi_str_tlr"
ioctl_write_buf!(write_axi_str_tlr, 0, 6, u8);                                  // ID: 006 : Transmit Length Register (TLR)

// Makro für Schreibzugriff auf Register "axi_str_rdfr"
ioctl_write_buf!(write_axi_str_rdfr, 0, 7, u8);                                 // ID: 007 : Receive DATA FIFO reset (RDFR)

// Makro für Lesezugriff auf Register "axi_str_rdfo"
ioctl_read_buf!(read_axi_str_rdfo, 0, 8, u8);                                   // ID: 008 : Receive Data FIFO Occupancy (RDFO)

// Makro für Lesezugriff auf Register "axi4_str_rdfd"
ioctl_read_buf!(read_axi4_str_rdfd, 0, 9, u8);                                  // ID: 009 : Receive Data FIFO 32-bit Wide Data Write Port (RDFD)

// Makro für Lesezugriff auf Register "axi_str_rlr"
ioctl_read_buf!(read_axi_str_rlr, 0, 10, u8);                                   // ID: 010 : Receive Length Register (RLR)

// Makro für Schreibzugriff auf Register "axi_str_srr"
ioctl_write_buf!(write_axi_str_srr, 0, 11, u8);                                 // ID: 011 : AXI4-Stream Reset (SRR)

// Makro für Schreibzugriff auf Register "axi_str_tdr"
ioctl_write_buf!(write_axi_str_tdr, 0, 12, u8);                                 // ID: 012 : Transmit Destination Register (TDR)

// Makro für Schreibzugriff auf Register "axi_str_rdr"
ioctl_write_buf!(write_axi_str_rdr, 0, 13, u8);                                 // ID: 013 : Receive Destination Register (RDR)

// Makro für Lesezugriff auf Register "axi_str_rdr"
ioctl_read_buf!(read_axi_str_rdr, 0, 13, u8);                                   // ID: 013 : Receive Destination Register (RDR)

// Makro für Schreibzugriff auf Register "axi_str_tx_id"
ioctl_write_buf!(write_axi_str_tx_id, 0, 14, u8);                               // ID: 014 : Transmit ID Register

// Makro für Schreibzugriff auf Register "axi_str_tx_user"
ioctl_write_buf!(write_axi_str_tx_user, 0, 15, u8);                             // ID: 015 : Transmit USER Register

// Makro für Schreibzugriff auf Register "axi_str_rx_id"
ioctl_write_buf!(write_axi_str_rx_id, 0, 16, u8);                               // ID: 016 : Receive ID Register

// Makro für Lesezugriff auf Register "axi_str_rx_id"
ioctl_read_buf!(read_axi_str_rx_id, 0, 16, u8);                                 // ID: 016 : Receive ID Register

// Makro für Schreibzugriff auf Register "axi_str_rx_user"
ioctl_write_buf!(write_axi_str_rx_user, 0, 17, u8);                             // ID: 017 : Receive USER Register

// Makro für Lesezugriff auf Register "axi_str_rx_user"
ioctl_read_buf!(read_axi_str_rx_user, 0, 17, u8);                               // ID: 017 : Receive USER Register

//#####################  register bank initialization #####################

/// Registerbank initialisieren.
pub fn init() -> c_int {
  info_println!(" -> FIFO: initialisieren");

  // Registerbank öffnen
  info_println!(" -> FIFO: Device öffnen");
  let fd = open_reg_bank();

  fd
}

/// open device for register
fn open_reg_bank() -> c_int {
  let fname = CString::new("/dev/hh_amv_psfifo_dev").unwrap();
  unsafe { libc::open(fname.as_ptr(), libc::O_NONBLOCK | libc::O_RDWR) }
}


//#####################  register bank public functions #####################


/// ID: 001 : set "Interrupt Status Register (ISR)"
pub fn set_axi_str_isr(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("axi_str_isr Schreibgeschützes Register");
  Ok(())
}

/// ID: 001 : get "Interrupt Status Register (ISR)"
pub fn get_axi_str_isr(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_axi_str_isr(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 002 : set "Interrupt Enable Register (IER)"
pub fn set_axi_str_ier(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_ier(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 002 : get "Interrupt Enable Register (IER)"
pub fn get_axi_str_ier(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_axi_str_ier(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 003 : set "Transmit Data FIFO Reset (TDFR)"
pub fn set_axi_str_tdfr(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_tdfr(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 003 : get "Transmit Data FIFO Reset (TDFR)"
pub fn get_axi_str_tdfr(fd: RawFd) -> u32 {
  // Register is write only
  let mut data = [0u8; 4];
  unsafe {
    std::mem::transmute(data)
  }
}

/// ID: 004 : set "Transmit Data FIFO Vacancy (TDFV)"
pub fn set_axi_str_tdfv(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_tdfv(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 004 : get "Transmit Data FIFO Vacancy (TDFV)"
pub fn get_axi_str_tdfv(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_axi_str_tdfv(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 005 : set "Transmit Data FIFO 32-bit Wide Data Write Port (TDFD)"
pub fn set_axi4_str_tdfd(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi4_str_tdfd(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 005 : get "Transmit Data FIFO 32-bit Wide Data Write Port (TDFD)"
pub fn get_axi4_str_tdfd(fd: RawFd) -> u32 {
  // Register is write only
  let mut data = [0u8; 4];
  unsafe {
    std::mem::transmute(data)
  }
}

/// ID: 006 : set "Transmit Length Register (TLR)"
pub fn set_axi_str_tlr(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_tlr(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 006 : get "Transmit Length Register (TLR)"
pub fn get_axi_str_tlr(fd: RawFd) -> u32 {
  // Register is write only
  let mut data = [0u8; 4];
  unsafe {
    std::mem::transmute(data)
  }
}

/// ID: 007 : set "Receive DATA FIFO reset (RDFR)"
pub fn set_axi_str_rdfr(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_rdfr(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 007 : get "Receive DATA FIFO reset (RDFR)"
pub fn get_axi_str_rdfr(fd: RawFd) -> u32 {
  // Register is write only
  let mut data = [0u8; 4];
  unsafe {
    std::mem::transmute(data)
  }
}

/// ID: 008 : set "Receive Data FIFO Occupancy (RDFO)"
pub fn set_axi_str_rdfo(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("axi_str_rdfo Schreibgeschützes Register");
  Ok(())
}

/// ID: 008 : get "Receive Data FIFO Occupancy (RDFO)"
pub fn get_axi_str_rdfo(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_axi_str_rdfo(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 009 : set "Receive Data FIFO 32-bit Wide Data Write Port (RDFD)"
pub fn set_axi4_str_rdfd(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("axi4_str_rdfd Schreibgeschützes Register");
  Ok(())
}

/// ID: 009 : get "Receive Data FIFO 32-bit Wide Data Write Port (RDFD)"
pub fn get_axi4_str_rdfd(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_axi4_str_rdfd(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 010 : set "Receive Length Register (RLR)"
pub fn set_axi_str_rlr(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("axi_str_rlr Schreibgeschützes Register");
  Ok(())
}

/// ID: 010 : get "Receive Length Register (RLR)"
pub fn get_axi_str_rlr(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_axi_str_rlr(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 011 : set "AXI4-Stream Reset (SRR)"
pub fn set_axi_str_srr(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_srr(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 011 : get "AXI4-Stream Reset (SRR)"
pub fn get_axi_str_srr(fd: RawFd) -> u32 {
  // Register is write only
  let mut data = [0u8; 4];
  unsafe {
    std::mem::transmute(data)
  }
}

/// ID: 012 : set "Transmit Destination Register (TDR)"
pub fn set_axi_str_tdr(fd: RawFd, data: u32) -> Result<(), &'static str> {
  if data >= 1 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_tdr(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 12 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 012 : get "Transmit Destination Register (TDR)"
pub fn get_axi_str_tdr(fd: RawFd) -> u32 {
  // Register is write only
  let mut data = [0u8; 4];
  unsafe {
    std::mem::transmute(data)
  }
}

/// ID: 013 : set "Receive Destination Register (RDR)"
pub fn set_axi_str_rdr(fd: RawFd, data: u32) -> Result<(), &'static str> {
  if data >= 2 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_rdr(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 13 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 013 : get "Receive Destination Register (RDR)"
pub fn get_axi_str_rdr(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_axi_str_rdr(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 014 : set "Transmit ID Register"
pub fn set_axi_str_tx_id(fd: RawFd, data: u32) -> Result<(), &'static str> {
  if data >= 3 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_tx_id(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 14 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 014 : get "Transmit ID Register"
pub fn get_axi_str_tx_id(fd: RawFd) -> u32 {
  // Register is write only
  let mut data = [0u8; 4];
  unsafe {
    std::mem::transmute(data)
  }
}

/// ID: 015 : set "Transmit USER Register"
pub fn set_axi_str_tx_user(fd: RawFd, data: u32) -> Result<(), &'static str> {
  if data >= 4 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_tx_user(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 15 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 015 : get "Transmit USER Register"
pub fn get_axi_str_tx_user(fd: RawFd) -> u32 {
  // Register is write only
  let mut data = [0u8; 4];
  unsafe {
    std::mem::transmute(data)
  }
}

/// ID: 016 : set "Receive ID Register"
pub fn set_axi_str_rx_id(fd: RawFd, data: u32) -> Result<(), &'static str> {
  if data >= 5 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_rx_id(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 16 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 016 : get "Receive ID Register"
pub fn get_axi_str_rx_id(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_axi_str_rx_id(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 017 : set "Receive USER Register"
pub fn set_axi_str_rx_user(fd: RawFd, data: u32) -> Result<(), &'static str> {
  if data >= 6 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_axi_str_rx_user(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 17 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 017 : get "Receive USER Register"
pub fn get_axi_str_rx_user(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_axi_str_rx_user(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}




//#####################  register bank public functions collected in vector #####################


