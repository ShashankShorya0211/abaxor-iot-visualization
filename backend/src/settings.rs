//! Einstellungen und Standardwerte festlegen

#![allow(dead_code)]

// Applikation-Versionierung
pub const VERSION                                          : &str = "V3.0";
pub const VERSION_DATE                                     : &str = "2020-09-10";
pub const FIRMWARE_MAJOR                                   : u16 = 3;

// Sensor Identifikation (Standardwerte)
pub const SENSOR_TYP                                       : &str = "HH_AMV";
pub const SENSOR_SERIENNUMMER                              : &str = "No Serial";

// Divisor für ADC Werte
pub const DIV_ADC_VALUE                                    : i16 = 1;

// Anzahl der hinterlegten Mehrpunktabgleiche für diese Messstelle
pub const MEHRPUNKTABGLEICH_ANZAHL_PUNKTE_JE_MESSSTELLE          : usize = 10;    // Maximale Anzahl an Stützstellen für den Mehrpunktabgleich

// Anzahl an Oszillatoren
pub const ANZAHL_AN_OSZILLATOREN                           : usize = 1;     // Anzahl von Oszillatoren

// Anzahl an Messstellen
pub const MAXIMALE_ANZAHL_AN_MESSSTELLEN                   : usize = 12;    // Maximale Anzahl von Messstellen
pub const ANZAHL_AN_ZUSATZ_KANAELE_FIFO_STREAM             : usize = 2;     // Vibrationsdaten + Triggerdaten

// Offsetangleich/-anpassung
pub const OFFSETWERT_OHNE_MITTELWERTBILDUNG                : bool = false;  // Offsetangleich ohne Mittelwertbildung
pub const MESSUNG_OFFSETANGLEICH_SCHWELLWERT_OBEN          : i16 = 2*DIV_ADC_VALUE;    // Automatischer Offsetangleich oberhalb diesen Wertes
pub const MESSUNG_OFFSETANGLEICH_SCHWELLWERT_UNTEN         : i16 = -2*DIV_ADC_VALUE;   // Automatischer Offsetangleich unterhalb diesen Wertes
pub const MESSUNG_OFFSET_SOLLWERT                          : i16 = 0;       // Standardwert für Offset-Sollwert
pub const MESSUNG_OFFSETSTELLWERT_STARTWERT_FEIN           : u16 = 2048;    // Standardwert für Offsetstellwert Fein
pub const MESSUNG_OFFSETSTELLWERT_STARTWERT_GROB           : u16 = 2048;    // Standardwert für Offsetstellwert Grob
pub const MESSUNG_OFFSETSTELLWERT_MAX                      : u16 = 0x0FFF;  // oberer Anschlag Offsetstellwert Grob/Fein
pub const MESSUNG_OFFSETSTELLWERT_MIN                      : u16 = 0x0000;  // unterer Anschlag Offsetstellwert Grob/Fein
pub const MESSUNG_OFFSETANGLEICH_STELLWERT_MIN_FEIN        : i32 = 100;     // Minimaler Offsetstellwert für Fein
pub const MESSUNG_OFFSETANGLEICH_STELLWERT_MAX_FEIN        : i32 = 4000;    // Maximaler Offsetstellwert für Fein
pub const MESSUNG_OFFSETANGLEICH_STELLWERT_MIN_GROB        : i32 = 100;     // Maximaler Offsetstellwert für Grob
pub const MESSUNG_OFFSETANGLEICH_STELLWERT_MAX_GROB        : i32 = 4000;    // Maximaler Offsetstellwert für Grob
pub const MESSUNG_OFFSET_TOLERANZ_OFFSETANPASSUNG_FEIN     : u16 = 1*16;    // Maximale Abweichung vom Offset-Sollwert für Offsetangleich
pub const MESSUNG_OFFSET_SPITZE_SPITZE_MAX                 : u16 = 60*16;   // Maximaler Spitze-Spitze-Wert bei Offsetangleich
pub const OFFSET_FINE_APPROXIMATION_VIA_SOFTLOGIC          : bool = false;   // Offsetabgleich über die Softlogik durchführen

// Empfangsempfindlichkeit
pub const MESSUNG_EMPFANGSEMPFINDLICHKEIT_DEFAULTWERT      : u16 = 0x0198;   // Verstärkung in AN231
pub const MESSUNG_EMPFANGSEMPFINDLICHKEIT_GAIN_DEFAULTWERT : u16 = 0x8000;   // Verstärkung in PL

pub const MESSUNG_OSZILLATOR_PERIODEN_DAUER                : u32 = 0x0050;   // x * 20ns

pub const MESSUNG_MAXIMALER_MASSEWERT                      : u16 = 0xFFFF;   // Maximaler Massewert

// Oszillator konfigurieren (optional)
pub const OSZILLATOR_LOW_PASSFILTER_AKTIVIEREN             : bool = false;   //
pub const OSZILLATOR_BAND_PASSFILTER_AKTIVIEREN            : bool = false;   //

// Triggerung
pub const TRIGGER_MAX_DAUER                                : u16 = 10000;    // Millisekunden

// Verhältnis zwischen Stellwertgröße grob und fein
pub const VALUE_RESISTOR_COARSE_KOHM                       : u16 = 15;
pub const VALUE_RESISTOR_FINE_KOHM                         : u16 = 150;
pub const RATIO_OF_COARSE_TO_FINE                          : u16 = VALUE_RESISTOR_FINE_KOHM / VALUE_RESISTOR_COARSE_KOHM;

// Offsetwerte für Selbsttest
pub const SELF_TEST_OFFSET_THRESHOLD                       : i16 = 1000;     // positiver und negativer Schwellwert (Offset zum aktuellen Offsetwert Wert)
pub const SELF_TEST_MAX_TRIES                              : u16 = 10;       // Maximale Anzahl an Versuchen bis Schwellwert zu erreichen ist
pub const SELF_TEST_OFFSET_INC                             : u16 = 10;      // Schrittgröße bei Inkrementierung/Dekeremtierung

//-------------------------------------------------------------------

// Name der Konfigurationsdatei
pub const FILENAME_CONFIG_FILE_BIN                         : &str = "config.bin";
pub const FILENAME_CONFIG_FILE_JSON                        : &str = "config.json";

// Konstanten
pub const POWER_32_1                                       : u32 = 0xFFFFFFFF;

// Maximale Anzahl an Clients für die Interprozesskommunikation
// client 1: hh_amv-net
// client 2: hh_amv-udp
// client 3: hh_amv-systemd
pub const MAX_NUMBER_OF_IPC_CLIENTS                        : usize = 4;

// Puffergröße für IPC-Socket
pub const BUFFER_SIZE_IPC_SOCKET                           : usize = 2*1024;
pub const BUFFER_SIZE_RAW_SOCKET                           : usize = 1024;

// Empfangsports für IPC-Socket
pub const IPC_SOCKET_PORT                                  : u16 = 1024;
pub const IPC_SOCKET_PORT_MULTICAST                        : u16 = 1025;

// Empfangsports für Service-IPC-Socket
pub const IPC_SOCKET_PORT_SYSTEMD_01                       : u16 = 1040;
//pub const IPC_SOCKET_PORT_SYSTEMD_02                       : u16 = 1041;

// Empfangsports für Rohdaten-Socket
pub const RAW_SOCKET_PORT                                  : u16 = 1030;
pub const RAW_SOCKET_CONTROL_PORT                          : u16 = 1031;

// Empfangsports für automatische Nachrichten
pub const IPC_SOCKET_PORT_AUTO_MSG                         : u16 = 1050;

// Messergebnisse direkt aus dem Register-Bank lesen oder aus der result-Struktur.
// In der result-Struktur sind immer die Messdaten der letzen Messung gespeichert.
pub const IPC_USE_REG_BANK                                 : bool = false;

// Maximale Anzahl an Meldungen
pub const MAX_NUMBER_OF_MEASURMENT_STATS                   : usize = 20;

// Sleep-Time
pub const SLEEP_TIME_THREADS                               : u64 = 5;  //ms
pub const SLEEP_TIME_LISTENER                              : u64 = 10; //ms
pub const SLEEP_TIME_HANDLE_CLIENT                         : u64 = 5;  //ms
pub const SLEEP_TIME_REGBANK_READING                       : u64 = 10; //us
pub const SLEEP_TIME_FIFO_CONTROL                          : u64 = 10; //ms
pub const SLEEP_TIME_FIFO_READING                          : u64 = 1;  //ms

// Polling-Intervall
pub const INTERVAL_POLL_NEW_MEASUREMENT                    : u64 = 100;  //us

// Timeout für
pub const TIMEOUT_READING_THREAD_CHANNEL                   : u64 = 100; //us
pub const TRY_READ_FIFO_STREAM_NEW_PACKAGE                 : u32 = 512;

// FIFO-Stream
pub const FIFO_NUM_STREAM_CH                               : u8 = (MAXIMALE_ANZAHL_AN_MESSSTELLEN + ANZAHL_AN_ZUSATZ_KANAELE_FIFO_STREAM) as u8;
pub const FIFO_STREAM_DATA_HEADER_MARKING                  : u8 = 0xA5;
pub const FIFO_STREAM_DATA_HEADER_SIZE                     : usize = 8;     // bytes
pub const FIFO_STREAM_DATA_PACKET_SIZE                     : usize = 1024;  // bytes
pub const FIFO_STREAM_WORD_SIZE                            : usize = 4;     // bytes
pub const FIFO_STREAM_DATA_WORDS_PER_PACKET                : usize = (FIFO_STREAM_DATA_PACKET_SIZE - FIFO_STREAM_DATA_HEADER_SIZE) / FIFO_STREAM_WORD_SIZE; // words
pub const FIFO_NUM_SAMPLES_PER_WORD                        : usize = 2;     // bytes

// ----------------- Vorgaben für Debug-Zwecke -------------------

// Ziel für Printausgabe
#[cfg(feature="verbose")]
pub const PRINT_IN_SYSLOG                                  : bool = false;

#[cfg(not(feature="verbose"))]
pub const PRINT_IN_SYSLOG                                  : bool = true;

// Mindestlevel für Printausgabe
// 0 - keine Ausgabe, 1 - Fehler, 2 - Warnung, 3 - Info, 4 - Debug , 5 - Debug2
#[cfg(feature="verbose")]
pub const PRINT_LEVEL                                      : u8 = 4;

#[cfg(not(feature="verbose"))]
pub const PRINT_LEVEL                                      : u8 = 1;

// Werkskalibrierung + Offsetabgleich für Channnel 0 unterbinden
pub const NO_CHANNEL_00                                    : bool = false;
pub const NO_CHANNEL_01                                    : bool = false;

// Stellwert-Schrittweite bei Werkskalibrierung
pub const CALIBRATION_STEP_WIDTH_COARSE                    : u16 = 2;
pub const CALIBRATION_STEP_WIDTH_FINE                      : u16 = 1;

// Werkskalibrierung
pub const CALIBRATION_OFFSET_TIMEOUT                       : u64 = 60;      // s, Timeout für Offset-Werkskalibrierung
pub const CALIBRATION_OFFSET_INTERVAL                      : u32 = 10;      // ms, Dauer bis neuer Offsetstellwert gesetzt wird
pub const CALIBRATION_OFFSET_TOLERANCE                     : u16 = 3*DIV_ADC_VALUE as u16;    // Maximale Abweichung vom Offset-Sollwert für Werkskalibrierung

