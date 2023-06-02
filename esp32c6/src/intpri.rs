#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - register description"]
    pub cpu_int_enable: CPU_INT_ENABLE,
    #[doc = "0x04 - register description"]
    pub cpu_int_type: CPU_INT_TYPE,
    #[doc = "0x08 - register description"]
    pub cpu_int_eip_status: CPU_INT_EIP_STATUS,
    #[doc = "0x0c - register description"]
    pub cpu_int_pri_0: CPU_INT_PRI_0,
    #[doc = "0x10 - register description"]
    pub cpu_int_pri_1: CPU_INT_PRI_1,
    #[doc = "0x14 - register description"]
    pub cpu_int_pri_2: CPU_INT_PRI_2,
    #[doc = "0x18 - register description"]
    pub cpu_int_pri_3: CPU_INT_PRI_3,
    #[doc = "0x1c - register description"]
    pub cpu_int_pri_4: CPU_INT_PRI_4,
    #[doc = "0x20 - register description"]
    pub cpu_int_pri_5: CPU_INT_PRI_5,
    #[doc = "0x24 - register description"]
    pub cpu_int_pri_6: CPU_INT_PRI_6,
    #[doc = "0x28 - register description"]
    pub cpu_int_pri_7: CPU_INT_PRI_7,
    #[doc = "0x2c - register description"]
    pub cpu_int_pri_8: CPU_INT_PRI_8,
    #[doc = "0x30 - register description"]
    pub cpu_int_pri_9: CPU_INT_PRI_9,
    #[doc = "0x34 - register description"]
    pub cpu_int_pri_10: CPU_INT_PRI_10,
    #[doc = "0x38 - register description"]
    pub cpu_int_pri_11: CPU_INT_PRI_11,
    #[doc = "0x3c - register description"]
    pub cpu_int_pri_12: CPU_INT_PRI_12,
    #[doc = "0x40 - register description"]
    pub cpu_int_pri_13: CPU_INT_PRI_13,
    #[doc = "0x44 - register description"]
    pub cpu_int_pri_14: CPU_INT_PRI_14,
    #[doc = "0x48 - register description"]
    pub cpu_int_pri_15: CPU_INT_PRI_15,
    #[doc = "0x4c - register description"]
    pub cpu_int_pri_16: CPU_INT_PRI_16,
    #[doc = "0x50 - register description"]
    pub cpu_int_pri_17: CPU_INT_PRI_17,
    #[doc = "0x54 - register description"]
    pub cpu_int_pri_18: CPU_INT_PRI_18,
    #[doc = "0x58 - register description"]
    pub cpu_int_pri_19: CPU_INT_PRI_19,
    #[doc = "0x5c - register description"]
    pub cpu_int_pri_20: CPU_INT_PRI_20,
    #[doc = "0x60 - register description"]
    pub cpu_int_pri_21: CPU_INT_PRI_21,
    #[doc = "0x64 - register description"]
    pub cpu_int_pri_22: CPU_INT_PRI_22,
    #[doc = "0x68 - register description"]
    pub cpu_int_pri_23: CPU_INT_PRI_23,
    #[doc = "0x6c - register description"]
    pub cpu_int_pri_24: CPU_INT_PRI_24,
    #[doc = "0x70 - register description"]
    pub cpu_int_pri_25: CPU_INT_PRI_25,
    #[doc = "0x74 - register description"]
    pub cpu_int_pri_26: CPU_INT_PRI_26,
    #[doc = "0x78 - register description"]
    pub cpu_int_pri_27: CPU_INT_PRI_27,
    #[doc = "0x7c - register description"]
    pub cpu_int_pri_28: CPU_INT_PRI_28,
    #[doc = "0x80 - register description"]
    pub cpu_int_pri_29: CPU_INT_PRI_29,
    #[doc = "0x84 - register description"]
    pub cpu_int_pri_30: CPU_INT_PRI_30,
    #[doc = "0x88 - register description"]
    pub cpu_int_pri_31: CPU_INT_PRI_31,
    #[doc = "0x8c - register description"]
    pub cpu_int_thresh: CPU_INT_THRESH,
    #[doc = "0x90 - register description"]
    pub cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    #[doc = "0x94 - register description"]
    pub cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    #[doc = "0x98 - register description"]
    pub cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    #[doc = "0x9c - register description"]
    pub cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    #[doc = "0xa0 - register description"]
    pub date: DATE,
    #[doc = "0xa4 - register description"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0xa8 - register description"]
    pub cpu_int_clear: CPU_INT_CLEAR,
    #[doc = "0xac - redcy eco register."]
    pub rnd_eco: RND_ECO,
    #[doc = "0xb0 - redcy eco low register."]
    pub rnd_eco_low: RND_ECO_LOW,
    _reserved45: [u8; 0x0348],
    #[doc = "0x3fc - redcy eco high register."]
    pub rnd_eco_high: RND_ECO_HIGH,
}
#[doc = "CPU_INT_ENABLE (rw) register accessor: an alias for `Reg<CPU_INT_ENABLE_SPEC>`"]
pub type CPU_INT_ENABLE = crate::Reg<cpu_int_enable::CPU_INT_ENABLE_SPEC>;
#[doc = "register description"]
pub mod cpu_int_enable;
#[doc = "CPU_INT_TYPE (rw) register accessor: an alias for `Reg<CPU_INT_TYPE_SPEC>`"]
pub type CPU_INT_TYPE = crate::Reg<cpu_int_type::CPU_INT_TYPE_SPEC>;
#[doc = "register description"]
pub mod cpu_int_type;
#[doc = "CPU_INT_EIP_STATUS (r) register accessor: an alias for `Reg<CPU_INT_EIP_STATUS_SPEC>`"]
pub type CPU_INT_EIP_STATUS = crate::Reg<cpu_int_eip_status::CPU_INT_EIP_STATUS_SPEC>;
#[doc = "register description"]
pub mod cpu_int_eip_status;
#[doc = "CPU_INT_PRI_0 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_0_SPEC>`"]
pub type CPU_INT_PRI_0 = crate::Reg<cpu_int_pri_0::CPU_INT_PRI_0_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_0;
#[doc = "CPU_INT_PRI_1 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_1_SPEC>`"]
pub type CPU_INT_PRI_1 = crate::Reg<cpu_int_pri_1::CPU_INT_PRI_1_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_1;
#[doc = "CPU_INT_PRI_2 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_2_SPEC>`"]
pub type CPU_INT_PRI_2 = crate::Reg<cpu_int_pri_2::CPU_INT_PRI_2_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_2;
#[doc = "CPU_INT_PRI_3 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_3_SPEC>`"]
pub type CPU_INT_PRI_3 = crate::Reg<cpu_int_pri_3::CPU_INT_PRI_3_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_3;
#[doc = "CPU_INT_PRI_4 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_4_SPEC>`"]
pub type CPU_INT_PRI_4 = crate::Reg<cpu_int_pri_4::CPU_INT_PRI_4_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_4;
#[doc = "CPU_INT_PRI_5 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_5_SPEC>`"]
pub type CPU_INT_PRI_5 = crate::Reg<cpu_int_pri_5::CPU_INT_PRI_5_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_5;
#[doc = "CPU_INT_PRI_6 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_6_SPEC>`"]
pub type CPU_INT_PRI_6 = crate::Reg<cpu_int_pri_6::CPU_INT_PRI_6_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_6;
#[doc = "CPU_INT_PRI_7 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_7_SPEC>`"]
pub type CPU_INT_PRI_7 = crate::Reg<cpu_int_pri_7::CPU_INT_PRI_7_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_7;
#[doc = "CPU_INT_PRI_8 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_8_SPEC>`"]
pub type CPU_INT_PRI_8 = crate::Reg<cpu_int_pri_8::CPU_INT_PRI_8_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_8;
#[doc = "CPU_INT_PRI_9 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_9_SPEC>`"]
pub type CPU_INT_PRI_9 = crate::Reg<cpu_int_pri_9::CPU_INT_PRI_9_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_9;
#[doc = "CPU_INT_PRI_10 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_10_SPEC>`"]
pub type CPU_INT_PRI_10 = crate::Reg<cpu_int_pri_10::CPU_INT_PRI_10_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_10;
#[doc = "CPU_INT_PRI_11 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_11_SPEC>`"]
pub type CPU_INT_PRI_11 = crate::Reg<cpu_int_pri_11::CPU_INT_PRI_11_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_11;
#[doc = "CPU_INT_PRI_12 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_12_SPEC>`"]
pub type CPU_INT_PRI_12 = crate::Reg<cpu_int_pri_12::CPU_INT_PRI_12_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_12;
#[doc = "CPU_INT_PRI_13 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_13_SPEC>`"]
pub type CPU_INT_PRI_13 = crate::Reg<cpu_int_pri_13::CPU_INT_PRI_13_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_13;
#[doc = "CPU_INT_PRI_14 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_14_SPEC>`"]
pub type CPU_INT_PRI_14 = crate::Reg<cpu_int_pri_14::CPU_INT_PRI_14_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_14;
#[doc = "CPU_INT_PRI_15 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_15_SPEC>`"]
pub type CPU_INT_PRI_15 = crate::Reg<cpu_int_pri_15::CPU_INT_PRI_15_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_15;
#[doc = "CPU_INT_PRI_16 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_16_SPEC>`"]
pub type CPU_INT_PRI_16 = crate::Reg<cpu_int_pri_16::CPU_INT_PRI_16_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_16;
#[doc = "CPU_INT_PRI_17 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_17_SPEC>`"]
pub type CPU_INT_PRI_17 = crate::Reg<cpu_int_pri_17::CPU_INT_PRI_17_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_17;
#[doc = "CPU_INT_PRI_18 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_18_SPEC>`"]
pub type CPU_INT_PRI_18 = crate::Reg<cpu_int_pri_18::CPU_INT_PRI_18_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_18;
#[doc = "CPU_INT_PRI_19 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_19_SPEC>`"]
pub type CPU_INT_PRI_19 = crate::Reg<cpu_int_pri_19::CPU_INT_PRI_19_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_19;
#[doc = "CPU_INT_PRI_20 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_20_SPEC>`"]
pub type CPU_INT_PRI_20 = crate::Reg<cpu_int_pri_20::CPU_INT_PRI_20_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_20;
#[doc = "CPU_INT_PRI_21 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_21_SPEC>`"]
pub type CPU_INT_PRI_21 = crate::Reg<cpu_int_pri_21::CPU_INT_PRI_21_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_21;
#[doc = "CPU_INT_PRI_22 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_22_SPEC>`"]
pub type CPU_INT_PRI_22 = crate::Reg<cpu_int_pri_22::CPU_INT_PRI_22_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_22;
#[doc = "CPU_INT_PRI_23 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_23_SPEC>`"]
pub type CPU_INT_PRI_23 = crate::Reg<cpu_int_pri_23::CPU_INT_PRI_23_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_23;
#[doc = "CPU_INT_PRI_24 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_24_SPEC>`"]
pub type CPU_INT_PRI_24 = crate::Reg<cpu_int_pri_24::CPU_INT_PRI_24_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_24;
#[doc = "CPU_INT_PRI_25 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_25_SPEC>`"]
pub type CPU_INT_PRI_25 = crate::Reg<cpu_int_pri_25::CPU_INT_PRI_25_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_25;
#[doc = "CPU_INT_PRI_26 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_26_SPEC>`"]
pub type CPU_INT_PRI_26 = crate::Reg<cpu_int_pri_26::CPU_INT_PRI_26_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_26;
#[doc = "CPU_INT_PRI_27 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_27_SPEC>`"]
pub type CPU_INT_PRI_27 = crate::Reg<cpu_int_pri_27::CPU_INT_PRI_27_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_27;
#[doc = "CPU_INT_PRI_28 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_28_SPEC>`"]
pub type CPU_INT_PRI_28 = crate::Reg<cpu_int_pri_28::CPU_INT_PRI_28_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_28;
#[doc = "CPU_INT_PRI_29 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_29_SPEC>`"]
pub type CPU_INT_PRI_29 = crate::Reg<cpu_int_pri_29::CPU_INT_PRI_29_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_29;
#[doc = "CPU_INT_PRI_30 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_30_SPEC>`"]
pub type CPU_INT_PRI_30 = crate::Reg<cpu_int_pri_30::CPU_INT_PRI_30_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_30;
#[doc = "CPU_INT_PRI_31 (rw) register accessor: an alias for `Reg<CPU_INT_PRI_31_SPEC>`"]
pub type CPU_INT_PRI_31 = crate::Reg<cpu_int_pri_31::CPU_INT_PRI_31_SPEC>;
#[doc = "register description"]
pub mod cpu_int_pri_31;
#[doc = "CPU_INT_THRESH (rw) register accessor: an alias for `Reg<CPU_INT_THRESH_SPEC>`"]
pub type CPU_INT_THRESH = crate::Reg<cpu_int_thresh::CPU_INT_THRESH_SPEC>;
#[doc = "register description"]
pub mod cpu_int_thresh;
#[doc = "CPU_INTR_FROM_CPU_0 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_0_SPEC>`"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_0;
#[doc = "CPU_INTR_FROM_CPU_1 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_1_SPEC>`"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_1;
#[doc = "CPU_INTR_FROM_CPU_2 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_2_SPEC>`"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_2;
#[doc = "CPU_INTR_FROM_CPU_3 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_3_SPEC>`"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = "register description"]
pub mod cpu_intr_from_cpu_3;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "register description"]
pub mod date;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "register description"]
pub mod clock_gate;
#[doc = "CPU_INT_CLEAR (rw) register accessor: an alias for `Reg<CPU_INT_CLEAR_SPEC>`"]
pub type CPU_INT_CLEAR = crate::Reg<cpu_int_clear::CPU_INT_CLEAR_SPEC>;
#[doc = "register description"]
pub mod cpu_int_clear;
#[doc = "RND_ECO (rw) register accessor: an alias for `Reg<RND_ECO_SPEC>`"]
pub type RND_ECO = crate::Reg<rnd_eco::RND_ECO_SPEC>;
#[doc = "redcy eco register."]
pub mod rnd_eco;
#[doc = "RND_ECO_LOW (rw) register accessor: an alias for `Reg<RND_ECO_LOW_SPEC>`"]
pub type RND_ECO_LOW = crate::Reg<rnd_eco_low::RND_ECO_LOW_SPEC>;
#[doc = "redcy eco low register."]
pub mod rnd_eco_low;
#[doc = "RND_ECO_HIGH (rw) register accessor: an alias for `Reg<RND_ECO_HIGH_SPEC>`"]
pub type RND_ECO_HIGH = crate::Reg<rnd_eco_high::RND_ECO_HIGH_SPEC>;
#[doc = "redcy eco high register."]
pub mod rnd_eco_high;
