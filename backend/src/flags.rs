//! Flag-Bits für Parameter

#![allow(dead_code)]

// Registerbank "Kommando" Bitflags
pub const NO_COMMAND:           u16 = 0x0000;
pub const OSCILLATOR_EN:        u16 = 0x0001;
pub const OFFSET_SET:           u16 = 0x0002; // High-Flanke
pub const STREAM_TRANSFER_EN:   u16 = 0x0004;
pub const STREAM_RAMP_EN:       u16 = 0x0008;
pub const SOFTWARE_TRIGGER:     u16 = 0x0010; // High-Flanke
pub const OFFSET_ADJUSTMENT_EN: u16 = 0x0020;
pub const DAC_OFFSET_RESET:     u16 = 0x0040;
pub const MEASURE_ENABLE:       u16 = 0x0080;
pub const HIGH_RES_EABLE:       u16 = 0x0100;
pub const STREAM_SELECT:        u16 = 0x0200;

// Registerbank "Version"
pub const ANALOG_BOARD_TYP_UNKNOW:           u16 = 0x00;
pub const ANALOG_BOARD_TYP_12CH_REV_30:      u16 = (0x01 << 3) | 0x01;  // Analogboard-Typ & Analogboard-Revision
pub const FPGA_DESIGN_REVISION_MASK:         u16 = 0x00FF;
pub const DEVICE_TYPE_MASK:                  u16 = 0xFF00;
pub const DEVICE_TYPE_MASK_SHIFT:            u16 = 8;

// Registerbank "Status" Bitflags
pub const TRIGGER_EN:           u32 = 1u32 << 0;
pub const NEW_MEAS_RESULTS:     u32 = 1u32 << 1;
pub const OFFSET_ERROR_CH0:     u32 = 1u32 << 10;
pub const OFFSET_ERROR_CH1:     u32 = 1u32 << 11;
pub const OFFSET_ERROR_CH2:     u32 = 1u32 << 12;
pub const OFFSET_ERROR_CH3:     u32 = 1u32 << 13;
pub const OFFSET_ERROR_CH4:     u32 = 1u32 << 14;
pub const OFFSET_ERROR_CH5:     u32 = 1u32 << 15;
pub const OFFSET_ERROR_CH6:     u32 = 1u32 << 16;
pub const OFFSET_ERROR_CH7:     u32 = 1u32 << 17;
pub const OFFSET_ERROR_CH8:     u32 = 1u32 << 18;
pub const OFFSET_ERROR_CH9:     u32 = 1u32 << 19;
pub const OFFSET_ERROR_CH10:    u32 = 1u32 << 20;
pub const OFFSET_ERROR_CH11:    u32 = 1u32 << 21;

// Registerbank "Konfiguration des Triggers" Bitflags
pub const OPTO_INPUT_01_EN:     u32 = 0x0001;
pub const OPTO_INPUT_02_EN:     u32 = 0x0002;
pub const OPTO_INPUT_03_EN:     u32 = 0x0004;
pub const OPTO_INPUT_04_EN:     u32 = 0x0008;
pub const SOFTTRIGGER_EN:       u32 = 0x0010;
pub const OPTO_INVERSE_01:      u32 = 0x0020;
pub const OPTO_INVERSE_02:      u32 = 0x0040;
pub const OPTO_INVERSE_03:      u32 = 0x0080;
pub const OPTO_INVERSE_04:      u32 = 0x0100;
pub const SOFTTRIGGER_INVERSE:  u32 = 0x0200;
pub const OPTO_REG_DEFAULT:     u32 = 0x0001; //TODO: Optische Eingänge festlegen
pub const OPTO_INPUTS_DISABLE:  u32 = 0x0000;

// FIFO-Register "TDFR" Bitflags
pub const RESET_KEY:            u32 = 0xA5;

// FIFO-Register "RDFO"
pub const OCCUPANCY_MASK:       u32 = 0x1FFFF;
pub const OCCUPANCY_SHIFT:      u32 = 0x0;

// measurement_mode in Results
pub const MODE_INIT:            u8 = 0x00;
pub const MODE_CALIB:           u8 = 0x01;
pub const MODE_ADJUSTMENT:      u8 = 0x02;
pub const MODE_MEASUREMENT:     u8 = 0x03;

// Streaming Mode
pub const STREAM_WITH_OFFSET:     u8 = 0x00;
pub const STREAM_WITHOUT_OFFSET:  u8 = 0x01;
pub const STREAM_WITH_RAMP:       u8 = 0x02;
pub const STREAM_GLITCHES_TEST:   u8 = 0x03;

// Target
pub const STREAM_IN_CSV:        u8 = 0x00;
pub const STREAM_IN_NIVADA:     u8 = 0x01;

// Konfiguration speichern in
pub const CONFIG_IN_RAM:        usize = 0x00;
pub const CONFIG_IN_FLASH:      usize = 0x01;

// Thread Steuerung
pub enum StateThread {
  Exit,
  Standby,
  Run,
  Stop,
  Break,
}

// Arten der Masseberechnung (P0009) (Wert 0x02 - 0xB reserviert)
pub const MASS_CALCULATION_INTEGRAL:           u8 = 0x00;
pub const MASS_CALCULATION_PEAK_VALUE:         u8 = 0x01;
pub const MASS_CALCULATION_POS_OFFSET_LIFT:    u8 = 0x0C;
pub const MASS_CALCULATION_NEG_OFFSET_LIFT:    u8 = 0x0D;

// Offsetbehandlung nach steigender Flanke (P0013) (Wert 0x00 - 0x02)
pub const OFFSET_TREATMENT_DETERMINATION:      u8 = 0x00;
pub const OFFSET_TREATMENT_ADJUSTMENT:         u8 = 0x01;
pub const OFFSET_TREATMENT_ADAPTION:           u8 = 0x02;

// Konfigurationsbits (TAG 13)
pub const CFG_BITS_AUTO_MSG_ENABLE:            u32 = 0x01000000;      // Daten an konfigurierte Adresse senden
pub const CFG_BITS_AUTO_MSG_MULTI_MSG:         u32 = 0x02000000;      // Daten als Multi-Antwort senden
pub const CFG_BITS_AUTO_MSG_MEAS_RESULTS:      u32 = 0x04000000;      // Messdaten senden
pub const CFG_BITS_AUTO_MSG_MEAS_INFO:         u32 = 0x08000000;      // Zusatzinfo senden
pub const CFG_BITS_AUTO_MSG_OFFSET_RESULTS:    u32 = 0x10000000;      // Ergebnis Offsetabgleich senden

// Flags für Selbsttest
pub const POS_THREASHOLD_EXCEED:               u8 = 0x01;
pub const NEG_THREASHOLD_EXCEED:               u8 = 0x02;
pub const SELFTEST_OK:                         u8 = 0x03;

