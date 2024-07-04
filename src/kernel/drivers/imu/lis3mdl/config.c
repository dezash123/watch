#include <stdbool.h>

typedef enum {
  LowPower,
  MediumPerformance,
  HighPerformance,
  UltraHighPerformance,
} PowerLevel;

typedef enum {
    Hz0_625,
    Hz1_25,
    Hz2_5,
    Hz5,
    Hz10,
    Hz20,
    Hz40,
    Hz80,
} SlowDataRate;

typedef enum {
  Hz1000,
  Hz560,
  Hz300,
  Hz155,
} FastDataRate;

typedef enum {
    G4,
    G8,
    G12,
    G16,
} MagneticFieldStrengthRange;

typedef struct {
  // register 1
  bool enable_thermometer: 1;
  PowerLevel xy_power_level: 2;
  SlowDataRate slow_data_rate: 3;
  bool enable_fast_data_rate: 1,
  self_test: 1;
  // register 2
  unsigned char: 1;
  MagneticFieldStrengthRange range: 2;
  unsigned char: 5;
  // register 3
  unsigned char: 2;
  bool low_power_mode: 1;
  u8: 2;
  bool spi_3_wire: 1, 
  power_down: 1,
  single_conversion: 1; // false for continuous
  // register 4
  u8: 4;
  PowerLevel z_power_level: 2;
  bool big_endian: 1;
  u8: 1;
  // register 5
  bool fast_read: 1, // only send MS byte
  block_data: 1; // dont update until data has been read
  unsigned char: 6;
} Lis3mdlDeviceConfig;

typedef struct {
  // register 1
  bool x_interrupt_enable: 1,
  y_interrupt_enable: 1,
  z_interrupt_enable: 1;
  u8: 1,
  ONE: 1;
  bool interrupt_is_high: 1,
  latched_interrupt: 1,
  interrupt_enable: 1;
  // skip int source register
  u8;
  // register 2 and 3
  u16 interrupt_threshold;
} Lis3mdlInterruptConfig;

typedef struct {
  Lis3mdlDeviceConfig device;
  Lis3mdlInterruptConfig interrupt;
} Lis3mdlConfig;

double convert(MagneticFieldStrengthRange: range) {
  switch (range) {
    case G4:
      return 1.0 / 16834.0;
    case G8: 
      return 1.0 / 8192.0;
    case G12:
      return 1.0 / 4096.0;
    case G16:
      return 1.0 / 2048.0;
  }
}

typedef struct {
  SlowDataRate rate;
  PowerLevel xy_power_level,
  z_power_level;
} SlowDataRateContainer;

typedef struct {
  union {
    PowerLevel power_level;
    FastDataRate data_rate;
  } xy_data_rate,
  z_data_rate;
  SlowDataRate slow_rate;
} FastDataRateContainer;

typedef union {
  SlowDataRateContainer slow;
  FastDataRateContainer fast;
} DataRate;

typedef enum {
  Slow,
  Fast,
} DataRateType;

typedef struct {
  DataRateType type;
  DataRate rate;
} DataRateContainer;

Lis3mdlConfig update_data_rate(Lis3mdlConfig original, DataRateContainer new_rate) {
  PowerLevel xy_power_level, z_power_level;
  SlowDataRate slow_rate;
  original.enable_fast_data_rate = new_rate.type;
  switch (new_rate.type) {
    case Fast:
      xy_power_level = new_rate.rate.fast.xy_data_rate.power_level;
      z_power_level = new_rate.rate.fast.z_data_rate.power_level;
      slow_rate = new_rate.rate.fast.slow_rate;
      break;
    case Slow:
      xy_power_level = new_rate.rate.slow.xy_power_level;
      z_power_level = new_rate.rate.slow.z_power_level;
      slow_rate = new_rate.rate.rate;
  }
  original.slow_data_rate = slow_rate;
  original.xy_power_level = xy_power_level;
  original.z_power_level = z_power_level;
  return original;
}


