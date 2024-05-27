#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    ctrl: CTRL,
    ctrl2: CTRL2,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    clk_conf: CLK_CONF,
    int_ena_w1ts: INT_ENA_W1TS,
    int_ena_w1tc: INT_ENA_W1TC,
    wakeup_ctrl: WAKEUP_CTRL,
    sample_rate: SAMPLE_RATE,
    rnd_eco_low: RND_ECO_LOW,
    rnd_eco_high: RND_ECO_HIGH,
    rnd_eco_cs: RND_ECO_CS,
}
impl RegisterBlock {
    ///0x00 - Tsens configuration.
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    ///0x04 - Tsens configuration.
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    ///0x08 - Tsens interrupt raw registers.
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x0c - Tsens interrupt status registers.
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x10 - Tsens interrupt enable registers.
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x14 - Tsens interrupt clear registers.
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x18 - Tsens regbank configuration registers.
    #[inline(always)]
    pub const fn clk_conf(&self) -> &CLK_CONF {
        &self.clk_conf
    }
    ///0x1c - Tsens wakeup interrupt enable assert.
    #[inline(always)]
    pub const fn int_ena_w1ts(&self) -> &INT_ENA_W1TS {
        &self.int_ena_w1ts
    }
    ///0x20 - Tsens wakeup interrupt enable deassert.
    #[inline(always)]
    pub const fn int_ena_w1tc(&self) -> &INT_ENA_W1TC {
        &self.int_ena_w1tc
    }
    ///0x24 - Tsens wakeup control registers.
    #[inline(always)]
    pub const fn wakeup_ctrl(&self) -> &WAKEUP_CTRL {
        &self.wakeup_ctrl
    }
    ///0x28 - Hardware automatic sampling control registers.
    #[inline(always)]
    pub const fn sample_rate(&self) -> &SAMPLE_RATE {
        &self.sample_rate
    }
    ///0x2c - N/A
    #[inline(always)]
    pub const fn rnd_eco_low(&self) -> &RND_ECO_LOW {
        &self.rnd_eco_low
    }
    ///0x30 - N/A
    #[inline(always)]
    pub const fn rnd_eco_high(&self) -> &RND_ECO_HIGH {
        &self.rnd_eco_high
    }
    ///0x34 - N/A
    #[inline(always)]
    pub const fn rnd_eco_cs(&self) -> &RND_ECO_CS {
        &self.rnd_eco_cs
    }
}
/**CTRL (rw) register accessor: Tsens configuration.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl`] module*/
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///Tsens configuration.
pub mod ctrl;
/**CTRL2 (rw) register accessor: Tsens configuration.

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl2`] module*/
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
///Tsens configuration.
pub mod ctrl2;
/**INT_RAW (rw) register accessor: Tsens interrupt raw registers.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///Tsens interrupt raw registers.
pub mod int_raw;
/**INT_ST (r) register accessor: Tsens interrupt status registers.

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///Tsens interrupt status registers.
pub mod int_st;
/**INT_ENA (rw) register accessor: Tsens interrupt enable registers.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///Tsens interrupt enable registers.
pub mod int_ena;
/**INT_CLR (w) register accessor: Tsens interrupt clear registers.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///Tsens interrupt clear registers.
pub mod int_clr;
/**CLK_CONF (rw) register accessor: Tsens regbank configuration registers.

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_conf`] module*/
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
///Tsens regbank configuration registers.
pub mod clk_conf;
/**INT_ENA_W1TS (w) register accessor: Tsens wakeup interrupt enable assert.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena_w1ts`] module*/
pub type INT_ENA_W1TS = crate::Reg<int_ena_w1ts::INT_ENA_W1TS_SPEC>;
///Tsens wakeup interrupt enable assert.
pub mod int_ena_w1ts;
/**INT_ENA_W1TC (w) register accessor: Tsens wakeup interrupt enable deassert.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena_w1tc`] module*/
pub type INT_ENA_W1TC = crate::Reg<int_ena_w1tc::INT_ENA_W1TC_SPEC>;
///Tsens wakeup interrupt enable deassert.
pub mod int_ena_w1tc;
/**WAKEUP_CTRL (rw) register accessor: Tsens wakeup control registers.

You can [`read`](crate::generic::Reg::read) this register and get [`wakeup_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wakeup_ctrl`] module*/
pub type WAKEUP_CTRL = crate::Reg<wakeup_ctrl::WAKEUP_CTRL_SPEC>;
///Tsens wakeup control registers.
pub mod wakeup_ctrl;
/**SAMPLE_RATE (rw) register accessor: Hardware automatic sampling control registers.

You can [`read`](crate::generic::Reg::read) this register and get [`sample_rate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_rate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sample_rate`] module*/
pub type SAMPLE_RATE = crate::Reg<sample_rate::SAMPLE_RATE_SPEC>;
///Hardware automatic sampling control registers.
pub mod sample_rate;
/**RND_ECO_LOW (rw) register accessor: N/A

You can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rnd_eco_low`] module*/
pub type RND_ECO_LOW = crate::Reg<rnd_eco_low::RND_ECO_LOW_SPEC>;
///N/A
pub mod rnd_eco_low;
/**RND_ECO_HIGH (rw) register accessor: N/A

You can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rnd_eco_high`] module*/
pub type RND_ECO_HIGH = crate::Reg<rnd_eco_high::RND_ECO_HIGH_SPEC>;
///N/A
pub mod rnd_eco_high;
/**RND_ECO_CS (rw) register accessor: N/A

You can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rnd_eco_cs`] module*/
pub type RND_ECO_CS = crate::Reg<rnd_eco_cs::RND_ECO_CS_SPEC>;
///N/A
pub mod rnd_eco_cs;
