typedef struct {
  // embedded functions reg A
  bool mlc_before_fsm: 1,
  : 1,
  enable_sig_motion_detection: 1,
  enable_tilt_detection: 1,
  enable_pedometer: 1,  
  : 1,
  enable_sflp_game: 1,
  : 1;
  // embedded functions reg B
  u8: 3;
  bool mlc_after_fsm: 1,
  fifo_compression: 1;
  u8: 2;
  bool enable_fsm: 1; 
} Lsm6dsv16xFunctionConfig;

typedef struct {
} Lsm6dsv16xInterruptConfig;
