//! Registerbank lesen und schreiben. Autogeneriertes Modul!
//! Generiert mit convert2c-1.0 !

/*
 * $Rev:: 33                   $
 * $Author:: seblaesi          $
 * $Date:: 2021-08-30 10:16:07#$
*/

#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused)]

use nix::{ioctl_read_buf, ioctl_write_buf};
use std::os::unix::prelude::*;
use libc::{c_int};
use std::ffi::CString;
use super::*;
use log::{info, warn, error, debug};


//##################### register bank ioctl declarations #####################

// Makro für Lesezugriff auf Register "version"
ioctl_read_buf!(read_version, 0, 1, u8);                                        // ID: 001 : Version

// Makro für Schreibzugriff auf Register "command"
ioctl_write_buf!(write_command, 0, 2, u8);                                      // ID: 002 : Command

// Makro für Lesezugriff auf Register "command"
ioctl_read_buf!(read_command, 0, 2, u8);                                        // ID: 002 : Command

// Makro für Lesezugriff auf Register "status"
ioctl_read_buf!(read_status, 0, 3, u8);                                         // ID: 003 : Status

// Makro für Schreibzugriff auf Register "test_regbank"
ioctl_write_buf!(write_test_regbank, 0, 4, u8);                                 // ID: 004 : Test Regbank

// Makro für Lesezugriff auf Register "test_regbank"
ioctl_read_buf!(read_test_regbank, 0, 4, u8);                                   // ID: 004 : Test Regbank

// Makro für Schreibzugriff auf Register "debug_channel"
ioctl_write_buf!(write_debug_channel, 0, 5, u8);                                // ID: 005 : Debug Channel

// Makro für Lesezugriff auf Register "debug_channel"
ioctl_read_buf!(read_debug_channel, 0, 5, u8);                                  // ID: 005 : Debug Channel

// Makro für Lesezugriff auf Register "debug_0"
ioctl_read_buf!(read_debug_0, 0, 6, u8);                                        // ID: 006 : Debug0

// Makro für Lesezugriff auf Register "debug_1"
ioctl_read_buf!(read_debug_1, 0, 7, u8);                                        // ID: 007 : Debug1

// Makro für Lesezugriff auf Register "debug_2"
ioctl_read_buf!(read_debug_2, 0, 8, u8);                                        // ID: 008 : Debug2

// Makro für Lesezugriff auf Register "opto_in"
ioctl_read_buf!(read_opto_in, 0, 9, u8);                                        // ID: 009 : Opto_in

// Makro für Lesezugriff auf Register "sp_result_raw_ch00"
ioctl_read_buf!(read_sp_result_raw_ch00, 0, 10, u8);                            // ID: 010 : RAW Result Ch0

// Makro für Lesezugriff auf Register "sp_result_raw_ch01"
ioctl_read_buf!(read_sp_result_raw_ch01, 0, 11, u8);                            // ID: 011 : RAW Result Ch1

// Makro für Lesezugriff auf Register "sp_result_raw_ch02"
ioctl_read_buf!(read_sp_result_raw_ch02, 0, 12, u8);                            // ID: 012 : RAW Result Ch2

// Makro für Lesezugriff auf Register "sp_result_raw_ch03"
ioctl_read_buf!(read_sp_result_raw_ch03, 0, 13, u8);                            // ID: 013 : RAW Result Ch3

// Makro für Lesezugriff auf Register "sp_result_raw_ch04"
ioctl_read_buf!(read_sp_result_raw_ch04, 0, 14, u8);                            // ID: 014 : RAW Result Ch4

// Makro für Lesezugriff auf Register "sp_result_raw_ch05"
ioctl_read_buf!(read_sp_result_raw_ch05, 0, 15, u8);                            // ID: 015 : RAW Result Ch5

// Makro für Lesezugriff auf Register "sp_result_raw_ch06"
ioctl_read_buf!(read_sp_result_raw_ch06, 0, 16, u8);                            // ID: 016 : RAW Result Ch6

// Makro für Lesezugriff auf Register "sp_result_raw_ch07"
ioctl_read_buf!(read_sp_result_raw_ch07, 0, 17, u8);                            // ID: 017 : RAW Result Ch7

// Makro für Lesezugriff auf Register "sp_result_raw_ch08"
ioctl_read_buf!(read_sp_result_raw_ch08, 0, 18, u8);                            // ID: 018 : RAW Result Ch8

// Makro für Lesezugriff auf Register "sp_result_raw_ch09"
ioctl_read_buf!(read_sp_result_raw_ch09, 0, 19, u8);                            // ID: 019 : RAW Result Ch9

// Makro für Lesezugriff auf Register "sp_result_raw_ch10"
ioctl_read_buf!(read_sp_result_raw_ch10, 0, 20, u8);                            // ID: 020 : RAW Result Ch10

// Makro für Lesezugriff auf Register "sp_result_raw_ch11"
ioctl_read_buf!(read_sp_result_raw_ch11, 0, 21, u8);                            // ID: 021 : RAW Result Ch11

// Makro für Lesezugriff auf Register "sp_result_filter_ch00"
ioctl_read_buf!(read_sp_result_filter_ch00, 0, 22, u8);                         // ID: 022 : Filter Result Ch0

// Makro für Lesezugriff auf Register "sp_result_filter_ch01"
ioctl_read_buf!(read_sp_result_filter_ch01, 0, 23, u8);                         // ID: 023 : Filter Result Ch1

// Makro für Lesezugriff auf Register "sp_result_filter_ch02"
ioctl_read_buf!(read_sp_result_filter_ch02, 0, 24, u8);                         // ID: 024 : Filter Result Ch2

// Makro für Lesezugriff auf Register "sp_result_filter_ch03"
ioctl_read_buf!(read_sp_result_filter_ch03, 0, 25, u8);                         // ID: 025 : Filter Result Ch3

// Makro für Lesezugriff auf Register "sp_result_filter_ch04"
ioctl_read_buf!(read_sp_result_filter_ch04, 0, 26, u8);                         // ID: 026 : Filter Result Ch4

// Makro für Lesezugriff auf Register "sp_result_filter_ch05"
ioctl_read_buf!(read_sp_result_filter_ch05, 0, 27, u8);                         // ID: 027 : Filter Result Ch5

// Makro für Lesezugriff auf Register "sp_result_filter_ch06"
ioctl_read_buf!(read_sp_result_filter_ch06, 0, 28, u8);                         // ID: 028 : Filter Result Ch6

// Makro für Lesezugriff auf Register "sp_result_filter_ch07"
ioctl_read_buf!(read_sp_result_filter_ch07, 0, 29, u8);                         // ID: 029 : Filter Result Ch7

// Makro für Lesezugriff auf Register "sp_result_filter_ch08"
ioctl_read_buf!(read_sp_result_filter_ch08, 0, 30, u8);                         // ID: 030 : Filter Result Ch8

// Makro für Lesezugriff auf Register "sp_result_filter_ch09"
ioctl_read_buf!(read_sp_result_filter_ch09, 0, 31, u8);                         // ID: 031 : Filter Result Ch9

// Makro für Lesezugriff auf Register "sp_result_filter_ch10"
ioctl_read_buf!(read_sp_result_filter_ch10, 0, 32, u8);                         // ID: 032 : Filter Result Ch10

// Makro für Lesezugriff auf Register "sp_result_filter_ch11"
ioctl_read_buf!(read_sp_result_filter_ch11, 0, 33, u8);                         // ID: 033 : Filter Result Ch11

// Makro für Lesezugriff auf Register "sp_start_offset_ch00"
ioctl_read_buf!(read_sp_start_offset_ch00, 0, 34, u8);                          // ID: 034 : Offset Start Ch0

// Makro für Lesezugriff auf Register "sp_start_offset_ch01"
ioctl_read_buf!(read_sp_start_offset_ch01, 0, 35, u8);                          // ID: 035 : Offset Start Ch1

// Makro für Lesezugriff auf Register "sp_start_offset_ch02"
ioctl_read_buf!(read_sp_start_offset_ch02, 0, 36, u8);                          // ID: 036 : Offset Start Ch2

// Makro für Lesezugriff auf Register "sp_start_offset_ch03"
ioctl_read_buf!(read_sp_start_offset_ch03, 0, 37, u8);                          // ID: 037 : Offset Start Ch3

// Makro für Lesezugriff auf Register "sp_start_offset_ch04"
ioctl_read_buf!(read_sp_start_offset_ch04, 0, 38, u8);                          // ID: 038 : Offset Start Ch4

// Makro für Lesezugriff auf Register "sp_start_offset_ch05"
ioctl_read_buf!(read_sp_start_offset_ch05, 0, 39, u8);                          // ID: 039 : Offset Start Ch5

// Makro für Lesezugriff auf Register "sp_start_offset_ch06"
ioctl_read_buf!(read_sp_start_offset_ch06, 0, 40, u8);                          // ID: 040 : Offset Start Ch6

// Makro für Lesezugriff auf Register "sp_start_offset_ch07"
ioctl_read_buf!(read_sp_start_offset_ch07, 0, 41, u8);                          // ID: 041 : Offset Start Ch7

// Makro für Lesezugriff auf Register "sp_start_offset_ch08"
ioctl_read_buf!(read_sp_start_offset_ch08, 0, 42, u8);                          // ID: 042 : Offset Start Ch8

// Makro für Lesezugriff auf Register "sp_start_offset_ch09"
ioctl_read_buf!(read_sp_start_offset_ch09, 0, 43, u8);                          // ID: 043 : Offset Start Ch9

// Makro für Lesezugriff auf Register "sp_start_offset_ch10"
ioctl_read_buf!(read_sp_start_offset_ch10, 0, 44, u8);                          // ID: 044 : Offset Start Ch10

// Makro für Lesezugriff auf Register "sp_start_offset_ch11"
ioctl_read_buf!(read_sp_start_offset_ch11, 0, 45, u8);                          // ID: 045 : Offset Start Ch11

// Makro für Lesezugriff auf Register "sp_start_min_ch00"
ioctl_read_buf!(read_sp_start_min_ch00, 0, 46, u8);                             // ID: 046 : Min Start Ch0

// Makro für Lesezugriff auf Register "sp_start_min_ch01"
ioctl_read_buf!(read_sp_start_min_ch01, 0, 47, u8);                             // ID: 047 : Min Start Ch1

// Makro für Lesezugriff auf Register "sp_start_min_ch02"
ioctl_read_buf!(read_sp_start_min_ch02, 0, 48, u8);                             // ID: 048 : Min Start Ch2

// Makro für Lesezugriff auf Register "sp_start_min_ch03"
ioctl_read_buf!(read_sp_start_min_ch03, 0, 49, u8);                             // ID: 049 : Min Start Ch3

// Makro für Lesezugriff auf Register "sp_start_min_ch04"
ioctl_read_buf!(read_sp_start_min_ch04, 0, 50, u8);                             // ID: 050 : Min Start Ch4

// Makro für Lesezugriff auf Register "sp_start_min_ch05"
ioctl_read_buf!(read_sp_start_min_ch05, 0, 51, u8);                             // ID: 051 : Min Start Ch5

// Makro für Lesezugriff auf Register "sp_start_min_ch06"
ioctl_read_buf!(read_sp_start_min_ch06, 0, 52, u8);                             // ID: 052 : Min Start Ch6

// Makro für Lesezugriff auf Register "sp_start_min_ch07"
ioctl_read_buf!(read_sp_start_min_ch07, 0, 53, u8);                             // ID: 053 : Min Start Ch7

// Makro für Lesezugriff auf Register "sp_start_min_ch08"
ioctl_read_buf!(read_sp_start_min_ch08, 0, 54, u8);                             // ID: 054 : Min Start Ch8

// Makro für Lesezugriff auf Register "sp_start_min_ch09"
ioctl_read_buf!(read_sp_start_min_ch09, 0, 55, u8);                             // ID: 055 : Min Start Ch9

// Makro für Lesezugriff auf Register "sp_start_min_ch10"
ioctl_read_buf!(read_sp_start_min_ch10, 0, 56, u8);                             // ID: 056 : Min Start Ch10

// Makro für Lesezugriff auf Register "sp_start_min_ch11"
ioctl_read_buf!(read_sp_start_min_ch11, 0, 57, u8);                             // ID: 057 : Min Start Ch11

// Makro für Lesezugriff auf Register "sp_start_max_ch00"
ioctl_read_buf!(read_sp_start_max_ch00, 0, 58, u8);                             // ID: 058 : Max Start Ch0

// Makro für Lesezugriff auf Register "sp_start_max_ch01"
ioctl_read_buf!(read_sp_start_max_ch01, 0, 59, u8);                             // ID: 059 : Max Start Ch1

// Makro für Lesezugriff auf Register "sp_start_max_ch02"
ioctl_read_buf!(read_sp_start_max_ch02, 0, 60, u8);                             // ID: 060 : Max Start Ch2

// Makro für Lesezugriff auf Register "sp_start_max_ch03"
ioctl_read_buf!(read_sp_start_max_ch03, 0, 61, u8);                             // ID: 061 : Max Start Ch3

// Makro für Lesezugriff auf Register "sp_start_max_ch04"
ioctl_read_buf!(read_sp_start_max_ch04, 0, 62, u8);                             // ID: 062 : Max Start Ch4

// Makro für Lesezugriff auf Register "sp_start_max_ch05"
ioctl_read_buf!(read_sp_start_max_ch05, 0, 63, u8);                             // ID: 063 : Max Start Ch5

// Makro für Lesezugriff auf Register "sp_start_max_ch06"
ioctl_read_buf!(read_sp_start_max_ch06, 0, 64, u8);                             // ID: 064 : Max Start Ch6

// Makro für Lesezugriff auf Register "sp_start_max_ch07"
ioctl_read_buf!(read_sp_start_max_ch07, 0, 65, u8);                             // ID: 065 : Max Start Ch7

// Makro für Lesezugriff auf Register "sp_start_max_ch08"
ioctl_read_buf!(read_sp_start_max_ch08, 0, 66, u8);                             // ID: 066 : Max Start Ch8

// Makro für Lesezugriff auf Register "sp_start_max_ch09"
ioctl_read_buf!(read_sp_start_max_ch09, 0, 67, u8);                             // ID: 067 : Max Start Ch9

// Makro für Lesezugriff auf Register "sp_start_max_ch10"
ioctl_read_buf!(read_sp_start_max_ch10, 0, 68, u8);                             // ID: 068 : Max Start Ch10

// Makro für Lesezugriff auf Register "sp_start_max_ch11"
ioctl_read_buf!(read_sp_start_max_ch11, 0, 69, u8);                             // ID: 069 : Max Start Ch11

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch00"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch00, 0, 70, u8);              // ID: 070 : Offset Measure Window Start Ch0

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch01"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch01, 0, 71, u8);              // ID: 071 : Offset Measure Window Start Ch1

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch02"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch02, 0, 72, u8);              // ID: 072 : Offset Measure Window Start Ch2

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch03"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch03, 0, 73, u8);              // ID: 073 : Offset Measure Window Start Ch3

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch04"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch04, 0, 74, u8);              // ID: 074 : Offset Measure Window Start Ch4

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch05"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch05, 0, 75, u8);              // ID: 075 : Offset Measure Window Start Ch5

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch06"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch06, 0, 76, u8);              // ID: 076 : Offset Measure Window Start Ch6

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch07"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch07, 0, 77, u8);              // ID: 077 : Offset Measure Window Start Ch7

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch08"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch08, 0, 78, u8);              // ID: 078 : Offset Measure Window Start Ch8

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch09"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch09, 0, 79, u8);              // ID: 079 : Offset Measure Window Start Ch9

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch10"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch10, 0, 80, u8);              // ID: 080 : Offset Measure Window Start Ch10

// Makro für Lesezugriff auf Register "sp_offset_meas_window_start_ch11"
ioctl_read_buf!(read_sp_offset_meas_window_start_ch11, 0, 81, u8);              // ID: 081 : Offset Measure Window Start Ch11

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch00"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch00, 0, 82, u8);                // ID: 082 : Offset Measure Window End Ch0

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch01"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch01, 0, 83, u8);                // ID: 083 : Offset Measure Window End Ch1

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch02"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch02, 0, 84, u8);                // ID: 084 : Offset Measure Window End Ch2

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch03"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch03, 0, 85, u8);                // ID: 085 : Offset Measure Window End Ch3

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch04"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch04, 0, 86, u8);                // ID: 086 : Offset Measure Window End Ch4

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch05"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch05, 0, 87, u8);                // ID: 087 : Offset Measure Window End Ch5

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch06"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch06, 0, 88, u8);                // ID: 088 : Offset Measure Window End Ch6

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch07"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch07, 0, 89, u8);                // ID: 089 : Offset Measure Window End Ch7

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch08"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch08, 0, 90, u8);                // ID: 090 : Offset Measure Window End Ch8

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch09"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch09, 0, 91, u8);                // ID: 091 : Offset Measure Window End Ch9

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch10"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch10, 0, 92, u8);                // ID: 092 : Offset Measure Window End Ch10

// Makro für Lesezugriff auf Register "sp_offset_meas_window_end_ch11"
ioctl_read_buf!(read_sp_offset_meas_window_end_ch11, 0, 93, u8);                // ID: 093 : Offset Measure Window End Ch11

// Makro für Lesezugriff auf Register "sp_result_sum_ch00"
ioctl_read_buf!(read_sp_result_sum_ch00, 0, 94, u8);                            // ID: 094 : Sum Result Ch0

// Makro für Lesezugriff auf Register "sp_result_sum_ch01"
ioctl_read_buf!(read_sp_result_sum_ch01, 0, 95, u8);                            // ID: 095 : Sum Result Ch1

// Makro für Lesezugriff auf Register "sp_result_sum_ch02"
ioctl_read_buf!(read_sp_result_sum_ch02, 0, 96, u8);                            // ID: 096 : Sum Result Ch2

// Makro für Lesezugriff auf Register "sp_result_sum_ch03"
ioctl_read_buf!(read_sp_result_sum_ch03, 0, 97, u8);                            // ID: 097 : Sum Result Ch3

// Makro für Lesezugriff auf Register "sp_result_sum_ch04"
ioctl_read_buf!(read_sp_result_sum_ch04, 0, 98, u8);                            // ID: 098 : Sum Result Ch4

// Makro für Lesezugriff auf Register "sp_result_sum_ch05"
ioctl_read_buf!(read_sp_result_sum_ch05, 0, 99, u8);                            // ID: 099 : Sum Result Ch5

// Makro für Lesezugriff auf Register "sp_result_sum_ch06"
ioctl_read_buf!(read_sp_result_sum_ch06, 0, 100, u8);                           // ID: 100 : Sum Result Ch6

// Makro für Lesezugriff auf Register "sp_result_sum_ch07"
ioctl_read_buf!(read_sp_result_sum_ch07, 0, 101, u8);                           // ID: 101 : Sum Result Ch7

// Makro für Lesezugriff auf Register "sp_result_sum_ch08"
ioctl_read_buf!(read_sp_result_sum_ch08, 0, 102, u8);                           // ID: 102 : Sum Result Ch8

// Makro für Lesezugriff auf Register "sp_result_sum_ch09"
ioctl_read_buf!(read_sp_result_sum_ch09, 0, 103, u8);                           // ID: 103 : Sum Result Ch9

// Makro für Lesezugriff auf Register "sp_result_sum_ch10"
ioctl_read_buf!(read_sp_result_sum_ch10, 0, 104, u8);                           // ID: 104 : Sum Result Ch10

// Makro für Lesezugriff auf Register "sp_result_sum_ch11"
ioctl_read_buf!(read_sp_result_sum_ch11, 0, 105, u8);                           // ID: 105 : Sum Result Ch11

// Makro für Lesezugriff auf Register "sp_result_min_ch00"
ioctl_read_buf!(read_sp_result_min_ch00, 0, 106, u8);                           // ID: 106 : Min Result Ch0

// Makro für Lesezugriff auf Register "sp_result_min_ch01"
ioctl_read_buf!(read_sp_result_min_ch01, 0, 107, u8);                           // ID: 107 : Min Result Ch1

// Makro für Lesezugriff auf Register "sp_result_min_ch02"
ioctl_read_buf!(read_sp_result_min_ch02, 0, 108, u8);                           // ID: 108 : Min Result Ch2

// Makro für Lesezugriff auf Register "sp_result_min_ch03"
ioctl_read_buf!(read_sp_result_min_ch03, 0, 109, u8);                           // ID: 109 : Min Result Ch3

// Makro für Lesezugriff auf Register "sp_result_min_ch04"
ioctl_read_buf!(read_sp_result_min_ch04, 0, 110, u8);                           // ID: 110 : Min Result Ch4

// Makro für Lesezugriff auf Register "sp_result_min_ch05"
ioctl_read_buf!(read_sp_result_min_ch05, 0, 111, u8);                           // ID: 111 : Min Result Ch5

// Makro für Lesezugriff auf Register "sp_result_min_ch06"
ioctl_read_buf!(read_sp_result_min_ch06, 0, 112, u8);                           // ID: 112 : Min Result Ch6

// Makro für Lesezugriff auf Register "sp_result_min_ch07"
ioctl_read_buf!(read_sp_result_min_ch07, 0, 113, u8);                           // ID: 113 : Min Result Ch7

// Makro für Lesezugriff auf Register "sp_result_min_ch08"
ioctl_read_buf!(read_sp_result_min_ch08, 0, 114, u8);                           // ID: 114 : Min Result Ch8

// Makro für Lesezugriff auf Register "sp_result_min_ch09"
ioctl_read_buf!(read_sp_result_min_ch09, 0, 115, u8);                           // ID: 115 : Min Result Ch9

// Makro für Lesezugriff auf Register "sp_result_min_ch10"
ioctl_read_buf!(read_sp_result_min_ch10, 0, 116, u8);                           // ID: 116 : Min Result Ch10

// Makro für Lesezugriff auf Register "sp_result_min_ch11"
ioctl_read_buf!(read_sp_result_min_ch11, 0, 117, u8);                           // ID: 117 : Min Result Ch11

// Makro für Lesezugriff auf Register "sp_result_max_ch00"
ioctl_read_buf!(read_sp_result_max_ch00, 0, 118, u8);                           // ID: 118 : Max Result Ch0

// Makro für Lesezugriff auf Register "sp_result_max_ch01"
ioctl_read_buf!(read_sp_result_max_ch01, 0, 119, u8);                           // ID: 119 : Max Result Ch1

// Makro für Lesezugriff auf Register "sp_result_max_ch02"
ioctl_read_buf!(read_sp_result_max_ch02, 0, 120, u8);                           // ID: 120 : Max Result Ch2

// Makro für Lesezugriff auf Register "sp_result_max_ch03"
ioctl_read_buf!(read_sp_result_max_ch03, 0, 121, u8);                           // ID: 121 : Max Result Ch3

// Makro für Lesezugriff auf Register "sp_result_max_ch04"
ioctl_read_buf!(read_sp_result_max_ch04, 0, 122, u8);                           // ID: 122 : Max Result Ch4

// Makro für Lesezugriff auf Register "sp_result_max_ch05"
ioctl_read_buf!(read_sp_result_max_ch05, 0, 123, u8);                           // ID: 123 : Max Result Ch5

// Makro für Lesezugriff auf Register "sp_result_max_ch06"
ioctl_read_buf!(read_sp_result_max_ch06, 0, 124, u8);                           // ID: 124 : Max Result Ch6

// Makro für Lesezugriff auf Register "sp_result_max_ch07"
ioctl_read_buf!(read_sp_result_max_ch07, 0, 125, u8);                           // ID: 125 : Max Result Ch7

// Makro für Lesezugriff auf Register "sp_result_max_ch08"
ioctl_read_buf!(read_sp_result_max_ch08, 0, 126, u8);                           // ID: 126 : Max Result Ch8

// Makro für Lesezugriff auf Register "sp_result_max_ch09"
ioctl_read_buf!(read_sp_result_max_ch09, 0, 127, u8);                           // ID: 127 : Max Result Ch9

// Makro für Lesezugriff auf Register "sp_result_max_ch10"
ioctl_read_buf!(read_sp_result_max_ch10, 0, 128, u8);                           // ID: 128 : Max Result Ch10

// Makro für Lesezugriff auf Register "sp_result_max_ch11"
ioctl_read_buf!(read_sp_result_max_ch11, 0, 129, u8);                           // ID: 129 : Max Result Ch11

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch00"
ioctl_read_buf!(read_sp_result_pos_max_ch00, 0, 130, u8);                       // ID: 130 : Max Result Pos Ch0

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch01"
ioctl_read_buf!(read_sp_result_pos_max_ch01, 0, 131, u8);                       // ID: 131 : Max Result Pos Ch1

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch02"
ioctl_read_buf!(read_sp_result_pos_max_ch02, 0, 132, u8);                       // ID: 132 : Max Result Pos Ch2

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch03"
ioctl_read_buf!(read_sp_result_pos_max_ch03, 0, 133, u8);                       // ID: 133 : Max Result Pos Ch3

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch04"
ioctl_read_buf!(read_sp_result_pos_max_ch04, 0, 134, u8);                       // ID: 134 : Max Result Pos Ch4

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch05"
ioctl_read_buf!(read_sp_result_pos_max_ch05, 0, 135, u8);                       // ID: 135 : Max Result Pos Ch5

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch06"
ioctl_read_buf!(read_sp_result_pos_max_ch06, 0, 136, u8);                       // ID: 136 : Max Result Pos Ch6

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch07"
ioctl_read_buf!(read_sp_result_pos_max_ch07, 0, 137, u8);                       // ID: 137 : Max Result Pos Ch7

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch08"
ioctl_read_buf!(read_sp_result_pos_max_ch08, 0, 138, u8);                       // ID: 138 : Max Result Pos Ch8

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch09"
ioctl_read_buf!(read_sp_result_pos_max_ch09, 0, 139, u8);                       // ID: 139 : Max Result Pos Ch9

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch10"
ioctl_read_buf!(read_sp_result_pos_max_ch10, 0, 140, u8);                       // ID: 140 : Max Result Pos Ch10

// Makro für Lesezugriff auf Register "sp_result_pos_max_ch11"
ioctl_read_buf!(read_sp_result_pos_max_ch11, 0, 141, u8);                       // ID: 141 : Max Result Pos Ch11

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch00"
ioctl_read_buf!(read_sp_result_1st_crossing_ch00, 0, 142, u8);                  // ID: 142 : First Crossing P0006 Ch0

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch01"
ioctl_read_buf!(read_sp_result_1st_crossing_ch01, 0, 143, u8);                  // ID: 143 : First Crossing P0006 Ch1

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch02"
ioctl_read_buf!(read_sp_result_1st_crossing_ch02, 0, 144, u8);                  // ID: 144 : First Crossing P0006 Ch2

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch03"
ioctl_read_buf!(read_sp_result_1st_crossing_ch03, 0, 145, u8);                  // ID: 145 : First Crossing P0006 Ch3

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch04"
ioctl_read_buf!(read_sp_result_1st_crossing_ch04, 0, 146, u8);                  // ID: 146 : First Crossing P0006 Ch4

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch05"
ioctl_read_buf!(read_sp_result_1st_crossing_ch05, 0, 147, u8);                  // ID: 147 : First Crossing P0006 Ch5

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch06"
ioctl_read_buf!(read_sp_result_1st_crossing_ch06, 0, 148, u8);                  // ID: 148 : First Crossing P0006 Ch6

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch07"
ioctl_read_buf!(read_sp_result_1st_crossing_ch07, 0, 149, u8);                  // ID: 149 : First Crossing P0006 Ch7

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch08"
ioctl_read_buf!(read_sp_result_1st_crossing_ch08, 0, 150, u8);                  // ID: 150 : First Crossing P0006 Ch8

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch09"
ioctl_read_buf!(read_sp_result_1st_crossing_ch09, 0, 151, u8);                  // ID: 151 : First Crossing P0006 Ch9

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch10"
ioctl_read_buf!(read_sp_result_1st_crossing_ch10, 0, 152, u8);                  // ID: 152 : First Crossing P0006 Ch10

// Makro für Lesezugriff auf Register "sp_result_1st_crossing_ch11"
ioctl_read_buf!(read_sp_result_1st_crossing_ch11, 0, 153, u8);                  // ID: 153 : First Crossing P0006 Ch11

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch00"
ioctl_read_buf!(read_sp_result_last_crossing_ch00, 0, 154, u8);                 // ID: 154 : Last Crossing P0006 Ch0

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch01"
ioctl_read_buf!(read_sp_result_last_crossing_ch01, 0, 155, u8);                 // ID: 155 : Last Crossing P0006 Ch1

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch02"
ioctl_read_buf!(read_sp_result_last_crossing_ch02, 0, 156, u8);                 // ID: 156 : Last Crossing P0006 Ch2

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch03"
ioctl_read_buf!(read_sp_result_last_crossing_ch03, 0, 157, u8);                 // ID: 157 : Last Crossing P0006 Ch3

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch04"
ioctl_read_buf!(read_sp_result_last_crossing_ch04, 0, 158, u8);                 // ID: 158 : Last Crossing P0006 Ch4

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch05"
ioctl_read_buf!(read_sp_result_last_crossing_ch05, 0, 159, u8);                 // ID: 159 : Last Crossing P0006 Ch5

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch06"
ioctl_read_buf!(read_sp_result_last_crossing_ch06, 0, 160, u8);                 // ID: 160 : Last Crossing P0006 Ch6

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch07"
ioctl_read_buf!(read_sp_result_last_crossing_ch07, 0, 161, u8);                 // ID: 161 : Last Crossing P0006 Ch7

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch08"
ioctl_read_buf!(read_sp_result_last_crossing_ch08, 0, 162, u8);                 // ID: 162 : Last Crossing P0006 Ch8

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch09"
ioctl_read_buf!(read_sp_result_last_crossing_ch09, 0, 163, u8);                 // ID: 163 : Last Crossing P0006 Ch9

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch10"
ioctl_read_buf!(read_sp_result_last_crossing_ch10, 0, 164, u8);                 // ID: 164 : Last Crossing P0006 Ch10

// Makro für Lesezugriff auf Register "sp_result_last_crossing_ch11"
ioctl_read_buf!(read_sp_result_last_crossing_ch11, 0, 165, u8);                 // ID: 165 : Last Crossing P0006 Ch11

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch00"
ioctl_read_buf!(read_sp_result_reversal_pt_ch00, 0, 166, u8);                   // ID: 166 : Reversal Points Ch0

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch01"
ioctl_read_buf!(read_sp_result_reversal_pt_ch01, 0, 167, u8);                   // ID: 167 : Reversal Points Ch1

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch02"
ioctl_read_buf!(read_sp_result_reversal_pt_ch02, 0, 168, u8);                   // ID: 168 : Reversal Points Ch2

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch03"
ioctl_read_buf!(read_sp_result_reversal_pt_ch03, 0, 169, u8);                   // ID: 169 : Reversal Points Ch3

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch04"
ioctl_read_buf!(read_sp_result_reversal_pt_ch04, 0, 170, u8);                   // ID: 170 : Reversal Points Ch4

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch05"
ioctl_read_buf!(read_sp_result_reversal_pt_ch05, 0, 171, u8);                   // ID: 171 : Reversal Points Ch5

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch06"
ioctl_read_buf!(read_sp_result_reversal_pt_ch06, 0, 172, u8);                   // ID: 172 : Reversal Points Ch6

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch07"
ioctl_read_buf!(read_sp_result_reversal_pt_ch07, 0, 173, u8);                   // ID: 173 : Reversal Points Ch7

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch08"
ioctl_read_buf!(read_sp_result_reversal_pt_ch08, 0, 174, u8);                   // ID: 174 : Reversal Points Ch8

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch09"
ioctl_read_buf!(read_sp_result_reversal_pt_ch09, 0, 175, u8);                   // ID: 175 : Reversal Points Ch9

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch10"
ioctl_read_buf!(read_sp_result_reversal_pt_ch10, 0, 176, u8);                   // ID: 176 : Reversal Points Ch10

// Makro für Lesezugriff auf Register "sp_result_reversal_pt_ch11"
ioctl_read_buf!(read_sp_result_reversal_pt_ch11, 0, 177, u8);                   // ID: 177 : Reversal Points Ch11

// Makro für Lesezugriff auf Register "offset_dac_value_ch00"
ioctl_read_buf!(read_offset_dac_value_ch00, 0, 178, u8);                        // ID: 178 : DAC Value Offset Ch0

// Makro für Lesezugriff auf Register "offset_dac_value_ch01"
ioctl_read_buf!(read_offset_dac_value_ch01, 0, 179, u8);                        // ID: 179 : DAC Value Offset Ch1

// Makro für Lesezugriff auf Register "offset_dac_value_ch02"
ioctl_read_buf!(read_offset_dac_value_ch02, 0, 180, u8);                        // ID: 180 : DAC Value Offset Ch2

// Makro für Lesezugriff auf Register "offset_dac_value_ch03"
ioctl_read_buf!(read_offset_dac_value_ch03, 0, 181, u8);                        // ID: 181 : DAC Value Offset Ch3

// Makro für Lesezugriff auf Register "offset_dac_value_ch04"
ioctl_read_buf!(read_offset_dac_value_ch04, 0, 182, u8);                        // ID: 182 : DAC Value Offset Ch4

// Makro für Lesezugriff auf Register "offset_dac_value_ch05"
ioctl_read_buf!(read_offset_dac_value_ch05, 0, 183, u8);                        // ID: 183 : DAC Value Offset Ch5

// Makro für Lesezugriff auf Register "offset_dac_value_ch06"
ioctl_read_buf!(read_offset_dac_value_ch06, 0, 184, u8);                        // ID: 184 : DAC Value Offset Ch6

// Makro für Lesezugriff auf Register "offset_dac_value_ch07"
ioctl_read_buf!(read_offset_dac_value_ch07, 0, 185, u8);                        // ID: 185 : DAC Value Offset Ch7

// Makro für Lesezugriff auf Register "offset_dac_value_ch08"
ioctl_read_buf!(read_offset_dac_value_ch08, 0, 186, u8);                        // ID: 186 : DAC Value Offset Ch8

// Makro für Lesezugriff auf Register "offset_dac_value_ch09"
ioctl_read_buf!(read_offset_dac_value_ch09, 0, 187, u8);                        // ID: 187 : DAC Value Offset Ch9

// Makro für Lesezugriff auf Register "offset_dac_value_ch10"
ioctl_read_buf!(read_offset_dac_value_ch10, 0, 188, u8);                        // ID: 188 : DAC Value Offset Ch10

// Makro für Lesezugriff auf Register "offset_dac_value_ch11"
ioctl_read_buf!(read_offset_dac_value_ch11, 0, 189, u8);                        // ID: 189 : DAC Value Offset Ch11

// Makro für Lesezugriff auf Register "trigger_duration"
ioctl_read_buf!(read_trigger_duration, 0, 190, u8);                             // ID: 190 : Trigger Duration

// Makro für Lesezugriff auf Register "vibration"
ioctl_read_buf!(read_vibration, 0, 191, u8);                                    // ID: 191 : Vibration

// Makro für Lesezugriff auf Register "time_current"
ioctl_read_buf!(read_time_current, 0, 192, u8);                                 // ID: 192 : Current Time

// Makro für Lesezugriff auf Register "time_trigger_rising_edge"
ioctl_read_buf!(read_time_trigger_rising_edge, 0, 193, u8);                     // ID: 193 : Trigger Rising Edge Time

// Makro für Schreibzugriff auf Register "time_current_set_value"
ioctl_write_buf!(write_time_current_set_value, 0, 194, u8);                     // ID: 194 : Setvalue Current Time

// Makro für Lesezugriff auf Register "time_current_set_value"
ioctl_read_buf!(read_time_current_set_value, 0, 194, u8);                       // ID: 194 : Setvalue Current Time

// Makro für Schreibzugriff auf Register "dac_value_ch00_lsb"
ioctl_write_buf!(write_dac_value_ch00_lsb, 0, 195, u8);                         // ID: 195 : DAC-Value Ch0 LSB

// Makro für Lesezugriff auf Register "dac_value_ch00_lsb"
ioctl_read_buf!(read_dac_value_ch00_lsb, 0, 195, u8);                           // ID: 195 : DAC-Value Ch0 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch00_msb"
ioctl_write_buf!(write_dac_value_ch00_msb, 0, 196, u8);                         // ID: 196 : DAC-Value Ch0 MSB

// Makro für Lesezugriff auf Register "dac_value_ch00_msb"
ioctl_read_buf!(read_dac_value_ch00_msb, 0, 196, u8);                           // ID: 196 : DAC-Value Ch0 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch01_lsb"
ioctl_write_buf!(write_dac_value_ch01_lsb, 0, 197, u8);                         // ID: 197 : DAC-Value Ch1 LSB

// Makro für Lesezugriff auf Register "dac_value_ch01_lsb"
ioctl_read_buf!(read_dac_value_ch01_lsb, 0, 197, u8);                           // ID: 197 : DAC-Value Ch1 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch01_msb"
ioctl_write_buf!(write_dac_value_ch01_msb, 0, 198, u8);                         // ID: 198 : DAC-Value Ch1 MSB

// Makro für Lesezugriff auf Register "dac_value_ch01_msb"
ioctl_read_buf!(read_dac_value_ch01_msb, 0, 198, u8);                           // ID: 198 : DAC-Value Ch1 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch02_lsb"
ioctl_write_buf!(write_dac_value_ch02_lsb, 0, 199, u8);                         // ID: 199 : DAC-Value Ch2 LSB

// Makro für Lesezugriff auf Register "dac_value_ch02_lsb"
ioctl_read_buf!(read_dac_value_ch02_lsb, 0, 199, u8);                           // ID: 199 : DAC-Value Ch2 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch02_msb"
ioctl_write_buf!(write_dac_value_ch02_msb, 0, 200, u8);                         // ID: 200 : DAC-Value Ch2 MSB

// Makro für Lesezugriff auf Register "dac_value_ch02_msb"
ioctl_read_buf!(read_dac_value_ch02_msb, 0, 200, u8);                           // ID: 200 : DAC-Value Ch2 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch03_lsb"
ioctl_write_buf!(write_dac_value_ch03_lsb, 0, 201, u8);                         // ID: 201 : DAC-Value Ch3 LSB

// Makro für Lesezugriff auf Register "dac_value_ch03_lsb"
ioctl_read_buf!(read_dac_value_ch03_lsb, 0, 201, u8);                           // ID: 201 : DAC-Value Ch3 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch03_msb"
ioctl_write_buf!(write_dac_value_ch03_msb, 0, 202, u8);                         // ID: 202 : DAC-Value Ch3 MSB

// Makro für Lesezugriff auf Register "dac_value_ch03_msb"
ioctl_read_buf!(read_dac_value_ch03_msb, 0, 202, u8);                           // ID: 202 : DAC-Value Ch3 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch04_lsb"
ioctl_write_buf!(write_dac_value_ch04_lsb, 0, 203, u8);                         // ID: 203 : DAC-Value Ch4 LSB

// Makro für Lesezugriff auf Register "dac_value_ch04_lsb"
ioctl_read_buf!(read_dac_value_ch04_lsb, 0, 203, u8);                           // ID: 203 : DAC-Value Ch4 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch04_msb"
ioctl_write_buf!(write_dac_value_ch04_msb, 0, 204, u8);                         // ID: 204 : DAC-Value Ch4 MSB

// Makro für Lesezugriff auf Register "dac_value_ch04_msb"
ioctl_read_buf!(read_dac_value_ch04_msb, 0, 204, u8);                           // ID: 204 : DAC-Value Ch4 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch05_lsb"
ioctl_write_buf!(write_dac_value_ch05_lsb, 0, 205, u8);                         // ID: 205 : DAC-Value Ch5 LSB

// Makro für Lesezugriff auf Register "dac_value_ch05_lsb"
ioctl_read_buf!(read_dac_value_ch05_lsb, 0, 205, u8);                           // ID: 205 : DAC-Value Ch5 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch05_msb"
ioctl_write_buf!(write_dac_value_ch05_msb, 0, 206, u8);                         // ID: 206 : DAC-Value Ch5 MSB

// Makro für Lesezugriff auf Register "dac_value_ch05_msb"
ioctl_read_buf!(read_dac_value_ch05_msb, 0, 206, u8);                           // ID: 206 : DAC-Value Ch5 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch06_lsb"
ioctl_write_buf!(write_dac_value_ch06_lsb, 0, 207, u8);                         // ID: 207 : DAC-Value Ch6 LSB

// Makro für Lesezugriff auf Register "dac_value_ch06_lsb"
ioctl_read_buf!(read_dac_value_ch06_lsb, 0, 207, u8);                           // ID: 207 : DAC-Value Ch6 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch06_msb"
ioctl_write_buf!(write_dac_value_ch06_msb, 0, 208, u8);                         // ID: 208 : DAC-Value Ch6 MSB

// Makro für Lesezugriff auf Register "dac_value_ch06_msb"
ioctl_read_buf!(read_dac_value_ch06_msb, 0, 208, u8);                           // ID: 208 : DAC-Value Ch6 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch07_lsb"
ioctl_write_buf!(write_dac_value_ch07_lsb, 0, 209, u8);                         // ID: 209 : DAC-Value Ch7 LSB

// Makro für Lesezugriff auf Register "dac_value_ch07_lsb"
ioctl_read_buf!(read_dac_value_ch07_lsb, 0, 209, u8);                           // ID: 209 : DAC-Value Ch7 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch07_msb"
ioctl_write_buf!(write_dac_value_ch07_msb, 0, 210, u8);                         // ID: 210 : DAC-Value Ch7 MSB

// Makro für Lesezugriff auf Register "dac_value_ch07_msb"
ioctl_read_buf!(read_dac_value_ch07_msb, 0, 210, u8);                           // ID: 210 : DAC-Value Ch7 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch08_lsb"
ioctl_write_buf!(write_dac_value_ch08_lsb, 0, 211, u8);                         // ID: 211 : DAC-Value Ch8 LSB

// Makro für Lesezugriff auf Register "dac_value_ch08_lsb"
ioctl_read_buf!(read_dac_value_ch08_lsb, 0, 211, u8);                           // ID: 211 : DAC-Value Ch8 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch08_msb"
ioctl_write_buf!(write_dac_value_ch08_msb, 0, 212, u8);                         // ID: 212 : DAC-Value Ch8 MSB

// Makro für Lesezugriff auf Register "dac_value_ch08_msb"
ioctl_read_buf!(read_dac_value_ch08_msb, 0, 212, u8);                           // ID: 212 : DAC-Value Ch8 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch09_lsb"
ioctl_write_buf!(write_dac_value_ch09_lsb, 0, 213, u8);                         // ID: 213 : DAC-Value Ch9 LSB

// Makro für Lesezugriff auf Register "dac_value_ch09_lsb"
ioctl_read_buf!(read_dac_value_ch09_lsb, 0, 213, u8);                           // ID: 213 : DAC-Value Ch9 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch09_msb"
ioctl_write_buf!(write_dac_value_ch09_msb, 0, 214, u8);                         // ID: 214 : DAC-Value Ch9 MSB

// Makro für Lesezugriff auf Register "dac_value_ch09_msb"
ioctl_read_buf!(read_dac_value_ch09_msb, 0, 214, u8);                           // ID: 214 : DAC-Value Ch9 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch10_lsb"
ioctl_write_buf!(write_dac_value_ch10_lsb, 0, 215, u8);                         // ID: 215 : DAC-Value Ch10 LSB

// Makro für Lesezugriff auf Register "dac_value_ch10_lsb"
ioctl_read_buf!(read_dac_value_ch10_lsb, 0, 215, u8);                           // ID: 215 : DAC-Value Ch10 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch10_msb"
ioctl_write_buf!(write_dac_value_ch10_msb, 0, 216, u8);                         // ID: 216 : DAC-Value Ch10 MSB

// Makro für Lesezugriff auf Register "dac_value_ch10_msb"
ioctl_read_buf!(read_dac_value_ch10_msb, 0, 216, u8);                           // ID: 216 : DAC-Value Ch10 MSB

// Makro für Schreibzugriff auf Register "dac_value_ch11_lsb"
ioctl_write_buf!(write_dac_value_ch11_lsb, 0, 217, u8);                         // ID: 217 : DAC-Value Ch11 LSB

// Makro für Lesezugriff auf Register "dac_value_ch11_lsb"
ioctl_read_buf!(read_dac_value_ch11_lsb, 0, 217, u8);                           // ID: 217 : DAC-Value Ch11 LSB

// Makro für Schreibzugriff auf Register "dac_value_ch11_msb"
ioctl_write_buf!(write_dac_value_ch11_msb, 0, 218, u8);                         // ID: 218 : DAC-Value Ch11 MSB

// Makro für Lesezugriff auf Register "dac_value_ch11_msb"
ioctl_read_buf!(read_dac_value_ch11_msb, 0, 218, u8);                           // ID: 218 : DAC-Value Ch11 MSB

// Makro für Schreibzugriff auf Register "sp_gain_ch00"
ioctl_write_buf!(write_sp_gain_ch00, 0, 219, u8);                               // ID: 219 : Verstärkung Ch0

// Makro für Lesezugriff auf Register "sp_gain_ch00"
ioctl_read_buf!(read_sp_gain_ch00, 0, 219, u8);                                 // ID: 219 : Verstärkung Ch0

// Makro für Schreibzugriff auf Register "sp_gain_ch01"
ioctl_write_buf!(write_sp_gain_ch01, 0, 220, u8);                               // ID: 220 : Verstärkung Ch1

// Makro für Lesezugriff auf Register "sp_gain_ch01"
ioctl_read_buf!(read_sp_gain_ch01, 0, 220, u8);                                 // ID: 220 : Verstärkung Ch1

// Makro für Schreibzugriff auf Register "sp_gain_ch02"
ioctl_write_buf!(write_sp_gain_ch02, 0, 221, u8);                               // ID: 221 : Verstärkung Ch2

// Makro für Lesezugriff auf Register "sp_gain_ch02"
ioctl_read_buf!(read_sp_gain_ch02, 0, 221, u8);                                 // ID: 221 : Verstärkung Ch2

// Makro für Schreibzugriff auf Register "sp_gain_ch03"
ioctl_write_buf!(write_sp_gain_ch03, 0, 222, u8);                               // ID: 222 : Verstärkung Ch3

// Makro für Lesezugriff auf Register "sp_gain_ch03"
ioctl_read_buf!(read_sp_gain_ch03, 0, 222, u8);                                 // ID: 222 : Verstärkung Ch3

// Makro für Schreibzugriff auf Register "sp_gain_ch04"
ioctl_write_buf!(write_sp_gain_ch04, 0, 223, u8);                               // ID: 223 : Verstärkung Ch4

// Makro für Lesezugriff auf Register "sp_gain_ch04"
ioctl_read_buf!(read_sp_gain_ch04, 0, 223, u8);                                 // ID: 223 : Verstärkung Ch4

// Makro für Schreibzugriff auf Register "sp_gain_ch05"
ioctl_write_buf!(write_sp_gain_ch05, 0, 224, u8);                               // ID: 224 : Verstärkung Ch5

// Makro für Lesezugriff auf Register "sp_gain_ch05"
ioctl_read_buf!(read_sp_gain_ch05, 0, 224, u8);                                 // ID: 224 : Verstärkung Ch5

// Makro für Schreibzugriff auf Register "sp_gain_ch06"
ioctl_write_buf!(write_sp_gain_ch06, 0, 225, u8);                               // ID: 225 : Verstärkung Ch6

// Makro für Lesezugriff auf Register "sp_gain_ch06"
ioctl_read_buf!(read_sp_gain_ch06, 0, 225, u8);                                 // ID: 225 : Verstärkung Ch6

// Makro für Schreibzugriff auf Register "sp_gain_ch07"
ioctl_write_buf!(write_sp_gain_ch07, 0, 226, u8);                               // ID: 226 : Verstärkung Ch7

// Makro für Lesezugriff auf Register "sp_gain_ch07"
ioctl_read_buf!(read_sp_gain_ch07, 0, 226, u8);                                 // ID: 226 : Verstärkung Ch7

// Makro für Schreibzugriff auf Register "sp_gain_ch08"
ioctl_write_buf!(write_sp_gain_ch08, 0, 227, u8);                               // ID: 227 : Verstärkung Ch8

// Makro für Lesezugriff auf Register "sp_gain_ch08"
ioctl_read_buf!(read_sp_gain_ch08, 0, 227, u8);                                 // ID: 227 : Verstärkung Ch8

// Makro für Schreibzugriff auf Register "sp_gain_ch09"
ioctl_write_buf!(write_sp_gain_ch09, 0, 228, u8);                               // ID: 228 : Verstärkung Ch9

// Makro für Lesezugriff auf Register "sp_gain_ch09"
ioctl_read_buf!(read_sp_gain_ch09, 0, 228, u8);                                 // ID: 228 : Verstärkung Ch9

// Makro für Schreibzugriff auf Register "sp_gain_ch10"
ioctl_write_buf!(write_sp_gain_ch10, 0, 229, u8);                               // ID: 229 : Verstärkung Ch10

// Makro für Lesezugriff auf Register "sp_gain_ch10"
ioctl_read_buf!(read_sp_gain_ch10, 0, 229, u8);                                 // ID: 229 : Verstärkung Ch10

// Makro für Schreibzugriff auf Register "sp_gain_ch11"
ioctl_write_buf!(write_sp_gain_ch11, 0, 230, u8);                               // ID: 230 : Verstärkung Ch11

// Makro für Lesezugriff auf Register "sp_gain_ch11"
ioctl_read_buf!(read_sp_gain_ch11, 0, 230, u8);                                 // ID: 230 : Verstärkung Ch11

// Makro für Schreibzugriff auf Register "osc_period_0"
ioctl_write_buf!(write_osc_period_0, 0, 231, u8);                               // ID: 231 : Oszillator Periodendauer 0

// Makro für Lesezugriff auf Register "osc_period_0"
ioctl_read_buf!(read_osc_period_0, 0, 231, u8);                                 // ID: 231 : Oszillator Periodendauer 0

// Makro für Schreibzugriff auf Register "config_trigger"
ioctl_write_buf!(write_config_trigger, 0, 232, u8);                             // ID: 232 : Konfiguration des Triggers

// Makro für Lesezugriff auf Register "config_trigger"
ioctl_read_buf!(read_config_trigger, 0, 232, u8);                               // ID: 232 : Konfiguration des Triggers

// Makro für Schreibzugriff auf Register "tx_pow_p0000"
ioctl_write_buf!(write_tx_pow_p0000, 0, 233, u8);                               // ID: 233 : Sendeleistung P000

// Makro für Lesezugriff auf Register "tx_pow_p0000"
ioctl_read_buf!(read_tx_pow_p0000, 0, 233, u8);                                 // ID: 233 : Sendeleistung P000

// Makro für Schreibzugriff auf Register "level_p0001"
ioctl_write_buf!(write_level_p0001, 0, 234, u8);                                // ID: 234 : Schwelle Integral- und Peakbildung P0001

// Makro für Lesezugriff auf Register "level_p0001"
ioctl_read_buf!(read_level_p0001, 0, 234, u8);                                  // ID: 234 : Schwelle Integral- und Peakbildung P0001

// Makro für Schreibzugriff auf Register "sample_avg_mean_slope_zero_p0002"
ioctl_write_buf!(write_sample_avg_mean_slope_zero_p0002, 0, 235, u8);           // ID: 235 : Sample für Mittelung schräge Nulllinie P0002

// Makro für Lesezugriff auf Register "sample_avg_mean_slope_zero_p0002"
ioctl_read_buf!(read_sample_avg_mean_slope_zero_p0002, 0, 235, u8);             // ID: 235 : Sample für Mittelung schräge Nulllinie P0002

// Makro für Schreibzugriff auf Register "sample_avg_mean_offset_p0003"
ioctl_write_buf!(write_sample_avg_mean_offset_p0003, 0, 236, u8);               // ID: 236 : Sample für Offsetmittelung P0003

// Makro für Lesezugriff auf Register "sample_avg_mean_offset_p0003"
ioctl_read_buf!(read_sample_avg_mean_offset_p0003, 0, 236, u8);                 // ID: 236 : Sample für Offsetmittelung P0003

// Makro für Schreibzugriff auf Register "monitoring_measure_window_p0004"
ioctl_write_buf!(write_monitoring_measure_window_p0004, 0, 237, u8);            // ID: 237 : Überwachung Messfenster P0004

// Makro für Lesezugriff auf Register "monitoring_measure_window_p0004"
ioctl_read_buf!(read_monitoring_measure_window_p0004, 0, 237, u8);              // ID: 237 : Überwachung Messfenster P0004

// Makro für Schreibzugriff auf Register "monitoring_measure_window_level_p0005"
ioctl_write_buf!(write_monitoring_measure_window_level_p0005, 0, 238, u8);      // ID: 238 : Schwellwert Überwachung Messfenster P0005

// Makro für Lesezugriff auf Register "monitoring_measure_window_level_p0005"
ioctl_read_buf!(read_monitoring_measure_window_level_p0005, 0, 238, u8);        // ID: 238 : Schwellwert Überwachung Messfenster P0005

// Makro für Schreibzugriff auf Register "level_raw_values_p0006"
ioctl_write_buf!(write_level_raw_values_p0006, 0, 239, u8);                     // ID: 239 : Schwellwert Rohdaten P0006

// Makro für Lesezugriff auf Register "level_raw_values_p0006"
ioctl_read_buf!(read_level_raw_values_p0006, 0, 239, u8);                       // ID: 239 : Schwellwert Rohdaten P0006

// Makro für Schreibzugriff auf Register "offset_alignment_timeout_p0007"
ioctl_write_buf!(write_offset_alignment_timeout_p0007, 0, 240, u8);             // ID: 240 : Timeout Offsetabgleich P0007

// Makro für Lesezugriff auf Register "offset_alignment_timeout_p0007"
ioctl_read_buf!(read_offset_alignment_timeout_p0007, 0, 240, u8);               // ID: 240 : Timeout Offsetabgleich P0007

// Makro für Schreibzugriff auf Register "offset_alignment_duration_p0008"
ioctl_write_buf!(write_offset_alignment_duration_p0008, 0, 241, u8);            // ID: 241 : Dauer Offsetanpassung P0008

// Makro für Lesezugriff auf Register "offset_alignment_duration_p0008"
ioctl_read_buf!(read_offset_alignment_duration_p0008, 0, 241, u8);              // ID: 241 : Dauer Offsetanpassung P0008

// Makro für Schreibzugriff auf Register "type_mass_det_p0009"
ioctl_write_buf!(write_type_mass_det_p0009, 0, 242, u8);                        // ID: 242 : Art der Masseberechnung P0009

// Makro für Lesezugriff auf Register "type_mass_det_p0009"
ioctl_read_buf!(read_type_mass_det_p0009, 0, 242, u8);                          // ID: 242 : Art der Masseberechnung P0009

// Makro für Schreibzugriff auf Register "type_slope_zero_p000a"
ioctl_write_buf!(write_type_slope_zero_p000a, 0, 243, u8);                      // ID: 243 : Art Berechnung schräge Nulllinie P000A

// Makro für Lesezugriff auf Register "type_slope_zero_p000a"
ioctl_read_buf!(read_type_slope_zero_p000a, 0, 243, u8);                        // ID: 243 : Art Berechnung schräge Nulllinie P000A

// Makro für Schreibzugriff auf Register "slope_zero_level_1_p000b"
ioctl_write_buf!(write_slope_zero_level_1_p000b, 0, 244, u8);                   // ID: 244 : schräge Nulllinie Schwellwert1 P000B

// Makro für Lesezugriff auf Register "slope_zero_level_1_p000b"
ioctl_read_buf!(read_slope_zero_level_1_p000b, 0, 244, u8);                     // ID: 244 : schräge Nulllinie Schwellwert1 P000B

// Makro für Schreibzugriff auf Register "slope_zero_level_2_p000c"
ioctl_write_buf!(write_slope_zero_level_2_p000c, 0, 245, u8);                   // ID: 245 : schräge Nulllinie Schwellwert2 P000C

// Makro für Lesezugriff auf Register "slope_zero_level_2_p000c"
ioctl_read_buf!(read_slope_zero_level_2_p000c, 0, 245, u8);                     // ID: 245 : schräge Nulllinie Schwellwert2 P000C

// Makro für Schreibzugriff auf Register "slope_zero_t1_p000d"
ioctl_write_buf!(write_slope_zero_t1_p000d, 0, 246, u8);                        // ID: 246 : schräge Nulllinie t1 P000D

// Makro für Lesezugriff auf Register "slope_zero_t1_p000d"
ioctl_read_buf!(read_slope_zero_t1_p000d, 0, 246, u8);                          // ID: 246 : schräge Nulllinie t1 P000D

// Makro für Schreibzugriff auf Register "slope_zero_t2_p000e"
ioctl_write_buf!(write_slope_zero_t2_p000e, 0, 247, u8);                        // ID: 247 : schräge Nulllinie t2 P000E

// Makro für Lesezugriff auf Register "slope_zero_t2_p000e"
ioctl_read_buf!(read_slope_zero_t2_p000e, 0, 247, u8);                          // ID: 247 : schräge Nulllinie t2 P000E

// Makro für Schreibzugriff auf Register "offset_max_amplitude_p000f"
ioctl_write_buf!(write_offset_max_amplitude_p000f, 0, 248, u8);                 // ID: 248 : max. Amplitude Offset P000F

// Makro für Lesezugriff auf Register "offset_max_amplitude_p000f"
ioctl_read_buf!(read_offset_max_amplitude_p000f, 0, 248, u8);                   // ID: 248 : max. Amplitude Offset P000F

// Makro für Schreibzugriff auf Register "peak_neg_max_p0010"
ioctl_write_buf!(write_peak_neg_max_p0010, 0, 249, u8);                         // ID: 249 : max. negativer Peak P0010

// Makro für Lesezugriff auf Register "peak_neg_max_p0010"
ioctl_read_buf!(read_peak_neg_max_p0010, 0, 249, u8);                           // ID: 249 : max. negativer Peak P0010

// Makro für Schreibzugriff auf Register "reversals_max_number_p0011"
ioctl_write_buf!(write_reversals_max_number_p0011, 0, 250, u8);                 // ID: 250 : max. Anzahl Umkehrpunkte P0011

// Makro für Lesezugriff auf Register "reversals_max_number_p0011"
ioctl_read_buf!(read_reversals_max_number_p0011, 0, 250, u8);                   // ID: 250 : max. Anzahl Umkehrpunkte P0011

// Makro für Schreibzugriff auf Register "reversals_level_p0012"
ioctl_write_buf!(write_reversals_level_p0012, 0, 251, u8);                      // ID: 251 : Schwelle für Umkehrpunkte P0012

// Makro für Lesezugriff auf Register "reversals_level_p0012"
ioctl_read_buf!(read_reversals_level_p0012, 0, 251, u8);                        // ID: 251 : Schwelle für Umkehrpunkte P0012

// Makro für Schreibzugriff auf Register "offset_treatment_type_p0013"
ioctl_write_buf!(write_offset_treatment_type_p0013, 0, 252, u8);                // ID: 252 : Art Offsetbehandlung P0013

// Makro für Lesezugriff auf Register "offset_treatment_type_p0013"
ioctl_read_buf!(read_offset_treatment_type_p0013, 0, 252, u8);                  // ID: 252 : Art Offsetbehandlung P0013

// Makro für Schreibzugriff auf Register "sample_avg_mean_raw_p0014"
ioctl_write_buf!(write_sample_avg_mean_raw_p0014, 0, 253, u8);                  // ID: 253 : Sample für gleitendes Mittel Rohdaten P0014

// Makro für Lesezugriff auf Register "sample_avg_mean_raw_p0014"
ioctl_read_buf!(read_sample_avg_mean_raw_p0014, 0, 253, u8);                    // ID: 253 : Sample für gleitendes Mittel Rohdaten P0014

// Makro für Schreibzugriff auf Register "sample_offset_det_p0015"
ioctl_write_buf!(write_sample_offset_det_p0015, 0, 254, u8);                    // ID: 254 : Sample Offsetermittlung P0015

// Makro für Lesezugriff auf Register "sample_offset_det_p0015"
ioctl_read_buf!(read_sample_offset_det_p0015, 0, 254, u8);                      // ID: 254 : Sample Offsetermittlung P0015

// Makro für Schreibzugriff auf Register "refvalue_offset_p0016"
ioctl_write_buf!(write_refvalue_offset_p0016, 0, 255, u8);                      // ID: 255 : Sollwert für den Offsetwert P0016

// Makro für Lesezugriff auf Register "refvalue_offset_p0016"
ioctl_read_buf!(read_refvalue_offset_p0016, 0, 255, u8);                        // ID: 255 : Sollwert für den Offsetwert P0016

// Makro für Schreibzugriff auf Register "meas_window_size_p0017"
ioctl_write_buf!(write_meas_window_size_p0017, 1, 0, u8);                       // ID: 256 : Triggerdauer bei Softtrigger P0017

// Makro für Lesezugriff auf Register "meas_window_size_p0017"
ioctl_read_buf!(read_meas_window_size_p0017, 1, 0, u8);                         // ID: 256 : Triggerdauer bei Softtrigger P0017

// Makro für Lesezugriff auf Register "acceleration_x"
ioctl_read_buf!(read_acceleration_x, 1, 1, u8);                                 // ID: 257 : Beschleunigung X

// Makro für Lesezugriff auf Register "acceleration_y"
ioctl_read_buf!(read_acceleration_y, 1, 2, u8);                                 // ID: 258 : Beschleunigung Y

// Makro für Lesezugriff auf Register "acceleration_z"
ioctl_read_buf!(read_acceleration_z, 1, 3, u8);                                 // ID: 259 : Beschleunigung Z

// Makro für Lesezugriff auf Register "acceleration_temperature"
ioctl_read_buf!(read_acceleration_temperature, 1, 4, u8);                       // ID: 260 : Temperatur Digitalboard

// Makro für Lesezugriff auf Register "offset_alignment_duration"
ioctl_read_buf!(read_offset_alignment_duration, 1, 5, u8);                      // ID: 261 : Dauer des Offsetabgleichs / Offsetanpassung

// Makro für Lesezugriff auf Register "measurement_number"
ioctl_read_buf!(read_measurement_number, 1, 6, u8);                             // ID: 262 : Messungsnummer

// Makro für Schreibzugriff auf Register "filter_gain"
ioctl_write_buf!(write_filter_gain, 1, 7, u8);                                  // ID: 263 : Filter Gain

// Makro für Lesezugriff auf Register "filter_gain"
ioctl_read_buf!(read_filter_gain, 1, 7, u8);                                    // ID: 263 : Filter Gain

// Makro für Schreibzugriff auf Register "dac_settling_time"
ioctl_write_buf!(write_dac_settling_time, 1, 8, u8);                            // ID: 264 : DAC Settling Time

// Makro für Lesezugriff auf Register "dac_settling_time"
ioctl_read_buf!(read_dac_settling_time, 1, 8, u8);                              // ID: 264 : DAC Settling Time

// Makro für Lesezugriff auf Register "state_abl"
ioctl_read_buf!(read_state_abl, 1, 9, u8);                                      // ID: 265 : Zustand Abl

// Makro für Lesezugriff auf Register "state_handle_trigger"
ioctl_read_buf!(read_state_handle_trigger, 1, 10, u8);                          // ID: 266 : Zustand handle_trigger

// Makro für Lesezugriff auf Register "state_stream_select"
ioctl_read_buf!(read_state_stream_select, 1, 11, u8);                           // ID: 267 : Zustand stream_select

// Makro für Schreibzugriff auf Register "offset_det_rb_base_addr"
ioctl_write_buf!(write_offset_det_rb_base_addr, 1, 12, u8);                     // ID: 268 : Offset Determination Ringpuffer Adresse

// Makro für Lesezugriff auf Register "offset_det_rb_base_addr"
ioctl_read_buf!(read_offset_det_rb_base_addr, 1, 12, u8);                       // ID: 268 : Offset Determination Ringpuffer Adresse

// Makro für Schreibzugriff auf Register "offset_det_rb_size"
ioctl_write_buf!(write_offset_det_rb_size, 1, 13, u8);                          // ID: 269 : Offset Determination Ringpuffer Größe

// Makro für Lesezugriff auf Register "offset_det_rb_size"
ioctl_read_buf!(read_offset_det_rb_size, 1, 13, u8);                            // ID: 269 : Offset Determination Ringpuffer Größe

// Makro für Lesezugriff auf Register "offset_det_rb_wr_ptr"
ioctl_read_buf!(read_offset_det_rb_wr_ptr, 1, 14, u8);                          // ID: 270 : Offset Determination Ringpuffer Write Pointer

// Makro für Lesezugriff auf Register "offset_det_rb_rd_ptr"
ioctl_read_buf!(read_offset_det_rb_rd_ptr, 1, 15, u8);                          // ID: 271 : Offset Determination Ringpuffer Read Pointer

// Makro für Lesezugriff auf Register "hls_slope_baseline_version"
ioctl_read_buf!(read_hls_slope_baseline_version, 1, 16, u8);                    // ID: 272 : Schräge Nulllinie Version

// Makro für Lesezugriff auf Register "hls_slope_baseline_status"
ioctl_read_buf!(read_hls_slope_baseline_status, 1, 17, u8);                     // ID: 273 : Schräge Nulllinie Status

// Makro für Lesezugriff auf Register "hls_slope_baseline_runs"
ioctl_read_buf!(read_hls_slope_baseline_runs, 1, 18, u8);                       // ID: 274 : Schräge Nulllinie Runs

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch00"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch00, 1, 19, u8);                   // ID: 275 : Schräge Nulllinie Sum Ch00

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch01"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch01, 1, 20, u8);                   // ID: 276 : Schräge Nulllinie Sum Ch01

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch02"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch02, 1, 21, u8);                   // ID: 277 : Schräge Nulllinie Sum Ch02

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch03"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch03, 1, 22, u8);                   // ID: 278 : Schräge Nulllinie Sum Ch03

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch04"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch04, 1, 23, u8);                   // ID: 279 : Schräge Nulllinie Sum Ch04

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch05"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch05, 1, 24, u8);                   // ID: 280 : Schräge Nulllinie Sum Ch05

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch06"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch06, 1, 25, u8);                   // ID: 281 : Schräge Nulllinie Sum Ch06

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch07"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch07, 1, 26, u8);                   // ID: 282 : Schräge Nulllinie Sum Ch07

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch08"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch08, 1, 27, u8);                   // ID: 283 : Schräge Nulllinie Sum Ch08

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch09"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch09, 1, 28, u8);                   // ID: 284 : Schräge Nulllinie Sum Ch09

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch10"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch10, 1, 29, u8);                   // ID: 285 : Schräge Nulllinie Sum Ch10

// Makro für Lesezugriff auf Register "hls_slope_baseline_sum_ch11"
ioctl_read_buf!(read_hls_slope_baseline_sum_ch11, 1, 30, u8);                   // ID: 286 : Schräge Nulllinie Sum Ch11

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch00"
ioctl_read_buf!(read_hls_slope_baseline_max_ch00, 1, 31, u8);                   // ID: 287 : Schräge Nulllinie Max Ch00

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch01"
ioctl_read_buf!(read_hls_slope_baseline_max_ch01, 1, 32, u8);                   // ID: 288 : Schräge Nulllinie Max Ch01

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch02"
ioctl_read_buf!(read_hls_slope_baseline_max_ch02, 1, 33, u8);                   // ID: 289 : Schräge Nulllinie Max Ch02

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch03"
ioctl_read_buf!(read_hls_slope_baseline_max_ch03, 1, 34, u8);                   // ID: 290 : Schräge Nulllinie Max Ch03

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch04"
ioctl_read_buf!(read_hls_slope_baseline_max_ch04, 1, 35, u8);                   // ID: 291 : Schräge Nulllinie Max Ch04

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch05"
ioctl_read_buf!(read_hls_slope_baseline_max_ch05, 1, 36, u8);                   // ID: 292 : Schräge Nulllinie Max Ch05

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch06"
ioctl_read_buf!(read_hls_slope_baseline_max_ch06, 1, 37, u8);                   // ID: 293 : Schräge Nulllinie Max Ch06

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch07"
ioctl_read_buf!(read_hls_slope_baseline_max_ch07, 1, 38, u8);                   // ID: 294 : Schräge Nulllinie Max Ch07

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch08"
ioctl_read_buf!(read_hls_slope_baseline_max_ch08, 1, 39, u8);                   // ID: 295 : Schräge Nulllinie Max Ch08

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch09"
ioctl_read_buf!(read_hls_slope_baseline_max_ch09, 1, 40, u8);                   // ID: 296 : Schräge Nulllinie Max Ch09

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch10"
ioctl_read_buf!(read_hls_slope_baseline_max_ch10, 1, 41, u8);                   // ID: 297 : Schräge Nulllinie Max Ch10

// Makro für Lesezugriff auf Register "hls_slope_baseline_max_ch11"
ioctl_read_buf!(read_hls_slope_baseline_max_ch11, 1, 42, u8);                   // ID: 298 : Schräge Nulllinie Max Ch11

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch00"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch00, 1, 43, u8); // ID: 299 : Schräge Nulllinie linkes Sample Ch00

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch01"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch01, 1, 44, u8); // ID: 300 : Schräge Nulllinie linkes Sample Ch01

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch02"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch02, 1, 45, u8); // ID: 301 : Schräge Nulllinie linkes Sample Ch02

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch03"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch03, 1, 46, u8); // ID: 302 : Schräge Nulllinie linkes Sample Ch03

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch04"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch04, 1, 47, u8); // ID: 303 : Schräge Nulllinie linkes Sample Ch04

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch05"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch05, 1, 48, u8); // ID: 304 : Schräge Nulllinie linkes Sample Ch05

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch06"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch06, 1, 49, u8); // ID: 305 : Schräge Nulllinie linkes Sample Ch06

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch07"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch07, 1, 50, u8); // ID: 306 : Schräge Nulllinie linkes Sample Ch07

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch08"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch08, 1, 51, u8); // ID: 307 : Schräge Nulllinie linkes Sample Ch08

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch09"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch09, 1, 52, u8); // ID: 308 : Schräge Nulllinie linkes Sample Ch09

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch10"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch10, 1, 53, u8); // ID: 309 : Schräge Nulllinie linkes Sample Ch10

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_sample_pos_ch11"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_sample_pos_ch11, 1, 54, u8); // ID: 310 : Schräge Nulllinie linkes Sample Ch11

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch00"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch00, 1, 55, u8);// ID: 311 : Schräge Nulllinie rechtes Sample Ch00

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch01"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch01, 1, 56, u8);// ID: 312 : Schräge Nulllinie rechtes Sample Ch01

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch02"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch02, 1, 57, u8);// ID: 313 : Schräge Nulllinie rechtes Sample Ch02

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch03"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch03, 1, 58, u8);// ID: 314 : Schräge Nulllinie rechtes Sample Ch03

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch04"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch04, 1, 59, u8);// ID: 315 : Schräge Nulllinie rechtes Sample Ch04

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch05"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch05, 1, 60, u8);// ID: 316 : Schräge Nulllinie rechtes Sample Ch05

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch06"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch06, 1, 61, u8);// ID: 317 : Schräge Nulllinie rechtes Sample Ch06

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch07"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch07, 1, 62, u8);// ID: 318 : Schräge Nulllinie rechtes Sample Ch07

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch08"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch08, 1, 63, u8);// ID: 319 : Schräge Nulllinie rechtes Sample Ch08

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch09"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch09, 1, 64, u8);// ID: 320 : Schräge Nulllinie rechtes Sample Ch09

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch10"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch10, 1, 65, u8);// ID: 321 : Schräge Nulllinie rechtes Sample Ch10

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_sample_pos_ch11"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_sample_pos_ch11, 1, 66, u8);// ID: 322 : Schräge Nulllinie rechtes Sample Ch11

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch00"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch00, 1, 67, u8);      // ID: 323 : Schräge Nulllinie Linkes Level Ch00

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch01"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch01, 1, 68, u8);      // ID: 324 : Schräge Nulllinie Linkes Level Ch01

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch02"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch02, 1, 69, u8);      // ID: 325 : Schräge Nulllinie Linkes Level Ch02

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch03"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch03, 1, 70, u8);      // ID: 326 : Schräge Nulllinie Linkes Level Ch03

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch04"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch04, 1, 71, u8);      // ID: 327 : Schräge Nulllinie Linkes Level Ch04

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch05"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch05, 1, 72, u8);      // ID: 328 : Schräge Nulllinie linkes Level Ch05

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch06"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch06, 1, 73, u8);      // ID: 329 : Schräge Nulllinie linkes Level Ch06

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch07"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch07, 1, 74, u8);      // ID: 330 : Schräge Nulllinie linkes Level Ch07

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch08"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch08, 1, 75, u8);      // ID: 331 : Schräge Nulllinie linkes Level Ch08

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch09"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch09, 1, 76, u8);      // ID: 332 : Schräge Nulllinie linkes Level Ch09

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch10"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch10, 1, 77, u8);      // ID: 333 : Schräge Nulllinie linkes Level Ch10

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_bound_level_ch11"
ioctl_read_buf!(read_hls_slope_baseline_left_bound_level_ch11, 1, 78, u8);      // ID: 334 : Schräge Nulllinie linkes Level Ch11

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch00"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch00, 1, 79, u8);     // ID: 335 : Schräge Nulllinie recchtes Level Ch00

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch01"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch01, 1, 80, u8);     // ID: 336 : Schräge Nulllinie recchtes Level Ch01

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch02"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch02, 1, 81, u8);     // ID: 337 : Schräge Nulllinie recchtes Level Ch02

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch03"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch03, 1, 82, u8);     // ID: 338 : Schräge Nulllinie recchtes Level Ch03

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch04"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch04, 1, 83, u8);     // ID: 339 : Schräge Nulllinie recchtes Level Ch04

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch05"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch05, 1, 84, u8);     // ID: 340 : Schräge Nulllinie recchtes Level Ch05

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch06"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch06, 1, 85, u8);     // ID: 341 : Schräge Nulllinie recchtes Level Ch06

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch07"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch07, 1, 86, u8);     // ID: 342 : Schräge Nulllinie recchtes Level Ch07

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch08"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch08, 1, 87, u8);     // ID: 343 : Schräge Nulllinie recchtes Level Ch08

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch09"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch09, 1, 88, u8);     // ID: 344 : Schräge Nulllinie recchtes Level Ch09

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch10"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch10, 1, 89, u8);     // ID: 345 : Schräge Nulllinie recchtes Level Ch10

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_bound_level_ch11"
ioctl_read_buf!(read_hls_slope_baseline_right_bound_level_ch11, 1, 90, u8);     // ID: 346 : Schräge Nulllinie recchtes Level Ch11

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch00"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch00, 1, 91, u8);            // ID: 347 : Schräge Nulllinie Überschreiten P0B Ch00

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch01"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch01, 1, 92, u8);            // ID: 348 : Schräge Nulllinie Überschreiten P0B Ch01

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch02"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch02, 1, 93, u8);            // ID: 349 : Schräge Nulllinie Überschreiten P0B Ch02

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch03"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch03, 1, 94, u8);            // ID: 350 : Schräge Nulllinie Überschreiten P0B Ch03

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch04"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch04, 1, 95, u8);            // ID: 351 : Schräge Nulllinie Überschreiten P0B Ch04

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch05"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch05, 1, 96, u8);            // ID: 352 : Schräge Nulllinie Überschreiten P0B Ch05

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch06"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch06, 1, 97, u8);            // ID: 353 : Schräge Nulllinie Überschreiten P0B Ch06

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch07"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch07, 1, 98, u8);            // ID: 354 : Schräge Nulllinie Überschreiten P0B Ch07

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch08"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch08, 1, 99, u8);            // ID: 355 : Schräge Nulllinie Überschreiten P0B Ch08

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch09"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch09, 1, 100, u8);           // ID: 356 : Schräge Nulllinie Überschreiten P0B Ch09

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch10"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch10, 1, 101, u8);           // ID: 357 : Schräge Nulllinie Überschreiten P0B Ch10

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_b_ch11"
ioctl_read_buf!(read_hls_slope_baseline_crossing_b_ch11, 1, 102, u8);           // ID: 358 : Schräge Nulllinie Überschreiten P0B Ch11

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch00"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch00, 1, 103, u8);           // ID: 359 : Schräge Nulllinie Überschreiten P0C Ch00

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch01"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch01, 1, 104, u8);           // ID: 360 : Schräge Nulllinie Überschreiten P0C Ch01

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch02"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch02, 1, 105, u8);           // ID: 361 : Schräge Nulllinie Überschreiten P0C Ch02

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch03"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch03, 1, 106, u8);           // ID: 362 : Schräge Nulllinie Überschreiten P0C Ch03

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch04"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch04, 1, 107, u8);           // ID: 363 : Schräge Nulllinie Überschreiten P0C Ch04

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch05"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch05, 1, 108, u8);           // ID: 364 : Schräge Nulllinie Überschreiten P0C Ch05

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch06"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch06, 1, 109, u8);           // ID: 365 : Schräge Nulllinie Überschreiten P0C Ch06

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch07"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch07, 1, 110, u8);           // ID: 366 : Schräge Nulllinie Überschreiten P0C Ch07

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch08"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch08, 1, 111, u8);           // ID: 367 : Schräge Nulllinie Überschreiten P0C Ch08

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch09"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch09, 1, 112, u8);           // ID: 368 : Schräge Nulllinie Überschreiten P0C Ch09

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch10"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch10, 1, 113, u8);           // ID: 369 : Schräge Nulllinie Überschreiten P0C Ch10

// Makro für Lesezugriff auf Register "hls_slope_baseline_crossing_c_ch11"
ioctl_read_buf!(read_hls_slope_baseline_crossing_c_ch11, 1, 114, u8);           // ID: 370 : Schräge Nulllinie Überschreiten P0C Ch11

// Makro für Lesezugriff auf Register "hls_slope_baseline_last_sample"
ioctl_read_buf!(read_hls_slope_baseline_last_sample, 1, 115, u8);               // ID: 371 : Letztes Sample

// Makro für Lesezugriff auf Register "hls_slope_baseline_left_valid"
ioctl_read_buf!(read_hls_slope_baseline_left_valid, 1, 116, u8);                // ID: 372 : Left valid

// Makro für Lesezugriff auf Register "hls_slope_baseline_right_valid"
ioctl_read_buf!(read_hls_slope_baseline_right_valid, 1, 117, u8);               // ID: 373 : Right valid

//#####################  register bank initialization #####################

/// Registerbank initialisieren.
pub fn init() -> c_int {
  info_println!(" -> Registerbank: initialisieren");

  // Registerbank öffnen
  info_println!(" -> Registerbank: Device öffnen");
  let fd = open_reg_bank();

  let mut err_cnt = 0u8;

  // Registerbank Schreib- und Lesetest
  info_println!(" -> Registerbank: Schreib- und Lesetest");
  for x in 0..100 {
    let rand_value = rand::random::<u32>();
    set_test_regbank(fd, rand_value);
    let reg = get_test_regbank(fd);
    if reg != rand_value {
      err_cnt += 1;
    }
  }

  if err_cnt > 0 {
    error_println!("Register-Test fehlgeschlagen. Fehlversuche: {0}/100", err_cnt);
   }

  // Registerbank Werte auslesen
  info_println!(" -> Registerbank: verschiedene Register-Einträge ausgeben");
  let fpga_version = get_version(fd);
  info_println!(" # Version 0x{0:04X}", fpga_version);

  fd
}

/// open device for register
pub fn open_reg_bank() -> c_int {
  let fname = CString::new("/dev/hh_amv_psreg_dev").unwrap();
  unsafe { libc::open(fname.as_ptr(), libc::O_NONBLOCK | libc::O_RDWR) }
}


//#####################  register bank public functions #####################


/// ID: 001 : set "Version"
pub fn set_version(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("version Schreibgeschützes Register");
  Ok(())
}

/// ID: 001 : get "Version"
pub fn get_version(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_version(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 002 : set "Command"
pub fn set_command(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_command(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 002 : get "Command"
pub fn get_command(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_command(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 003 : set "Status"
pub fn set_status(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("status Schreibgeschützes Register");
  Ok(())
}

/// ID: 003 : get "Status"
pub fn get_status(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_status(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 004 : set "Test Regbank"
pub fn set_test_regbank(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_test_regbank(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 004 : get "Test Regbank"
pub fn get_test_regbank(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_test_regbank(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 005 : set "Debug Channel"
pub fn set_debug_channel(fd: RawFd, data: u8) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_debug_channel(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 005 : get "Debug Channel"
pub fn get_debug_channel(fd: RawFd) -> u8 {
  let mut data = [0u8; 1];
  unsafe {
    read_debug_channel(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 006 : set "Debug0"
pub fn set_debug_0(fd: RawFd, data: u8) -> Result<(), &'static str> {
  // Register is read only
  error_println!("debug_0 Schreibgeschützes Register");
  Ok(())
}

/// ID: 006 : get "Debug0"
pub fn get_debug_0(fd: RawFd) -> u8 {
  let mut data = [0u8; 1];
  unsafe {
    read_debug_0(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 007 : set "Debug1"
pub fn set_debug_1(fd: RawFd, data: u8) -> Result<(), &'static str> {
  // Register is read only
  error_println!("debug_1 Schreibgeschützes Register");
  Ok(())
}

/// ID: 007 : get "Debug1"
pub fn get_debug_1(fd: RawFd) -> u8 {
  let mut data = [0u8; 1];
  unsafe {
    read_debug_1(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 008 : set "Debug2"
pub fn set_debug_2(fd: RawFd, data: u8) -> Result<(), &'static str> {
  // Register is read only
  error_println!("debug_2 Schreibgeschützes Register");
  Ok(())
}

/// ID: 008 : get "Debug2"
pub fn get_debug_2(fd: RawFd) -> u8 {
  let mut data = [0u8; 1];
  unsafe {
    read_debug_2(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 009 : set "Opto_in"
pub fn set_opto_in(fd: RawFd, data: u8) -> Result<(), &'static str> {
  // Register is read only
  error_println!("opto_in Schreibgeschützes Register");
  Ok(())
}

/// ID: 009 : get "Opto_in"
pub fn get_opto_in(fd: RawFd) -> u8 {
  let mut data = [0u8; 1];
  unsafe {
    read_opto_in(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 010 : set "RAW Result Ch0"
pub fn set_sp_result_raw_ch00(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 010 : get "RAW Result Ch0"
pub fn get_sp_result_raw_ch00(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 011 : set "RAW Result Ch1"
pub fn set_sp_result_raw_ch01(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 011 : get "RAW Result Ch1"
pub fn get_sp_result_raw_ch01(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 012 : set "RAW Result Ch2"
pub fn set_sp_result_raw_ch02(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 012 : get "RAW Result Ch2"
pub fn get_sp_result_raw_ch02(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 013 : set "RAW Result Ch3"
pub fn set_sp_result_raw_ch03(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 013 : get "RAW Result Ch3"
pub fn get_sp_result_raw_ch03(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 014 : set "RAW Result Ch4"
pub fn set_sp_result_raw_ch04(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 014 : get "RAW Result Ch4"
pub fn get_sp_result_raw_ch04(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 015 : set "RAW Result Ch5"
pub fn set_sp_result_raw_ch05(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 015 : get "RAW Result Ch5"
pub fn get_sp_result_raw_ch05(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 016 : set "RAW Result Ch6"
pub fn set_sp_result_raw_ch06(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 016 : get "RAW Result Ch6"
pub fn get_sp_result_raw_ch06(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 017 : set "RAW Result Ch7"
pub fn set_sp_result_raw_ch07(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 017 : get "RAW Result Ch7"
pub fn get_sp_result_raw_ch07(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 018 : set "RAW Result Ch8"
pub fn set_sp_result_raw_ch08(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 018 : get "RAW Result Ch8"
pub fn get_sp_result_raw_ch08(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 019 : set "RAW Result Ch9"
pub fn set_sp_result_raw_ch09(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 019 : get "RAW Result Ch9"
pub fn get_sp_result_raw_ch09(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 020 : set "RAW Result Ch10"
pub fn set_sp_result_raw_ch10(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 020 : get "RAW Result Ch10"
pub fn get_sp_result_raw_ch10(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 021 : set "RAW Result Ch11"
pub fn set_sp_result_raw_ch11(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_raw_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 021 : get "RAW Result Ch11"
pub fn get_sp_result_raw_ch11(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_raw_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 022 : set "Filter Result Ch0"
pub fn set_sp_result_filter_ch00(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 022 : get "Filter Result Ch0"
pub fn get_sp_result_filter_ch00(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 023 : set "Filter Result Ch1"
pub fn set_sp_result_filter_ch01(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 023 : get "Filter Result Ch1"
pub fn get_sp_result_filter_ch01(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 024 : set "Filter Result Ch2"
pub fn set_sp_result_filter_ch02(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 024 : get "Filter Result Ch2"
pub fn get_sp_result_filter_ch02(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 025 : set "Filter Result Ch3"
pub fn set_sp_result_filter_ch03(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 025 : get "Filter Result Ch3"
pub fn get_sp_result_filter_ch03(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 026 : set "Filter Result Ch4"
pub fn set_sp_result_filter_ch04(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 026 : get "Filter Result Ch4"
pub fn get_sp_result_filter_ch04(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 027 : set "Filter Result Ch5"
pub fn set_sp_result_filter_ch05(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 027 : get "Filter Result Ch5"
pub fn get_sp_result_filter_ch05(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 028 : set "Filter Result Ch6"
pub fn set_sp_result_filter_ch06(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 028 : get "Filter Result Ch6"
pub fn get_sp_result_filter_ch06(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 029 : set "Filter Result Ch7"
pub fn set_sp_result_filter_ch07(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 029 : get "Filter Result Ch7"
pub fn get_sp_result_filter_ch07(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 030 : set "Filter Result Ch8"
pub fn set_sp_result_filter_ch08(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 030 : get "Filter Result Ch8"
pub fn get_sp_result_filter_ch08(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 031 : set "Filter Result Ch9"
pub fn set_sp_result_filter_ch09(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 031 : get "Filter Result Ch9"
pub fn get_sp_result_filter_ch09(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 032 : set "Filter Result Ch10"
pub fn set_sp_result_filter_ch10(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 032 : get "Filter Result Ch10"
pub fn get_sp_result_filter_ch10(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 033 : set "Filter Result Ch11"
pub fn set_sp_result_filter_ch11(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_filter_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 033 : get "Filter Result Ch11"
pub fn get_sp_result_filter_ch11(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_filter_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 034 : set "Offset Start Ch0"
pub fn set_sp_start_offset_ch00(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 034 : get "Offset Start Ch0"
pub fn get_sp_start_offset_ch00(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 035 : set "Offset Start Ch1"
pub fn set_sp_start_offset_ch01(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 035 : get "Offset Start Ch1"
pub fn get_sp_start_offset_ch01(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 036 : set "Offset Start Ch2"
pub fn set_sp_start_offset_ch02(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 036 : get "Offset Start Ch2"
pub fn get_sp_start_offset_ch02(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 037 : set "Offset Start Ch3"
pub fn set_sp_start_offset_ch03(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 037 : get "Offset Start Ch3"
pub fn get_sp_start_offset_ch03(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 038 : set "Offset Start Ch4"
pub fn set_sp_start_offset_ch04(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 038 : get "Offset Start Ch4"
pub fn get_sp_start_offset_ch04(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 039 : set "Offset Start Ch5"
pub fn set_sp_start_offset_ch05(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 039 : get "Offset Start Ch5"
pub fn get_sp_start_offset_ch05(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 040 : set "Offset Start Ch6"
pub fn set_sp_start_offset_ch06(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 040 : get "Offset Start Ch6"
pub fn get_sp_start_offset_ch06(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 041 : set "Offset Start Ch7"
pub fn set_sp_start_offset_ch07(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 041 : get "Offset Start Ch7"
pub fn get_sp_start_offset_ch07(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 042 : set "Offset Start Ch8"
pub fn set_sp_start_offset_ch08(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 042 : get "Offset Start Ch8"
pub fn get_sp_start_offset_ch08(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 043 : set "Offset Start Ch9"
pub fn set_sp_start_offset_ch09(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 043 : get "Offset Start Ch9"
pub fn get_sp_start_offset_ch09(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 044 : set "Offset Start Ch10"
pub fn set_sp_start_offset_ch10(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 044 : get "Offset Start Ch10"
pub fn get_sp_start_offset_ch10(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 045 : set "Offset Start Ch11"
pub fn set_sp_start_offset_ch11(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_offset_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 045 : get "Offset Start Ch11"
pub fn get_sp_start_offset_ch11(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_offset_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 046 : set "Min Start Ch0"
pub fn set_sp_start_min_ch00(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 046 : get "Min Start Ch0"
pub fn get_sp_start_min_ch00(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 047 : set "Min Start Ch1"
pub fn set_sp_start_min_ch01(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 047 : get "Min Start Ch1"
pub fn get_sp_start_min_ch01(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 048 : set "Min Start Ch2"
pub fn set_sp_start_min_ch02(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 048 : get "Min Start Ch2"
pub fn get_sp_start_min_ch02(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 049 : set "Min Start Ch3"
pub fn set_sp_start_min_ch03(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 049 : get "Min Start Ch3"
pub fn get_sp_start_min_ch03(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 050 : set "Min Start Ch4"
pub fn set_sp_start_min_ch04(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 050 : get "Min Start Ch4"
pub fn get_sp_start_min_ch04(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 051 : set "Min Start Ch5"
pub fn set_sp_start_min_ch05(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 051 : get "Min Start Ch5"
pub fn get_sp_start_min_ch05(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 052 : set "Min Start Ch6"
pub fn set_sp_start_min_ch06(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 052 : get "Min Start Ch6"
pub fn get_sp_start_min_ch06(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 053 : set "Min Start Ch7"
pub fn set_sp_start_min_ch07(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 053 : get "Min Start Ch7"
pub fn get_sp_start_min_ch07(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 054 : set "Min Start Ch8"
pub fn set_sp_start_min_ch08(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 054 : get "Min Start Ch8"
pub fn get_sp_start_min_ch08(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 055 : set "Min Start Ch9"
pub fn set_sp_start_min_ch09(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 055 : get "Min Start Ch9"
pub fn get_sp_start_min_ch09(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 056 : set "Min Start Ch10"
pub fn set_sp_start_min_ch10(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 056 : get "Min Start Ch10"
pub fn get_sp_start_min_ch10(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 057 : set "Min Start Ch11"
pub fn set_sp_start_min_ch11(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_min_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 057 : get "Min Start Ch11"
pub fn get_sp_start_min_ch11(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_min_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 058 : set "Max Start Ch0"
pub fn set_sp_start_max_ch00(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 058 : get "Max Start Ch0"
pub fn get_sp_start_max_ch00(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 059 : set "Max Start Ch1"
pub fn set_sp_start_max_ch01(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 059 : get "Max Start Ch1"
pub fn get_sp_start_max_ch01(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 060 : set "Max Start Ch2"
pub fn set_sp_start_max_ch02(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 060 : get "Max Start Ch2"
pub fn get_sp_start_max_ch02(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 061 : set "Max Start Ch3"
pub fn set_sp_start_max_ch03(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 061 : get "Max Start Ch3"
pub fn get_sp_start_max_ch03(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 062 : set "Max Start Ch4"
pub fn set_sp_start_max_ch04(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 062 : get "Max Start Ch4"
pub fn get_sp_start_max_ch04(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 063 : set "Max Start Ch5"
pub fn set_sp_start_max_ch05(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 063 : get "Max Start Ch5"
pub fn get_sp_start_max_ch05(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 064 : set "Max Start Ch6"
pub fn set_sp_start_max_ch06(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 064 : get "Max Start Ch6"
pub fn get_sp_start_max_ch06(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 065 : set "Max Start Ch7"
pub fn set_sp_start_max_ch07(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 065 : get "Max Start Ch7"
pub fn get_sp_start_max_ch07(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 066 : set "Max Start Ch8"
pub fn set_sp_start_max_ch08(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 066 : get "Max Start Ch8"
pub fn get_sp_start_max_ch08(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 067 : set "Max Start Ch9"
pub fn set_sp_start_max_ch09(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 067 : get "Max Start Ch9"
pub fn get_sp_start_max_ch09(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 068 : set "Max Start Ch10"
pub fn set_sp_start_max_ch10(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 068 : get "Max Start Ch10"
pub fn get_sp_start_max_ch10(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 069 : set "Max Start Ch11"
pub fn set_sp_start_max_ch11(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_start_max_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 069 : get "Max Start Ch11"
pub fn get_sp_start_max_ch11(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_start_max_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 070 : set "Offset Measure Window Start Ch0"
pub fn set_sp_offset_meas_window_start_ch00(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 070 : get "Offset Measure Window Start Ch0"
pub fn get_sp_offset_meas_window_start_ch00(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 071 : set "Offset Measure Window Start Ch1"
pub fn set_sp_offset_meas_window_start_ch01(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 071 : get "Offset Measure Window Start Ch1"
pub fn get_sp_offset_meas_window_start_ch01(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 072 : set "Offset Measure Window Start Ch2"
pub fn set_sp_offset_meas_window_start_ch02(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 072 : get "Offset Measure Window Start Ch2"
pub fn get_sp_offset_meas_window_start_ch02(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 073 : set "Offset Measure Window Start Ch3"
pub fn set_sp_offset_meas_window_start_ch03(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 073 : get "Offset Measure Window Start Ch3"
pub fn get_sp_offset_meas_window_start_ch03(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 074 : set "Offset Measure Window Start Ch4"
pub fn set_sp_offset_meas_window_start_ch04(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 074 : get "Offset Measure Window Start Ch4"
pub fn get_sp_offset_meas_window_start_ch04(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 075 : set "Offset Measure Window Start Ch5"
pub fn set_sp_offset_meas_window_start_ch05(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 075 : get "Offset Measure Window Start Ch5"
pub fn get_sp_offset_meas_window_start_ch05(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 076 : set "Offset Measure Window Start Ch6"
pub fn set_sp_offset_meas_window_start_ch06(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 076 : get "Offset Measure Window Start Ch6"
pub fn get_sp_offset_meas_window_start_ch06(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 077 : set "Offset Measure Window Start Ch7"
pub fn set_sp_offset_meas_window_start_ch07(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 077 : get "Offset Measure Window Start Ch7"
pub fn get_sp_offset_meas_window_start_ch07(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 078 : set "Offset Measure Window Start Ch8"
pub fn set_sp_offset_meas_window_start_ch08(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 078 : get "Offset Measure Window Start Ch8"
pub fn get_sp_offset_meas_window_start_ch08(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 079 : set "Offset Measure Window Start Ch9"
pub fn set_sp_offset_meas_window_start_ch09(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 079 : get "Offset Measure Window Start Ch9"
pub fn get_sp_offset_meas_window_start_ch09(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 080 : set "Offset Measure Window Start Ch10"
pub fn set_sp_offset_meas_window_start_ch10(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 080 : get "Offset Measure Window Start Ch10"
pub fn get_sp_offset_meas_window_start_ch10(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 081 : set "Offset Measure Window Start Ch11"
pub fn set_sp_offset_meas_window_start_ch11(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_start_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 081 : get "Offset Measure Window Start Ch11"
pub fn get_sp_offset_meas_window_start_ch11(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_start_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 082 : set "Offset Measure Window End Ch0"
pub fn set_sp_offset_meas_window_end_ch00(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 082 : get "Offset Measure Window End Ch0"
pub fn get_sp_offset_meas_window_end_ch00(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 083 : set "Offset Measure Window End Ch1"
pub fn set_sp_offset_meas_window_end_ch01(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 083 : get "Offset Measure Window End Ch1"
pub fn get_sp_offset_meas_window_end_ch01(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 084 : set "Offset Measure Window End Ch2"
pub fn set_sp_offset_meas_window_end_ch02(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 084 : get "Offset Measure Window End Ch2"
pub fn get_sp_offset_meas_window_end_ch02(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 085 : set "Offset Measure Window End Ch3"
pub fn set_sp_offset_meas_window_end_ch03(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 085 : get "Offset Measure Window End Ch3"
pub fn get_sp_offset_meas_window_end_ch03(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 086 : set "Offset Measure Window End Ch4"
pub fn set_sp_offset_meas_window_end_ch04(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 086 : get "Offset Measure Window End Ch4"
pub fn get_sp_offset_meas_window_end_ch04(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 087 : set "Offset Measure Window End Ch5"
pub fn set_sp_offset_meas_window_end_ch05(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 087 : get "Offset Measure Window End Ch5"
pub fn get_sp_offset_meas_window_end_ch05(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 088 : set "Offset Measure Window End Ch6"
pub fn set_sp_offset_meas_window_end_ch06(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 088 : get "Offset Measure Window End Ch6"
pub fn get_sp_offset_meas_window_end_ch06(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 089 : set "Offset Measure Window End Ch7"
pub fn set_sp_offset_meas_window_end_ch07(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 089 : get "Offset Measure Window End Ch7"
pub fn get_sp_offset_meas_window_end_ch07(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 090 : set "Offset Measure Window End Ch8"
pub fn set_sp_offset_meas_window_end_ch08(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 090 : get "Offset Measure Window End Ch8"
pub fn get_sp_offset_meas_window_end_ch08(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 091 : set "Offset Measure Window End Ch9"
pub fn set_sp_offset_meas_window_end_ch09(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 091 : get "Offset Measure Window End Ch9"
pub fn get_sp_offset_meas_window_end_ch09(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 092 : set "Offset Measure Window End Ch10"
pub fn set_sp_offset_meas_window_end_ch10(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 092 : get "Offset Measure Window End Ch10"
pub fn get_sp_offset_meas_window_end_ch10(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 093 : set "Offset Measure Window End Ch11"
pub fn set_sp_offset_meas_window_end_ch11(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_offset_meas_window_end_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 093 : get "Offset Measure Window End Ch11"
pub fn get_sp_offset_meas_window_end_ch11(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_offset_meas_window_end_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 094 : set "Sum Result Ch0"
pub fn set_sp_result_sum_ch00(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 094 : get "Sum Result Ch0"
pub fn get_sp_result_sum_ch00(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 095 : set "Sum Result Ch1"
pub fn set_sp_result_sum_ch01(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 095 : get "Sum Result Ch1"
pub fn get_sp_result_sum_ch01(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 096 : set "Sum Result Ch2"
pub fn set_sp_result_sum_ch02(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 096 : get "Sum Result Ch2"
pub fn get_sp_result_sum_ch02(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 097 : set "Sum Result Ch3"
pub fn set_sp_result_sum_ch03(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 097 : get "Sum Result Ch3"
pub fn get_sp_result_sum_ch03(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 098 : set "Sum Result Ch4"
pub fn set_sp_result_sum_ch04(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 098 : get "Sum Result Ch4"
pub fn get_sp_result_sum_ch04(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 099 : set "Sum Result Ch5"
pub fn set_sp_result_sum_ch05(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 099 : get "Sum Result Ch5"
pub fn get_sp_result_sum_ch05(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 100 : set "Sum Result Ch6"
pub fn set_sp_result_sum_ch06(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 100 : get "Sum Result Ch6"
pub fn get_sp_result_sum_ch06(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 101 : set "Sum Result Ch7"
pub fn set_sp_result_sum_ch07(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 101 : get "Sum Result Ch7"
pub fn get_sp_result_sum_ch07(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 102 : set "Sum Result Ch8"
pub fn set_sp_result_sum_ch08(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 102 : get "Sum Result Ch8"
pub fn get_sp_result_sum_ch08(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 103 : set "Sum Result Ch9"
pub fn set_sp_result_sum_ch09(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 103 : get "Sum Result Ch9"
pub fn get_sp_result_sum_ch09(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 104 : set "Sum Result Ch10"
pub fn set_sp_result_sum_ch10(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 104 : get "Sum Result Ch10"
pub fn get_sp_result_sum_ch10(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 105 : set "Sum Result Ch11"
pub fn set_sp_result_sum_ch11(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_sum_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 105 : get "Sum Result Ch11"
pub fn get_sp_result_sum_ch11(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_sp_result_sum_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 106 : set "Min Result Ch0"
pub fn set_sp_result_min_ch00(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 106 : get "Min Result Ch0"
pub fn get_sp_result_min_ch00(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 107 : set "Min Result Ch1"
pub fn set_sp_result_min_ch01(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 107 : get "Min Result Ch1"
pub fn get_sp_result_min_ch01(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 108 : set "Min Result Ch2"
pub fn set_sp_result_min_ch02(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 108 : get "Min Result Ch2"
pub fn get_sp_result_min_ch02(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 109 : set "Min Result Ch3"
pub fn set_sp_result_min_ch03(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 109 : get "Min Result Ch3"
pub fn get_sp_result_min_ch03(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 110 : set "Min Result Ch4"
pub fn set_sp_result_min_ch04(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 110 : get "Min Result Ch4"
pub fn get_sp_result_min_ch04(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 111 : set "Min Result Ch5"
pub fn set_sp_result_min_ch05(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 111 : get "Min Result Ch5"
pub fn get_sp_result_min_ch05(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 112 : set "Min Result Ch6"
pub fn set_sp_result_min_ch06(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 112 : get "Min Result Ch6"
pub fn get_sp_result_min_ch06(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 113 : set "Min Result Ch7"
pub fn set_sp_result_min_ch07(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 113 : get "Min Result Ch7"
pub fn get_sp_result_min_ch07(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 114 : set "Min Result Ch8"
pub fn set_sp_result_min_ch08(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 114 : get "Min Result Ch8"
pub fn get_sp_result_min_ch08(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 115 : set "Min Result Ch9"
pub fn set_sp_result_min_ch09(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 115 : get "Min Result Ch9"
pub fn get_sp_result_min_ch09(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 116 : set "Min Result Ch10"
pub fn set_sp_result_min_ch10(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 116 : get "Min Result Ch10"
pub fn get_sp_result_min_ch10(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 117 : set "Min Result Ch11"
pub fn set_sp_result_min_ch11(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_min_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 117 : get "Min Result Ch11"
pub fn get_sp_result_min_ch11(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_min_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 118 : set "Max Result Ch0"
pub fn set_sp_result_max_ch00(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 118 : get "Max Result Ch0"
pub fn get_sp_result_max_ch00(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 119 : set "Max Result Ch1"
pub fn set_sp_result_max_ch01(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 119 : get "Max Result Ch1"
pub fn get_sp_result_max_ch01(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 120 : set "Max Result Ch2"
pub fn set_sp_result_max_ch02(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 120 : get "Max Result Ch2"
pub fn get_sp_result_max_ch02(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 121 : set "Max Result Ch3"
pub fn set_sp_result_max_ch03(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 121 : get "Max Result Ch3"
pub fn get_sp_result_max_ch03(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 122 : set "Max Result Ch4"
pub fn set_sp_result_max_ch04(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 122 : get "Max Result Ch4"
pub fn get_sp_result_max_ch04(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 123 : set "Max Result Ch5"
pub fn set_sp_result_max_ch05(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 123 : get "Max Result Ch5"
pub fn get_sp_result_max_ch05(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 124 : set "Max Result Ch6"
pub fn set_sp_result_max_ch06(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 124 : get "Max Result Ch6"
pub fn get_sp_result_max_ch06(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 125 : set "Max Result Ch7"
pub fn set_sp_result_max_ch07(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 125 : get "Max Result Ch7"
pub fn get_sp_result_max_ch07(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 126 : set "Max Result Ch8"
pub fn set_sp_result_max_ch08(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 126 : get "Max Result Ch8"
pub fn get_sp_result_max_ch08(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 127 : set "Max Result Ch9"
pub fn set_sp_result_max_ch09(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 127 : get "Max Result Ch9"
pub fn get_sp_result_max_ch09(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 128 : set "Max Result Ch10"
pub fn set_sp_result_max_ch10(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 128 : get "Max Result Ch10"
pub fn get_sp_result_max_ch10(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 129 : set "Max Result Ch11"
pub fn set_sp_result_max_ch11(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_max_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 129 : get "Max Result Ch11"
pub fn get_sp_result_max_ch11(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_max_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 130 : set "Max Result Pos Ch0"
pub fn set_sp_result_pos_max_ch00(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 130 : get "Max Result Pos Ch0"
pub fn get_sp_result_pos_max_ch00(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 131 : set "Max Result Pos Ch1"
pub fn set_sp_result_pos_max_ch01(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 131 : get "Max Result Pos Ch1"
pub fn get_sp_result_pos_max_ch01(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 132 : set "Max Result Pos Ch2"
pub fn set_sp_result_pos_max_ch02(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 132 : get "Max Result Pos Ch2"
pub fn get_sp_result_pos_max_ch02(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 133 : set "Max Result Pos Ch3"
pub fn set_sp_result_pos_max_ch03(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 133 : get "Max Result Pos Ch3"
pub fn get_sp_result_pos_max_ch03(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 134 : set "Max Result Pos Ch4"
pub fn set_sp_result_pos_max_ch04(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 134 : get "Max Result Pos Ch4"
pub fn get_sp_result_pos_max_ch04(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 135 : set "Max Result Pos Ch5"
pub fn set_sp_result_pos_max_ch05(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 135 : get "Max Result Pos Ch5"
pub fn get_sp_result_pos_max_ch05(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 136 : set "Max Result Pos Ch6"
pub fn set_sp_result_pos_max_ch06(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 136 : get "Max Result Pos Ch6"
pub fn get_sp_result_pos_max_ch06(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 137 : set "Max Result Pos Ch7"
pub fn set_sp_result_pos_max_ch07(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 137 : get "Max Result Pos Ch7"
pub fn get_sp_result_pos_max_ch07(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 138 : set "Max Result Pos Ch8"
pub fn set_sp_result_pos_max_ch08(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 138 : get "Max Result Pos Ch8"
pub fn get_sp_result_pos_max_ch08(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 139 : set "Max Result Pos Ch9"
pub fn set_sp_result_pos_max_ch09(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 139 : get "Max Result Pos Ch9"
pub fn get_sp_result_pos_max_ch09(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 140 : set "Max Result Pos Ch10"
pub fn set_sp_result_pos_max_ch10(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 140 : get "Max Result Pos Ch10"
pub fn get_sp_result_pos_max_ch10(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 141 : set "Max Result Pos Ch11"
pub fn set_sp_result_pos_max_ch11(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_pos_max_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 141 : get "Max Result Pos Ch11"
pub fn get_sp_result_pos_max_ch11(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_pos_max_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 142 : set "First Crossing P0006 Ch0"
pub fn set_sp_result_1st_crossing_ch00(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 142 : get "First Crossing P0006 Ch0"
pub fn get_sp_result_1st_crossing_ch00(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 143 : set "First Crossing P0006 Ch1"
pub fn set_sp_result_1st_crossing_ch01(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 143 : get "First Crossing P0006 Ch1"
pub fn get_sp_result_1st_crossing_ch01(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 144 : set "First Crossing P0006 Ch2"
pub fn set_sp_result_1st_crossing_ch02(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 144 : get "First Crossing P0006 Ch2"
pub fn get_sp_result_1st_crossing_ch02(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 145 : set "First Crossing P0006 Ch3"
pub fn set_sp_result_1st_crossing_ch03(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 145 : get "First Crossing P0006 Ch3"
pub fn get_sp_result_1st_crossing_ch03(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 146 : set "First Crossing P0006 Ch4"
pub fn set_sp_result_1st_crossing_ch04(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 146 : get "First Crossing P0006 Ch4"
pub fn get_sp_result_1st_crossing_ch04(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 147 : set "First Crossing P0006 Ch5"
pub fn set_sp_result_1st_crossing_ch05(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 147 : get "First Crossing P0006 Ch5"
pub fn get_sp_result_1st_crossing_ch05(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 148 : set "First Crossing P0006 Ch6"
pub fn set_sp_result_1st_crossing_ch06(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 148 : get "First Crossing P0006 Ch6"
pub fn get_sp_result_1st_crossing_ch06(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 149 : set "First Crossing P0006 Ch7"
pub fn set_sp_result_1st_crossing_ch07(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 149 : get "First Crossing P0006 Ch7"
pub fn get_sp_result_1st_crossing_ch07(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 150 : set "First Crossing P0006 Ch8"
pub fn set_sp_result_1st_crossing_ch08(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 150 : get "First Crossing P0006 Ch8"
pub fn get_sp_result_1st_crossing_ch08(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 151 : set "First Crossing P0006 Ch9"
pub fn set_sp_result_1st_crossing_ch09(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 151 : get "First Crossing P0006 Ch9"
pub fn get_sp_result_1st_crossing_ch09(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 152 : set "First Crossing P0006 Ch10"
pub fn set_sp_result_1st_crossing_ch10(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 152 : get "First Crossing P0006 Ch10"
pub fn get_sp_result_1st_crossing_ch10(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 153 : set "First Crossing P0006 Ch11"
pub fn set_sp_result_1st_crossing_ch11(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_1st_crossing_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 153 : get "First Crossing P0006 Ch11"
pub fn get_sp_result_1st_crossing_ch11(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_1st_crossing_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 154 : set "Last Crossing P0006 Ch0"
pub fn set_sp_result_last_crossing_ch00(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 154 : get "Last Crossing P0006 Ch0"
pub fn get_sp_result_last_crossing_ch00(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 155 : set "Last Crossing P0006 Ch1"
pub fn set_sp_result_last_crossing_ch01(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 155 : get "Last Crossing P0006 Ch1"
pub fn get_sp_result_last_crossing_ch01(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 156 : set "Last Crossing P0006 Ch2"
pub fn set_sp_result_last_crossing_ch02(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 156 : get "Last Crossing P0006 Ch2"
pub fn get_sp_result_last_crossing_ch02(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 157 : set "Last Crossing P0006 Ch3"
pub fn set_sp_result_last_crossing_ch03(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 157 : get "Last Crossing P0006 Ch3"
pub fn get_sp_result_last_crossing_ch03(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 158 : set "Last Crossing P0006 Ch4"
pub fn set_sp_result_last_crossing_ch04(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 158 : get "Last Crossing P0006 Ch4"
pub fn get_sp_result_last_crossing_ch04(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 159 : set "Last Crossing P0006 Ch5"
pub fn set_sp_result_last_crossing_ch05(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 159 : get "Last Crossing P0006 Ch5"
pub fn get_sp_result_last_crossing_ch05(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 160 : set "Last Crossing P0006 Ch6"
pub fn set_sp_result_last_crossing_ch06(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 160 : get "Last Crossing P0006 Ch6"
pub fn get_sp_result_last_crossing_ch06(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 161 : set "Last Crossing P0006 Ch7"
pub fn set_sp_result_last_crossing_ch07(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 161 : get "Last Crossing P0006 Ch7"
pub fn get_sp_result_last_crossing_ch07(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 162 : set "Last Crossing P0006 Ch8"
pub fn set_sp_result_last_crossing_ch08(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 162 : get "Last Crossing P0006 Ch8"
pub fn get_sp_result_last_crossing_ch08(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 163 : set "Last Crossing P0006 Ch9"
pub fn set_sp_result_last_crossing_ch09(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 163 : get "Last Crossing P0006 Ch9"
pub fn get_sp_result_last_crossing_ch09(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 164 : set "Last Crossing P0006 Ch10"
pub fn set_sp_result_last_crossing_ch10(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 164 : get "Last Crossing P0006 Ch10"
pub fn get_sp_result_last_crossing_ch10(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 165 : set "Last Crossing P0006 Ch11"
pub fn set_sp_result_last_crossing_ch11(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_last_crossing_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 165 : get "Last Crossing P0006 Ch11"
pub fn get_sp_result_last_crossing_ch11(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_last_crossing_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 166 : set "Reversal Points Ch0"
pub fn set_sp_result_reversal_pt_ch00(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 166 : get "Reversal Points Ch0"
pub fn get_sp_result_reversal_pt_ch00(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 167 : set "Reversal Points Ch1"
pub fn set_sp_result_reversal_pt_ch01(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 167 : get "Reversal Points Ch1"
pub fn get_sp_result_reversal_pt_ch01(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 168 : set "Reversal Points Ch2"
pub fn set_sp_result_reversal_pt_ch02(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 168 : get "Reversal Points Ch2"
pub fn get_sp_result_reversal_pt_ch02(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 169 : set "Reversal Points Ch3"
pub fn set_sp_result_reversal_pt_ch03(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 169 : get "Reversal Points Ch3"
pub fn get_sp_result_reversal_pt_ch03(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 170 : set "Reversal Points Ch4"
pub fn set_sp_result_reversal_pt_ch04(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 170 : get "Reversal Points Ch4"
pub fn get_sp_result_reversal_pt_ch04(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 171 : set "Reversal Points Ch5"
pub fn set_sp_result_reversal_pt_ch05(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 171 : get "Reversal Points Ch5"
pub fn get_sp_result_reversal_pt_ch05(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 172 : set "Reversal Points Ch6"
pub fn set_sp_result_reversal_pt_ch06(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 172 : get "Reversal Points Ch6"
pub fn get_sp_result_reversal_pt_ch06(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 173 : set "Reversal Points Ch7"
pub fn set_sp_result_reversal_pt_ch07(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 173 : get "Reversal Points Ch7"
pub fn get_sp_result_reversal_pt_ch07(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 174 : set "Reversal Points Ch8"
pub fn set_sp_result_reversal_pt_ch08(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 174 : get "Reversal Points Ch8"
pub fn get_sp_result_reversal_pt_ch08(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 175 : set "Reversal Points Ch9"
pub fn set_sp_result_reversal_pt_ch09(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 175 : get "Reversal Points Ch9"
pub fn get_sp_result_reversal_pt_ch09(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 176 : set "Reversal Points Ch10"
pub fn set_sp_result_reversal_pt_ch10(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 176 : get "Reversal Points Ch10"
pub fn get_sp_result_reversal_pt_ch10(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 177 : set "Reversal Points Ch11"
pub fn set_sp_result_reversal_pt_ch11(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("sp_result_reversal_pt_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 177 : get "Reversal Points Ch11"
pub fn get_sp_result_reversal_pt_ch11(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_result_reversal_pt_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 178 : set "DAC Value Offset Ch0"
pub fn set_offset_dac_value_ch00(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 178 : get "DAC Value Offset Ch0"
pub fn get_offset_dac_value_ch00(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 179 : set "DAC Value Offset Ch1"
pub fn set_offset_dac_value_ch01(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 179 : get "DAC Value Offset Ch1"
pub fn get_offset_dac_value_ch01(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 180 : set "DAC Value Offset Ch2"
pub fn set_offset_dac_value_ch02(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 180 : get "DAC Value Offset Ch2"
pub fn get_offset_dac_value_ch02(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 181 : set "DAC Value Offset Ch3"
pub fn set_offset_dac_value_ch03(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 181 : get "DAC Value Offset Ch3"
pub fn get_offset_dac_value_ch03(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 182 : set "DAC Value Offset Ch4"
pub fn set_offset_dac_value_ch04(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 182 : get "DAC Value Offset Ch4"
pub fn get_offset_dac_value_ch04(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 183 : set "DAC Value Offset Ch5"
pub fn set_offset_dac_value_ch05(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 183 : get "DAC Value Offset Ch5"
pub fn get_offset_dac_value_ch05(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 184 : set "DAC Value Offset Ch6"
pub fn set_offset_dac_value_ch06(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 184 : get "DAC Value Offset Ch6"
pub fn get_offset_dac_value_ch06(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 185 : set "DAC Value Offset Ch7"
pub fn set_offset_dac_value_ch07(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 185 : get "DAC Value Offset Ch7"
pub fn get_offset_dac_value_ch07(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 186 : set "DAC Value Offset Ch8"
pub fn set_offset_dac_value_ch08(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 186 : get "DAC Value Offset Ch8"
pub fn get_offset_dac_value_ch08(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 187 : set "DAC Value Offset Ch9"
pub fn set_offset_dac_value_ch09(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 187 : get "DAC Value Offset Ch9"
pub fn get_offset_dac_value_ch09(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 188 : set "DAC Value Offset Ch10"
pub fn set_offset_dac_value_ch10(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 188 : get "DAC Value Offset Ch10"
pub fn get_offset_dac_value_ch10(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 189 : set "DAC Value Offset Ch11"
pub fn set_offset_dac_value_ch11(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_dac_value_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 189 : get "DAC Value Offset Ch11"
pub fn get_offset_dac_value_ch11(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_dac_value_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 190 : set "Trigger Duration"
pub fn set_trigger_duration(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("trigger_duration Schreibgeschützes Register");
  Ok(())
}

/// ID: 190 : get "Trigger Duration"
pub fn get_trigger_duration(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_trigger_duration(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 191 : set "Vibration"
pub fn set_vibration(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("vibration Schreibgeschützes Register");
  Ok(())
}

/// ID: 191 : get "Vibration"
pub fn get_vibration(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_vibration(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 192 : set "Current Time"
pub fn set_time_current(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("time_current Schreibgeschützes Register");
  Ok(())
}

/// ID: 192 : get "Current Time"
pub fn get_time_current(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_time_current(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 193 : set "Trigger Rising Edge Time"
pub fn set_time_trigger_rising_edge(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("time_trigger_rising_edge Schreibgeschützes Register");
  Ok(())
}

/// ID: 193 : get "Trigger Rising Edge Time"
pub fn get_time_trigger_rising_edge(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_time_trigger_rising_edge(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 194 : set "Setvalue Current Time"
pub fn set_time_current_set_value(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_time_current_set_value(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 194 : get "Setvalue Current Time"
pub fn get_time_current_set_value(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_time_current_set_value(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 195 : set "DAC-Value Ch0 LSB"
pub fn set_dac_value_ch00_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch00_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 195 : get "DAC-Value Ch0 LSB"
pub fn get_dac_value_ch00_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch00_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 196 : set "DAC-Value Ch0 MSB"
pub fn set_dac_value_ch00_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch00_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 196 : get "DAC-Value Ch0 MSB"
pub fn get_dac_value_ch00_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch00_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 197 : set "DAC-Value Ch1 LSB"
pub fn set_dac_value_ch01_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch01_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 197 : get "DAC-Value Ch1 LSB"
pub fn get_dac_value_ch01_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch01_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 198 : set "DAC-Value Ch1 MSB"
pub fn set_dac_value_ch01_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch01_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 198 : get "DAC-Value Ch1 MSB"
pub fn get_dac_value_ch01_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch01_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 199 : set "DAC-Value Ch2 LSB"
pub fn set_dac_value_ch02_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch02_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 199 : get "DAC-Value Ch2 LSB"
pub fn get_dac_value_ch02_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch02_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 200 : set "DAC-Value Ch2 MSB"
pub fn set_dac_value_ch02_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch02_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 200 : get "DAC-Value Ch2 MSB"
pub fn get_dac_value_ch02_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch02_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 201 : set "DAC-Value Ch3 LSB"
pub fn set_dac_value_ch03_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch03_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 201 : get "DAC-Value Ch3 LSB"
pub fn get_dac_value_ch03_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch03_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 202 : set "DAC-Value Ch3 MSB"
pub fn set_dac_value_ch03_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch03_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 202 : get "DAC-Value Ch3 MSB"
pub fn get_dac_value_ch03_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch03_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 203 : set "DAC-Value Ch4 LSB"
pub fn set_dac_value_ch04_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch04_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 203 : get "DAC-Value Ch4 LSB"
pub fn get_dac_value_ch04_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch04_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 204 : set "DAC-Value Ch4 MSB"
pub fn set_dac_value_ch04_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch04_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 204 : get "DAC-Value Ch4 MSB"
pub fn get_dac_value_ch04_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch04_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 205 : set "DAC-Value Ch5 LSB"
pub fn set_dac_value_ch05_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch05_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 205 : get "DAC-Value Ch5 LSB"
pub fn get_dac_value_ch05_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch05_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 206 : set "DAC-Value Ch5 MSB"
pub fn set_dac_value_ch05_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch05_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 206 : get "DAC-Value Ch5 MSB"
pub fn get_dac_value_ch05_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch05_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 207 : set "DAC-Value Ch6 LSB"
pub fn set_dac_value_ch06_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch06_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 207 : get "DAC-Value Ch6 LSB"
pub fn get_dac_value_ch06_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch06_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 208 : set "DAC-Value Ch6 MSB"
pub fn set_dac_value_ch06_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch06_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 208 : get "DAC-Value Ch6 MSB"
pub fn get_dac_value_ch06_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch06_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 209 : set "DAC-Value Ch7 LSB"
pub fn set_dac_value_ch07_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch07_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 209 : get "DAC-Value Ch7 LSB"
pub fn get_dac_value_ch07_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch07_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 210 : set "DAC-Value Ch7 MSB"
pub fn set_dac_value_ch07_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch07_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 210 : get "DAC-Value Ch7 MSB"
pub fn get_dac_value_ch07_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch07_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 211 : set "DAC-Value Ch8 LSB"
pub fn set_dac_value_ch08_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch08_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 211 : get "DAC-Value Ch8 LSB"
pub fn get_dac_value_ch08_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch08_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 212 : set "DAC-Value Ch8 MSB"
pub fn set_dac_value_ch08_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch08_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 212 : get "DAC-Value Ch8 MSB"
pub fn get_dac_value_ch08_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch08_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 213 : set "DAC-Value Ch9 LSB"
pub fn set_dac_value_ch09_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch09_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 213 : get "DAC-Value Ch9 LSB"
pub fn get_dac_value_ch09_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch09_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 214 : set "DAC-Value Ch9 MSB"
pub fn set_dac_value_ch09_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch09_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 214 : get "DAC-Value Ch9 MSB"
pub fn get_dac_value_ch09_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch09_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 215 : set "DAC-Value Ch10 LSB"
pub fn set_dac_value_ch10_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch10_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 215 : get "DAC-Value Ch10 LSB"
pub fn get_dac_value_ch10_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch10_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 216 : set "DAC-Value Ch10 MSB"
pub fn set_dac_value_ch10_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch10_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 216 : get "DAC-Value Ch10 MSB"
pub fn get_dac_value_ch10_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch10_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 217 : set "DAC-Value Ch11 LSB"
pub fn set_dac_value_ch11_lsb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch11_lsb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 217 : get "DAC-Value Ch11 LSB"
pub fn get_dac_value_ch11_lsb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch11_lsb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 218 : set "DAC-Value Ch11 MSB"
pub fn set_dac_value_ch11_msb(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_value_ch11_msb(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 218 : get "DAC-Value Ch11 MSB"
pub fn get_dac_value_ch11_msb(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_dac_value_ch11_msb(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 219 : set "Verstärkung Ch0"
pub fn set_sp_gain_ch00(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch00(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 219 : get "Verstärkung Ch0"
pub fn get_sp_gain_ch00(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 220 : set "Verstärkung Ch1"
pub fn set_sp_gain_ch01(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch01(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 220 : get "Verstärkung Ch1"
pub fn get_sp_gain_ch01(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 221 : set "Verstärkung Ch2"
pub fn set_sp_gain_ch02(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch02(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 221 : get "Verstärkung Ch2"
pub fn get_sp_gain_ch02(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 222 : set "Verstärkung Ch3"
pub fn set_sp_gain_ch03(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch03(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 222 : get "Verstärkung Ch3"
pub fn get_sp_gain_ch03(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 223 : set "Verstärkung Ch4"
pub fn set_sp_gain_ch04(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch04(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 223 : get "Verstärkung Ch4"
pub fn get_sp_gain_ch04(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 224 : set "Verstärkung Ch5"
pub fn set_sp_gain_ch05(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch05(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 224 : get "Verstärkung Ch5"
pub fn get_sp_gain_ch05(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 225 : set "Verstärkung Ch6"
pub fn set_sp_gain_ch06(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch06(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 225 : get "Verstärkung Ch6"
pub fn get_sp_gain_ch06(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 226 : set "Verstärkung Ch7"
pub fn set_sp_gain_ch07(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch07(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 226 : get "Verstärkung Ch7"
pub fn get_sp_gain_ch07(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 227 : set "Verstärkung Ch8"
pub fn set_sp_gain_ch08(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch08(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 227 : get "Verstärkung Ch8"
pub fn get_sp_gain_ch08(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 228 : set "Verstärkung Ch9"
pub fn set_sp_gain_ch09(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch09(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 228 : get "Verstärkung Ch9"
pub fn get_sp_gain_ch09(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 229 : set "Verstärkung Ch10"
pub fn set_sp_gain_ch10(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch10(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 229 : get "Verstärkung Ch10"
pub fn get_sp_gain_ch10(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 230 : set "Verstärkung Ch11"
pub fn set_sp_gain_ch11(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sp_gain_ch11(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 230 : get "Verstärkung Ch11"
pub fn get_sp_gain_ch11(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sp_gain_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 231 : set "Oszillator Periodendauer 0"
pub fn set_osc_period_0(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_osc_period_0(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 231 : get "Oszillator Periodendauer 0"
pub fn get_osc_period_0(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_osc_period_0(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 232 : set "Konfiguration des Triggers"
pub fn set_config_trigger(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_config_trigger(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 232 : get "Konfiguration des Triggers"
pub fn get_config_trigger(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_config_trigger(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 233 : set "Sendeleistung P000"
pub fn set_tx_pow_p0000(fd: RawFd, data: u16) -> Result<(), &'static str> {
  if data <= 4095 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_tx_pow_p0000(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 233 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 233 : get "Sendeleistung P000"
pub fn get_tx_pow_p0000(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_tx_pow_p0000(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 234 : set "Schwelle Integral- und Peakbildung P0001"
pub fn set_level_p0001(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_level_p0001(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 234 : get "Schwelle Integral- und Peakbildung P0001"
pub fn get_level_p0001(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_level_p0001(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 235 : set "Sample für Mittelung schräge Nulllinie P0002"
pub fn set_sample_avg_mean_slope_zero_p0002(fd: RawFd, data: u16) -> Result<(), &'static str> {
  if data >= 1 && data <= 2047 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sample_avg_mean_slope_zero_p0002(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 235 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 235 : get "Sample für Mittelung schräge Nulllinie P0002"
pub fn get_sample_avg_mean_slope_zero_p0002(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sample_avg_mean_slope_zero_p0002(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 236 : set "Sample für Offsetmittelung P0003"
pub fn set_sample_avg_mean_offset_p0003(fd: RawFd, data: u16) -> Result<(), &'static str> {
  if data >= 1 && data <= 2047 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sample_avg_mean_offset_p0003(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 236 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 236 : get "Sample für Offsetmittelung P0003"
pub fn get_sample_avg_mean_offset_p0003(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sample_avg_mean_offset_p0003(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 237 : set "Überwachung Messfenster P0004"
pub fn set_monitoring_measure_window_p0004(fd: RawFd, data: u8) -> Result<(), &'static str> {
  if data <= 1 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_monitoring_measure_window_p0004(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 237 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 237 : get "Überwachung Messfenster P0004"
pub fn get_monitoring_measure_window_p0004(fd: RawFd) -> u8 {
  let mut data = [0u8; 1];
  unsafe {
    read_monitoring_measure_window_p0004(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 238 : set "Schwellwert Überwachung Messfenster P0005"
pub fn set_monitoring_measure_window_level_p0005(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_monitoring_measure_window_level_p0005(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 238 : get "Schwellwert Überwachung Messfenster P0005"
pub fn get_monitoring_measure_window_level_p0005(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_monitoring_measure_window_level_p0005(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 239 : set "Schwellwert Rohdaten P0006"
pub fn set_level_raw_values_p0006(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_level_raw_values_p0006(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 239 : get "Schwellwert Rohdaten P0006"
pub fn get_level_raw_values_p0006(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_level_raw_values_p0006(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 240 : set "Timeout Offsetabgleich P0007"
pub fn set_offset_alignment_timeout_p0007(fd: RawFd, data: u16) -> Result<(), &'static str> {
  if data >= 1 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_offset_alignment_timeout_p0007(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 240 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 240 : get "Timeout Offsetabgleich P0007"
pub fn get_offset_alignment_timeout_p0007(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_offset_alignment_timeout_p0007(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 241 : set "Dauer Offsetanpassung P0008"
pub fn set_offset_alignment_duration_p0008(fd: RawFd, data: u16) -> Result<(), &'static str> {
  if data >= 1 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_offset_alignment_duration_p0008(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 241 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 241 : get "Dauer Offsetanpassung P0008"
pub fn get_offset_alignment_duration_p0008(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_offset_alignment_duration_p0008(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 242 : set "Art der Masseberechnung P0009"
pub fn set_type_mass_det_p0009(fd: RawFd, data: u8) -> Result<(), &'static str> {
  if data <= 13 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_type_mass_det_p0009(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 242 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 242 : get "Art der Masseberechnung P0009"
pub fn get_type_mass_det_p0009(fd: RawFd) -> u8 {
  let mut data = [0u8; 1];
  unsafe {
    read_type_mass_det_p0009(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 243 : set "Art Berechnung schräge Nulllinie P000A"
pub fn set_type_slope_zero_p000a(fd: RawFd, data: u8) -> Result<(), &'static str> {
  if data <= 2 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_type_slope_zero_p000a(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 243 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 243 : get "Art Berechnung schräge Nulllinie P000A"
pub fn get_type_slope_zero_p000a(fd: RawFd) -> u8 {
  let mut data = [0u8; 1];
  unsafe {
    read_type_slope_zero_p000a(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 244 : set "schräge Nulllinie Schwellwert1 P000B"
pub fn set_slope_zero_level_1_p000b(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_slope_zero_level_1_p000b(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 244 : get "schräge Nulllinie Schwellwert1 P000B"
pub fn get_slope_zero_level_1_p000b(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_slope_zero_level_1_p000b(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 245 : set "schräge Nulllinie Schwellwert2 P000C"
pub fn set_slope_zero_level_2_p000c(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_slope_zero_level_2_p000c(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 245 : get "schräge Nulllinie Schwellwert2 P000C"
pub fn get_slope_zero_level_2_p000c(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_slope_zero_level_2_p000c(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 246 : set "schräge Nulllinie t1 P000D"
pub fn set_slope_zero_t1_p000d(fd: RawFd, data: u16) -> Result<(), &'static str> {
  if data >= 1 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_slope_zero_t1_p000d(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 246 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 246 : get "schräge Nulllinie t1 P000D"
pub fn get_slope_zero_t1_p000d(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_slope_zero_t1_p000d(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 247 : set "schräge Nulllinie t2 P000E"
pub fn set_slope_zero_t2_p000e(fd: RawFd, data: u16) -> Result<(), &'static str> {
  if data >= 1 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_slope_zero_t2_p000e(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 247 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 247 : get "schräge Nulllinie t2 P000E"
pub fn get_slope_zero_t2_p000e(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_slope_zero_t2_p000e(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 248 : set "max. Amplitude Offset P000F"
pub fn set_offset_max_amplitude_p000f(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_offset_max_amplitude_p000f(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 248 : get "max. Amplitude Offset P000F"
pub fn get_offset_max_amplitude_p000f(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_offset_max_amplitude_p000f(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 249 : set "max. negativer Peak P0010"
pub fn set_peak_neg_max_p0010(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_peak_neg_max_p0010(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 249 : get "max. negativer Peak P0010"
pub fn get_peak_neg_max_p0010(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_peak_neg_max_p0010(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 250 : set "max. Anzahl Umkehrpunkte P0011"
pub fn set_reversals_max_number_p0011(fd: RawFd, data: u16) -> Result<(), &'static str> {
  if data <= 255 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_reversals_max_number_p0011(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 250 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 250 : get "max. Anzahl Umkehrpunkte P0011"
pub fn get_reversals_max_number_p0011(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_reversals_max_number_p0011(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 251 : set "Schwelle für Umkehrpunkte P0012"
pub fn set_reversals_level_p0012(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_reversals_level_p0012(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 251 : get "Schwelle für Umkehrpunkte P0012"
pub fn get_reversals_level_p0012(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_reversals_level_p0012(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 252 : set "Art Offsetbehandlung P0013"
pub fn set_offset_treatment_type_p0013(fd: RawFd, data: u8) -> Result<(), &'static str> {
  if data <= 3 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_offset_treatment_type_p0013(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 252 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 252 : get "Art Offsetbehandlung P0013"
pub fn get_offset_treatment_type_p0013(fd: RawFd) -> u8 {
  let mut data = [0u8; 1];
  unsafe {
    read_offset_treatment_type_p0013(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 253 : set "Sample für gleitendes Mittel Rohdaten P0014"
pub fn set_sample_avg_mean_raw_p0014(fd: RawFd, data: u8) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sample_avg_mean_raw_p0014(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 253 : get "Sample für gleitendes Mittel Rohdaten P0014"
pub fn get_sample_avg_mean_raw_p0014(fd: RawFd) -> u8 {
  let mut data = [0u8; 1];
  unsafe {
    read_sample_avg_mean_raw_p0014(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 254 : set "Sample Offsetermittlung P0015"
pub fn set_sample_offset_det_p0015(fd: RawFd, data: u16) -> Result<(), &'static str> {
  if data >= 1 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_sample_offset_det_p0015(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 254 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 254 : get "Sample Offsetermittlung P0015"
pub fn get_sample_offset_det_p0015(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_sample_offset_det_p0015(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 255 : set "Sollwert für den Offsetwert P0016"
pub fn set_refvalue_offset_p0016(fd: RawFd, data: i16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_refvalue_offset_p0016(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 255 : get "Sollwert für den Offsetwert P0016"
pub fn get_refvalue_offset_p0016(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_refvalue_offset_p0016(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 256 : set "Triggerdauer bei Softtrigger P0017"
pub fn set_meas_window_size_p0017(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_meas_window_size_p0017(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 256 : get "Triggerdauer bei Softtrigger P0017"
pub fn get_meas_window_size_p0017(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_meas_window_size_p0017(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 257 : set "Beschleunigung X"
pub fn set_acceleration_x(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("acceleration_x Schreibgeschützes Register");
  Ok(())
}

/// ID: 257 : get "Beschleunigung X"
pub fn get_acceleration_x(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_acceleration_x(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 258 : set "Beschleunigung Y"
pub fn set_acceleration_y(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("acceleration_y Schreibgeschützes Register");
  Ok(())
}

/// ID: 258 : get "Beschleunigung Y"
pub fn get_acceleration_y(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_acceleration_y(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 259 : set "Beschleunigung Z"
pub fn set_acceleration_z(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("acceleration_z Schreibgeschützes Register");
  Ok(())
}

/// ID: 259 : get "Beschleunigung Z"
pub fn get_acceleration_z(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_acceleration_z(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 260 : set "Temperatur Digitalboard"
pub fn set_acceleration_temperature(fd: RawFd, data: i16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("acceleration_temperature Schreibgeschützes Register");
  Ok(())
}

/// ID: 260 : get "Temperatur Digitalboard"
pub fn get_acceleration_temperature(fd: RawFd) -> i16 {
  let mut data = [0u8; 2];
  unsafe {
    read_acceleration_temperature(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 261 : set "Dauer des Offsetabgleichs / Offsetanpassung"
pub fn set_offset_alignment_duration(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_alignment_duration Schreibgeschützes Register");
  Ok(())
}

/// ID: 261 : get "Dauer des Offsetabgleichs / Offsetanpassung"
pub fn get_offset_alignment_duration(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_offset_alignment_duration(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 262 : set "Messungsnummer"
pub fn set_measurement_number(fd: RawFd, data: u16) -> Result<(), &'static str> {
  // Register is read only
  error_println!("measurement_number Schreibgeschützes Register");
  Ok(())
}

/// ID: 262 : get "Messungsnummer"
pub fn get_measurement_number(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_measurement_number(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 263 : set "Filter Gain"
pub fn set_filter_gain(fd: RawFd, data: u32) -> Result<(), &'static str> {
  if data <= 131071 {
    let bytes = data.to_le_bytes();
    unsafe {
      write_filter_gain(fd, &bytes).unwrap();
    };
    Ok(())
  } else {
    error_println!("Reg-ID: 263 Wertebereich überschritten");
    Ok(())
  }
}

/// ID: 263 : get "Filter Gain"
pub fn get_filter_gain(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_filter_gain(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 264 : set "DAC Settling Time"
pub fn set_dac_settling_time(fd: RawFd, data: u16) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_dac_settling_time(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 264 : get "DAC Settling Time"
pub fn get_dac_settling_time(fd: RawFd) -> u16 {
  let mut data = [0u8; 2];
  unsafe {
    read_dac_settling_time(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 265 : set "Zustand Abl"
pub fn set_state_abl(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("state_abl Schreibgeschützes Register");
  Ok(())
}

/// ID: 265 : get "Zustand Abl"
pub fn get_state_abl(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_state_abl(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 266 : set "Zustand handle_trigger"
pub fn set_state_handle_trigger(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("state_handle_trigger Schreibgeschützes Register");
  Ok(())
}

/// ID: 266 : get "Zustand handle_trigger"
pub fn get_state_handle_trigger(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_state_handle_trigger(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 267 : set "Zustand stream_select"
pub fn set_state_stream_select(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("state_stream_select Schreibgeschützes Register");
  Ok(())
}

/// ID: 267 : get "Zustand stream_select"
pub fn get_state_stream_select(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_state_stream_select(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 268 : set "Offset Determination Ringpuffer Adresse"
pub fn set_offset_det_rb_base_addr(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_offset_det_rb_base_addr(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 268 : get "Offset Determination Ringpuffer Adresse"
pub fn get_offset_det_rb_base_addr(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_det_rb_base_addr(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 269 : set "Offset Determination Ringpuffer Größe"
pub fn set_offset_det_rb_size(fd: RawFd, data: u32) -> Result<(), &'static str> {
    let bytes = data.to_le_bytes();
    unsafe {
      write_offset_det_rb_size(fd, &bytes).unwrap();
    };
    Ok(())
}

/// ID: 269 : get "Offset Determination Ringpuffer Größe"
pub fn get_offset_det_rb_size(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_det_rb_size(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 270 : set "Offset Determination Ringpuffer Write Pointer"
pub fn set_offset_det_rb_wr_ptr(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_det_rb_wr_ptr Schreibgeschützes Register");
  Ok(())
}

/// ID: 270 : get "Offset Determination Ringpuffer Write Pointer"
pub fn get_offset_det_rb_wr_ptr(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_det_rb_wr_ptr(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 271 : set "Offset Determination Ringpuffer Read Pointer"
pub fn set_offset_det_rb_rd_ptr(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("offset_det_rb_rd_ptr Schreibgeschützes Register");
  Ok(())
}

/// ID: 271 : get "Offset Determination Ringpuffer Read Pointer"
pub fn get_offset_det_rb_rd_ptr(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_offset_det_rb_rd_ptr(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 272 : set "Schräge Nulllinie Version"
pub fn set_hls_slope_baseline_version(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_version Schreibgeschützes Register");
  Ok(())
}

/// ID: 272 : get "Schräge Nulllinie Version"
pub fn get_hls_slope_baseline_version(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_version(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 273 : set "Schräge Nulllinie Status"
pub fn set_hls_slope_baseline_status(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_status Schreibgeschützes Register");
  Ok(())
}

/// ID: 273 : get "Schräge Nulllinie Status"
pub fn get_hls_slope_baseline_status(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_status(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 274 : set "Schräge Nulllinie Runs"
pub fn set_hls_slope_baseline_runs(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_runs Schreibgeschützes Register");
  Ok(())
}

/// ID: 274 : get "Schräge Nulllinie Runs"
pub fn get_hls_slope_baseline_runs(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_runs(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 275 : set "Schräge Nulllinie Sum Ch00"
pub fn set_hls_slope_baseline_sum_ch00(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 275 : get "Schräge Nulllinie Sum Ch00"
pub fn get_hls_slope_baseline_sum_ch00(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 276 : set "Schräge Nulllinie Sum Ch01"
pub fn set_hls_slope_baseline_sum_ch01(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 276 : get "Schräge Nulllinie Sum Ch01"
pub fn get_hls_slope_baseline_sum_ch01(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 277 : set "Schräge Nulllinie Sum Ch02"
pub fn set_hls_slope_baseline_sum_ch02(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 277 : get "Schräge Nulllinie Sum Ch02"
pub fn get_hls_slope_baseline_sum_ch02(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 278 : set "Schräge Nulllinie Sum Ch03"
pub fn set_hls_slope_baseline_sum_ch03(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 278 : get "Schräge Nulllinie Sum Ch03"
pub fn get_hls_slope_baseline_sum_ch03(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 279 : set "Schräge Nulllinie Sum Ch04"
pub fn set_hls_slope_baseline_sum_ch04(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 279 : get "Schräge Nulllinie Sum Ch04"
pub fn get_hls_slope_baseline_sum_ch04(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 280 : set "Schräge Nulllinie Sum Ch05"
pub fn set_hls_slope_baseline_sum_ch05(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 280 : get "Schräge Nulllinie Sum Ch05"
pub fn get_hls_slope_baseline_sum_ch05(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 281 : set "Schräge Nulllinie Sum Ch06"
pub fn set_hls_slope_baseline_sum_ch06(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 281 : get "Schräge Nulllinie Sum Ch06"
pub fn get_hls_slope_baseline_sum_ch06(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 282 : set "Schräge Nulllinie Sum Ch07"
pub fn set_hls_slope_baseline_sum_ch07(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 282 : get "Schräge Nulllinie Sum Ch07"
pub fn get_hls_slope_baseline_sum_ch07(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 283 : set "Schräge Nulllinie Sum Ch08"
pub fn set_hls_slope_baseline_sum_ch08(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 283 : get "Schräge Nulllinie Sum Ch08"
pub fn get_hls_slope_baseline_sum_ch08(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 284 : set "Schräge Nulllinie Sum Ch09"
pub fn set_hls_slope_baseline_sum_ch09(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 284 : get "Schräge Nulllinie Sum Ch09"
pub fn get_hls_slope_baseline_sum_ch09(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 285 : set "Schräge Nulllinie Sum Ch10"
pub fn set_hls_slope_baseline_sum_ch10(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 285 : get "Schräge Nulllinie Sum Ch10"
pub fn get_hls_slope_baseline_sum_ch10(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 286 : set "Schräge Nulllinie Sum Ch11"
pub fn set_hls_slope_baseline_sum_ch11(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_sum_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 286 : get "Schräge Nulllinie Sum Ch11"
pub fn get_hls_slope_baseline_sum_ch11(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_sum_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 287 : set "Schräge Nulllinie Max Ch00"
pub fn set_hls_slope_baseline_max_ch00(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 287 : get "Schräge Nulllinie Max Ch00"
pub fn get_hls_slope_baseline_max_ch00(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 288 : set "Schräge Nulllinie Max Ch01"
pub fn set_hls_slope_baseline_max_ch01(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 288 : get "Schräge Nulllinie Max Ch01"
pub fn get_hls_slope_baseline_max_ch01(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 289 : set "Schräge Nulllinie Max Ch02"
pub fn set_hls_slope_baseline_max_ch02(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 289 : get "Schräge Nulllinie Max Ch02"
pub fn get_hls_slope_baseline_max_ch02(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 290 : set "Schräge Nulllinie Max Ch03"
pub fn set_hls_slope_baseline_max_ch03(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 290 : get "Schräge Nulllinie Max Ch03"
pub fn get_hls_slope_baseline_max_ch03(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 291 : set "Schräge Nulllinie Max Ch04"
pub fn set_hls_slope_baseline_max_ch04(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 291 : get "Schräge Nulllinie Max Ch04"
pub fn get_hls_slope_baseline_max_ch04(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 292 : set "Schräge Nulllinie Max Ch05"
pub fn set_hls_slope_baseline_max_ch05(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 292 : get "Schräge Nulllinie Max Ch05"
pub fn get_hls_slope_baseline_max_ch05(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 293 : set "Schräge Nulllinie Max Ch06"
pub fn set_hls_slope_baseline_max_ch06(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 293 : get "Schräge Nulllinie Max Ch06"
pub fn get_hls_slope_baseline_max_ch06(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 294 : set "Schräge Nulllinie Max Ch07"
pub fn set_hls_slope_baseline_max_ch07(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 294 : get "Schräge Nulllinie Max Ch07"
pub fn get_hls_slope_baseline_max_ch07(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 295 : set "Schräge Nulllinie Max Ch08"
pub fn set_hls_slope_baseline_max_ch08(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 295 : get "Schräge Nulllinie Max Ch08"
pub fn get_hls_slope_baseline_max_ch08(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 296 : set "Schräge Nulllinie Max Ch09"
pub fn set_hls_slope_baseline_max_ch09(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 296 : get "Schräge Nulllinie Max Ch09"
pub fn get_hls_slope_baseline_max_ch09(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 297 : set "Schräge Nulllinie Max Ch10"
pub fn set_hls_slope_baseline_max_ch10(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 297 : get "Schräge Nulllinie Max Ch10"
pub fn get_hls_slope_baseline_max_ch10(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 298 : set "Schräge Nulllinie Max Ch11"
pub fn set_hls_slope_baseline_max_ch11(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_max_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 298 : get "Schräge Nulllinie Max Ch11"
pub fn get_hls_slope_baseline_max_ch11(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_max_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 299 : set "Schräge Nulllinie linkes Sample Ch00"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch00(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 299 : get "Schräge Nulllinie linkes Sample Ch00"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch00(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 300 : set "Schräge Nulllinie linkes Sample Ch01"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch01(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 300 : get "Schräge Nulllinie linkes Sample Ch01"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch01(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 301 : set "Schräge Nulllinie linkes Sample Ch02"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch02(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 301 : get "Schräge Nulllinie linkes Sample Ch02"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch02(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 302 : set "Schräge Nulllinie linkes Sample Ch03"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch03(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 302 : get "Schräge Nulllinie linkes Sample Ch03"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch03(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 303 : set "Schräge Nulllinie linkes Sample Ch04"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch04(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 303 : get "Schräge Nulllinie linkes Sample Ch04"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch04(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 304 : set "Schräge Nulllinie linkes Sample Ch05"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch05(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 304 : get "Schräge Nulllinie linkes Sample Ch05"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch05(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 305 : set "Schräge Nulllinie linkes Sample Ch06"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch06(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 305 : get "Schräge Nulllinie linkes Sample Ch06"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch06(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 306 : set "Schräge Nulllinie linkes Sample Ch07"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch07(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 306 : get "Schräge Nulllinie linkes Sample Ch07"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch07(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 307 : set "Schräge Nulllinie linkes Sample Ch08"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch08(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 307 : get "Schräge Nulllinie linkes Sample Ch08"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch08(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 308 : set "Schräge Nulllinie linkes Sample Ch09"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch09(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 308 : get "Schräge Nulllinie linkes Sample Ch09"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch09(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 309 : set "Schräge Nulllinie linkes Sample Ch10"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch10(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 309 : get "Schräge Nulllinie linkes Sample Ch10"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch10(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 310 : set "Schräge Nulllinie linkes Sample Ch11"
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch11(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_sample_pos_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 310 : get "Schräge Nulllinie linkes Sample Ch11"
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch11(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_sample_pos_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 311 : set "Schräge Nulllinie rechtes Sample Ch00"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch00(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 311 : get "Schräge Nulllinie rechtes Sample Ch00"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch00(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 312 : set "Schräge Nulllinie rechtes Sample Ch01"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch01(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 312 : get "Schräge Nulllinie rechtes Sample Ch01"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch01(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 313 : set "Schräge Nulllinie rechtes Sample Ch02"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch02(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 313 : get "Schräge Nulllinie rechtes Sample Ch02"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch02(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 314 : set "Schräge Nulllinie rechtes Sample Ch03"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch03(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 314 : get "Schräge Nulllinie rechtes Sample Ch03"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch03(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 315 : set "Schräge Nulllinie rechtes Sample Ch04"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch04(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 315 : get "Schräge Nulllinie rechtes Sample Ch04"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch04(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 316 : set "Schräge Nulllinie rechtes Sample Ch05"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch05(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 316 : get "Schräge Nulllinie rechtes Sample Ch05"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch05(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 317 : set "Schräge Nulllinie rechtes Sample Ch06"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch06(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 317 : get "Schräge Nulllinie rechtes Sample Ch06"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch06(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 318 : set "Schräge Nulllinie rechtes Sample Ch07"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch07(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 318 : get "Schräge Nulllinie rechtes Sample Ch07"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch07(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 319 : set "Schräge Nulllinie rechtes Sample Ch08"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch08(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 319 : get "Schräge Nulllinie rechtes Sample Ch08"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch08(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 320 : set "Schräge Nulllinie rechtes Sample Ch09"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch09(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 320 : get "Schräge Nulllinie rechtes Sample Ch09"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch09(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 321 : set "Schräge Nulllinie rechtes Sample Ch10"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch10(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 321 : get "Schräge Nulllinie rechtes Sample Ch10"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch10(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 322 : set "Schräge Nulllinie rechtes Sample Ch11"
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch11(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_sample_pos_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 322 : get "Schräge Nulllinie rechtes Sample Ch11"
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch11(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_sample_pos_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 323 : set "Schräge Nulllinie Linkes Level Ch00"
pub fn set_hls_slope_baseline_left_bound_level_ch00(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 323 : get "Schräge Nulllinie Linkes Level Ch00"
pub fn get_hls_slope_baseline_left_bound_level_ch00(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 324 : set "Schräge Nulllinie Linkes Level Ch01"
pub fn set_hls_slope_baseline_left_bound_level_ch01(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 324 : get "Schräge Nulllinie Linkes Level Ch01"
pub fn get_hls_slope_baseline_left_bound_level_ch01(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 325 : set "Schräge Nulllinie Linkes Level Ch02"
pub fn set_hls_slope_baseline_left_bound_level_ch02(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 325 : get "Schräge Nulllinie Linkes Level Ch02"
pub fn get_hls_slope_baseline_left_bound_level_ch02(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 326 : set "Schräge Nulllinie Linkes Level Ch03"
pub fn set_hls_slope_baseline_left_bound_level_ch03(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 326 : get "Schräge Nulllinie Linkes Level Ch03"
pub fn get_hls_slope_baseline_left_bound_level_ch03(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 327 : set "Schräge Nulllinie Linkes Level Ch04"
pub fn set_hls_slope_baseline_left_bound_level_ch04(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 327 : get "Schräge Nulllinie Linkes Level Ch04"
pub fn get_hls_slope_baseline_left_bound_level_ch04(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 328 : set "Schräge Nulllinie linkes Level Ch05"
pub fn set_hls_slope_baseline_left_bound_level_ch05(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 328 : get "Schräge Nulllinie linkes Level Ch05"
pub fn get_hls_slope_baseline_left_bound_level_ch05(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 329 : set "Schräge Nulllinie linkes Level Ch06"
pub fn set_hls_slope_baseline_left_bound_level_ch06(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 329 : get "Schräge Nulllinie linkes Level Ch06"
pub fn get_hls_slope_baseline_left_bound_level_ch06(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 330 : set "Schräge Nulllinie linkes Level Ch07"
pub fn set_hls_slope_baseline_left_bound_level_ch07(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 330 : get "Schräge Nulllinie linkes Level Ch07"
pub fn get_hls_slope_baseline_left_bound_level_ch07(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 331 : set "Schräge Nulllinie linkes Level Ch08"
pub fn set_hls_slope_baseline_left_bound_level_ch08(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 331 : get "Schräge Nulllinie linkes Level Ch08"
pub fn get_hls_slope_baseline_left_bound_level_ch08(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 332 : set "Schräge Nulllinie linkes Level Ch09"
pub fn set_hls_slope_baseline_left_bound_level_ch09(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 332 : get "Schräge Nulllinie linkes Level Ch09"
pub fn get_hls_slope_baseline_left_bound_level_ch09(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 333 : set "Schräge Nulllinie linkes Level Ch10"
pub fn set_hls_slope_baseline_left_bound_level_ch10(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 333 : get "Schräge Nulllinie linkes Level Ch10"
pub fn get_hls_slope_baseline_left_bound_level_ch10(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 334 : set "Schräge Nulllinie linkes Level Ch11"
pub fn set_hls_slope_baseline_left_bound_level_ch11(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_bound_level_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 334 : get "Schräge Nulllinie linkes Level Ch11"
pub fn get_hls_slope_baseline_left_bound_level_ch11(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_bound_level_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 335 : set "Schräge Nulllinie recchtes Level Ch00"
pub fn set_hls_slope_baseline_right_bound_level_ch00(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 335 : get "Schräge Nulllinie recchtes Level Ch00"
pub fn get_hls_slope_baseline_right_bound_level_ch00(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 336 : set "Schräge Nulllinie recchtes Level Ch01"
pub fn set_hls_slope_baseline_right_bound_level_ch01(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 336 : get "Schräge Nulllinie recchtes Level Ch01"
pub fn get_hls_slope_baseline_right_bound_level_ch01(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 337 : set "Schräge Nulllinie recchtes Level Ch02"
pub fn set_hls_slope_baseline_right_bound_level_ch02(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 337 : get "Schräge Nulllinie recchtes Level Ch02"
pub fn get_hls_slope_baseline_right_bound_level_ch02(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 338 : set "Schräge Nulllinie recchtes Level Ch03"
pub fn set_hls_slope_baseline_right_bound_level_ch03(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 338 : get "Schräge Nulllinie recchtes Level Ch03"
pub fn get_hls_slope_baseline_right_bound_level_ch03(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 339 : set "Schräge Nulllinie recchtes Level Ch04"
pub fn set_hls_slope_baseline_right_bound_level_ch04(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 339 : get "Schräge Nulllinie recchtes Level Ch04"
pub fn get_hls_slope_baseline_right_bound_level_ch04(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 340 : set "Schräge Nulllinie recchtes Level Ch05"
pub fn set_hls_slope_baseline_right_bound_level_ch05(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 340 : get "Schräge Nulllinie recchtes Level Ch05"
pub fn get_hls_slope_baseline_right_bound_level_ch05(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 341 : set "Schräge Nulllinie recchtes Level Ch06"
pub fn set_hls_slope_baseline_right_bound_level_ch06(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 341 : get "Schräge Nulllinie recchtes Level Ch06"
pub fn get_hls_slope_baseline_right_bound_level_ch06(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 342 : set "Schräge Nulllinie recchtes Level Ch07"
pub fn set_hls_slope_baseline_right_bound_level_ch07(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 342 : get "Schräge Nulllinie recchtes Level Ch07"
pub fn get_hls_slope_baseline_right_bound_level_ch07(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 343 : set "Schräge Nulllinie recchtes Level Ch08"
pub fn set_hls_slope_baseline_right_bound_level_ch08(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 343 : get "Schräge Nulllinie recchtes Level Ch08"
pub fn get_hls_slope_baseline_right_bound_level_ch08(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 344 : set "Schräge Nulllinie recchtes Level Ch09"
pub fn set_hls_slope_baseline_right_bound_level_ch09(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 344 : get "Schräge Nulllinie recchtes Level Ch09"
pub fn get_hls_slope_baseline_right_bound_level_ch09(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 345 : set "Schräge Nulllinie recchtes Level Ch10"
pub fn set_hls_slope_baseline_right_bound_level_ch10(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 345 : get "Schräge Nulllinie recchtes Level Ch10"
pub fn get_hls_slope_baseline_right_bound_level_ch10(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 346 : set "Schräge Nulllinie recchtes Level Ch11"
pub fn set_hls_slope_baseline_right_bound_level_ch11(fd: RawFd, data: i32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_bound_level_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 346 : get "Schräge Nulllinie recchtes Level Ch11"
pub fn get_hls_slope_baseline_right_bound_level_ch11(fd: RawFd) -> i32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_bound_level_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 347 : set "Schräge Nulllinie Überschreiten P0B Ch00"
pub fn set_hls_slope_baseline_crossing_b_ch00(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 347 : get "Schräge Nulllinie Überschreiten P0B Ch00"
pub fn get_hls_slope_baseline_crossing_b_ch00(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 348 : set "Schräge Nulllinie Überschreiten P0B Ch01"
pub fn set_hls_slope_baseline_crossing_b_ch01(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 348 : get "Schräge Nulllinie Überschreiten P0B Ch01"
pub fn get_hls_slope_baseline_crossing_b_ch01(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 349 : set "Schräge Nulllinie Überschreiten P0B Ch02"
pub fn set_hls_slope_baseline_crossing_b_ch02(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 349 : get "Schräge Nulllinie Überschreiten P0B Ch02"
pub fn get_hls_slope_baseline_crossing_b_ch02(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 350 : set "Schräge Nulllinie Überschreiten P0B Ch03"
pub fn set_hls_slope_baseline_crossing_b_ch03(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 350 : get "Schräge Nulllinie Überschreiten P0B Ch03"
pub fn get_hls_slope_baseline_crossing_b_ch03(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 351 : set "Schräge Nulllinie Überschreiten P0B Ch04"
pub fn set_hls_slope_baseline_crossing_b_ch04(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 351 : get "Schräge Nulllinie Überschreiten P0B Ch04"
pub fn get_hls_slope_baseline_crossing_b_ch04(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 352 : set "Schräge Nulllinie Überschreiten P0B Ch05"
pub fn set_hls_slope_baseline_crossing_b_ch05(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 352 : get "Schräge Nulllinie Überschreiten P0B Ch05"
pub fn get_hls_slope_baseline_crossing_b_ch05(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 353 : set "Schräge Nulllinie Überschreiten P0B Ch06"
pub fn set_hls_slope_baseline_crossing_b_ch06(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 353 : get "Schräge Nulllinie Überschreiten P0B Ch06"
pub fn get_hls_slope_baseline_crossing_b_ch06(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 354 : set "Schräge Nulllinie Überschreiten P0B Ch07"
pub fn set_hls_slope_baseline_crossing_b_ch07(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 354 : get "Schräge Nulllinie Überschreiten P0B Ch07"
pub fn get_hls_slope_baseline_crossing_b_ch07(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 355 : set "Schräge Nulllinie Überschreiten P0B Ch08"
pub fn set_hls_slope_baseline_crossing_b_ch08(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 355 : get "Schräge Nulllinie Überschreiten P0B Ch08"
pub fn get_hls_slope_baseline_crossing_b_ch08(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 356 : set "Schräge Nulllinie Überschreiten P0B Ch09"
pub fn set_hls_slope_baseline_crossing_b_ch09(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 356 : get "Schräge Nulllinie Überschreiten P0B Ch09"
pub fn get_hls_slope_baseline_crossing_b_ch09(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 357 : set "Schräge Nulllinie Überschreiten P0B Ch10"
pub fn set_hls_slope_baseline_crossing_b_ch10(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 357 : get "Schräge Nulllinie Überschreiten P0B Ch10"
pub fn get_hls_slope_baseline_crossing_b_ch10(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 358 : set "Schräge Nulllinie Überschreiten P0B Ch11"
pub fn set_hls_slope_baseline_crossing_b_ch11(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_b_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 358 : get "Schräge Nulllinie Überschreiten P0B Ch11"
pub fn get_hls_slope_baseline_crossing_b_ch11(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_b_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 359 : set "Schräge Nulllinie Überschreiten P0C Ch00"
pub fn set_hls_slope_baseline_crossing_c_ch00(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch00 Schreibgeschützes Register");
  Ok(())
}

/// ID: 359 : get "Schräge Nulllinie Überschreiten P0C Ch00"
pub fn get_hls_slope_baseline_crossing_c_ch00(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch00(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 360 : set "Schräge Nulllinie Überschreiten P0C Ch01"
pub fn set_hls_slope_baseline_crossing_c_ch01(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch01 Schreibgeschützes Register");
  Ok(())
}

/// ID: 360 : get "Schräge Nulllinie Überschreiten P0C Ch01"
pub fn get_hls_slope_baseline_crossing_c_ch01(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch01(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 361 : set "Schräge Nulllinie Überschreiten P0C Ch02"
pub fn set_hls_slope_baseline_crossing_c_ch02(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch02 Schreibgeschützes Register");
  Ok(())
}

/// ID: 361 : get "Schräge Nulllinie Überschreiten P0C Ch02"
pub fn get_hls_slope_baseline_crossing_c_ch02(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch02(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 362 : set "Schräge Nulllinie Überschreiten P0C Ch03"
pub fn set_hls_slope_baseline_crossing_c_ch03(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch03 Schreibgeschützes Register");
  Ok(())
}

/// ID: 362 : get "Schräge Nulllinie Überschreiten P0C Ch03"
pub fn get_hls_slope_baseline_crossing_c_ch03(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch03(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 363 : set "Schräge Nulllinie Überschreiten P0C Ch04"
pub fn set_hls_slope_baseline_crossing_c_ch04(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch04 Schreibgeschützes Register");
  Ok(())
}

/// ID: 363 : get "Schräge Nulllinie Überschreiten P0C Ch04"
pub fn get_hls_slope_baseline_crossing_c_ch04(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch04(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 364 : set "Schräge Nulllinie Überschreiten P0C Ch05"
pub fn set_hls_slope_baseline_crossing_c_ch05(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch05 Schreibgeschützes Register");
  Ok(())
}

/// ID: 364 : get "Schräge Nulllinie Überschreiten P0C Ch05"
pub fn get_hls_slope_baseline_crossing_c_ch05(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch05(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 365 : set "Schräge Nulllinie Überschreiten P0C Ch06"
pub fn set_hls_slope_baseline_crossing_c_ch06(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch06 Schreibgeschützes Register");
  Ok(())
}

/// ID: 365 : get "Schräge Nulllinie Überschreiten P0C Ch06"
pub fn get_hls_slope_baseline_crossing_c_ch06(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch06(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 366 : set "Schräge Nulllinie Überschreiten P0C Ch07"
pub fn set_hls_slope_baseline_crossing_c_ch07(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch07 Schreibgeschützes Register");
  Ok(())
}

/// ID: 366 : get "Schräge Nulllinie Überschreiten P0C Ch07"
pub fn get_hls_slope_baseline_crossing_c_ch07(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch07(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 367 : set "Schräge Nulllinie Überschreiten P0C Ch08"
pub fn set_hls_slope_baseline_crossing_c_ch08(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch08 Schreibgeschützes Register");
  Ok(())
}

/// ID: 367 : get "Schräge Nulllinie Überschreiten P0C Ch08"
pub fn get_hls_slope_baseline_crossing_c_ch08(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch08(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 368 : set "Schräge Nulllinie Überschreiten P0C Ch09"
pub fn set_hls_slope_baseline_crossing_c_ch09(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch09 Schreibgeschützes Register");
  Ok(())
}

/// ID: 368 : get "Schräge Nulllinie Überschreiten P0C Ch09"
pub fn get_hls_slope_baseline_crossing_c_ch09(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch09(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 369 : set "Schräge Nulllinie Überschreiten P0C Ch10"
pub fn set_hls_slope_baseline_crossing_c_ch10(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch10 Schreibgeschützes Register");
  Ok(())
}

/// ID: 369 : get "Schräge Nulllinie Überschreiten P0C Ch10"
pub fn get_hls_slope_baseline_crossing_c_ch10(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch10(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 370 : set "Schräge Nulllinie Überschreiten P0C Ch11"
pub fn set_hls_slope_baseline_crossing_c_ch11(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_crossing_c_ch11 Schreibgeschützes Register");
  Ok(())
}

/// ID: 370 : get "Schräge Nulllinie Überschreiten P0C Ch11"
pub fn get_hls_slope_baseline_crossing_c_ch11(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_crossing_c_ch11(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 371 : set "Letztes Sample"
pub fn set_hls_slope_baseline_last_sample(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_last_sample Schreibgeschützes Register");
  Ok(())
}

/// ID: 371 : get "Letztes Sample"
pub fn get_hls_slope_baseline_last_sample(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_last_sample(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 372 : set "Left valid"
pub fn set_hls_slope_baseline_left_valid(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_left_valid Schreibgeschützes Register");
  Ok(())
}

/// ID: 372 : get "Left valid"
pub fn get_hls_slope_baseline_left_valid(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_left_valid(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}

/// ID: 373 : set "Right valid"
pub fn set_hls_slope_baseline_right_valid(fd: RawFd, data: u32) -> Result<(), &'static str> {
  // Register is read only
  error_println!("hls_slope_baseline_right_valid Schreibgeschützes Register");
  Ok(())
}

/// ID: 373 : get "Right valid"
pub fn get_hls_slope_baseline_right_valid(fd: RawFd) -> u32 {
  let mut data = [0u8; 4];
  unsafe {
    read_hls_slope_baseline_right_valid(fd, &mut data).unwrap();
    std::mem::transmute(data)
  }
}




//#####################  register bank public functions collected in vector #####################


/// Function collection "Debug1..."
pub fn set_debug_num(fd: RawFd, channel: usize, data: u8) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u8) -> Result<(), &'static str> > = vec![
    set_debug_0,
    set_debug_1,
    set_debug_2,
  ];
  arr[channel](fd, data)
}


/// Function collection "RAW Result Ch1..."
pub fn set_sp_result_raw_ch_num(fd: RawFd, channel: usize, data: i16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i16) -> Result<(), &'static str> > = vec![
    set_sp_result_raw_ch00,
    set_sp_result_raw_ch01,
    set_sp_result_raw_ch02,
    set_sp_result_raw_ch03,
    set_sp_result_raw_ch04,
    set_sp_result_raw_ch05,
    set_sp_result_raw_ch06,
    set_sp_result_raw_ch07,
    set_sp_result_raw_ch08,
    set_sp_result_raw_ch09,
    set_sp_result_raw_ch10,
    set_sp_result_raw_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Filter Result Ch1..."
pub fn set_sp_result_filter_ch_num(fd: RawFd, channel: usize, data: i16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i16) -> Result<(), &'static str> > = vec![
    set_sp_result_filter_ch00,
    set_sp_result_filter_ch01,
    set_sp_result_filter_ch02,
    set_sp_result_filter_ch03,
    set_sp_result_filter_ch04,
    set_sp_result_filter_ch05,
    set_sp_result_filter_ch06,
    set_sp_result_filter_ch07,
    set_sp_result_filter_ch08,
    set_sp_result_filter_ch09,
    set_sp_result_filter_ch10,
    set_sp_result_filter_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Offset Start Ch1..."
pub fn set_sp_start_offset_ch_num(fd: RawFd, channel: usize, data: i16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i16) -> Result<(), &'static str> > = vec![
    set_sp_start_offset_ch00,
    set_sp_start_offset_ch01,
    set_sp_start_offset_ch02,
    set_sp_start_offset_ch03,
    set_sp_start_offset_ch04,
    set_sp_start_offset_ch05,
    set_sp_start_offset_ch06,
    set_sp_start_offset_ch07,
    set_sp_start_offset_ch08,
    set_sp_start_offset_ch09,
    set_sp_start_offset_ch10,
    set_sp_start_offset_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Min Start Ch1..."
pub fn set_sp_start_min_ch_num(fd: RawFd, channel: usize, data: i16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i16) -> Result<(), &'static str> > = vec![
    set_sp_start_min_ch00,
    set_sp_start_min_ch01,
    set_sp_start_min_ch02,
    set_sp_start_min_ch03,
    set_sp_start_min_ch04,
    set_sp_start_min_ch05,
    set_sp_start_min_ch06,
    set_sp_start_min_ch07,
    set_sp_start_min_ch08,
    set_sp_start_min_ch09,
    set_sp_start_min_ch10,
    set_sp_start_min_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Max Start Ch1..."
pub fn set_sp_start_max_ch_num(fd: RawFd, channel: usize, data: i16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i16) -> Result<(), &'static str> > = vec![
    set_sp_start_max_ch00,
    set_sp_start_max_ch01,
    set_sp_start_max_ch02,
    set_sp_start_max_ch03,
    set_sp_start_max_ch04,
    set_sp_start_max_ch05,
    set_sp_start_max_ch06,
    set_sp_start_max_ch07,
    set_sp_start_max_ch08,
    set_sp_start_max_ch09,
    set_sp_start_max_ch10,
    set_sp_start_max_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Offset Measure Window Start Ch1..."
pub fn set_sp_offset_meas_window_start_ch_num(fd: RawFd, channel: usize, data: i16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i16) -> Result<(), &'static str> > = vec![
    set_sp_offset_meas_window_start_ch00,
    set_sp_offset_meas_window_start_ch01,
    set_sp_offset_meas_window_start_ch02,
    set_sp_offset_meas_window_start_ch03,
    set_sp_offset_meas_window_start_ch04,
    set_sp_offset_meas_window_start_ch05,
    set_sp_offset_meas_window_start_ch06,
    set_sp_offset_meas_window_start_ch07,
    set_sp_offset_meas_window_start_ch08,
    set_sp_offset_meas_window_start_ch09,
    set_sp_offset_meas_window_start_ch10,
    set_sp_offset_meas_window_start_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Offset Measure Window End Ch1..."
pub fn set_sp_offset_meas_window_end_ch_num(fd: RawFd, channel: usize, data: i16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i16) -> Result<(), &'static str> > = vec![
    set_sp_offset_meas_window_end_ch00,
    set_sp_offset_meas_window_end_ch01,
    set_sp_offset_meas_window_end_ch02,
    set_sp_offset_meas_window_end_ch03,
    set_sp_offset_meas_window_end_ch04,
    set_sp_offset_meas_window_end_ch05,
    set_sp_offset_meas_window_end_ch06,
    set_sp_offset_meas_window_end_ch07,
    set_sp_offset_meas_window_end_ch08,
    set_sp_offset_meas_window_end_ch09,
    set_sp_offset_meas_window_end_ch10,
    set_sp_offset_meas_window_end_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Sum Result Ch1..."
pub fn set_sp_result_sum_ch_num(fd: RawFd, channel: usize, data: i32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i32) -> Result<(), &'static str> > = vec![
    set_sp_result_sum_ch00,
    set_sp_result_sum_ch01,
    set_sp_result_sum_ch02,
    set_sp_result_sum_ch03,
    set_sp_result_sum_ch04,
    set_sp_result_sum_ch05,
    set_sp_result_sum_ch06,
    set_sp_result_sum_ch07,
    set_sp_result_sum_ch08,
    set_sp_result_sum_ch09,
    set_sp_result_sum_ch10,
    set_sp_result_sum_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Min Result Ch1..."
pub fn set_sp_result_min_ch_num(fd: RawFd, channel: usize, data: i16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i16) -> Result<(), &'static str> > = vec![
    set_sp_result_min_ch00,
    set_sp_result_min_ch01,
    set_sp_result_min_ch02,
    set_sp_result_min_ch03,
    set_sp_result_min_ch04,
    set_sp_result_min_ch05,
    set_sp_result_min_ch06,
    set_sp_result_min_ch07,
    set_sp_result_min_ch08,
    set_sp_result_min_ch09,
    set_sp_result_min_ch10,
    set_sp_result_min_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Max Result Ch1..."
pub fn set_sp_result_max_ch_num(fd: RawFd, channel: usize, data: i16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i16) -> Result<(), &'static str> > = vec![
    set_sp_result_max_ch00,
    set_sp_result_max_ch01,
    set_sp_result_max_ch02,
    set_sp_result_max_ch03,
    set_sp_result_max_ch04,
    set_sp_result_max_ch05,
    set_sp_result_max_ch06,
    set_sp_result_max_ch07,
    set_sp_result_max_ch08,
    set_sp_result_max_ch09,
    set_sp_result_max_ch10,
    set_sp_result_max_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Max Result Pos Ch1..."
pub fn set_sp_result_pos_max_ch_num(fd: RawFd, channel: usize, data: u16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u16) -> Result<(), &'static str> > = vec![
    set_sp_result_pos_max_ch00,
    set_sp_result_pos_max_ch01,
    set_sp_result_pos_max_ch02,
    set_sp_result_pos_max_ch03,
    set_sp_result_pos_max_ch04,
    set_sp_result_pos_max_ch05,
    set_sp_result_pos_max_ch06,
    set_sp_result_pos_max_ch07,
    set_sp_result_pos_max_ch08,
    set_sp_result_pos_max_ch09,
    set_sp_result_pos_max_ch10,
    set_sp_result_pos_max_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "First Crossing P0006 Ch1..."
pub fn set_sp_result_1st_crossing_ch_num(fd: RawFd, channel: usize, data: u16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u16) -> Result<(), &'static str> > = vec![
    set_sp_result_1st_crossing_ch00,
    set_sp_result_1st_crossing_ch01,
    set_sp_result_1st_crossing_ch02,
    set_sp_result_1st_crossing_ch03,
    set_sp_result_1st_crossing_ch04,
    set_sp_result_1st_crossing_ch05,
    set_sp_result_1st_crossing_ch06,
    set_sp_result_1st_crossing_ch07,
    set_sp_result_1st_crossing_ch08,
    set_sp_result_1st_crossing_ch09,
    set_sp_result_1st_crossing_ch10,
    set_sp_result_1st_crossing_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Last Crossing P0006 Ch1..."
pub fn set_sp_result_last_crossing_ch_num(fd: RawFd, channel: usize, data: u16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u16) -> Result<(), &'static str> > = vec![
    set_sp_result_last_crossing_ch00,
    set_sp_result_last_crossing_ch01,
    set_sp_result_last_crossing_ch02,
    set_sp_result_last_crossing_ch03,
    set_sp_result_last_crossing_ch04,
    set_sp_result_last_crossing_ch05,
    set_sp_result_last_crossing_ch06,
    set_sp_result_last_crossing_ch07,
    set_sp_result_last_crossing_ch08,
    set_sp_result_last_crossing_ch09,
    set_sp_result_last_crossing_ch10,
    set_sp_result_last_crossing_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Reversal Points Ch1..."
pub fn set_sp_result_reversal_pt_ch_num(fd: RawFd, channel: usize, data: u16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u16) -> Result<(), &'static str> > = vec![
    set_sp_result_reversal_pt_ch00,
    set_sp_result_reversal_pt_ch01,
    set_sp_result_reversal_pt_ch02,
    set_sp_result_reversal_pt_ch03,
    set_sp_result_reversal_pt_ch04,
    set_sp_result_reversal_pt_ch05,
    set_sp_result_reversal_pt_ch06,
    set_sp_result_reversal_pt_ch07,
    set_sp_result_reversal_pt_ch08,
    set_sp_result_reversal_pt_ch09,
    set_sp_result_reversal_pt_ch10,
    set_sp_result_reversal_pt_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "DAC Value Offset Ch1..."
pub fn set_offset_dac_value_ch_num(fd: RawFd, channel: usize, data: i32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i32) -> Result<(), &'static str> > = vec![
    set_offset_dac_value_ch00,
    set_offset_dac_value_ch01,
    set_offset_dac_value_ch02,
    set_offset_dac_value_ch03,
    set_offset_dac_value_ch04,
    set_offset_dac_value_ch05,
    set_offset_dac_value_ch06,
    set_offset_dac_value_ch07,
    set_offset_dac_value_ch08,
    set_offset_dac_value_ch09,
    set_offset_dac_value_ch10,
    set_offset_dac_value_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "DAC-Value Ch0 MSB..."
pub fn set_dac_value_ch_num(fd: RawFd, channel: usize, data: u32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u32) -> Result<(), &'static str> > = vec![
    set_dac_value_ch00_lsb,
    set_dac_value_ch00_msb,
    set_dac_value_ch01_lsb,
    set_dac_value_ch01_msb,
    set_dac_value_ch02_lsb,
    set_dac_value_ch02_msb,
    set_dac_value_ch03_lsb,
    set_dac_value_ch03_msb,
    set_dac_value_ch04_lsb,
    set_dac_value_ch04_msb,
    set_dac_value_ch05_lsb,
    set_dac_value_ch05_msb,
    set_dac_value_ch06_lsb,
    set_dac_value_ch06_msb,
    set_dac_value_ch07_lsb,
    set_dac_value_ch07_msb,
    set_dac_value_ch08_lsb,
    set_dac_value_ch08_msb,
    set_dac_value_ch09_lsb,
    set_dac_value_ch09_msb,
    set_dac_value_ch10_lsb,
    set_dac_value_ch10_msb,
    set_dac_value_ch11_lsb,
    set_dac_value_ch11_msb,
  ];
  arr[channel](fd, data)
}


/// Function collection "Verstärkung Ch1..."
pub fn set_sp_gain_ch_num(fd: RawFd, channel: usize, data: u16) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u16) -> Result<(), &'static str> > = vec![
    set_sp_gain_ch00,
    set_sp_gain_ch01,
    set_sp_gain_ch02,
    set_sp_gain_ch03,
    set_sp_gain_ch04,
    set_sp_gain_ch05,
    set_sp_gain_ch06,
    set_sp_gain_ch07,
    set_sp_gain_ch08,
    set_sp_gain_ch09,
    set_sp_gain_ch10,
    set_sp_gain_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Schräge Nulllinie Sum Ch01..."
pub fn set_hls_slope_baseline_sum_ch_num(fd: RawFd, channel: usize, data: i32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i32) -> Result<(), &'static str> > = vec![
    set_hls_slope_baseline_sum_ch00,
    set_hls_slope_baseline_sum_ch01,
    set_hls_slope_baseline_sum_ch02,
    set_hls_slope_baseline_sum_ch03,
    set_hls_slope_baseline_sum_ch04,
    set_hls_slope_baseline_sum_ch05,
    set_hls_slope_baseline_sum_ch06,
    set_hls_slope_baseline_sum_ch07,
    set_hls_slope_baseline_sum_ch08,
    set_hls_slope_baseline_sum_ch09,
    set_hls_slope_baseline_sum_ch10,
    set_hls_slope_baseline_sum_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Schräge Nulllinie Max Ch01..."
pub fn set_hls_slope_baseline_max_ch_num(fd: RawFd, channel: usize, data: i32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i32) -> Result<(), &'static str> > = vec![
    set_hls_slope_baseline_max_ch00,
    set_hls_slope_baseline_max_ch01,
    set_hls_slope_baseline_max_ch02,
    set_hls_slope_baseline_max_ch03,
    set_hls_slope_baseline_max_ch04,
    set_hls_slope_baseline_max_ch05,
    set_hls_slope_baseline_max_ch06,
    set_hls_slope_baseline_max_ch07,
    set_hls_slope_baseline_max_ch08,
    set_hls_slope_baseline_max_ch09,
    set_hls_slope_baseline_max_ch10,
    set_hls_slope_baseline_max_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Schräge Nulllinie linkes Sample Ch01..."
pub fn set_hls_slope_baseline_left_bound_sample_pos_ch_num(fd: RawFd, channel: usize, data: u32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u32) -> Result<(), &'static str> > = vec![
    set_hls_slope_baseline_left_bound_sample_pos_ch00,
    set_hls_slope_baseline_left_bound_sample_pos_ch01,
    set_hls_slope_baseline_left_bound_sample_pos_ch02,
    set_hls_slope_baseline_left_bound_sample_pos_ch03,
    set_hls_slope_baseline_left_bound_sample_pos_ch04,
    set_hls_slope_baseline_left_bound_sample_pos_ch05,
    set_hls_slope_baseline_left_bound_sample_pos_ch06,
    set_hls_slope_baseline_left_bound_sample_pos_ch07,
    set_hls_slope_baseline_left_bound_sample_pos_ch08,
    set_hls_slope_baseline_left_bound_sample_pos_ch09,
    set_hls_slope_baseline_left_bound_sample_pos_ch10,
    set_hls_slope_baseline_left_bound_sample_pos_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Schräge Nulllinie rechtes Sample Ch01..."
pub fn set_hls_slope_baseline_right_bound_sample_pos_ch_num(fd: RawFd, channel: usize, data: u32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u32) -> Result<(), &'static str> > = vec![
    set_hls_slope_baseline_right_bound_sample_pos_ch00,
    set_hls_slope_baseline_right_bound_sample_pos_ch01,
    set_hls_slope_baseline_right_bound_sample_pos_ch02,
    set_hls_slope_baseline_right_bound_sample_pos_ch03,
    set_hls_slope_baseline_right_bound_sample_pos_ch04,
    set_hls_slope_baseline_right_bound_sample_pos_ch05,
    set_hls_slope_baseline_right_bound_sample_pos_ch06,
    set_hls_slope_baseline_right_bound_sample_pos_ch07,
    set_hls_slope_baseline_right_bound_sample_pos_ch08,
    set_hls_slope_baseline_right_bound_sample_pos_ch09,
    set_hls_slope_baseline_right_bound_sample_pos_ch10,
    set_hls_slope_baseline_right_bound_sample_pos_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Schräge Nulllinie Linkes Level Ch01..."
pub fn set_hls_slope_baseline_left_bound_level_ch_num(fd: RawFd, channel: usize, data: i32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i32) -> Result<(), &'static str> > = vec![
    set_hls_slope_baseline_left_bound_level_ch00,
    set_hls_slope_baseline_left_bound_level_ch01,
    set_hls_slope_baseline_left_bound_level_ch02,
    set_hls_slope_baseline_left_bound_level_ch03,
    set_hls_slope_baseline_left_bound_level_ch04,
    set_hls_slope_baseline_left_bound_level_ch05,
    set_hls_slope_baseline_left_bound_level_ch06,
    set_hls_slope_baseline_left_bound_level_ch07,
    set_hls_slope_baseline_left_bound_level_ch08,
    set_hls_slope_baseline_left_bound_level_ch09,
    set_hls_slope_baseline_left_bound_level_ch10,
    set_hls_slope_baseline_left_bound_level_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Schräge Nulllinie recchtes Level Ch01..."
pub fn set_hls_slope_baseline_right_bound_level_ch_num(fd: RawFd, channel: usize, data: i32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, i32) -> Result<(), &'static str> > = vec![
    set_hls_slope_baseline_right_bound_level_ch00,
    set_hls_slope_baseline_right_bound_level_ch01,
    set_hls_slope_baseline_right_bound_level_ch02,
    set_hls_slope_baseline_right_bound_level_ch03,
    set_hls_slope_baseline_right_bound_level_ch04,
    set_hls_slope_baseline_right_bound_level_ch05,
    set_hls_slope_baseline_right_bound_level_ch06,
    set_hls_slope_baseline_right_bound_level_ch07,
    set_hls_slope_baseline_right_bound_level_ch08,
    set_hls_slope_baseline_right_bound_level_ch09,
    set_hls_slope_baseline_right_bound_level_ch10,
    set_hls_slope_baseline_right_bound_level_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Schräge Nulllinie Überschreiten P0B Ch01..."
pub fn set_hls_slope_baseline_crossing_b_ch_num(fd: RawFd, channel: usize, data: u32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u32) -> Result<(), &'static str> > = vec![
    set_hls_slope_baseline_crossing_b_ch00,
    set_hls_slope_baseline_crossing_b_ch01,
    set_hls_slope_baseline_crossing_b_ch02,
    set_hls_slope_baseline_crossing_b_ch03,
    set_hls_slope_baseline_crossing_b_ch04,
    set_hls_slope_baseline_crossing_b_ch05,
    set_hls_slope_baseline_crossing_b_ch06,
    set_hls_slope_baseline_crossing_b_ch07,
    set_hls_slope_baseline_crossing_b_ch08,
    set_hls_slope_baseline_crossing_b_ch09,
    set_hls_slope_baseline_crossing_b_ch10,
    set_hls_slope_baseline_crossing_b_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Schräge Nulllinie Überschreiten P0C Ch01..."
pub fn set_hls_slope_baseline_crossing_c_ch_num(fd: RawFd, channel: usize, data: u32) -> Result<(), &'static str> {
  let arr: Vec<fn(RawFd, u32) -> Result<(), &'static str> > = vec![
    set_hls_slope_baseline_crossing_c_ch00,
    set_hls_slope_baseline_crossing_c_ch01,
    set_hls_slope_baseline_crossing_c_ch02,
    set_hls_slope_baseline_crossing_c_ch03,
    set_hls_slope_baseline_crossing_c_ch04,
    set_hls_slope_baseline_crossing_c_ch05,
    set_hls_slope_baseline_crossing_c_ch06,
    set_hls_slope_baseline_crossing_c_ch07,
    set_hls_slope_baseline_crossing_c_ch08,
    set_hls_slope_baseline_crossing_c_ch09,
    set_hls_slope_baseline_crossing_c_ch10,
    set_hls_slope_baseline_crossing_c_ch11,
  ];
  arr[channel](fd, data)
}


/// Function collection "Debug1..."
pub fn get_debug_num(fd: RawFd, channel: usize) -> u8 {
  let arr: Vec<fn(RawFd) -> u8 > = vec![
    get_debug_0,
    get_debug_1,
    get_debug_2,
  ];
  arr[channel](fd)
}


/// Function collection "RAW Result Ch1..."
pub fn get_sp_result_raw_ch_num(fd: RawFd, channel: usize) -> i16 {
  let arr: Vec<fn(RawFd) -> i16 > = vec![
    get_sp_result_raw_ch00,
    get_sp_result_raw_ch01,
    get_sp_result_raw_ch02,
    get_sp_result_raw_ch03,
    get_sp_result_raw_ch04,
    get_sp_result_raw_ch05,
    get_sp_result_raw_ch06,
    get_sp_result_raw_ch07,
    get_sp_result_raw_ch08,
    get_sp_result_raw_ch09,
    get_sp_result_raw_ch10,
    get_sp_result_raw_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Filter Result Ch1..."
pub fn get_sp_result_filter_ch_num(fd: RawFd, channel: usize) -> i16 {
  let arr: Vec<fn(RawFd) -> i16 > = vec![
    get_sp_result_filter_ch00,
    get_sp_result_filter_ch01,
    get_sp_result_filter_ch02,
    get_sp_result_filter_ch03,
    get_sp_result_filter_ch04,
    get_sp_result_filter_ch05,
    get_sp_result_filter_ch06,
    get_sp_result_filter_ch07,
    get_sp_result_filter_ch08,
    get_sp_result_filter_ch09,
    get_sp_result_filter_ch10,
    get_sp_result_filter_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Offset Start Ch1..."
pub fn get_sp_start_offset_ch_num(fd: RawFd, channel: usize) -> i16 {
  let arr: Vec<fn(RawFd) -> i16 > = vec![
    get_sp_start_offset_ch00,
    get_sp_start_offset_ch01,
    get_sp_start_offset_ch02,
    get_sp_start_offset_ch03,
    get_sp_start_offset_ch04,
    get_sp_start_offset_ch05,
    get_sp_start_offset_ch06,
    get_sp_start_offset_ch07,
    get_sp_start_offset_ch08,
    get_sp_start_offset_ch09,
    get_sp_start_offset_ch10,
    get_sp_start_offset_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Min Start Ch1..."
pub fn get_sp_start_min_ch_num(fd: RawFd, channel: usize) -> i16 {
  let arr: Vec<fn(RawFd) -> i16 > = vec![
    get_sp_start_min_ch00,
    get_sp_start_min_ch01,
    get_sp_start_min_ch02,
    get_sp_start_min_ch03,
    get_sp_start_min_ch04,
    get_sp_start_min_ch05,
    get_sp_start_min_ch06,
    get_sp_start_min_ch07,
    get_sp_start_min_ch08,
    get_sp_start_min_ch09,
    get_sp_start_min_ch10,
    get_sp_start_min_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Max Start Ch1..."
pub fn get_sp_start_max_ch_num(fd: RawFd, channel: usize) -> i16 {
  let arr: Vec<fn(RawFd) -> i16 > = vec![
    get_sp_start_max_ch00,
    get_sp_start_max_ch01,
    get_sp_start_max_ch02,
    get_sp_start_max_ch03,
    get_sp_start_max_ch04,
    get_sp_start_max_ch05,
    get_sp_start_max_ch06,
    get_sp_start_max_ch07,
    get_sp_start_max_ch08,
    get_sp_start_max_ch09,
    get_sp_start_max_ch10,
    get_sp_start_max_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Offset Measure Window Start Ch1..."
pub fn get_sp_offset_meas_window_start_ch_num(fd: RawFd, channel: usize) -> i16 {
  let arr: Vec<fn(RawFd) -> i16 > = vec![
    get_sp_offset_meas_window_start_ch00,
    get_sp_offset_meas_window_start_ch01,
    get_sp_offset_meas_window_start_ch02,
    get_sp_offset_meas_window_start_ch03,
    get_sp_offset_meas_window_start_ch04,
    get_sp_offset_meas_window_start_ch05,
    get_sp_offset_meas_window_start_ch06,
    get_sp_offset_meas_window_start_ch07,
    get_sp_offset_meas_window_start_ch08,
    get_sp_offset_meas_window_start_ch09,
    get_sp_offset_meas_window_start_ch10,
    get_sp_offset_meas_window_start_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Offset Measure Window End Ch1..."
pub fn get_sp_offset_meas_window_end_ch_num(fd: RawFd, channel: usize) -> i16 {
  let arr: Vec<fn(RawFd) -> i16 > = vec![
    get_sp_offset_meas_window_end_ch00,
    get_sp_offset_meas_window_end_ch01,
    get_sp_offset_meas_window_end_ch02,
    get_sp_offset_meas_window_end_ch03,
    get_sp_offset_meas_window_end_ch04,
    get_sp_offset_meas_window_end_ch05,
    get_sp_offset_meas_window_end_ch06,
    get_sp_offset_meas_window_end_ch07,
    get_sp_offset_meas_window_end_ch08,
    get_sp_offset_meas_window_end_ch09,
    get_sp_offset_meas_window_end_ch10,
    get_sp_offset_meas_window_end_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Sum Result Ch1..."
pub fn get_sp_result_sum_ch_num(fd: RawFd, channel: usize) -> i32 {
  let arr: Vec<fn(RawFd) -> i32 > = vec![
    get_sp_result_sum_ch00,
    get_sp_result_sum_ch01,
    get_sp_result_sum_ch02,
    get_sp_result_sum_ch03,
    get_sp_result_sum_ch04,
    get_sp_result_sum_ch05,
    get_sp_result_sum_ch06,
    get_sp_result_sum_ch07,
    get_sp_result_sum_ch08,
    get_sp_result_sum_ch09,
    get_sp_result_sum_ch10,
    get_sp_result_sum_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Min Result Ch1..."
pub fn get_sp_result_min_ch_num(fd: RawFd, channel: usize) -> i16 {
  let arr: Vec<fn(RawFd) -> i16 > = vec![
    get_sp_result_min_ch00,
    get_sp_result_min_ch01,
    get_sp_result_min_ch02,
    get_sp_result_min_ch03,
    get_sp_result_min_ch04,
    get_sp_result_min_ch05,
    get_sp_result_min_ch06,
    get_sp_result_min_ch07,
    get_sp_result_min_ch08,
    get_sp_result_min_ch09,
    get_sp_result_min_ch10,
    get_sp_result_min_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Max Result Ch1..."
pub fn get_sp_result_max_ch_num(fd: RawFd, channel: usize) -> i16 {
  let arr: Vec<fn(RawFd) -> i16 > = vec![
    get_sp_result_max_ch00,
    get_sp_result_max_ch01,
    get_sp_result_max_ch02,
    get_sp_result_max_ch03,
    get_sp_result_max_ch04,
    get_sp_result_max_ch05,
    get_sp_result_max_ch06,
    get_sp_result_max_ch07,
    get_sp_result_max_ch08,
    get_sp_result_max_ch09,
    get_sp_result_max_ch10,
    get_sp_result_max_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Max Result Pos Ch1..."
pub fn get_sp_result_pos_max_ch_num(fd: RawFd, channel: usize) -> u16 {
  let arr: Vec<fn(RawFd) -> u16 > = vec![
    get_sp_result_pos_max_ch00,
    get_sp_result_pos_max_ch01,
    get_sp_result_pos_max_ch02,
    get_sp_result_pos_max_ch03,
    get_sp_result_pos_max_ch04,
    get_sp_result_pos_max_ch05,
    get_sp_result_pos_max_ch06,
    get_sp_result_pos_max_ch07,
    get_sp_result_pos_max_ch08,
    get_sp_result_pos_max_ch09,
    get_sp_result_pos_max_ch10,
    get_sp_result_pos_max_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "First Crossing P0006 Ch1..."
pub fn get_sp_result_1st_crossing_ch_num(fd: RawFd, channel: usize) -> u16 {
  let arr: Vec<fn(RawFd) -> u16 > = vec![
    get_sp_result_1st_crossing_ch00,
    get_sp_result_1st_crossing_ch01,
    get_sp_result_1st_crossing_ch02,
    get_sp_result_1st_crossing_ch03,
    get_sp_result_1st_crossing_ch04,
    get_sp_result_1st_crossing_ch05,
    get_sp_result_1st_crossing_ch06,
    get_sp_result_1st_crossing_ch07,
    get_sp_result_1st_crossing_ch08,
    get_sp_result_1st_crossing_ch09,
    get_sp_result_1st_crossing_ch10,
    get_sp_result_1st_crossing_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Last Crossing P0006 Ch1..."
pub fn get_sp_result_last_crossing_ch_num(fd: RawFd, channel: usize) -> u16 {
  let arr: Vec<fn(RawFd) -> u16 > = vec![
    get_sp_result_last_crossing_ch00,
    get_sp_result_last_crossing_ch01,
    get_sp_result_last_crossing_ch02,
    get_sp_result_last_crossing_ch03,
    get_sp_result_last_crossing_ch04,
    get_sp_result_last_crossing_ch05,
    get_sp_result_last_crossing_ch06,
    get_sp_result_last_crossing_ch07,
    get_sp_result_last_crossing_ch08,
    get_sp_result_last_crossing_ch09,
    get_sp_result_last_crossing_ch10,
    get_sp_result_last_crossing_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Reversal Points Ch1..."
pub fn get_sp_result_reversal_pt_ch_num(fd: RawFd, channel: usize) -> u16 {
  let arr: Vec<fn(RawFd) -> u16 > = vec![
    get_sp_result_reversal_pt_ch00,
    get_sp_result_reversal_pt_ch01,
    get_sp_result_reversal_pt_ch02,
    get_sp_result_reversal_pt_ch03,
    get_sp_result_reversal_pt_ch04,
    get_sp_result_reversal_pt_ch05,
    get_sp_result_reversal_pt_ch06,
    get_sp_result_reversal_pt_ch07,
    get_sp_result_reversal_pt_ch08,
    get_sp_result_reversal_pt_ch09,
    get_sp_result_reversal_pt_ch10,
    get_sp_result_reversal_pt_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "DAC Value Offset Ch1..."
pub fn get_offset_dac_value_ch_num(fd: RawFd, channel: usize) -> i32 {
  let arr: Vec<fn(RawFd) -> i32 > = vec![
    get_offset_dac_value_ch00,
    get_offset_dac_value_ch01,
    get_offset_dac_value_ch02,
    get_offset_dac_value_ch03,
    get_offset_dac_value_ch04,
    get_offset_dac_value_ch05,
    get_offset_dac_value_ch06,
    get_offset_dac_value_ch07,
    get_offset_dac_value_ch08,
    get_offset_dac_value_ch09,
    get_offset_dac_value_ch10,
    get_offset_dac_value_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "DAC-Value Ch0 MSB..."
pub fn get_dac_value_ch_num(fd: RawFd, channel: usize) -> u32 {
  let arr: Vec<fn(RawFd) -> u32 > = vec![
    get_dac_value_ch00_lsb,
    get_dac_value_ch00_msb,
    get_dac_value_ch01_lsb,
    get_dac_value_ch01_msb,
    get_dac_value_ch02_lsb,
    get_dac_value_ch02_msb,
    get_dac_value_ch03_lsb,
    get_dac_value_ch03_msb,
    get_dac_value_ch04_lsb,
    get_dac_value_ch04_msb,
    get_dac_value_ch05_lsb,
    get_dac_value_ch05_msb,
    get_dac_value_ch06_lsb,
    get_dac_value_ch06_msb,
    get_dac_value_ch07_lsb,
    get_dac_value_ch07_msb,
    get_dac_value_ch08_lsb,
    get_dac_value_ch08_msb,
    get_dac_value_ch09_lsb,
    get_dac_value_ch09_msb,
    get_dac_value_ch10_lsb,
    get_dac_value_ch10_msb,
    get_dac_value_ch11_lsb,
    get_dac_value_ch11_msb,
  ];
  arr[channel](fd)
}


/// Function collection "Verstärkung Ch1..."
pub fn get_sp_gain_ch_num(fd: RawFd, channel: usize) -> u16 {
  let arr: Vec<fn(RawFd) -> u16 > = vec![
    get_sp_gain_ch00,
    get_sp_gain_ch01,
    get_sp_gain_ch02,
    get_sp_gain_ch03,
    get_sp_gain_ch04,
    get_sp_gain_ch05,
    get_sp_gain_ch06,
    get_sp_gain_ch07,
    get_sp_gain_ch08,
    get_sp_gain_ch09,
    get_sp_gain_ch10,
    get_sp_gain_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Schräge Nulllinie Sum Ch01..."
pub fn get_hls_slope_baseline_sum_ch_num(fd: RawFd, channel: usize) -> i32 {
  let arr: Vec<fn(RawFd) -> i32 > = vec![
    get_hls_slope_baseline_sum_ch00,
    get_hls_slope_baseline_sum_ch01,
    get_hls_slope_baseline_sum_ch02,
    get_hls_slope_baseline_sum_ch03,
    get_hls_slope_baseline_sum_ch04,
    get_hls_slope_baseline_sum_ch05,
    get_hls_slope_baseline_sum_ch06,
    get_hls_slope_baseline_sum_ch07,
    get_hls_slope_baseline_sum_ch08,
    get_hls_slope_baseline_sum_ch09,
    get_hls_slope_baseline_sum_ch10,
    get_hls_slope_baseline_sum_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Schräge Nulllinie Max Ch01..."
pub fn get_hls_slope_baseline_max_ch_num(fd: RawFd, channel: usize) -> i32 {
  let arr: Vec<fn(RawFd) -> i32 > = vec![
    get_hls_slope_baseline_max_ch00,
    get_hls_slope_baseline_max_ch01,
    get_hls_slope_baseline_max_ch02,
    get_hls_slope_baseline_max_ch03,
    get_hls_slope_baseline_max_ch04,
    get_hls_slope_baseline_max_ch05,
    get_hls_slope_baseline_max_ch06,
    get_hls_slope_baseline_max_ch07,
    get_hls_slope_baseline_max_ch08,
    get_hls_slope_baseline_max_ch09,
    get_hls_slope_baseline_max_ch10,
    get_hls_slope_baseline_max_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Schräge Nulllinie linkes Sample Ch01..."
pub fn get_hls_slope_baseline_left_bound_sample_pos_ch_num(fd: RawFd, channel: usize) -> u32 {
  let arr: Vec<fn(RawFd) -> u32 > = vec![
    get_hls_slope_baseline_left_bound_sample_pos_ch00,
    get_hls_slope_baseline_left_bound_sample_pos_ch01,
    get_hls_slope_baseline_left_bound_sample_pos_ch02,
    get_hls_slope_baseline_left_bound_sample_pos_ch03,
    get_hls_slope_baseline_left_bound_sample_pos_ch04,
    get_hls_slope_baseline_left_bound_sample_pos_ch05,
    get_hls_slope_baseline_left_bound_sample_pos_ch06,
    get_hls_slope_baseline_left_bound_sample_pos_ch07,
    get_hls_slope_baseline_left_bound_sample_pos_ch08,
    get_hls_slope_baseline_left_bound_sample_pos_ch09,
    get_hls_slope_baseline_left_bound_sample_pos_ch10,
    get_hls_slope_baseline_left_bound_sample_pos_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Schräge Nulllinie rechtes Sample Ch01..."
pub fn get_hls_slope_baseline_right_bound_sample_pos_ch_num(fd: RawFd, channel: usize) -> u32 {
  let arr: Vec<fn(RawFd) -> u32 > = vec![
    get_hls_slope_baseline_right_bound_sample_pos_ch00,
    get_hls_slope_baseline_right_bound_sample_pos_ch01,
    get_hls_slope_baseline_right_bound_sample_pos_ch02,
    get_hls_slope_baseline_right_bound_sample_pos_ch03,
    get_hls_slope_baseline_right_bound_sample_pos_ch04,
    get_hls_slope_baseline_right_bound_sample_pos_ch05,
    get_hls_slope_baseline_right_bound_sample_pos_ch06,
    get_hls_slope_baseline_right_bound_sample_pos_ch07,
    get_hls_slope_baseline_right_bound_sample_pos_ch08,
    get_hls_slope_baseline_right_bound_sample_pos_ch09,
    get_hls_slope_baseline_right_bound_sample_pos_ch10,
    get_hls_slope_baseline_right_bound_sample_pos_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Schräge Nulllinie Linkes Level Ch01..."
pub fn get_hls_slope_baseline_left_bound_level_ch_num(fd: RawFd, channel: usize) -> i32 {
  let arr: Vec<fn(RawFd) -> i32 > = vec![
    get_hls_slope_baseline_left_bound_level_ch00,
    get_hls_slope_baseline_left_bound_level_ch01,
    get_hls_slope_baseline_left_bound_level_ch02,
    get_hls_slope_baseline_left_bound_level_ch03,
    get_hls_slope_baseline_left_bound_level_ch04,
    get_hls_slope_baseline_left_bound_level_ch05,
    get_hls_slope_baseline_left_bound_level_ch06,
    get_hls_slope_baseline_left_bound_level_ch07,
    get_hls_slope_baseline_left_bound_level_ch08,
    get_hls_slope_baseline_left_bound_level_ch09,
    get_hls_slope_baseline_left_bound_level_ch10,
    get_hls_slope_baseline_left_bound_level_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Schräge Nulllinie recchtes Level Ch01..."
pub fn get_hls_slope_baseline_right_bound_level_ch_num(fd: RawFd, channel: usize) -> i32 {
  let arr: Vec<fn(RawFd) -> i32 > = vec![
    get_hls_slope_baseline_right_bound_level_ch00,
    get_hls_slope_baseline_right_bound_level_ch01,
    get_hls_slope_baseline_right_bound_level_ch02,
    get_hls_slope_baseline_right_bound_level_ch03,
    get_hls_slope_baseline_right_bound_level_ch04,
    get_hls_slope_baseline_right_bound_level_ch05,
    get_hls_slope_baseline_right_bound_level_ch06,
    get_hls_slope_baseline_right_bound_level_ch07,
    get_hls_slope_baseline_right_bound_level_ch08,
    get_hls_slope_baseline_right_bound_level_ch09,
    get_hls_slope_baseline_right_bound_level_ch10,
    get_hls_slope_baseline_right_bound_level_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Schräge Nulllinie Überschreiten P0B Ch01..."
pub fn get_hls_slope_baseline_crossing_b_ch_num(fd: RawFd, channel: usize) -> u32 {
  let arr: Vec<fn(RawFd) -> u32 > = vec![
    get_hls_slope_baseline_crossing_b_ch00,
    get_hls_slope_baseline_crossing_b_ch01,
    get_hls_slope_baseline_crossing_b_ch02,
    get_hls_slope_baseline_crossing_b_ch03,
    get_hls_slope_baseline_crossing_b_ch04,
    get_hls_slope_baseline_crossing_b_ch05,
    get_hls_slope_baseline_crossing_b_ch06,
    get_hls_slope_baseline_crossing_b_ch07,
    get_hls_slope_baseline_crossing_b_ch08,
    get_hls_slope_baseline_crossing_b_ch09,
    get_hls_slope_baseline_crossing_b_ch10,
    get_hls_slope_baseline_crossing_b_ch11,
  ];
  arr[channel](fd)
}


/// Function collection "Schräge Nulllinie Überschreiten P0C Ch01..."
pub fn get_hls_slope_baseline_crossing_c_ch_num(fd: RawFd, channel: usize) -> u32 {
  let arr: Vec<fn(RawFd) -> u32 > = vec![
    get_hls_slope_baseline_crossing_c_ch00,
    get_hls_slope_baseline_crossing_c_ch01,
    get_hls_slope_baseline_crossing_c_ch02,
    get_hls_slope_baseline_crossing_c_ch03,
    get_hls_slope_baseline_crossing_c_ch04,
    get_hls_slope_baseline_crossing_c_ch05,
    get_hls_slope_baseline_crossing_c_ch06,
    get_hls_slope_baseline_crossing_c_ch07,
    get_hls_slope_baseline_crossing_c_ch08,
    get_hls_slope_baseline_crossing_c_ch09,
    get_hls_slope_baseline_crossing_c_ch10,
    get_hls_slope_baseline_crossing_c_ch11,
  ];
  arr[channel](fd)
}

