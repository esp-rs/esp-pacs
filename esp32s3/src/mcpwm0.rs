#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM clock prescaler register."]
    pub clk_cfg: CLK_CFG,
    #[doc = "0x04 - PWM timer0 period and update method configuration register."]
    pub timer0_cfg0: TIMER0_CFG0,
    #[doc = "0x08 - PWM timer0 working mode and start/stop control configuration register."]
    pub timer0_cfg1: TIMER0_CFG1,
    #[doc = "0x0c - PWM timer0 sync function configuration register."]
    pub timer0_sync: TIMER0_SYNC,
    #[doc = "0x10 - PWM timer0 status register."]
    pub timer0_status: TIMER0_STATUS,
    #[doc = "0x14 - PWM timer1 period and update method configuration register."]
    pub timer1_cfg0: TIMER1_CFG0,
    #[doc = "0x18 - PWM timer1 working mode and start/stop control configuration register."]
    pub timer1_cfg1: TIMER1_CFG1,
    #[doc = "0x1c - PWM timer1 sync function configuration register."]
    pub timer1_sync: TIMER1_SYNC,
    #[doc = "0x20 - PWM timer1 status register."]
    pub timer1_status: TIMER1_STATUS,
    #[doc = "0x24 - PWM timer2 period and update method configuration register."]
    pub timer2_cfg0: TIMER2_CFG0,
    #[doc = "0x28 - PWM timer2 working mode and start/stop control configuration register."]
    pub timer2_cfg1: TIMER2_CFG1,
    #[doc = "0x2c - PWM timer2 sync function configuration register."]
    pub timer2_sync: TIMER2_SYNC,
    #[doc = "0x30 - PWM timer2 status register."]
    pub timer2_status: TIMER2_STATUS,
    #[doc = "0x34 - Synchronization input selection for three PWM timers."]
    pub timer_synci_cfg: TIMER_SYNCI_CFG,
    #[doc = "0x38 - Select specific timer for PWM operators."]
    pub operator_timersel: OPERATOR_TIMERSEL,
    #[doc = "0x3c - Transfer status and update method for time stamp registers A and B"]
    pub cmpr0_cfg: CMPR0_CFG,
    #[doc = "0x40 - Shadow register for register A."]
    pub cmpr0_value0: CMPR0_VALUE0,
    #[doc = "0x44 - Shadow register for register B."]
    pub cmpr0_value1: CMPR0_VALUE1,
    #[doc = "0x48 - Fault event T0 and T1 handling"]
    pub gen0_cfg0: GEN0_CFG0,
    #[doc = "0x4c - Permissives to force PWM0A and PWM0B outputs by software"]
    pub gen0_force: GEN0_FORCE,
    #[doc = "0x50 - Actions triggered by events on PWM0A"]
    pub gen0_a: GEN0_A,
    #[doc = "0x54 - Actions triggered by events on PWM0B"]
    pub gen0_b: GEN0_B,
    #[doc = "0x58 - dead time type selection and configuration"]
    pub db0_cfg: DB0_CFG,
    #[doc = "0x5c - Shadow register for falling edge delay (FED)."]
    pub db0_fed_cfg: DB0_FED_CFG,
    #[doc = "0x60 - Shadow register for rising edge delay (RED)."]
    pub db0_red_cfg: DB0_RED_CFG,
    #[doc = "0x64 - Carrier enable and configuratoin"]
    pub chopper0_cfg: CHOPPER0_CFG,
    #[doc = "0x68 - Actions on PWM0A and PWM0B trip events"]
    pub tz0_cfg0: TZ0_CFG0,
    #[doc = "0x6c - Software triggers for fault handler actions"]
    pub tz0_cfg1: TZ0_CFG1,
    #[doc = "0x70 - Status of fault events."]
    pub tz0_status: TZ0_STATUS,
    #[doc = "0x74 - Transfer status and update method for time stamp registers A and B"]
    pub cmpr1_cfg: CMPR1_CFG,
    #[doc = "0x78 - Shadow register for register A."]
    pub cmpr1_value0: CMPR1_VALUE0,
    #[doc = "0x7c - Shadow register for register B."]
    pub cmpr1_value1: CMPR1_VALUE1,
    #[doc = "0x80 - Fault event T0 and T1 handling"]
    pub gen1_cfg0: GEN1_CFG0,
    #[doc = "0x84 - Permissives to force PWM1A and PWM1B outputs by software"]
    pub gen1_force: GEN1_FORCE,
    #[doc = "0x88 - Actions triggered by events on PWM1A"]
    pub gen1_a: GEN1_A,
    #[doc = "0x8c - Actions triggered by events on PWM1B"]
    pub gen1_b: GEN1_B,
    #[doc = "0x90 - dead time type selection and configuration"]
    pub db1_cfg: DB1_CFG,
    #[doc = "0x94 - Shadow register for falling edge delay (FED)."]
    pub db1_fed_cfg: DB1_FED_CFG,
    #[doc = "0x98 - Shadow register for rising edge delay (RED)."]
    pub db1_red_cfg: DB1_RED_CFG,
    #[doc = "0x9c - Carrier enable and configuratoin"]
    pub chopper1_cfg: CHOPPER1_CFG,
    #[doc = "0xa0 - Actions on PWM1A and PWM1B trip events"]
    pub tz1_cfg0: TZ1_CFG0,
    #[doc = "0xa4 - Software triggers for fault handler actions"]
    pub tz1_cfg1: TZ1_CFG1,
    #[doc = "0xa8 - Status of fault events."]
    pub tz1_status: TZ1_STATUS,
    #[doc = "0xac - Transfer status and update method for time stamp registers A and B"]
    pub cmpr2_cfg: CMPR2_CFG,
    #[doc = "0xb0 - Shadow register for register A."]
    pub cmpr2_value0: CMPR2_VALUE0,
    #[doc = "0xb4 - Shadow register for register B."]
    pub cmpr2_value1: CMPR2_VALUE1,
    #[doc = "0xb8 - Fault event T0 and T1 handling"]
    pub gen2_cfg0: GEN2_CFG0,
    #[doc = "0xbc - Permissives to force PWM2A and PWM2B outputs by software"]
    pub gen2_force: GEN2_FORCE,
    #[doc = "0xc0 - Actions triggered by events on PWM2A"]
    pub gen2_a: GEN2_A,
    #[doc = "0xc4 - Actions triggered by events on PWM2B"]
    pub gen2_b: GEN2_B,
    #[doc = "0xc8 - dead time type selection and configuration"]
    pub db2_cfg: DB2_CFG,
    #[doc = "0xcc - Shadow register for falling edge delay (FED)."]
    pub db2_fed_cfg: DB2_FED_CFG,
    #[doc = "0xd0 - Shadow register for rising edge delay (RED)."]
    pub db2_red_cfg: DB2_RED_CFG,
    #[doc = "0xd4 - Carrier enable and configuratoin"]
    pub chopper2_cfg: CHOPPER2_CFG,
    #[doc = "0xd8 - Actions on PWM2A and PWM2B trip events"]
    pub tz2_cfg0: TZ2_CFG0,
    #[doc = "0xdc - Software triggers for fault handler actions"]
    pub tz2_cfg1: TZ2_CFG1,
    #[doc = "0xe0 - Status of fault events."]
    pub tz2_status: TZ2_STATUS,
    #[doc = "0xe4 - Fault detection configuration and status"]
    pub fault_detect: FAULT_DETECT,
    #[doc = "0xe8 - Configure capture timer"]
    pub cap_timer_cfg: CAP_TIMER_CFG,
    #[doc = "0xec - Phase for capture timer sync"]
    pub cap_timer_phase: CAP_TIMER_PHASE,
    #[doc = "0xf0 - Capture channel 0 configuration and enable"]
    pub cap_ch0_cfg: CAP_CH0_CFG,
    #[doc = "0xf4 - Capture channel 1 configuration and enable"]
    pub cap_ch1_cfg: CAP_CH1_CFG,
    #[doc = "0xf8 - Capture channel 2 configuration and enable"]
    pub cap_ch2_cfg: CAP_CH2_CFG,
    #[doc = "0xfc - Value of last capture on channel 0"]
    pub cap_ch0: CAP_CH0,
    #[doc = "0x100 - Value of last capture on channel 1"]
    pub cap_ch1: CAP_CH1,
    #[doc = "0x104 - Value of last capture on channel 2"]
    pub cap_ch2: CAP_CH2,
    #[doc = "0x108 - Edge of last capture trigger"]
    pub cap_status: CAP_STATUS,
    #[doc = "0x10c - Enable update."]
    pub update_cfg: UPDATE_CFG,
    #[doc = "0x110 - Interrupt enable bits"]
    pub int_ena: INT_ENA,
    #[doc = "0x114 - Raw interrupt status"]
    pub int_raw: INT_RAW,
    #[doc = "0x118 - Masked interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0x11c - Interrupt clear bits"]
    pub int_clr: INT_CLR,
    #[doc = "0x120 - MCPWM APB configuration register"]
    pub clk: CLK,
    #[doc = "0x124 - Version register."]
    pub version: VERSION,
}
#[doc = "CLK_CFG (rw) register accessor: an alias for `Reg<CLK_CFG_SPEC>`"]
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
#[doc = "PWM clock prescaler register."]
pub mod clk_cfg;
#[doc = "TIMER0_CFG0 (rw) register accessor: an alias for `Reg<TIMER0_CFG0_SPEC>`"]
pub type TIMER0_CFG0 = crate::Reg<timer0_cfg0::TIMER0_CFG0_SPEC>;
#[doc = "PWM timer0 period and update method configuration register."]
pub mod timer0_cfg0;
#[doc = "TIMER0_CFG1 (rw) register accessor: an alias for `Reg<TIMER0_CFG1_SPEC>`"]
pub type TIMER0_CFG1 = crate::Reg<timer0_cfg1::TIMER0_CFG1_SPEC>;
#[doc = "PWM timer0 working mode and start/stop control configuration register."]
pub mod timer0_cfg1;
#[doc = "TIMER0_SYNC (rw) register accessor: an alias for `Reg<TIMER0_SYNC_SPEC>`"]
pub type TIMER0_SYNC = crate::Reg<timer0_sync::TIMER0_SYNC_SPEC>;
#[doc = "PWM timer0 sync function configuration register."]
pub mod timer0_sync;
#[doc = "TIMER0_STATUS (r) register accessor: an alias for `Reg<TIMER0_STATUS_SPEC>`"]
pub type TIMER0_STATUS = crate::Reg<timer0_status::TIMER0_STATUS_SPEC>;
#[doc = "PWM timer0 status register."]
pub mod timer0_status;
#[doc = "TIMER1_CFG0 (rw) register accessor: an alias for `Reg<TIMER1_CFG0_SPEC>`"]
pub type TIMER1_CFG0 = crate::Reg<timer1_cfg0::TIMER1_CFG0_SPEC>;
#[doc = "PWM timer1 period and update method configuration register."]
pub mod timer1_cfg0;
#[doc = "TIMER1_CFG1 (rw) register accessor: an alias for `Reg<TIMER1_CFG1_SPEC>`"]
pub type TIMER1_CFG1 = crate::Reg<timer1_cfg1::TIMER1_CFG1_SPEC>;
#[doc = "PWM timer1 working mode and start/stop control configuration register."]
pub mod timer1_cfg1;
#[doc = "TIMER1_SYNC (rw) register accessor: an alias for `Reg<TIMER1_SYNC_SPEC>`"]
pub type TIMER1_SYNC = crate::Reg<timer1_sync::TIMER1_SYNC_SPEC>;
#[doc = "PWM timer1 sync function configuration register."]
pub mod timer1_sync;
#[doc = "TIMER1_STATUS (r) register accessor: an alias for `Reg<TIMER1_STATUS_SPEC>`"]
pub type TIMER1_STATUS = crate::Reg<timer1_status::TIMER1_STATUS_SPEC>;
#[doc = "PWM timer1 status register."]
pub mod timer1_status;
#[doc = "TIMER2_CFG0 (rw) register accessor: an alias for `Reg<TIMER2_CFG0_SPEC>`"]
pub type TIMER2_CFG0 = crate::Reg<timer2_cfg0::TIMER2_CFG0_SPEC>;
#[doc = "PWM timer2 period and update method configuration register."]
pub mod timer2_cfg0;
#[doc = "TIMER2_CFG1 (rw) register accessor: an alias for `Reg<TIMER2_CFG1_SPEC>`"]
pub type TIMER2_CFG1 = crate::Reg<timer2_cfg1::TIMER2_CFG1_SPEC>;
#[doc = "PWM timer2 working mode and start/stop control configuration register."]
pub mod timer2_cfg1;
#[doc = "TIMER2_SYNC (rw) register accessor: an alias for `Reg<TIMER2_SYNC_SPEC>`"]
pub type TIMER2_SYNC = crate::Reg<timer2_sync::TIMER2_SYNC_SPEC>;
#[doc = "PWM timer2 sync function configuration register."]
pub mod timer2_sync;
#[doc = "TIMER2_STATUS (r) register accessor: an alias for `Reg<TIMER2_STATUS_SPEC>`"]
pub type TIMER2_STATUS = crate::Reg<timer2_status::TIMER2_STATUS_SPEC>;
#[doc = "PWM timer2 status register."]
pub mod timer2_status;
#[doc = "TIMER_SYNCI_CFG (rw) register accessor: an alias for `Reg<TIMER_SYNCI_CFG_SPEC>`"]
pub type TIMER_SYNCI_CFG = crate::Reg<timer_synci_cfg::TIMER_SYNCI_CFG_SPEC>;
#[doc = "Synchronization input selection for three PWM timers."]
pub mod timer_synci_cfg;
#[doc = "OPERATOR_TIMERSEL (rw) register accessor: an alias for `Reg<OPERATOR_TIMERSEL_SPEC>`"]
pub type OPERATOR_TIMERSEL = crate::Reg<operator_timersel::OPERATOR_TIMERSEL_SPEC>;
#[doc = "Select specific timer for PWM operators."]
pub mod operator_timersel;
#[doc = "CMPR0_CFG (rw) register accessor: an alias for `Reg<CMPR0_CFG_SPEC>`"]
pub type CMPR0_CFG = crate::Reg<cmpr0_cfg::CMPR0_CFG_SPEC>;
#[doc = "Transfer status and update method for time stamp registers A and B"]
pub mod cmpr0_cfg;
#[doc = "CMPR0_VALUE0 (rw) register accessor: an alias for `Reg<CMPR0_VALUE0_SPEC>`"]
pub type CMPR0_VALUE0 = crate::Reg<cmpr0_value0::CMPR0_VALUE0_SPEC>;
#[doc = "Shadow register for register A."]
pub mod cmpr0_value0;
#[doc = "CMPR0_VALUE1 (rw) register accessor: an alias for `Reg<CMPR0_VALUE1_SPEC>`"]
pub type CMPR0_VALUE1 = crate::Reg<cmpr0_value1::CMPR0_VALUE1_SPEC>;
#[doc = "Shadow register for register B."]
pub mod cmpr0_value1;
#[doc = "GEN0_CFG0 (rw) register accessor: an alias for `Reg<GEN0_CFG0_SPEC>`"]
pub type GEN0_CFG0 = crate::Reg<gen0_cfg0::GEN0_CFG0_SPEC>;
#[doc = "Fault event T0 and T1 handling"]
pub mod gen0_cfg0;
#[doc = "GEN0_FORCE (rw) register accessor: an alias for `Reg<GEN0_FORCE_SPEC>`"]
pub type GEN0_FORCE = crate::Reg<gen0_force::GEN0_FORCE_SPEC>;
#[doc = "Permissives to force PWM0A and PWM0B outputs by software"]
pub mod gen0_force;
#[doc = "GEN0_A (rw) register accessor: an alias for `Reg<GEN0_A_SPEC>`"]
pub type GEN0_A = crate::Reg<gen0_a::GEN0_A_SPEC>;
#[doc = "Actions triggered by events on PWM0A"]
pub mod gen0_a;
#[doc = "GEN0_B (rw) register accessor: an alias for `Reg<GEN0_B_SPEC>`"]
pub type GEN0_B = crate::Reg<gen0_b::GEN0_B_SPEC>;
#[doc = "Actions triggered by events on PWM0B"]
pub mod gen0_b;
#[doc = "DB0_CFG (rw) register accessor: an alias for `Reg<DB0_CFG_SPEC>`"]
pub type DB0_CFG = crate::Reg<db0_cfg::DB0_CFG_SPEC>;
#[doc = "dead time type selection and configuration"]
pub mod db0_cfg;
#[doc = "DB0_FED_CFG (rw) register accessor: an alias for `Reg<DB0_FED_CFG_SPEC>`"]
pub type DB0_FED_CFG = crate::Reg<db0_fed_cfg::DB0_FED_CFG_SPEC>;
#[doc = "Shadow register for falling edge delay (FED)."]
pub mod db0_fed_cfg;
#[doc = "DB0_RED_CFG (rw) register accessor: an alias for `Reg<DB0_RED_CFG_SPEC>`"]
pub type DB0_RED_CFG = crate::Reg<db0_red_cfg::DB0_RED_CFG_SPEC>;
#[doc = "Shadow register for rising edge delay (RED)."]
pub mod db0_red_cfg;
#[doc = "CHOPPER0_CFG (rw) register accessor: an alias for `Reg<CHOPPER0_CFG_SPEC>`"]
pub type CHOPPER0_CFG = crate::Reg<chopper0_cfg::CHOPPER0_CFG_SPEC>;
#[doc = "Carrier enable and configuratoin"]
pub mod chopper0_cfg;
#[doc = "TZ0_CFG0 (rw) register accessor: an alias for `Reg<TZ0_CFG0_SPEC>`"]
pub type TZ0_CFG0 = crate::Reg<tz0_cfg0::TZ0_CFG0_SPEC>;
#[doc = "Actions on PWM0A and PWM0B trip events"]
pub mod tz0_cfg0;
#[doc = "TZ0_CFG1 (rw) register accessor: an alias for `Reg<TZ0_CFG1_SPEC>`"]
pub type TZ0_CFG1 = crate::Reg<tz0_cfg1::TZ0_CFG1_SPEC>;
#[doc = "Software triggers for fault handler actions"]
pub mod tz0_cfg1;
#[doc = "TZ0_STATUS (r) register accessor: an alias for `Reg<TZ0_STATUS_SPEC>`"]
pub type TZ0_STATUS = crate::Reg<tz0_status::TZ0_STATUS_SPEC>;
#[doc = "Status of fault events."]
pub mod tz0_status;
#[doc = "CMPR1_CFG (rw) register accessor: an alias for `Reg<CMPR1_CFG_SPEC>`"]
pub type CMPR1_CFG = crate::Reg<cmpr1_cfg::CMPR1_CFG_SPEC>;
#[doc = "Transfer status and update method for time stamp registers A and B"]
pub mod cmpr1_cfg;
#[doc = "CMPR1_VALUE0 (rw) register accessor: an alias for `Reg<CMPR1_VALUE0_SPEC>`"]
pub type CMPR1_VALUE0 = crate::Reg<cmpr1_value0::CMPR1_VALUE0_SPEC>;
#[doc = "Shadow register for register A."]
pub mod cmpr1_value0;
#[doc = "CMPR1_VALUE1 (rw) register accessor: an alias for `Reg<CMPR1_VALUE1_SPEC>`"]
pub type CMPR1_VALUE1 = crate::Reg<cmpr1_value1::CMPR1_VALUE1_SPEC>;
#[doc = "Shadow register for register B."]
pub mod cmpr1_value1;
#[doc = "GEN1_CFG0 (rw) register accessor: an alias for `Reg<GEN1_CFG0_SPEC>`"]
pub type GEN1_CFG0 = crate::Reg<gen1_cfg0::GEN1_CFG0_SPEC>;
#[doc = "Fault event T0 and T1 handling"]
pub mod gen1_cfg0;
#[doc = "GEN1_FORCE (rw) register accessor: an alias for `Reg<GEN1_FORCE_SPEC>`"]
pub type GEN1_FORCE = crate::Reg<gen1_force::GEN1_FORCE_SPEC>;
#[doc = "Permissives to force PWM1A and PWM1B outputs by software"]
pub mod gen1_force;
#[doc = "GEN1_A (rw) register accessor: an alias for `Reg<GEN1_A_SPEC>`"]
pub type GEN1_A = crate::Reg<gen1_a::GEN1_A_SPEC>;
#[doc = "Actions triggered by events on PWM1A"]
pub mod gen1_a;
#[doc = "GEN1_B (rw) register accessor: an alias for `Reg<GEN1_B_SPEC>`"]
pub type GEN1_B = crate::Reg<gen1_b::GEN1_B_SPEC>;
#[doc = "Actions triggered by events on PWM1B"]
pub mod gen1_b;
#[doc = "DB1_CFG (rw) register accessor: an alias for `Reg<DB1_CFG_SPEC>`"]
pub type DB1_CFG = crate::Reg<db1_cfg::DB1_CFG_SPEC>;
#[doc = "dead time type selection and configuration"]
pub mod db1_cfg;
#[doc = "DB1_FED_CFG (rw) register accessor: an alias for `Reg<DB1_FED_CFG_SPEC>`"]
pub type DB1_FED_CFG = crate::Reg<db1_fed_cfg::DB1_FED_CFG_SPEC>;
#[doc = "Shadow register for falling edge delay (FED)."]
pub mod db1_fed_cfg;
#[doc = "DB1_RED_CFG (rw) register accessor: an alias for `Reg<DB1_RED_CFG_SPEC>`"]
pub type DB1_RED_CFG = crate::Reg<db1_red_cfg::DB1_RED_CFG_SPEC>;
#[doc = "Shadow register for rising edge delay (RED)."]
pub mod db1_red_cfg;
#[doc = "CHOPPER1_CFG (rw) register accessor: an alias for `Reg<CHOPPER1_CFG_SPEC>`"]
pub type CHOPPER1_CFG = crate::Reg<chopper1_cfg::CHOPPER1_CFG_SPEC>;
#[doc = "Carrier enable and configuratoin"]
pub mod chopper1_cfg;
#[doc = "TZ1_CFG0 (rw) register accessor: an alias for `Reg<TZ1_CFG0_SPEC>`"]
pub type TZ1_CFG0 = crate::Reg<tz1_cfg0::TZ1_CFG0_SPEC>;
#[doc = "Actions on PWM1A and PWM1B trip events"]
pub mod tz1_cfg0;
#[doc = "TZ1_CFG1 (rw) register accessor: an alias for `Reg<TZ1_CFG1_SPEC>`"]
pub type TZ1_CFG1 = crate::Reg<tz1_cfg1::TZ1_CFG1_SPEC>;
#[doc = "Software triggers for fault handler actions"]
pub mod tz1_cfg1;
#[doc = "TZ1_STATUS (r) register accessor: an alias for `Reg<TZ1_STATUS_SPEC>`"]
pub type TZ1_STATUS = crate::Reg<tz1_status::TZ1_STATUS_SPEC>;
#[doc = "Status of fault events."]
pub mod tz1_status;
#[doc = "CMPR2_CFG (rw) register accessor: an alias for `Reg<CMPR2_CFG_SPEC>`"]
pub type CMPR2_CFG = crate::Reg<cmpr2_cfg::CMPR2_CFG_SPEC>;
#[doc = "Transfer status and update method for time stamp registers A and B"]
pub mod cmpr2_cfg;
#[doc = "CMPR2_VALUE0 (rw) register accessor: an alias for `Reg<CMPR2_VALUE0_SPEC>`"]
pub type CMPR2_VALUE0 = crate::Reg<cmpr2_value0::CMPR2_VALUE0_SPEC>;
#[doc = "Shadow register for register A."]
pub mod cmpr2_value0;
#[doc = "CMPR2_VALUE1 (rw) register accessor: an alias for `Reg<CMPR2_VALUE1_SPEC>`"]
pub type CMPR2_VALUE1 = crate::Reg<cmpr2_value1::CMPR2_VALUE1_SPEC>;
#[doc = "Shadow register for register B."]
pub mod cmpr2_value1;
#[doc = "GEN2_CFG0 (rw) register accessor: an alias for `Reg<GEN2_CFG0_SPEC>`"]
pub type GEN2_CFG0 = crate::Reg<gen2_cfg0::GEN2_CFG0_SPEC>;
#[doc = "Fault event T0 and T1 handling"]
pub mod gen2_cfg0;
#[doc = "GEN2_FORCE (rw) register accessor: an alias for `Reg<GEN2_FORCE_SPEC>`"]
pub type GEN2_FORCE = crate::Reg<gen2_force::GEN2_FORCE_SPEC>;
#[doc = "Permissives to force PWM2A and PWM2B outputs by software"]
pub mod gen2_force;
#[doc = "GEN2_A (rw) register accessor: an alias for `Reg<GEN2_A_SPEC>`"]
pub type GEN2_A = crate::Reg<gen2_a::GEN2_A_SPEC>;
#[doc = "Actions triggered by events on PWM2A"]
pub mod gen2_a;
#[doc = "GEN2_B (rw) register accessor: an alias for `Reg<GEN2_B_SPEC>`"]
pub type GEN2_B = crate::Reg<gen2_b::GEN2_B_SPEC>;
#[doc = "Actions triggered by events on PWM2B"]
pub mod gen2_b;
#[doc = "DB2_CFG (rw) register accessor: an alias for `Reg<DB2_CFG_SPEC>`"]
pub type DB2_CFG = crate::Reg<db2_cfg::DB2_CFG_SPEC>;
#[doc = "dead time type selection and configuration"]
pub mod db2_cfg;
#[doc = "DB2_FED_CFG (rw) register accessor: an alias for `Reg<DB2_FED_CFG_SPEC>`"]
pub type DB2_FED_CFG = crate::Reg<db2_fed_cfg::DB2_FED_CFG_SPEC>;
#[doc = "Shadow register for falling edge delay (FED)."]
pub mod db2_fed_cfg;
#[doc = "DB2_RED_CFG (rw) register accessor: an alias for `Reg<DB2_RED_CFG_SPEC>`"]
pub type DB2_RED_CFG = crate::Reg<db2_red_cfg::DB2_RED_CFG_SPEC>;
#[doc = "Shadow register for rising edge delay (RED)."]
pub mod db2_red_cfg;
#[doc = "CHOPPER2_CFG (rw) register accessor: an alias for `Reg<CHOPPER2_CFG_SPEC>`"]
pub type CHOPPER2_CFG = crate::Reg<chopper2_cfg::CHOPPER2_CFG_SPEC>;
#[doc = "Carrier enable and configuratoin"]
pub mod chopper2_cfg;
#[doc = "TZ2_CFG0 (rw) register accessor: an alias for `Reg<TZ2_CFG0_SPEC>`"]
pub type TZ2_CFG0 = crate::Reg<tz2_cfg0::TZ2_CFG0_SPEC>;
#[doc = "Actions on PWM2A and PWM2B trip events"]
pub mod tz2_cfg0;
#[doc = "TZ2_CFG1 (rw) register accessor: an alias for `Reg<TZ2_CFG1_SPEC>`"]
pub type TZ2_CFG1 = crate::Reg<tz2_cfg1::TZ2_CFG1_SPEC>;
#[doc = "Software triggers for fault handler actions"]
pub mod tz2_cfg1;
#[doc = "TZ2_STATUS (r) register accessor: an alias for `Reg<TZ2_STATUS_SPEC>`"]
pub type TZ2_STATUS = crate::Reg<tz2_status::TZ2_STATUS_SPEC>;
#[doc = "Status of fault events."]
pub mod tz2_status;
#[doc = "FAULT_DETECT (rw) register accessor: an alias for `Reg<FAULT_DETECT_SPEC>`"]
pub type FAULT_DETECT = crate::Reg<fault_detect::FAULT_DETECT_SPEC>;
#[doc = "Fault detection configuration and status"]
pub mod fault_detect;
#[doc = "CAP_TIMER_CFG (rw) register accessor: an alias for `Reg<CAP_TIMER_CFG_SPEC>`"]
pub type CAP_TIMER_CFG = crate::Reg<cap_timer_cfg::CAP_TIMER_CFG_SPEC>;
#[doc = "Configure capture timer"]
pub mod cap_timer_cfg;
#[doc = "CAP_TIMER_PHASE (rw) register accessor: an alias for `Reg<CAP_TIMER_PHASE_SPEC>`"]
pub type CAP_TIMER_PHASE = crate::Reg<cap_timer_phase::CAP_TIMER_PHASE_SPEC>;
#[doc = "Phase for capture timer sync"]
pub mod cap_timer_phase;
#[doc = "CAP_CH0_CFG (rw) register accessor: an alias for `Reg<CAP_CH0_CFG_SPEC>`"]
pub type CAP_CH0_CFG = crate::Reg<cap_ch0_cfg::CAP_CH0_CFG_SPEC>;
#[doc = "Capture channel 0 configuration and enable"]
pub mod cap_ch0_cfg;
#[doc = "CAP_CH1_CFG (rw) register accessor: an alias for `Reg<CAP_CH1_CFG_SPEC>`"]
pub type CAP_CH1_CFG = crate::Reg<cap_ch1_cfg::CAP_CH1_CFG_SPEC>;
#[doc = "Capture channel 1 configuration and enable"]
pub mod cap_ch1_cfg;
#[doc = "CAP_CH2_CFG (rw) register accessor: an alias for `Reg<CAP_CH2_CFG_SPEC>`"]
pub type CAP_CH2_CFG = crate::Reg<cap_ch2_cfg::CAP_CH2_CFG_SPEC>;
#[doc = "Capture channel 2 configuration and enable"]
pub mod cap_ch2_cfg;
#[doc = "CAP_CH0 (r) register accessor: an alias for `Reg<CAP_CH0_SPEC>`"]
pub type CAP_CH0 = crate::Reg<cap_ch0::CAP_CH0_SPEC>;
#[doc = "Value of last capture on channel 0"]
pub mod cap_ch0;
#[doc = "CAP_CH1 (r) register accessor: an alias for `Reg<CAP_CH1_SPEC>`"]
pub type CAP_CH1 = crate::Reg<cap_ch1::CAP_CH1_SPEC>;
#[doc = "Value of last capture on channel 1"]
pub mod cap_ch1;
#[doc = "CAP_CH2 (r) register accessor: an alias for `Reg<CAP_CH2_SPEC>`"]
pub type CAP_CH2 = crate::Reg<cap_ch2::CAP_CH2_SPEC>;
#[doc = "Value of last capture on channel 2"]
pub mod cap_ch2;
#[doc = "CAP_STATUS (r) register accessor: an alias for `Reg<CAP_STATUS_SPEC>`"]
pub type CAP_STATUS = crate::Reg<cap_status::CAP_STATUS_SPEC>;
#[doc = "Edge of last capture trigger"]
pub mod cap_status;
#[doc = "UPDATE_CFG (rw) register accessor: an alias for `Reg<UPDATE_CFG_SPEC>`"]
pub type UPDATE_CFG = crate::Reg<update_cfg::UPDATE_CFG_SPEC>;
#[doc = "Enable update."]
pub mod update_cfg;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CLK (rw) register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "MCPWM APB configuration register"]
pub mod clk;
#[doc = "VERSION (rw) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version register."]
pub mod version;
