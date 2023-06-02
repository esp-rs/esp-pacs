#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub bt_select: BT_SELECT,
    #[doc = "0x04 - "]
    pub out: OUT,
    #[doc = "0x08 - "]
    pub out_w1ts: OUT_W1TS,
    #[doc = "0x0c - "]
    pub out_w1tc: OUT_W1TC,
    #[doc = "0x10 - "]
    pub out1: OUT1,
    #[doc = "0x14 - "]
    pub out1_w1ts: OUT1_W1TS,
    #[doc = "0x18 - "]
    pub out1_w1tc: OUT1_W1TC,
    #[doc = "0x1c - "]
    pub sdio_select: SDIO_SELECT,
    #[doc = "0x20 - "]
    pub enable: ENABLE,
    #[doc = "0x24 - "]
    pub enable_w1ts: ENABLE_W1TS,
    #[doc = "0x28 - "]
    pub enable_w1tc: ENABLE_W1TC,
    #[doc = "0x2c - "]
    pub enable1: ENABLE1,
    #[doc = "0x30 - "]
    pub enable1_w1ts: ENABLE1_W1TS,
    #[doc = "0x34 - "]
    pub enable1_w1tc: ENABLE1_W1TC,
    #[doc = "0x38 - "]
    pub strap: STRAP,
    #[doc = "0x3c - "]
    pub in_: IN,
    #[doc = "0x40 - "]
    pub in1: IN1,
    #[doc = "0x44 - "]
    pub status: STATUS,
    #[doc = "0x48 - "]
    pub status_w1ts: STATUS_W1TS,
    #[doc = "0x4c - "]
    pub status_w1tc: STATUS_W1TC,
    #[doc = "0x50 - "]
    pub status1: STATUS1,
    #[doc = "0x54 - "]
    pub status1_w1ts: STATUS1_W1TS,
    #[doc = "0x58 - "]
    pub status1_w1tc: STATUS1_W1TC,
    _reserved23: [u8; 0x04],
    #[doc = "0x60 - "]
    pub acpu_int: ACPU_INT,
    #[doc = "0x64 - "]
    pub acpu_nmi_int: ACPU_NMI_INT,
    #[doc = "0x68 - "]
    pub pcpu_int: PCPU_INT,
    #[doc = "0x6c - "]
    pub pcpu_nmi_int: PCPU_NMI_INT,
    #[doc = "0x70 - "]
    pub cpusdio_int: CPUSDIO_INT,
    #[doc = "0x74 - "]
    pub acpu_int1: ACPU_INT1,
    #[doc = "0x78 - "]
    pub acpu_nmi_int1: ACPU_NMI_INT1,
    #[doc = "0x7c - "]
    pub pcpu_int1: PCPU_INT1,
    #[doc = "0x80 - "]
    pub pcpu_nmi_int1: PCPU_NMI_INT1,
    #[doc = "0x84 - "]
    pub cpusdio_int1: CPUSDIO_INT1,
    #[doc = "0x88..0x128 - "]
    pub pin: [PIN; 40],
    #[doc = "0x128 - "]
    pub cali_conf: CALI_CONF,
    #[doc = "0x12c - "]
    pub cali_data: CALI_DATA,
    #[doc = "0x130..0x530 - "]
    pub func_in_sel_cfg: [FUNC_IN_SEL_CFG; 256],
    #[doc = "0x530..0x5d0 - "]
    pub func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 40],
}
#[doc = "BT_SELECT (rw) register accessor: an alias for `Reg<BT_SELECT_SPEC>`"]
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
#[doc = ""]
pub mod bt_select;
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = ""]
pub mod out;
#[doc = "OUT_W1TS (rw) register accessor: an alias for `Reg<OUT_W1TS_SPEC>`"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = ""]
pub mod out_w1ts;
#[doc = "OUT_W1TC (rw) register accessor: an alias for `Reg<OUT_W1TC_SPEC>`"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = ""]
pub mod out_w1tc;
#[doc = "OUT1 (rw) register accessor: an alias for `Reg<OUT1_SPEC>`"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = ""]
pub mod out1;
#[doc = "OUT1_W1TS (rw) register accessor: an alias for `Reg<OUT1_W1TS_SPEC>`"]
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
#[doc = ""]
pub mod out1_w1ts;
#[doc = "OUT1_W1TC (rw) register accessor: an alias for `Reg<OUT1_W1TC_SPEC>`"]
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
#[doc = ""]
pub mod out1_w1tc;
#[doc = "SDIO_SELECT (rw) register accessor: an alias for `Reg<SDIO_SELECT_SPEC>`"]
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
#[doc = ""]
pub mod sdio_select;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = ""]
pub mod enable;
#[doc = "ENABLE_W1TS (rw) register accessor: an alias for `Reg<ENABLE_W1TS_SPEC>`"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = ""]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (rw) register accessor: an alias for `Reg<ENABLE_W1TC_SPEC>`"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = ""]
pub mod enable_w1tc;
#[doc = "ENABLE1 (rw) register accessor: an alias for `Reg<ENABLE1_SPEC>`"]
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
#[doc = ""]
pub mod enable1;
#[doc = "ENABLE1_W1TS (rw) register accessor: an alias for `Reg<ENABLE1_W1TS_SPEC>`"]
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
#[doc = ""]
pub mod enable1_w1ts;
#[doc = "ENABLE1_W1TC (rw) register accessor: an alias for `Reg<ENABLE1_W1TC_SPEC>`"]
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
#[doc = ""]
pub mod enable1_w1tc;
#[doc = "STRAP (r) register accessor: an alias for `Reg<STRAP_SPEC>`"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = ""]
pub mod strap;
#[doc = "IN (rw) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = ""]
pub mod in_;
#[doc = "IN1 (rw) register accessor: an alias for `Reg<IN1_SPEC>`"]
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
#[doc = ""]
pub mod in1;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "STATUS_W1TS (rw) register accessor: an alias for `Reg<STATUS_W1TS_SPEC>`"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = ""]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (rw) register accessor: an alias for `Reg<STATUS_W1TC_SPEC>`"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = ""]
pub mod status_w1tc;
#[doc = "STATUS1 (rw) register accessor: an alias for `Reg<STATUS1_SPEC>`"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = ""]
pub mod status1;
#[doc = "STATUS1_W1TS (rw) register accessor: an alias for `Reg<STATUS1_W1TS_SPEC>`"]
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
#[doc = ""]
pub mod status1_w1ts;
#[doc = "STATUS1_W1TC (rw) register accessor: an alias for `Reg<STATUS1_W1TC_SPEC>`"]
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
#[doc = ""]
pub mod status1_w1tc;
#[doc = "ACPU_INT (r) register accessor: an alias for `Reg<ACPU_INT_SPEC>`"]
pub type ACPU_INT = crate::Reg<acpu_int::ACPU_INT_SPEC>;
#[doc = ""]
pub mod acpu_int;
#[doc = "ACPU_NMI_INT (r) register accessor: an alias for `Reg<ACPU_NMI_INT_SPEC>`"]
pub type ACPU_NMI_INT = crate::Reg<acpu_nmi_int::ACPU_NMI_INT_SPEC>;
#[doc = ""]
pub mod acpu_nmi_int;
#[doc = "PCPU_INT (r) register accessor: an alias for `Reg<PCPU_INT_SPEC>`"]
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
#[doc = ""]
pub mod pcpu_int;
#[doc = "PCPU_NMI_INT (r) register accessor: an alias for `Reg<PCPU_NMI_INT_SPEC>`"]
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
#[doc = ""]
pub mod pcpu_nmi_int;
#[doc = "CPUSDIO_INT (r) register accessor: an alias for `Reg<CPUSDIO_INT_SPEC>`"]
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
#[doc = ""]
pub mod cpusdio_int;
#[doc = "ACPU_INT1 (r) register accessor: an alias for `Reg<ACPU_INT1_SPEC>`"]
pub type ACPU_INT1 = crate::Reg<acpu_int1::ACPU_INT1_SPEC>;
#[doc = ""]
pub mod acpu_int1;
#[doc = "ACPU_NMI_INT1 (r) register accessor: an alias for `Reg<ACPU_NMI_INT1_SPEC>`"]
pub type ACPU_NMI_INT1 = crate::Reg<acpu_nmi_int1::ACPU_NMI_INT1_SPEC>;
#[doc = ""]
pub mod acpu_nmi_int1;
#[doc = "PCPU_INT1 (r) register accessor: an alias for `Reg<PCPU_INT1_SPEC>`"]
pub type PCPU_INT1 = crate::Reg<pcpu_int1::PCPU_INT1_SPEC>;
#[doc = ""]
pub mod pcpu_int1;
#[doc = "PCPU_NMI_INT1 (r) register accessor: an alias for `Reg<PCPU_NMI_INT1_SPEC>`"]
pub type PCPU_NMI_INT1 = crate::Reg<pcpu_nmi_int1::PCPU_NMI_INT1_SPEC>;
#[doc = ""]
pub mod pcpu_nmi_int1;
#[doc = "CPUSDIO_INT1 (rw) register accessor: an alias for `Reg<CPUSDIO_INT1_SPEC>`"]
pub type CPUSDIO_INT1 = crate::Reg<cpusdio_int1::CPUSDIO_INT1_SPEC>;
#[doc = ""]
pub mod cpusdio_int1;
#[doc = "PIN (rw) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = ""]
pub mod pin;
#[doc = "cali_conf (rw) register accessor: an alias for `Reg<CALI_CONF_SPEC>`"]
pub type CALI_CONF = crate::Reg<cali_conf::CALI_CONF_SPEC>;
#[doc = ""]
pub mod cali_conf;
#[doc = "cali_data (r) register accessor: an alias for `Reg<CALI_DATA_SPEC>`"]
pub type CALI_DATA = crate::Reg<cali_data::CALI_DATA_SPEC>;
#[doc = ""]
pub mod cali_data;
#[doc = "FUNC_IN_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC_IN_SEL_CFG_SPEC>`"]
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
#[doc = ""]
pub mod func_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: an alias for `Reg<FUNC_OUT_SEL_CFG_SPEC>`"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = ""]
pub mod func_out_sel_cfg;
