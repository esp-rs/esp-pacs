#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - USB_DEVICE_EP1_REG."]
    pub ep1: EP1,
    #[doc = "0x04 - USB_DEVICE_EP1_CONF_REG."]
    pub ep1_conf: EP1_CONF,
    #[doc = "0x08 - USB_DEVICE_INT_RAW_REG."]
    pub int_raw: INT_RAW,
    #[doc = "0x0c - USB_DEVICE_INT_ST_REG."]
    pub int_st: INT_ST,
    #[doc = "0x10 - USB_DEVICE_INT_ENA_REG."]
    pub int_ena: INT_ENA,
    #[doc = "0x14 - USB_DEVICE_INT_CLR_REG."]
    pub int_clr: INT_CLR,
    #[doc = "0x18 - USB_DEVICE_CONF0_REG."]
    pub conf0: CONF0,
    #[doc = "0x1c - USB_DEVICE_TEST_REG."]
    pub test: TEST,
    #[doc = "0x20 - USB_DEVICE_JFIFO_ST_REG."]
    pub jfifo_st: JFIFO_ST,
    #[doc = "0x24 - USB_DEVICE_FRAM_NUM_REG."]
    pub fram_num: FRAM_NUM,
    #[doc = "0x28 - USB_DEVICE_IN_EP0_ST_REG."]
    pub in_ep0_st: IN_EP0_ST,
    #[doc = "0x2c - USB_DEVICE_IN_EP1_ST_REG."]
    pub in_ep1_st: IN_EP1_ST,
    #[doc = "0x30 - USB_DEVICE_IN_EP2_ST_REG."]
    pub in_ep2_st: IN_EP2_ST,
    #[doc = "0x34 - USB_DEVICE_IN_EP3_ST_REG."]
    pub in_ep3_st: IN_EP3_ST,
    #[doc = "0x38 - USB_DEVICE_OUT_EP0_ST_REG."]
    pub out_ep0_st: OUT_EP0_ST,
    #[doc = "0x3c - USB_DEVICE_OUT_EP1_ST_REG."]
    pub out_ep1_st: OUT_EP1_ST,
    #[doc = "0x40 - USB_DEVICE_OUT_EP2_ST_REG."]
    pub out_ep2_st: OUT_EP2_ST,
    #[doc = "0x44 - USB_DEVICE_MISC_CONF_REG."]
    pub misc_conf: MISC_CONF,
    #[doc = "0x48 - USB_DEVICE_MEM_CONF_REG."]
    pub mem_conf: MEM_CONF,
    _reserved19: [u8; 0x34],
    #[doc = "0x80 - USB_DEVICE_DATE_REG."]
    pub date: DATE,
}
#[doc = "EP1 (rw) register accessor: an alias for `Reg<EP1_SPEC>`"]
pub type EP1 = crate::Reg<ep1::EP1_SPEC>;
#[doc = "USB_DEVICE_EP1_REG."]
pub mod ep1;
#[doc = "EP1_CONF (rw) register accessor: an alias for `Reg<EP1_CONF_SPEC>`"]
pub type EP1_CONF = crate::Reg<ep1_conf::EP1_CONF_SPEC>;
#[doc = "USB_DEVICE_EP1_CONF_REG."]
pub mod ep1_conf;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "USB_DEVICE_INT_RAW_REG."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "USB_DEVICE_INT_ST_REG."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "USB_DEVICE_INT_ENA_REG."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "USB_DEVICE_INT_CLR_REG."]
pub mod int_clr;
#[doc = "CONF0 (rw) register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "USB_DEVICE_CONF0_REG."]
pub mod conf0;
#[doc = "TEST (rw) register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "USB_DEVICE_TEST_REG."]
pub mod test;
#[doc = "JFIFO_ST (rw) register accessor: an alias for `Reg<JFIFO_ST_SPEC>`"]
pub type JFIFO_ST = crate::Reg<jfifo_st::JFIFO_ST_SPEC>;
#[doc = "USB_DEVICE_JFIFO_ST_REG."]
pub mod jfifo_st;
#[doc = "FRAM_NUM (r) register accessor: an alias for `Reg<FRAM_NUM_SPEC>`"]
pub type FRAM_NUM = crate::Reg<fram_num::FRAM_NUM_SPEC>;
#[doc = "USB_DEVICE_FRAM_NUM_REG."]
pub mod fram_num;
#[doc = "IN_EP0_ST (r) register accessor: an alias for `Reg<IN_EP0_ST_SPEC>`"]
pub type IN_EP0_ST = crate::Reg<in_ep0_st::IN_EP0_ST_SPEC>;
#[doc = "USB_DEVICE_IN_EP0_ST_REG."]
pub mod in_ep0_st;
#[doc = "IN_EP1_ST (r) register accessor: an alias for `Reg<IN_EP1_ST_SPEC>`"]
pub type IN_EP1_ST = crate::Reg<in_ep1_st::IN_EP1_ST_SPEC>;
#[doc = "USB_DEVICE_IN_EP1_ST_REG."]
pub mod in_ep1_st;
#[doc = "IN_EP2_ST (r) register accessor: an alias for `Reg<IN_EP2_ST_SPEC>`"]
pub type IN_EP2_ST = crate::Reg<in_ep2_st::IN_EP2_ST_SPEC>;
#[doc = "USB_DEVICE_IN_EP2_ST_REG."]
pub mod in_ep2_st;
#[doc = "IN_EP3_ST (r) register accessor: an alias for `Reg<IN_EP3_ST_SPEC>`"]
pub type IN_EP3_ST = crate::Reg<in_ep3_st::IN_EP3_ST_SPEC>;
#[doc = "USB_DEVICE_IN_EP3_ST_REG."]
pub mod in_ep3_st;
#[doc = "OUT_EP0_ST (r) register accessor: an alias for `Reg<OUT_EP0_ST_SPEC>`"]
pub type OUT_EP0_ST = crate::Reg<out_ep0_st::OUT_EP0_ST_SPEC>;
#[doc = "USB_DEVICE_OUT_EP0_ST_REG."]
pub mod out_ep0_st;
#[doc = "OUT_EP1_ST (r) register accessor: an alias for `Reg<OUT_EP1_ST_SPEC>`"]
pub type OUT_EP1_ST = crate::Reg<out_ep1_st::OUT_EP1_ST_SPEC>;
#[doc = "USB_DEVICE_OUT_EP1_ST_REG."]
pub mod out_ep1_st;
#[doc = "OUT_EP2_ST (r) register accessor: an alias for `Reg<OUT_EP2_ST_SPEC>`"]
pub type OUT_EP2_ST = crate::Reg<out_ep2_st::OUT_EP2_ST_SPEC>;
#[doc = "USB_DEVICE_OUT_EP2_ST_REG."]
pub mod out_ep2_st;
#[doc = "MISC_CONF (rw) register accessor: an alias for `Reg<MISC_CONF_SPEC>`"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "USB_DEVICE_MISC_CONF_REG."]
pub mod misc_conf;
#[doc = "MEM_CONF (rw) register accessor: an alias for `Reg<MEM_CONF_SPEC>`"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = "USB_DEVICE_MEM_CONF_REG."]
pub mod mem_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "USB_DEVICE_DATE_REG."]
pub mod date;
