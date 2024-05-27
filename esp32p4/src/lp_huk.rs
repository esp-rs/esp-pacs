#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    clk: CLK,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    _reserved5: [u8; 0x08],
    conf: CONF,
    start: START,
    state: STATE,
    _reserved8: [u8; 0x08],
    status: STATUS,
    _reserved9: [u8; 0xc4],
    date: DATE,
    info_mem: [INFO_MEM; 96],
}
impl RegisterBlock {
    ///0x04 - HUK Generator clock gate control register
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    ///0x08 - HUK Generator interrupt raw register, valid in level.
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x0c - HUK Generator interrupt status register.
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x10 - HUK Generator interrupt enable register.
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x14 - HUK Generator interrupt clear register.
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x20 - HUK Generator configuration register
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x24 - HUK Generator control register
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    ///0x28 - HUK Generator state register
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    ///0x34 - HUK Generator HUK status register
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0xfc - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    ///0x100..0x280 - The memory that stores HUK info.
    #[inline(always)]
    pub const fn info_mem(&self, n: usize) -> &INFO_MEM {
        &self.info_mem[n]
    }
    ///Iterator for array of:
    ///0x100..0x280 - The memory that stores HUK info.
    #[inline(always)]
    pub fn info_mem_iter(&self) -> impl Iterator<Item = &INFO_MEM> {
        self.info_mem.iter()
    }
}
/**CLK (rw) register accessor: HUK Generator clock gate control register

You can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk`] module*/
pub type CLK = crate::Reg<clk::CLK_SPEC>;
///HUK Generator clock gate control register
pub mod clk;
/**INT_RAW (r) register accessor: HUK Generator interrupt raw register, valid in level.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///HUK Generator interrupt raw register, valid in level.
pub mod int_raw;
/**INT_ST (r) register accessor: HUK Generator interrupt status register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///HUK Generator interrupt status register.
pub mod int_st;
/**INT_ENA (rw) register accessor: HUK Generator interrupt enable register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///HUK Generator interrupt enable register.
pub mod int_ena;
/**INT_CLR (w) register accessor: HUK Generator interrupt clear register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///HUK Generator interrupt clear register.
pub mod int_clr;
/**CONF (rw) register accessor: HUK Generator configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///HUK Generator configuration register
pub mod conf;
/**START (w) register accessor: HUK Generator control register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@start`] module*/
pub type START = crate::Reg<start::START_SPEC>;
///HUK Generator control register
pub mod start;
/**STATE (r) register accessor: HUK Generator state register

You can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@state`] module*/
pub type STATE = crate::Reg<state::STATE_SPEC>;
///HUK Generator state register
pub mod state;
/**STATUS (r) register accessor: HUK Generator HUK status register

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///HUK Generator HUK status register
pub mod status;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
/**INFO_MEM (rw) register accessor: The memory that stores HUK info.

You can [`read`](crate::generic::Reg::read) this register and get [`info_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`info_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@info_mem`] module*/
pub type INFO_MEM = crate::Reg<info_mem::INFO_MEM_SPEC>;
///The memory that stores HUK info.
pub mod info_mem;
