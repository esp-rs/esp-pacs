#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    ep1: EP1,
    ep1_conf: EP1_CONF,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    conf0: CONF0,
    test: TEST,
    jfifo_st: JFIFO_ST,
    fram_num: FRAM_NUM,
    in_ep0_st: IN_EP0_ST,
    in_ep1_st: IN_EP1_ST,
    in_ep2_st: IN_EP2_ST,
    in_ep3_st: IN_EP3_ST,
    out_ep0_st: OUT_EP0_ST,
    out_ep1_st: OUT_EP1_ST,
    out_ep2_st: OUT_EP2_ST,
    misc_conf: MISC_CONF,
    mem_conf: MEM_CONF,
    _reserved19: [u8; 0x34],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Endpoint 1 FIFO register
    #[inline(always)]
    pub const fn ep1(&self) -> &EP1 {
        &self.ep1
    }
    ///0x04 - Endpoint 1 configure and status register
    #[inline(always)]
    pub const fn ep1_conf(&self) -> &EP1_CONF {
        &self.ep1_conf
    }
    ///0x08 - Raw status interrupt
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x0c - Masked interrupt
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x10 - Interrupt enable bits
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x14 - Interrupt clear bits
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x18 - Configure 0 register
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    ///0x1c - USB Internal PHY test register
    #[inline(always)]
    pub const fn test(&self) -> &TEST {
        &self.test
    }
    ///0x20 - USB-JTAG FIFO status
    #[inline(always)]
    pub const fn jfifo_st(&self) -> &JFIFO_ST {
        &self.jfifo_st
    }
    ///0x24 - SOF frame number
    #[inline(always)]
    pub const fn fram_num(&self) -> &FRAM_NUM {
        &self.fram_num
    }
    ///0x28 - IN Endpoint 0 status
    #[inline(always)]
    pub const fn in_ep0_st(&self) -> &IN_EP0_ST {
        &self.in_ep0_st
    }
    ///0x2c - IN Endpoint 1 status
    #[inline(always)]
    pub const fn in_ep1_st(&self) -> &IN_EP1_ST {
        &self.in_ep1_st
    }
    ///0x30 - IN Endpoint 2 status
    #[inline(always)]
    pub const fn in_ep2_st(&self) -> &IN_EP2_ST {
        &self.in_ep2_st
    }
    ///0x34 - IN Endpoint 3 status
    #[inline(always)]
    pub const fn in_ep3_st(&self) -> &IN_EP3_ST {
        &self.in_ep3_st
    }
    ///0x38 - OUT Endpoint 0 status
    #[inline(always)]
    pub const fn out_ep0_st(&self) -> &OUT_EP0_ST {
        &self.out_ep0_st
    }
    ///0x3c - OUT Endpoint 1 status
    #[inline(always)]
    pub const fn out_ep1_st(&self) -> &OUT_EP1_ST {
        &self.out_ep1_st
    }
    ///0x40 - OUT Endpoint 2 status
    #[inline(always)]
    pub const fn out_ep2_st(&self) -> &OUT_EP2_ST {
        &self.out_ep2_st
    }
    ///0x44 - MISC register
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    ///0x48 - Power control
    #[inline(always)]
    pub const fn mem_conf(&self) -> &MEM_CONF {
        &self.mem_conf
    }
    ///0x80 - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**EP1 (rw) register accessor: Endpoint 1 FIFO register

You can [`read`](crate::generic::Reg::read) this register and get [`ep1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ep1`] module*/
pub type EP1 = crate::Reg<ep1::EP1_SPEC>;
///Endpoint 1 FIFO register
pub mod ep1;
/**EP1_CONF (rw) register accessor: Endpoint 1 configure and status register

You can [`read`](crate::generic::Reg::read) this register and get [`ep1_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ep1_conf`] module*/
pub type EP1_CONF = crate::Reg<ep1_conf::EP1_CONF_SPEC>;
///Endpoint 1 configure and status register
pub mod ep1_conf;
/**INT_RAW (rw) register accessor: Raw status interrupt

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///Raw status interrupt
pub mod int_raw;
/**INT_ST (r) register accessor: Masked interrupt

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///Masked interrupt
pub mod int_st;
/**INT_ENA (rw) register accessor: Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///Interrupt enable bits
pub mod int_ena;
/**INT_CLR (w) register accessor: Interrupt clear bits

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///Interrupt clear bits
pub mod int_clr;
/**CONF0 (rw) register accessor: Configure 0 register

You can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf0`] module*/
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
///Configure 0 register
pub mod conf0;
/**TEST (rw) register accessor: USB Internal PHY test register

You can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@test`] module*/
pub type TEST = crate::Reg<test::TEST_SPEC>;
///USB Internal PHY test register
pub mod test;
/**JFIFO_ST (rw) register accessor: USB-JTAG FIFO status

You can [`read`](crate::generic::Reg::read) this register and get [`jfifo_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jfifo_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jfifo_st`] module*/
pub type JFIFO_ST = crate::Reg<jfifo_st::JFIFO_ST_SPEC>;
///USB-JTAG FIFO status
pub mod jfifo_st;
/**FRAM_NUM (r) register accessor: SOF frame number

You can [`read`](crate::generic::Reg::read) this register and get [`fram_num::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fram_num`] module*/
pub type FRAM_NUM = crate::Reg<fram_num::FRAM_NUM_SPEC>;
///SOF frame number
pub mod fram_num;
/**IN_EP0_ST (r) register accessor: IN Endpoint 0 status

You can [`read`](crate::generic::Reg::read) this register and get [`in_ep0_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_ep0_st`] module*/
pub type IN_EP0_ST = crate::Reg<in_ep0_st::IN_EP0_ST_SPEC>;
///IN Endpoint 0 status
pub mod in_ep0_st;
/**IN_EP1_ST (r) register accessor: IN Endpoint 1 status

You can [`read`](crate::generic::Reg::read) this register and get [`in_ep1_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_ep1_st`] module*/
pub type IN_EP1_ST = crate::Reg<in_ep1_st::IN_EP1_ST_SPEC>;
///IN Endpoint 1 status
pub mod in_ep1_st;
/**IN_EP2_ST (r) register accessor: IN Endpoint 2 status

You can [`read`](crate::generic::Reg::read) this register and get [`in_ep2_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_ep2_st`] module*/
pub type IN_EP2_ST = crate::Reg<in_ep2_st::IN_EP2_ST_SPEC>;
///IN Endpoint 2 status
pub mod in_ep2_st;
/**IN_EP3_ST (r) register accessor: IN Endpoint 3 status

You can [`read`](crate::generic::Reg::read) this register and get [`in_ep3_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_ep3_st`] module*/
pub type IN_EP3_ST = crate::Reg<in_ep3_st::IN_EP3_ST_SPEC>;
///IN Endpoint 3 status
pub mod in_ep3_st;
/**OUT_EP0_ST (r) register accessor: OUT Endpoint 0 status

You can [`read`](crate::generic::Reg::read) this register and get [`out_ep0_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_ep0_st`] module*/
pub type OUT_EP0_ST = crate::Reg<out_ep0_st::OUT_EP0_ST_SPEC>;
///OUT Endpoint 0 status
pub mod out_ep0_st;
/**OUT_EP1_ST (r) register accessor: OUT Endpoint 1 status

You can [`read`](crate::generic::Reg::read) this register and get [`out_ep1_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_ep1_st`] module*/
pub type OUT_EP1_ST = crate::Reg<out_ep1_st::OUT_EP1_ST_SPEC>;
///OUT Endpoint 1 status
pub mod out_ep1_st;
/**OUT_EP2_ST (r) register accessor: OUT Endpoint 2 status

You can [`read`](crate::generic::Reg::read) this register and get [`out_ep2_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_ep2_st`] module*/
pub type OUT_EP2_ST = crate::Reg<out_ep2_st::OUT_EP2_ST_SPEC>;
///OUT Endpoint 2 status
pub mod out_ep2_st;
/**MISC_CONF (rw) register accessor: MISC register

You can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@misc_conf`] module*/
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
///MISC register
pub mod misc_conf;
/**MEM_CONF (rw) register accessor: Power control

You can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_conf`] module*/
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
///Power control
pub mod mem_conf;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
