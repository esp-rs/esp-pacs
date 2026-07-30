#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    usb_serial_jtag_ep1: USB_SERIAL_JTAG_EP1,
    usb_serial_jtag_ep1_conf: USB_SERIAL_JTAG_EP1_CONF,
    usb_serial_jtag_int_raw: USB_SERIAL_JTAG_INT_RAW,
    usb_serial_jtag_int_st: USB_SERIAL_JTAG_INT_ST,
    usb_serial_jtag_int_ena: USB_SERIAL_JTAG_INT_ENA,
    usb_serial_jtag_int_clr: USB_SERIAL_JTAG_INT_CLR,
    usb_serial_jtag_conf0: USB_SERIAL_JTAG_CONF0,
    usb_serial_jtag_test: USB_SERIAL_JTAG_TEST,
    usb_serial_jtag_jfifo_st: USB_SERIAL_JTAG_JFIFO_ST,
    usb_serial_jtag_fram_num: USB_SERIAL_JTAG_FRAM_NUM,
    usb_serial_jtag_in_ep0_st: USB_SERIAL_JTAG_IN_EP0_ST,
    usb_serial_jtag_in_ep1_st: USB_SERIAL_JTAG_IN_EP1_ST,
    usb_serial_jtag_in_ep2_st: USB_SERIAL_JTAG_IN_EP2_ST,
    usb_serial_jtag_in_ep3_st: USB_SERIAL_JTAG_IN_EP3_ST,
    usb_serial_jtag_out_ep0_st: USB_SERIAL_JTAG_OUT_EP0_ST,
    usb_serial_jtag_out_ep1_st: USB_SERIAL_JTAG_OUT_EP1_ST,
    usb_serial_jtag_out_ep2_st: USB_SERIAL_JTAG_OUT_EP2_ST,
    usb_serial_jtag_misc_conf: USB_SERIAL_JTAG_MISC_CONF,
    usb_serial_jtag_mem_conf: USB_SERIAL_JTAG_MEM_CONF,
    usb_serial_jtag_chip_rst: USB_SERIAL_JTAG_CHIP_RST,
    usb_serial_jtag_set_line_code_w0: USB_SERIAL_JTAG_SET_LINE_CODE_W0,
    usb_serial_jtag_set_line_code_w1: USB_SERIAL_JTAG_SET_LINE_CODE_W1,
    usb_serial_jtag_get_line_code_w0: USB_SERIAL_JTAG_GET_LINE_CODE_W0,
    usb_serial_jtag_get_line_code_w1: USB_SERIAL_JTAG_GET_LINE_CODE_W1,
    usb_serial_jtag_config_update: USB_SERIAL_JTAG_CONFIG_UPDATE,
    usb_serial_jtag_ser_afifo_config: USB_SERIAL_JTAG_SER_AFIFO_CONFIG,
    usb_serial_jtag_bus_reset_st: USB_SERIAL_JTAG_BUS_RESET_ST,
    usb_serial_jtag_serial_ep_timeout0: USB_SERIAL_JTAG_SERIAL_EP_TIMEOUT0,
    usb_serial_jtag_serial_ep_timeout1: USB_SERIAL_JTAG_SERIAL_EP_TIMEOUT1,
    _reserved29: [u8; 0x0c],
    usb_serial_jtag_date: USB_SERIAL_JTAG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - FIFO access for the CDC-ACM data IN and OUT endpoints."]
    #[inline(always)]
    pub const fn usb_serial_jtag_ep1(&self) -> &USB_SERIAL_JTAG_EP1 {
        &self.usb_serial_jtag_ep1
    }
    #[doc = "0x04 - Configuration and control registers for the CDC-ACM FIFOs."]
    #[inline(always)]
    pub const fn usb_serial_jtag_ep1_conf(&self) -> &USB_SERIAL_JTAG_EP1_CONF {
        &self.usb_serial_jtag_ep1_conf
    }
    #[doc = "0x08 - Interrupt raw status register."]
    #[inline(always)]
    pub const fn usb_serial_jtag_int_raw(&self) -> &USB_SERIAL_JTAG_INT_RAW {
        &self.usb_serial_jtag_int_raw
    }
    #[doc = "0x0c - Interrupt status register."]
    #[inline(always)]
    pub const fn usb_serial_jtag_int_st(&self) -> &USB_SERIAL_JTAG_INT_ST {
        &self.usb_serial_jtag_int_st
    }
    #[doc = "0x10 - Interrupt enable status register."]
    #[inline(always)]
    pub const fn usb_serial_jtag_int_ena(&self) -> &USB_SERIAL_JTAG_INT_ENA {
        &self.usb_serial_jtag_int_ena
    }
    #[doc = "0x14 - Interrupt clear status register."]
    #[inline(always)]
    pub const fn usb_serial_jtag_int_clr(&self) -> &USB_SERIAL_JTAG_INT_CLR {
        &self.usb_serial_jtag_int_clr
    }
    #[doc = "0x18 - PHY hardware configuration."]
    #[inline(always)]
    pub const fn usb_serial_jtag_conf0(&self) -> &USB_SERIAL_JTAG_CONF0 {
        &self.usb_serial_jtag_conf0
    }
    #[doc = "0x1c - Registers used for debugging the PHY."]
    #[inline(always)]
    pub const fn usb_serial_jtag_test(&self) -> &USB_SERIAL_JTAG_TEST {
        &self.usb_serial_jtag_test
    }
    #[doc = "0x20 - JTAG FIFO status and control registers."]
    #[inline(always)]
    pub const fn usb_serial_jtag_jfifo_st(&self) -> &USB_SERIAL_JTAG_JFIFO_ST {
        &self.usb_serial_jtag_jfifo_st
    }
    #[doc = "0x24 - Last received SOF frame index register."]
    #[inline(always)]
    pub const fn usb_serial_jtag_fram_num(&self) -> &USB_SERIAL_JTAG_FRAM_NUM {
        &self.usb_serial_jtag_fram_num
    }
    #[doc = "0x28 - Control IN endpoint status information."]
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep0_st(&self) -> &USB_SERIAL_JTAG_IN_EP0_ST {
        &self.usb_serial_jtag_in_ep0_st
    }
    #[doc = "0x2c - CDC-ACM IN endpoint status information."]
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep1_st(&self) -> &USB_SERIAL_JTAG_IN_EP1_ST {
        &self.usb_serial_jtag_in_ep1_st
    }
    #[doc = "0x30 - CDC-ACM interrupt IN endpoint status information."]
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep2_st(&self) -> &USB_SERIAL_JTAG_IN_EP2_ST {
        &self.usb_serial_jtag_in_ep2_st
    }
    #[doc = "0x34 - JTAG IN endpoint status information."]
    #[inline(always)]
    pub const fn usb_serial_jtag_in_ep3_st(&self) -> &USB_SERIAL_JTAG_IN_EP3_ST {
        &self.usb_serial_jtag_in_ep3_st
    }
    #[doc = "0x38 - Control OUT endpoint status information."]
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep0_st(&self) -> &USB_SERIAL_JTAG_OUT_EP0_ST {
        &self.usb_serial_jtag_out_ep0_st
    }
    #[doc = "0x3c - CDC-ACM OUT endpoint status information."]
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep1_st(&self) -> &USB_SERIAL_JTAG_OUT_EP1_ST {
        &self.usb_serial_jtag_out_ep1_st
    }
    #[doc = "0x40 - JTAG OUT endpoint status information."]
    #[inline(always)]
    pub const fn usb_serial_jtag_out_ep2_st(&self) -> &USB_SERIAL_JTAG_OUT_EP2_ST {
        &self.usb_serial_jtag_out_ep2_st
    }
    #[doc = "0x44 - Clock enable control"]
    #[inline(always)]
    pub const fn usb_serial_jtag_misc_conf(&self) -> &USB_SERIAL_JTAG_MISC_CONF {
        &self.usb_serial_jtag_misc_conf
    }
    #[doc = "0x48 - Memory power control"]
    #[inline(always)]
    pub const fn usb_serial_jtag_mem_conf(&self) -> &USB_SERIAL_JTAG_MEM_CONF {
        &self.usb_serial_jtag_mem_conf
    }
    #[doc = "0x4c - CDC-ACM chip reset control."]
    #[inline(always)]
    pub const fn usb_serial_jtag_chip_rst(&self) -> &USB_SERIAL_JTAG_CHIP_RST {
        &self.usb_serial_jtag_chip_rst
    }
    #[doc = "0x50 - W0 of SET_LINE_CODING command."]
    #[inline(always)]
    pub const fn usb_serial_jtag_set_line_code_w0(&self) -> &USB_SERIAL_JTAG_SET_LINE_CODE_W0 {
        &self.usb_serial_jtag_set_line_code_w0
    }
    #[doc = "0x54 - W1 of SET_LINE_CODING command."]
    #[inline(always)]
    pub const fn usb_serial_jtag_set_line_code_w1(&self) -> &USB_SERIAL_JTAG_SET_LINE_CODE_W1 {
        &self.usb_serial_jtag_set_line_code_w1
    }
    #[doc = "0x58 - W0 of GET_LINE_CODING command."]
    #[inline(always)]
    pub const fn usb_serial_jtag_get_line_code_w0(&self) -> &USB_SERIAL_JTAG_GET_LINE_CODE_W0 {
        &self.usb_serial_jtag_get_line_code_w0
    }
    #[doc = "0x5c - W1 of GET_LINE_CODING command."]
    #[inline(always)]
    pub const fn usb_serial_jtag_get_line_code_w1(&self) -> &USB_SERIAL_JTAG_GET_LINE_CODE_W1 {
        &self.usb_serial_jtag_get_line_code_w1
    }
    #[doc = "0x60 - Configuration registers' value update"]
    #[inline(always)]
    pub const fn usb_serial_jtag_config_update(&self) -> &USB_SERIAL_JTAG_CONFIG_UPDATE {
        &self.usb_serial_jtag_config_update
    }
    #[doc = "0x64 - Serial AFIFO configure register"]
    #[inline(always)]
    pub const fn usb_serial_jtag_ser_afifo_config(&self) -> &USB_SERIAL_JTAG_SER_AFIFO_CONFIG {
        &self.usb_serial_jtag_ser_afifo_config
    }
    #[doc = "0x68 - USB Bus reset status register"]
    #[inline(always)]
    pub const fn usb_serial_jtag_bus_reset_st(&self) -> &USB_SERIAL_JTAG_BUS_RESET_ST {
        &self.usb_serial_jtag_bus_reset_st
    }
    #[doc = "0x6c - USB uart out endpoint timeout configuration."]
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_ep_timeout0(&self) -> &USB_SERIAL_JTAG_SERIAL_EP_TIMEOUT0 {
        &self.usb_serial_jtag_serial_ep_timeout0
    }
    #[doc = "0x70 - USB uart out endpoint timeout configuration."]
    #[inline(always)]
    pub const fn usb_serial_jtag_serial_ep_timeout1(&self) -> &USB_SERIAL_JTAG_SERIAL_EP_TIMEOUT1 {
        &self.usb_serial_jtag_serial_ep_timeout1
    }
    #[doc = "0x80 - Date register"]
    #[inline(always)]
    pub const fn usb_serial_jtag_date(&self) -> &USB_SERIAL_JTAG_DATE {
        &self.usb_serial_jtag_date
    }
}
#[doc = "USB_SERIAL_JTAG_EP1 (rw) register accessor: FIFO access for the CDC-ACM data IN and OUT endpoints.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_ep1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_ep1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_ep1`] module"]
pub type USB_SERIAL_JTAG_EP1 = crate::Reg<usb_serial_jtag_ep1::USB_SERIAL_JTAG_EP1_SPEC>;
#[doc = "FIFO access for the CDC-ACM data IN and OUT endpoints."]
pub mod usb_serial_jtag_ep1;
#[doc = "USB_SERIAL_JTAG_EP1_CONF (rw) register accessor: Configuration and control registers for the CDC-ACM FIFOs.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_ep1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_ep1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_ep1_conf`] module"]
pub type USB_SERIAL_JTAG_EP1_CONF =
    crate::Reg<usb_serial_jtag_ep1_conf::USB_SERIAL_JTAG_EP1_CONF_SPEC>;
#[doc = "Configuration and control registers for the CDC-ACM FIFOs."]
pub mod usb_serial_jtag_ep1_conf;
#[doc = "USB_SERIAL_JTAG_INT_RAW (rw) register accessor: Interrupt raw status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_int_raw`] module"]
pub type USB_SERIAL_JTAG_INT_RAW =
    crate::Reg<usb_serial_jtag_int_raw::USB_SERIAL_JTAG_INT_RAW_SPEC>;
#[doc = "Interrupt raw status register."]
pub mod usb_serial_jtag_int_raw;
#[doc = "USB_SERIAL_JTAG_INT_ST (r) register accessor: Interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_int_st`] module"]
pub type USB_SERIAL_JTAG_INT_ST = crate::Reg<usb_serial_jtag_int_st::USB_SERIAL_JTAG_INT_ST_SPEC>;
#[doc = "Interrupt status register."]
pub mod usb_serial_jtag_int_st;
#[doc = "USB_SERIAL_JTAG_INT_ENA (rw) register accessor: Interrupt enable status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_int_ena`] module"]
pub type USB_SERIAL_JTAG_INT_ENA =
    crate::Reg<usb_serial_jtag_int_ena::USB_SERIAL_JTAG_INT_ENA_SPEC>;
#[doc = "Interrupt enable status register."]
pub mod usb_serial_jtag_int_ena;
#[doc = "USB_SERIAL_JTAG_INT_CLR (w) register accessor: Interrupt clear status register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_int_clr`] module"]
pub type USB_SERIAL_JTAG_INT_CLR =
    crate::Reg<usb_serial_jtag_int_clr::USB_SERIAL_JTAG_INT_CLR_SPEC>;
#[doc = "Interrupt clear status register."]
pub mod usb_serial_jtag_int_clr;
#[doc = "USB_SERIAL_JTAG_CONF0 (rw) register accessor: PHY hardware configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_conf0`] module"]
pub type USB_SERIAL_JTAG_CONF0 = crate::Reg<usb_serial_jtag_conf0::USB_SERIAL_JTAG_CONF0_SPEC>;
#[doc = "PHY hardware configuration."]
pub mod usb_serial_jtag_conf0;
#[doc = "USB_SERIAL_JTAG_TEST (rw) register accessor: Registers used for debugging the PHY.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_test`] module"]
pub type USB_SERIAL_JTAG_TEST = crate::Reg<usb_serial_jtag_test::USB_SERIAL_JTAG_TEST_SPEC>;
#[doc = "Registers used for debugging the PHY."]
pub mod usb_serial_jtag_test;
#[doc = "USB_SERIAL_JTAG_JFIFO_ST (rw) register accessor: JTAG FIFO status and control registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_jfifo_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_jfifo_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_jfifo_st`] module"]
pub type USB_SERIAL_JTAG_JFIFO_ST =
    crate::Reg<usb_serial_jtag_jfifo_st::USB_SERIAL_JTAG_JFIFO_ST_SPEC>;
#[doc = "JTAG FIFO status and control registers."]
pub mod usb_serial_jtag_jfifo_st;
#[doc = "USB_SERIAL_JTAG_FRAM_NUM (r) register accessor: Last received SOF frame index register.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_fram_num::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_fram_num`] module"]
pub type USB_SERIAL_JTAG_FRAM_NUM =
    crate::Reg<usb_serial_jtag_fram_num::USB_SERIAL_JTAG_FRAM_NUM_SPEC>;
#[doc = "Last received SOF frame index register."]
pub mod usb_serial_jtag_fram_num;
#[doc = "USB_SERIAL_JTAG_IN_EP0_ST (r) register accessor: Control IN endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_in_ep0_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_in_ep0_st`] module"]
pub type USB_SERIAL_JTAG_IN_EP0_ST =
    crate::Reg<usb_serial_jtag_in_ep0_st::USB_SERIAL_JTAG_IN_EP0_ST_SPEC>;
#[doc = "Control IN endpoint status information."]
pub mod usb_serial_jtag_in_ep0_st;
#[doc = "USB_SERIAL_JTAG_IN_EP1_ST (r) register accessor: CDC-ACM IN endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_in_ep1_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_in_ep1_st`] module"]
pub type USB_SERIAL_JTAG_IN_EP1_ST =
    crate::Reg<usb_serial_jtag_in_ep1_st::USB_SERIAL_JTAG_IN_EP1_ST_SPEC>;
#[doc = "CDC-ACM IN endpoint status information."]
pub mod usb_serial_jtag_in_ep1_st;
#[doc = "USB_SERIAL_JTAG_IN_EP2_ST (r) register accessor: CDC-ACM interrupt IN endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_in_ep2_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_in_ep2_st`] module"]
pub type USB_SERIAL_JTAG_IN_EP2_ST =
    crate::Reg<usb_serial_jtag_in_ep2_st::USB_SERIAL_JTAG_IN_EP2_ST_SPEC>;
#[doc = "CDC-ACM interrupt IN endpoint status information."]
pub mod usb_serial_jtag_in_ep2_st;
#[doc = "USB_SERIAL_JTAG_IN_EP3_ST (r) register accessor: JTAG IN endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_in_ep3_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_in_ep3_st`] module"]
pub type USB_SERIAL_JTAG_IN_EP3_ST =
    crate::Reg<usb_serial_jtag_in_ep3_st::USB_SERIAL_JTAG_IN_EP3_ST_SPEC>;
#[doc = "JTAG IN endpoint status information."]
pub mod usb_serial_jtag_in_ep3_st;
#[doc = "USB_SERIAL_JTAG_OUT_EP0_ST (r) register accessor: Control OUT endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_out_ep0_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_out_ep0_st`] module"]
pub type USB_SERIAL_JTAG_OUT_EP0_ST =
    crate::Reg<usb_serial_jtag_out_ep0_st::USB_SERIAL_JTAG_OUT_EP0_ST_SPEC>;
#[doc = "Control OUT endpoint status information."]
pub mod usb_serial_jtag_out_ep0_st;
#[doc = "USB_SERIAL_JTAG_OUT_EP1_ST (r) register accessor: CDC-ACM OUT endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_out_ep1_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_out_ep1_st`] module"]
pub type USB_SERIAL_JTAG_OUT_EP1_ST =
    crate::Reg<usb_serial_jtag_out_ep1_st::USB_SERIAL_JTAG_OUT_EP1_ST_SPEC>;
#[doc = "CDC-ACM OUT endpoint status information."]
pub mod usb_serial_jtag_out_ep1_st;
#[doc = "USB_SERIAL_JTAG_OUT_EP2_ST (r) register accessor: JTAG OUT endpoint status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_out_ep2_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_out_ep2_st`] module"]
pub type USB_SERIAL_JTAG_OUT_EP2_ST =
    crate::Reg<usb_serial_jtag_out_ep2_st::USB_SERIAL_JTAG_OUT_EP2_ST_SPEC>;
#[doc = "JTAG OUT endpoint status information."]
pub mod usb_serial_jtag_out_ep2_st;
#[doc = "USB_SERIAL_JTAG_MISC_CONF (rw) register accessor: Clock enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_misc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_misc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_misc_conf`] module"]
pub type USB_SERIAL_JTAG_MISC_CONF =
    crate::Reg<usb_serial_jtag_misc_conf::USB_SERIAL_JTAG_MISC_CONF_SPEC>;
#[doc = "Clock enable control"]
pub mod usb_serial_jtag_misc_conf;
#[doc = "USB_SERIAL_JTAG_MEM_CONF (rw) register accessor: Memory power control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_mem_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_mem_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_mem_conf`] module"]
pub type USB_SERIAL_JTAG_MEM_CONF =
    crate::Reg<usb_serial_jtag_mem_conf::USB_SERIAL_JTAG_MEM_CONF_SPEC>;
#[doc = "Memory power control"]
pub mod usb_serial_jtag_mem_conf;
#[doc = "USB_SERIAL_JTAG_CHIP_RST (rw) register accessor: CDC-ACM chip reset control.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_chip_rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_chip_rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_chip_rst`] module"]
pub type USB_SERIAL_JTAG_CHIP_RST =
    crate::Reg<usb_serial_jtag_chip_rst::USB_SERIAL_JTAG_CHIP_RST_SPEC>;
#[doc = "CDC-ACM chip reset control."]
pub mod usb_serial_jtag_chip_rst;
#[doc = "USB_SERIAL_JTAG_SET_LINE_CODE_W0 (r) register accessor: W0 of SET_LINE_CODING command.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_set_line_code_w0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_set_line_code_w0`] module"]
pub type USB_SERIAL_JTAG_SET_LINE_CODE_W0 =
    crate::Reg<usb_serial_jtag_set_line_code_w0::USB_SERIAL_JTAG_SET_LINE_CODE_W0_SPEC>;
#[doc = "W0 of SET_LINE_CODING command."]
pub mod usb_serial_jtag_set_line_code_w0;
#[doc = "USB_SERIAL_JTAG_SET_LINE_CODE_W1 (r) register accessor: W1 of SET_LINE_CODING command.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_set_line_code_w1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_set_line_code_w1`] module"]
pub type USB_SERIAL_JTAG_SET_LINE_CODE_W1 =
    crate::Reg<usb_serial_jtag_set_line_code_w1::USB_SERIAL_JTAG_SET_LINE_CODE_W1_SPEC>;
#[doc = "W1 of SET_LINE_CODING command."]
pub mod usb_serial_jtag_set_line_code_w1;
#[doc = "USB_SERIAL_JTAG_GET_LINE_CODE_W0 (rw) register accessor: W0 of GET_LINE_CODING command.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_get_line_code_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_get_line_code_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_get_line_code_w0`] module"]
pub type USB_SERIAL_JTAG_GET_LINE_CODE_W0 =
    crate::Reg<usb_serial_jtag_get_line_code_w0::USB_SERIAL_JTAG_GET_LINE_CODE_W0_SPEC>;
#[doc = "W0 of GET_LINE_CODING command."]
pub mod usb_serial_jtag_get_line_code_w0;
#[doc = "USB_SERIAL_JTAG_GET_LINE_CODE_W1 (rw) register accessor: W1 of GET_LINE_CODING command.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_get_line_code_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_get_line_code_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_get_line_code_w1`] module"]
pub type USB_SERIAL_JTAG_GET_LINE_CODE_W1 =
    crate::Reg<usb_serial_jtag_get_line_code_w1::USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC>;
#[doc = "W1 of GET_LINE_CODING command."]
pub mod usb_serial_jtag_get_line_code_w1;
#[doc = "USB_SERIAL_JTAG_CONFIG_UPDATE (w) register accessor: Configuration registers' value update\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_config_update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_config_update`] module"]
pub type USB_SERIAL_JTAG_CONFIG_UPDATE =
    crate::Reg<usb_serial_jtag_config_update::USB_SERIAL_JTAG_CONFIG_UPDATE_SPEC>;
#[doc = "Configuration registers' value update"]
pub mod usb_serial_jtag_config_update;
#[doc = "USB_SERIAL_JTAG_SER_AFIFO_CONFIG (rw) register accessor: Serial AFIFO configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_ser_afifo_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_ser_afifo_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_ser_afifo_config`] module"]
pub type USB_SERIAL_JTAG_SER_AFIFO_CONFIG =
    crate::Reg<usb_serial_jtag_ser_afifo_config::USB_SERIAL_JTAG_SER_AFIFO_CONFIG_SPEC>;
#[doc = "Serial AFIFO configure register"]
pub mod usb_serial_jtag_ser_afifo_config;
#[doc = "USB_SERIAL_JTAG_BUS_RESET_ST (r) register accessor: USB Bus reset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_bus_reset_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_bus_reset_st`] module"]
pub type USB_SERIAL_JTAG_BUS_RESET_ST =
    crate::Reg<usb_serial_jtag_bus_reset_st::USB_SERIAL_JTAG_BUS_RESET_ST_SPEC>;
#[doc = "USB Bus reset status register"]
pub mod usb_serial_jtag_bus_reset_st;
#[doc = "USB_SERIAL_JTAG_SERIAL_EP_TIMEOUT0 (rw) register accessor: USB uart out endpoint timeout configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_serial_ep_timeout0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_serial_ep_timeout0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_serial_ep_timeout0`] module"]
pub type USB_SERIAL_JTAG_SERIAL_EP_TIMEOUT0 =
    crate::Reg<usb_serial_jtag_serial_ep_timeout0::USB_SERIAL_JTAG_SERIAL_EP_TIMEOUT0_SPEC>;
#[doc = "USB uart out endpoint timeout configuration."]
pub mod usb_serial_jtag_serial_ep_timeout0;
#[doc = "USB_SERIAL_JTAG_SERIAL_EP_TIMEOUT1 (rw) register accessor: USB uart out endpoint timeout configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_serial_ep_timeout1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_serial_ep_timeout1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_serial_ep_timeout1`] module"]
pub type USB_SERIAL_JTAG_SERIAL_EP_TIMEOUT1 =
    crate::Reg<usb_serial_jtag_serial_ep_timeout1::USB_SERIAL_JTAG_SERIAL_EP_TIMEOUT1_SPEC>;
#[doc = "USB uart out endpoint timeout configuration."]
pub mod usb_serial_jtag_serial_ep_timeout1;
#[doc = "USB_SERIAL_JTAG_DATE (rw) register accessor: Date register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_serial_jtag_date`] module"]
pub type USB_SERIAL_JTAG_DATE = crate::Reg<usb_serial_jtag_date::USB_SERIAL_JTAG_DATE_SPEC>;
#[doc = "Date register"]
pub mod usb_serial_jtag_date;
