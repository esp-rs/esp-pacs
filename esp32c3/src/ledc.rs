#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - LEDC_LSCH%s_CONF%s."]
    pub ch0_conf0: CH_CONF0,
    #[doc = "0x04 - LEDC_LSCH%s_HPOINT."]
    pub ch0_hpoint: CH_HPOINT,
    #[doc = "0x08 - LEDC_LSCH%s_DUTY."]
    pub ch0_duty: CH_DUTY,
    #[doc = "0x0c - LEDC_LSCH%s_CONF1."]
    pub ch0_conf1: CH_CONF1,
    #[doc = "0x10 - LEDC_LSCH%s_DUTY_R."]
    pub ch0_duty_r: CH_DUTY_R,
    #[doc = "0x14 - LEDC_LSCH%s_CONF%s."]
    pub ch1_conf0: CH_CONF0,
    #[doc = "0x18 - LEDC_LSCH%s_HPOINT."]
    pub ch1_hpoint: CH_HPOINT,
    #[doc = "0x1c - LEDC_LSCH%s_DUTY."]
    pub ch1_duty: CH_DUTY,
    #[doc = "0x20 - LEDC_LSCH%s_CONF1."]
    pub ch1_conf1: CH_CONF1,
    #[doc = "0x24 - LEDC_LSCH%s_DUTY_R."]
    pub ch1_duty_r: CH_DUTY_R,
    #[doc = "0x28 - LEDC_LSCH%s_CONF%s."]
    pub ch2_conf0: CH_CONF0,
    #[doc = "0x2c - LEDC_LSCH%s_HPOINT."]
    pub ch2_hpoint: CH_HPOINT,
    #[doc = "0x30 - LEDC_LSCH%s_DUTY."]
    pub ch2_duty: CH_DUTY,
    #[doc = "0x34 - LEDC_LSCH%s_CONF1."]
    pub ch2_conf1: CH_CONF1,
    #[doc = "0x38 - LEDC_LSCH%s_DUTY_R."]
    pub ch2_duty_r: CH_DUTY_R,
    #[doc = "0x3c - LEDC_LSCH%s_CONF%s."]
    pub ch3_conf0: CH_CONF0,
    #[doc = "0x40 - LEDC_LSCH%s_HPOINT."]
    pub ch3_hpoint: CH_HPOINT,
    #[doc = "0x44 - LEDC_LSCH%s_DUTY."]
    pub ch3_duty: CH_DUTY,
    #[doc = "0x48 - LEDC_LSCH%s_CONF1."]
    pub ch3_conf1: CH_CONF1,
    #[doc = "0x4c - LEDC_LSCH%s_DUTY_R."]
    pub ch3_duty_r: CH_DUTY_R,
    #[doc = "0x50 - LEDC_LSCH%s_CONF%s."]
    pub ch4_conf0: CH_CONF0,
    #[doc = "0x54 - LEDC_LSCH%s_HPOINT."]
    pub ch4_hpoint: CH_HPOINT,
    #[doc = "0x58 - LEDC_LSCH%s_DUTY."]
    pub ch4_duty: CH_DUTY,
    #[doc = "0x5c - LEDC_LSCH%s_CONF1."]
    pub ch4_conf1: CH_CONF1,
    #[doc = "0x60 - LEDC_LSCH%s_DUTY_R."]
    pub ch4_duty_r: CH_DUTY_R,
    #[doc = "0x64 - LEDC_LSCH%s_CONF%s."]
    pub ch5_conf0: CH_CONF0,
    #[doc = "0x68 - LEDC_LSCH%s_HPOINT."]
    pub ch5_hpoint: CH_HPOINT,
    #[doc = "0x6c - LEDC_LSCH%s_DUTY."]
    pub ch5_duty: CH_DUTY,
    #[doc = "0x70 - LEDC_LSCH%s_CONF1."]
    pub ch5_conf1: CH_CONF1,
    #[doc = "0x74 - LEDC_LSCH%s_DUTY_R."]
    pub ch5_duty_r: CH_DUTY_R,
    _reserved30: [u8; 0x28],
    #[doc = "0xa0 - LEDC_LSTIMER%s_CONF."]
    pub timer0_conf: TIMER_CONF,
    #[doc = "0xa4 - LEDC_LSTIMER%s_VALUE."]
    pub timer0_value: TIMER_VALUE,
    #[doc = "0xa8 - LEDC_LSTIMER%s_CONF."]
    pub timer1_conf: TIMER_CONF,
    #[doc = "0xac - LEDC_LSTIMER%s_VALUE."]
    pub timer1_value: TIMER_VALUE,
    #[doc = "0xb0 - LEDC_LSTIMER%s_CONF."]
    pub timer2_conf: TIMER_CONF,
    #[doc = "0xb4 - LEDC_LSTIMER%s_VALUE."]
    pub timer2_value: TIMER_VALUE,
    #[doc = "0xb8 - LEDC_LSTIMER%s_CONF."]
    pub timer3_conf: TIMER_CONF,
    #[doc = "0xbc - LEDC_LSTIMER%s_VALUE."]
    pub timer3_value: TIMER_VALUE,
    #[doc = "0xc0 - LEDC_INT_RAW."]
    pub int_raw: INT_RAW,
    #[doc = "0xc4 - LEDC_INT_ST."]
    pub int_st: INT_ST,
    #[doc = "0xc8 - LEDC_INT_ENA."]
    pub int_ena: INT_ENA,
    #[doc = "0xcc - LEDC_INT_CLR."]
    pub int_clr: INT_CLR,
    #[doc = "0xd0 - LEDC_CONF."]
    pub conf: CONF,
    _reserved43: [u8; 0x28],
    #[doc = "0xfc - LEDC_DATE."]
    pub date: DATE,
}
#[doc = "CH_CONF0 (rw) register accessor: an alias for `Reg<CH_CONF0_SPEC>`"]
pub type CH_CONF0 = crate::Reg<ch_conf0::CH_CONF0_SPEC>;
#[doc = "LEDC_LSCH%s_CONF%s."]
pub mod ch_conf0;
#[doc = "CH_HPOINT (rw) register accessor: an alias for `Reg<CH_HPOINT_SPEC>`"]
pub type CH_HPOINT = crate::Reg<ch_hpoint::CH_HPOINT_SPEC>;
#[doc = "LEDC_LSCH%s_HPOINT."]
pub mod ch_hpoint;
#[doc = "CH_DUTY (rw) register accessor: an alias for `Reg<CH_DUTY_SPEC>`"]
pub type CH_DUTY = crate::Reg<ch_duty::CH_DUTY_SPEC>;
#[doc = "LEDC_LSCH%s_DUTY."]
pub mod ch_duty;
#[doc = "CH_CONF1 (rw) register accessor: an alias for `Reg<CH_CONF1_SPEC>`"]
pub type CH_CONF1 = crate::Reg<ch_conf1::CH_CONF1_SPEC>;
#[doc = "LEDC_LSCH%s_CONF1."]
pub mod ch_conf1;
#[doc = "CH_DUTY_R (r) register accessor: an alias for `Reg<CH_DUTY_R_SPEC>`"]
pub type CH_DUTY_R = crate::Reg<ch_duty_r::CH_DUTY_R_SPEC>;
#[doc = "LEDC_LSCH%s_DUTY_R."]
pub mod ch_duty_r;
#[doc = "TIMER_CONF (rw) register accessor: an alias for `Reg<TIMER_CONF_SPEC>`"]
pub type TIMER_CONF = crate::Reg<timer_conf::TIMER_CONF_SPEC>;
#[doc = "LEDC_LSTIMER%s_CONF."]
pub mod timer_conf;
#[doc = "TIMER_VALUE (r) register accessor: an alias for `Reg<TIMER_VALUE_SPEC>`"]
pub type TIMER_VALUE = crate::Reg<timer_value::TIMER_VALUE_SPEC>;
#[doc = "LEDC_LSTIMER%s_VALUE."]
pub mod timer_value;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "LEDC_INT_RAW."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "LEDC_INT_ST."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "LEDC_INT_ENA."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "LEDC_INT_CLR."]
pub mod int_clr;
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "LEDC_CONF."]
pub mod conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "LEDC_DATE."]
pub mod date;
