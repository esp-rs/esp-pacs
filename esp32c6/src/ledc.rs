#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration register 0 for channel %s"]
    pub ch0_conf0: CH_CONF0,
    #[doc = "0x04 - High point register for channel %s"]
    pub ch0_hpoint: CH_HPOINT,
    #[doc = "0x08 - Initial duty cycle for channel %s"]
    pub ch0_duty: CH_DUTY,
    #[doc = "0x0c - Configuration register 1 for channel %s"]
    pub ch0_conf1: CH_CONF1,
    #[doc = "0x10 - Current duty cycle for channel %s"]
    pub ch0_duty_r: CH_DUTY_R,
    #[doc = "0x14 - Configuration register 0 for channel %s"]
    pub ch1_conf0: CH_CONF0,
    #[doc = "0x18 - High point register for channel %s"]
    pub ch1_hpoint: CH_HPOINT,
    #[doc = "0x1c - Initial duty cycle for channel %s"]
    pub ch1_duty: CH_DUTY,
    #[doc = "0x20 - Configuration register 1 for channel %s"]
    pub ch1_conf1: CH_CONF1,
    #[doc = "0x24 - Current duty cycle for channel %s"]
    pub ch1_duty_r: CH_DUTY_R,
    #[doc = "0x28 - Configuration register 0 for channel %s"]
    pub ch2_conf0: CH_CONF0,
    #[doc = "0x2c - High point register for channel %s"]
    pub ch2_hpoint: CH_HPOINT,
    #[doc = "0x30 - Initial duty cycle for channel %s"]
    pub ch2_duty: CH_DUTY,
    #[doc = "0x34 - Configuration register 1 for channel %s"]
    pub ch2_conf1: CH_CONF1,
    #[doc = "0x38 - Current duty cycle for channel %s"]
    pub ch2_duty_r: CH_DUTY_R,
    #[doc = "0x3c - Configuration register 0 for channel %s"]
    pub ch3_conf0: CH_CONF0,
    #[doc = "0x40 - High point register for channel %s"]
    pub ch3_hpoint: CH_HPOINT,
    #[doc = "0x44 - Initial duty cycle for channel %s"]
    pub ch3_duty: CH_DUTY,
    #[doc = "0x48 - Configuration register 1 for channel %s"]
    pub ch3_conf1: CH_CONF1,
    #[doc = "0x4c - Current duty cycle for channel %s"]
    pub ch3_duty_r: CH_DUTY_R,
    #[doc = "0x50 - Configuration register 0 for channel %s"]
    pub ch4_conf0: CH_CONF0,
    #[doc = "0x54 - High point register for channel %s"]
    pub ch4_hpoint: CH_HPOINT,
    #[doc = "0x58 - Initial duty cycle for channel %s"]
    pub ch4_duty: CH_DUTY,
    #[doc = "0x5c - Configuration register 1 for channel %s"]
    pub ch4_conf1: CH_CONF1,
    #[doc = "0x60 - Current duty cycle for channel %s"]
    pub ch4_duty_r: CH_DUTY_R,
    #[doc = "0x64 - Configuration register 0 for channel %s"]
    pub ch5_conf0: CH_CONF0,
    #[doc = "0x68 - High point register for channel %s"]
    pub ch5_hpoint: CH_HPOINT,
    #[doc = "0x6c - Initial duty cycle for channel %s"]
    pub ch5_duty: CH_DUTY,
    #[doc = "0x70 - Configuration register 1 for channel %s"]
    pub ch5_conf1: CH_CONF1,
    #[doc = "0x74 - Current duty cycle for channel %s"]
    pub ch5_duty_r: CH_DUTY_R,
    _reserved30: [u8; 0x28],
    #[doc = "0xa0 - Timer %s configuration"]
    pub timer0_conf: TIMER_CONF,
    #[doc = "0xa4 - Timer %s current counter value"]
    pub timer0_value: TIMER_VALUE,
    #[doc = "0xa8 - Timer %s configuration"]
    pub timer1_conf: TIMER_CONF,
    #[doc = "0xac - Timer %s current counter value"]
    pub timer1_value: TIMER_VALUE,
    #[doc = "0xb0 - Timer %s configuration"]
    pub timer2_conf: TIMER_CONF,
    #[doc = "0xb4 - Timer %s current counter value"]
    pub timer2_value: TIMER_VALUE,
    #[doc = "0xb8 - Timer %s configuration"]
    pub timer3_conf: TIMER_CONF,
    #[doc = "0xbc - Timer %s current counter value"]
    pub timer3_value: TIMER_VALUE,
    #[doc = "0xc0 - Raw interrupt status"]
    pub int_raw: INT_RAW,
    #[doc = "0xc4 - Masked interrupt status"]
    pub int_st: INT_ST,
    #[doc = "0xc8 - Interrupt enable bits"]
    pub int_ena: INT_ENA,
    #[doc = "0xcc - Interrupt clear bits"]
    pub int_clr: INT_CLR,
    _reserved42: [u8; 0x30],
    #[doc = "0x100 - Ledc ch%s gamma ram write register."]
    pub ch0_gamma_wr: CH_GAMMA_WR,
    #[doc = "0x104 - Ledc ch%s gamma ram write address register."]
    pub ch0_gamma_wr_addr: CH_GAMMA_WR_ADDR,
    #[doc = "0x108 - Ledc ch%s gamma ram read address register."]
    pub ch0_gamma_rd_addr: CH_GAMMA_RD_ADDR,
    #[doc = "0x10c - Ledc ch%s gamma ram read data register."]
    pub ch0_gamma_rd_data: CH_GAMMA_RD_DATA,
    #[doc = "0x110 - Ledc ch%s gamma ram write register."]
    pub ch1_gamma_wr: CH_GAMMA_WR,
    #[doc = "0x114 - Ledc ch%s gamma ram write address register."]
    pub ch1_gamma_wr_addr: CH_GAMMA_WR_ADDR,
    #[doc = "0x118 - Ledc ch%s gamma ram read address register."]
    pub ch1_gamma_rd_addr: CH_GAMMA_RD_ADDR,
    #[doc = "0x11c - Ledc ch%s gamma ram read data register."]
    pub ch1_gamma_rd_data: CH_GAMMA_RD_DATA,
    #[doc = "0x120 - Ledc ch%s gamma ram write register."]
    pub ch2_gamma_wr: CH_GAMMA_WR,
    #[doc = "0x124 - Ledc ch%s gamma ram write address register."]
    pub ch2_gamma_wr_addr: CH_GAMMA_WR_ADDR,
    #[doc = "0x128 - Ledc ch%s gamma ram read address register."]
    pub ch2_gamma_rd_addr: CH_GAMMA_RD_ADDR,
    #[doc = "0x12c - Ledc ch%s gamma ram read data register."]
    pub ch2_gamma_rd_data: CH_GAMMA_RD_DATA,
    #[doc = "0x130 - Ledc ch%s gamma ram write register."]
    pub ch3_gamma_wr: CH_GAMMA_WR,
    #[doc = "0x134 - Ledc ch%s gamma ram write address register."]
    pub ch3_gamma_wr_addr: CH_GAMMA_WR_ADDR,
    #[doc = "0x138 - Ledc ch%s gamma ram read address register."]
    pub ch3_gamma_rd_addr: CH_GAMMA_RD_ADDR,
    #[doc = "0x13c - Ledc ch%s gamma ram read data register."]
    pub ch3_gamma_rd_data: CH_GAMMA_RD_DATA,
    #[doc = "0x140 - Ledc ch%s gamma ram write register."]
    pub ch4_gamma_wr: CH_GAMMA_WR,
    #[doc = "0x144 - Ledc ch%s gamma ram write address register."]
    pub ch4_gamma_wr_addr: CH_GAMMA_WR_ADDR,
    #[doc = "0x148 - Ledc ch%s gamma ram read address register."]
    pub ch4_gamma_rd_addr: CH_GAMMA_RD_ADDR,
    #[doc = "0x14c - Ledc ch%s gamma ram read data register."]
    pub ch4_gamma_rd_data: CH_GAMMA_RD_DATA,
    #[doc = "0x150 - Ledc ch%s gamma ram write register."]
    pub ch5_gamma_wr: CH_GAMMA_WR,
    #[doc = "0x154 - Ledc ch%s gamma ram write address register."]
    pub ch5_gamma_wr_addr: CH_GAMMA_WR_ADDR,
    #[doc = "0x158 - Ledc ch%s gamma ram read address register."]
    pub ch5_gamma_rd_addr: CH_GAMMA_RD_ADDR,
    #[doc = "0x15c - Ledc ch%s gamma ram read data register."]
    pub ch5_gamma_rd_data: CH_GAMMA_RD_DATA,
    _reserved66: [u8; 0x20],
    #[doc = "0x180..0x198 - Ledc ch%s gamma config register."]
    pub ch_gamma_conf: [CH_GAMMA_CONF; 6],
    _reserved67: [u8; 0x08],
    #[doc = "0x1a0 - Ledc event task enable bit register0."]
    pub evt_task_en0: EVT_TASK_EN0,
    #[doc = "0x1a4 - Ledc event task enable bit register1."]
    pub evt_task_en1: EVT_TASK_EN1,
    #[doc = "0x1a8 - Ledc event task enable bit register2."]
    pub evt_task_en2: EVT_TASK_EN2,
    _reserved70: [u8; 0x04],
    #[doc = "0x1b0..0x1c0 - Ledc timer%s compare value register."]
    pub timer_cmp: [TIMER_CMP; 4],
    #[doc = "0x1c0..0x1d0 - Ledc timer%s count value capture register."]
    pub timer_cnt_cap: [TIMER_CNT_CAP; 4],
    _reserved72: [u8; 0x20],
    #[doc = "0x1f0 - Global ledc configuration register"]
    pub conf: CONF,
    _reserved73: [u8; 0x08],
    #[doc = "0x1fc - Version control register"]
    pub date: DATE,
}
#[doc = "CH_CONF0 (rw) register accessor: an alias for `Reg<CH_CONF0_SPEC>`"]
pub type CH_CONF0 = crate::Reg<ch_conf0::CH_CONF0_SPEC>;
#[doc = "Configuration register 0 for channel %s"]
pub mod ch_conf0;
#[doc = "CH_HPOINT (rw) register accessor: an alias for `Reg<CH_HPOINT_SPEC>`"]
pub type CH_HPOINT = crate::Reg<ch_hpoint::CH_HPOINT_SPEC>;
#[doc = "High point register for channel %s"]
pub mod ch_hpoint;
#[doc = "CH_DUTY (rw) register accessor: an alias for `Reg<CH_DUTY_SPEC>`"]
pub type CH_DUTY = crate::Reg<ch_duty::CH_DUTY_SPEC>;
#[doc = "Initial duty cycle for channel %s"]
pub mod ch_duty;
#[doc = "CH_CONF1 (rw) register accessor: an alias for `Reg<CH_CONF1_SPEC>`"]
pub type CH_CONF1 = crate::Reg<ch_conf1::CH_CONF1_SPEC>;
#[doc = "Configuration register 1 for channel %s"]
pub mod ch_conf1;
#[doc = "CH_DUTY_R (r) register accessor: an alias for `Reg<CH_DUTY_R_SPEC>`"]
pub type CH_DUTY_R = crate::Reg<ch_duty_r::CH_DUTY_R_SPEC>;
#[doc = "Current duty cycle for channel %s"]
pub mod ch_duty_r;
#[doc = "TIMER_CONF (rw) register accessor: an alias for `Reg<TIMER_CONF_SPEC>`"]
pub type TIMER_CONF = crate::Reg<timer_conf::TIMER_CONF_SPEC>;
#[doc = "Timer %s configuration"]
pub mod timer_conf;
#[doc = "TIMER_VALUE (r) register accessor: an alias for `Reg<TIMER_VALUE_SPEC>`"]
pub type TIMER_VALUE = crate::Reg<timer_value::TIMER_VALUE_SPEC>;
#[doc = "Timer %s current counter value"]
pub mod timer_value;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Raw interrupt status"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Masked interrupt status"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable bits"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear bits"]
pub mod int_clr;
#[doc = "CH_GAMMA_WR (rw) register accessor: an alias for `Reg<CH_GAMMA_WR_SPEC>`"]
pub type CH_GAMMA_WR = crate::Reg<ch_gamma_wr::CH_GAMMA_WR_SPEC>;
#[doc = "Ledc ch%s gamma ram write register."]
pub mod ch_gamma_wr;
#[doc = "CH_GAMMA_WR_ADDR (rw) register accessor: an alias for `Reg<CH_GAMMA_WR_ADDR_SPEC>`"]
pub type CH_GAMMA_WR_ADDR = crate::Reg<ch_gamma_wr_addr::CH_GAMMA_WR_ADDR_SPEC>;
#[doc = "Ledc ch%s gamma ram write address register."]
pub mod ch_gamma_wr_addr;
#[doc = "CH_GAMMA_RD_ADDR (rw) register accessor: an alias for `Reg<CH_GAMMA_RD_ADDR_SPEC>`"]
pub type CH_GAMMA_RD_ADDR = crate::Reg<ch_gamma_rd_addr::CH_GAMMA_RD_ADDR_SPEC>;
#[doc = "Ledc ch%s gamma ram read address register."]
pub mod ch_gamma_rd_addr;
#[doc = "CH_GAMMA_RD_DATA (r) register accessor: an alias for `Reg<CH_GAMMA_RD_DATA_SPEC>`"]
pub type CH_GAMMA_RD_DATA = crate::Reg<ch_gamma_rd_data::CH_GAMMA_RD_DATA_SPEC>;
#[doc = "Ledc ch%s gamma ram read data register."]
pub mod ch_gamma_rd_data;
#[doc = "CH_GAMMA_CONF (rw) register accessor: an alias for `Reg<CH_GAMMA_CONF_SPEC>`"]
pub type CH_GAMMA_CONF = crate::Reg<ch_gamma_conf::CH_GAMMA_CONF_SPEC>;
#[doc = "Ledc ch%s gamma config register."]
pub mod ch_gamma_conf;
#[doc = "EVT_TASK_EN0 (rw) register accessor: an alias for `Reg<EVT_TASK_EN0_SPEC>`"]
pub type EVT_TASK_EN0 = crate::Reg<evt_task_en0::EVT_TASK_EN0_SPEC>;
#[doc = "Ledc event task enable bit register0."]
pub mod evt_task_en0;
#[doc = "EVT_TASK_EN1 (rw) register accessor: an alias for `Reg<EVT_TASK_EN1_SPEC>`"]
pub type EVT_TASK_EN1 = crate::Reg<evt_task_en1::EVT_TASK_EN1_SPEC>;
#[doc = "Ledc event task enable bit register1."]
pub mod evt_task_en1;
#[doc = "EVT_TASK_EN2 (rw) register accessor: an alias for `Reg<EVT_TASK_EN2_SPEC>`"]
pub type EVT_TASK_EN2 = crate::Reg<evt_task_en2::EVT_TASK_EN2_SPEC>;
#[doc = "Ledc event task enable bit register2."]
pub mod evt_task_en2;
#[doc = "TIMER_CMP (rw) register accessor: an alias for `Reg<TIMER_CMP_SPEC>`"]
pub type TIMER_CMP = crate::Reg<timer_cmp::TIMER_CMP_SPEC>;
#[doc = "Ledc timer%s compare value register."]
pub mod timer_cmp;
#[doc = "TIMER_CNT_CAP (r) register accessor: an alias for `Reg<TIMER_CNT_CAP_SPEC>`"]
pub type TIMER_CNT_CAP = crate::Reg<timer_cnt_cap::TIMER_CNT_CAP_SPEC>;
#[doc = "Ledc timer%s count value capture register."]
pub mod timer_cnt_cap;
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Global ledc configuration register"]
pub mod conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
