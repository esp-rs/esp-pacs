#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - FIFO access for the CDC-ACM data IN and OUT endpoints."]
    pub ep1: EP1,
    #[doc = "0x04 - Configuration and control registers for the CDC-ACM FIFOs."]
    pub ep1_conf: EP1_CONF,
    #[doc = "0x08 - Interrupt raw status register."]
    pub int_raw: INT_RAW,
    #[doc = "0x0c - Interrupt status register."]
    pub int_st: INT_ST,
    #[doc = "0x10 - Interrupt enable status register."]
    pub int_ena: INT_ENA,
    #[doc = "0x14 - Interrupt clear status register."]
    pub int_clr: INT_CLR,
    #[doc = "0x18 - PHY hardware configuration."]
    pub conf0: CONF0,
    #[doc = "0x1c - Registers used for debugging the PHY."]
    pub test: TEST,
    #[doc = "0x20 - JTAG FIFO status and control registers."]
    pub jfifo_st: JFIFO_ST,
    #[doc = "0x24 - Last received SOF frame index register."]
    pub fram_num: FRAM_NUM,
    #[doc = "0x28 - Control IN endpoint status information."]
    pub in_ep0_st: IN_EP0_ST,
    #[doc = "0x2c - CDC-ACM IN endpoint status information."]
    pub in_ep1_st: IN_EP1_ST,
    #[doc = "0x30 - CDC-ACM interrupt IN endpoint status information."]
    pub in_ep2_st: IN_EP2_ST,
    #[doc = "0x34 - JTAG IN endpoint status information."]
    pub in_ep3_st: IN_EP3_ST,
    #[doc = "0x38 - Control OUT endpoint status information."]
    pub out_ep0_st: OUT_EP0_ST,
    #[doc = "0x3c - CDC-ACM OUT endpoint status information."]
    pub out_ep1_st: OUT_EP1_ST,
    #[doc = "0x40 - JTAG OUT endpoint status information."]
    pub out_ep2_st: OUT_EP2_ST,
    #[doc = "0x44 - Clock enable control"]
    pub misc_conf: MISC_CONF,
    #[doc = "0x48 - Memory power control"]
    pub mem_conf: MEM_CONF,
    #[doc = "0x4c - CDC-ACM chip reset control."]
    pub chip_rst: CHIP_RST,
    #[doc = "0x50 - W0 of SET_LINE_CODING command."]
    pub set_line_code_w0: SET_LINE_CODE_W0,
    #[doc = "0x54 - W1 of SET_LINE_CODING command."]
    pub set_line_code_w1: SET_LINE_CODE_W1,
    #[doc = "0x58 - W0 of GET_LINE_CODING command."]
    pub get_line_code_w0: GET_LINE_CODE_W0,
    #[doc = "0x5c - W1 of GET_LINE_CODING command."]
    pub get_line_code_w1: GET_LINE_CODE_W1,
    #[doc = "0x60 - Configuration registers' value update"]
    pub config_update: CONFIG_UPDATE,
    #[doc = "0x64 - Serial AFIFO configure register"]
    pub ser_afifo_config: SER_AFIFO_CONFIG,
    #[doc = "0x68 - USB Bus reset status register"]
    pub bus_reset_st: BUS_RESET_ST,
    _reserved27: [u8; 0x14],
    #[doc = "0x80 - Date register"]
    pub date: DATE,
}
#[doc = "EP1 (rw) register accessor: an alias for `Reg<EP1_SPEC>`"]
pub type EP1 = crate::Reg<ep1::EP1_SPEC>;
#[doc = "FIFO access for the CDC-ACM data IN and OUT endpoints."]
pub mod ep1;
#[doc = "EP1_CONF (rw) register accessor: an alias for `Reg<EP1_CONF_SPEC>`"]
pub type EP1_CONF = crate::Reg<ep1_conf::EP1_CONF_SPEC>;
#[doc = "Configuration and control registers for the CDC-ACM FIFOs."]
pub mod ep1_conf;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt raw status register."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable status register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear status register."]
pub mod int_clr;
#[doc = "CONF0 (rw) register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "PHY hardware configuration."]
pub mod conf0;
#[doc = "TEST (rw) register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Registers used for debugging the PHY."]
pub mod test;
#[doc = "JFIFO_ST (rw) register accessor: an alias for `Reg<JFIFO_ST_SPEC>`"]
pub type JFIFO_ST = crate::Reg<jfifo_st::JFIFO_ST_SPEC>;
#[doc = "JTAG FIFO status and control registers."]
pub mod jfifo_st;
#[doc = "FRAM_NUM (r) register accessor: an alias for `Reg<FRAM_NUM_SPEC>`"]
pub type FRAM_NUM = crate::Reg<fram_num::FRAM_NUM_SPEC>;
#[doc = "Last received SOF frame index register."]
pub mod fram_num;
#[doc = "IN_EP0_ST (r) register accessor: an alias for `Reg<IN_EP0_ST_SPEC>`"]
pub type IN_EP0_ST = crate::Reg<in_ep0_st::IN_EP0_ST_SPEC>;
#[doc = "Control IN endpoint status information."]
pub mod in_ep0_st;
#[doc = "IN_EP1_ST (r) register accessor: an alias for `Reg<IN_EP1_ST_SPEC>`"]
pub type IN_EP1_ST = crate::Reg<in_ep1_st::IN_EP1_ST_SPEC>;
#[doc = "CDC-ACM IN endpoint status information."]
pub mod in_ep1_st;
#[doc = "IN_EP2_ST (r) register accessor: an alias for `Reg<IN_EP2_ST_SPEC>`"]
pub type IN_EP2_ST = crate::Reg<in_ep2_st::IN_EP2_ST_SPEC>;
#[doc = "CDC-ACM interrupt IN endpoint status information."]
pub mod in_ep2_st;
#[doc = "IN_EP3_ST (r) register accessor: an alias for `Reg<IN_EP3_ST_SPEC>`"]
pub type IN_EP3_ST = crate::Reg<in_ep3_st::IN_EP3_ST_SPEC>;
#[doc = "JTAG IN endpoint status information."]
pub mod in_ep3_st;
#[doc = "OUT_EP0_ST (r) register accessor: an alias for `Reg<OUT_EP0_ST_SPEC>`"]
pub type OUT_EP0_ST = crate::Reg<out_ep0_st::OUT_EP0_ST_SPEC>;
#[doc = "Control OUT endpoint status information."]
pub mod out_ep0_st;
#[doc = "OUT_EP1_ST (r) register accessor: an alias for `Reg<OUT_EP1_ST_SPEC>`"]
pub type OUT_EP1_ST = crate::Reg<out_ep1_st::OUT_EP1_ST_SPEC>;
#[doc = "CDC-ACM OUT endpoint status information."]
pub mod out_ep1_st;
#[doc = "OUT_EP2_ST (r) register accessor: an alias for `Reg<OUT_EP2_ST_SPEC>`"]
pub type OUT_EP2_ST = crate::Reg<out_ep2_st::OUT_EP2_ST_SPEC>;
#[doc = "JTAG OUT endpoint status information."]
pub mod out_ep2_st;
#[doc = "MISC_CONF (rw) register accessor: an alias for `Reg<MISC_CONF_SPEC>`"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "Clock enable control"]
pub mod misc_conf;
#[doc = "MEM_CONF (rw) register accessor: an alias for `Reg<MEM_CONF_SPEC>`"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = "Memory power control"]
pub mod mem_conf;
#[doc = "CHIP_RST (rw) register accessor: an alias for `Reg<CHIP_RST_SPEC>`"]
pub type CHIP_RST = crate::Reg<chip_rst::CHIP_RST_SPEC>;
#[doc = "CDC-ACM chip reset control."]
pub mod chip_rst;
#[doc = "SET_LINE_CODE_W0 (r) register accessor: an alias for `Reg<SET_LINE_CODE_W0_SPEC>`"]
pub type SET_LINE_CODE_W0 = crate::Reg<set_line_code_w0::SET_LINE_CODE_W0_SPEC>;
#[doc = "W0 of SET_LINE_CODING command."]
pub mod set_line_code_w0;
#[doc = "SET_LINE_CODE_W1 (r) register accessor: an alias for `Reg<SET_LINE_CODE_W1_SPEC>`"]
pub type SET_LINE_CODE_W1 = crate::Reg<set_line_code_w1::SET_LINE_CODE_W1_SPEC>;
#[doc = "W1 of SET_LINE_CODING command."]
pub mod set_line_code_w1;
#[doc = "GET_LINE_CODE_W0 (rw) register accessor: an alias for `Reg<GET_LINE_CODE_W0_SPEC>`"]
pub type GET_LINE_CODE_W0 = crate::Reg<get_line_code_w0::GET_LINE_CODE_W0_SPEC>;
#[doc = "W0 of GET_LINE_CODING command."]
pub mod get_line_code_w0;
#[doc = "GET_LINE_CODE_W1 (rw) register accessor: an alias for `Reg<GET_LINE_CODE_W1_SPEC>`"]
pub type GET_LINE_CODE_W1 = crate::Reg<get_line_code_w1::GET_LINE_CODE_W1_SPEC>;
#[doc = "W1 of GET_LINE_CODING command."]
pub mod get_line_code_w1;
#[doc = "CONFIG_UPDATE (w) register accessor: an alias for `Reg<CONFIG_UPDATE_SPEC>`"]
pub type CONFIG_UPDATE = crate::Reg<config_update::CONFIG_UPDATE_SPEC>;
#[doc = "Configuration registers' value update"]
pub mod config_update;
#[doc = "SER_AFIFO_CONFIG (rw) register accessor: an alias for `Reg<SER_AFIFO_CONFIG_SPEC>`"]
pub type SER_AFIFO_CONFIG = crate::Reg<ser_afifo_config::SER_AFIFO_CONFIG_SPEC>;
#[doc = "Serial AFIFO configure register"]
pub mod ser_afifo_config;
#[doc = "BUS_RESET_ST (r) register accessor: an alias for `Reg<BUS_RESET_ST_SPEC>`"]
pub type BUS_RESET_ST = crate::Reg<bus_reset_st::BUS_RESET_ST_SPEC>;
#[doc = "USB Bus reset status register"]
pub mod bus_reset_st;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date register"]
pub mod date;
