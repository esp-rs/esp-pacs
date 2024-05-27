#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    rx_mode_cfg: RX_MODE_CFG,
    rx_data_cfg: RX_DATA_CFG,
    rx_genrl_cfg: RX_GENRL_CFG,
    rx_start_cfg: RX_START_CFG,
    tx_data_cfg: TX_DATA_CFG,
    tx_start_cfg: TX_START_CFG,
    tx_genrl_cfg: TX_GENRL_CFG,
    fifo_cfg: FIFO_CFG,
    reg_update: REG_UPDATE,
    st: ST,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    rx_st0: RX_ST0,
    rx_st1: RX_ST1,
    tx_st0: TX_ST0,
    rx_clk_cfg: RX_CLK_CFG,
    tx_clk_cfg: TX_CLK_CFG,
    _reserved19: [u8; 0xd4],
    clk: CLK,
    _reserved20: [u8; 0x02d8],
    version: VERSION,
}
impl RegisterBlock {
    ///0x00 - Parallel RX Sampling mode configuration register.
    #[inline(always)]
    pub const fn rx_mode_cfg(&self) -> &RX_MODE_CFG {
        &self.rx_mode_cfg
    }
    ///0x04 - Parallel RX data configuration register.
    #[inline(always)]
    pub const fn rx_data_cfg(&self) -> &RX_DATA_CFG {
        &self.rx_data_cfg
    }
    ///0x08 - Parallel RX general configuration register.
    #[inline(always)]
    pub const fn rx_genrl_cfg(&self) -> &RX_GENRL_CFG {
        &self.rx_genrl_cfg
    }
    ///0x0c - Parallel RX Start configuration register.
    #[inline(always)]
    pub const fn rx_start_cfg(&self) -> &RX_START_CFG {
        &self.rx_start_cfg
    }
    ///0x10 - Parallel TX data configuration register.
    #[inline(always)]
    pub const fn tx_data_cfg(&self) -> &TX_DATA_CFG {
        &self.tx_data_cfg
    }
    ///0x14 - Parallel TX Start configuration register.
    #[inline(always)]
    pub const fn tx_start_cfg(&self) -> &TX_START_CFG {
        &self.tx_start_cfg
    }
    ///0x18 - Parallel TX general configuration register.
    #[inline(always)]
    pub const fn tx_genrl_cfg(&self) -> &TX_GENRL_CFG {
        &self.tx_genrl_cfg
    }
    ///0x1c - Parallel IO FIFO configuration register.
    #[inline(always)]
    pub const fn fifo_cfg(&self) -> &FIFO_CFG {
        &self.fifo_cfg
    }
    ///0x20 - Parallel IO FIFO configuration register.
    #[inline(always)]
    pub const fn reg_update(&self) -> &REG_UPDATE {
        &self.reg_update
    }
    ///0x24 - Parallel IO module status register0.
    #[inline(always)]
    pub const fn st(&self) -> &ST {
        &self.st
    }
    ///0x28 - Parallel IO interrupt enable singal configuration register.
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x2c - Parallel IO interrupt raw singal status register.
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x30 - Parallel IO interrupt singal status register.
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x34 - Parallel IO interrupt clear singal configuration register.
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x38 - Parallel IO RX status register0
    #[inline(always)]
    pub const fn rx_st0(&self) -> &RX_ST0 {
        &self.rx_st0
    }
    ///0x3c - Parallel IO RX status register1
    #[inline(always)]
    pub const fn rx_st1(&self) -> &RX_ST1 {
        &self.rx_st1
    }
    ///0x40 - Parallel IO TX status register0
    #[inline(always)]
    pub const fn tx_st0(&self) -> &TX_ST0 {
        &self.tx_st0
    }
    ///0x44 - Parallel IO RX clk configuration register
    #[inline(always)]
    pub const fn rx_clk_cfg(&self) -> &RX_CLK_CFG {
        &self.rx_clk_cfg
    }
    ///0x48 - Parallel IO TX clk configuration register
    #[inline(always)]
    pub const fn tx_clk_cfg(&self) -> &TX_CLK_CFG {
        &self.tx_clk_cfg
    }
    ///0x120 - Parallel IO clk configuration register
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    ///0x3fc - Version register.
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
/**RX_MODE_CFG (rw) register accessor: Parallel RX Sampling mode configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`rx_mode_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_mode_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_mode_cfg`] module*/
pub type RX_MODE_CFG = crate::Reg<rx_mode_cfg::RX_MODE_CFG_SPEC>;
///Parallel RX Sampling mode configuration register.
pub mod rx_mode_cfg;
/**RX_DATA_CFG (rw) register accessor: Parallel RX data configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`rx_data_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_data_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_data_cfg`] module*/
pub type RX_DATA_CFG = crate::Reg<rx_data_cfg::RX_DATA_CFG_SPEC>;
///Parallel RX data configuration register.
pub mod rx_data_cfg;
/**RX_GENRL_CFG (rw) register accessor: Parallel RX general configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`rx_genrl_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_genrl_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_genrl_cfg`] module*/
pub type RX_GENRL_CFG = crate::Reg<rx_genrl_cfg::RX_GENRL_CFG_SPEC>;
///Parallel RX general configuration register.
pub mod rx_genrl_cfg;
/**RX_START_CFG (rw) register accessor: Parallel RX Start configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`rx_start_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_start_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_start_cfg`] module*/
pub type RX_START_CFG = crate::Reg<rx_start_cfg::RX_START_CFG_SPEC>;
///Parallel RX Start configuration register.
pub mod rx_start_cfg;
/**TX_DATA_CFG (rw) register accessor: Parallel TX data configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`tx_data_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_data_cfg`] module*/
pub type TX_DATA_CFG = crate::Reg<tx_data_cfg::TX_DATA_CFG_SPEC>;
///Parallel TX data configuration register.
pub mod tx_data_cfg;
/**TX_START_CFG (rw) register accessor: Parallel TX Start configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`tx_start_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_start_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_start_cfg`] module*/
pub type TX_START_CFG = crate::Reg<tx_start_cfg::TX_START_CFG_SPEC>;
///Parallel TX Start configuration register.
pub mod tx_start_cfg;
/**TX_GENRL_CFG (rw) register accessor: Parallel TX general configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`tx_genrl_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_genrl_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_genrl_cfg`] module*/
pub type TX_GENRL_CFG = crate::Reg<tx_genrl_cfg::TX_GENRL_CFG_SPEC>;
///Parallel TX general configuration register.
pub mod tx_genrl_cfg;
/**FIFO_CFG (rw) register accessor: Parallel IO FIFO configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fifo_cfg`] module*/
pub type FIFO_CFG = crate::Reg<fifo_cfg::FIFO_CFG_SPEC>;
///Parallel IO FIFO configuration register.
pub mod fifo_cfg;
/**REG_UPDATE (w) register accessor: Parallel IO FIFO configuration register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reg_update`] module*/
pub type REG_UPDATE = crate::Reg<reg_update::REG_UPDATE_SPEC>;
///Parallel IO FIFO configuration register.
pub mod reg_update;
/**ST (r) register accessor: Parallel IO module status register0.

You can [`read`](crate::generic::Reg::read) this register and get [`st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@st`] module*/
pub type ST = crate::Reg<st::ST_SPEC>;
///Parallel IO module status register0.
pub mod st;
/**INT_ENA (rw) register accessor: Parallel IO interrupt enable singal configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///Parallel IO interrupt enable singal configuration register.
pub mod int_ena;
/**INT_RAW (rw) register accessor: Parallel IO interrupt raw singal status register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///Parallel IO interrupt raw singal status register.
pub mod int_raw;
/**INT_ST (r) register accessor: Parallel IO interrupt singal status register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///Parallel IO interrupt singal status register.
pub mod int_st;
/**INT_CLR (w) register accessor: Parallel IO interrupt clear singal configuration register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///Parallel IO interrupt clear singal configuration register.
pub mod int_clr;
/**RX_ST0 (r) register accessor: Parallel IO RX status register0

You can [`read`](crate::generic::Reg::read) this register and get [`rx_st0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_st0`] module*/
pub type RX_ST0 = crate::Reg<rx_st0::RX_ST0_SPEC>;
///Parallel IO RX status register0
pub mod rx_st0;
/**RX_ST1 (r) register accessor: Parallel IO RX status register1

You can [`read`](crate::generic::Reg::read) this register and get [`rx_st1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_st1`] module*/
pub type RX_ST1 = crate::Reg<rx_st1::RX_ST1_SPEC>;
///Parallel IO RX status register1
pub mod rx_st1;
/**TX_ST0 (r) register accessor: Parallel IO TX status register0

You can [`read`](crate::generic::Reg::read) this register and get [`tx_st0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_st0`] module*/
pub type TX_ST0 = crate::Reg<tx_st0::TX_ST0_SPEC>;
///Parallel IO TX status register0
pub mod tx_st0;
/**RX_CLK_CFG (rw) register accessor: Parallel IO RX clk configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rx_clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_clk_cfg`] module*/
pub type RX_CLK_CFG = crate::Reg<rx_clk_cfg::RX_CLK_CFG_SPEC>;
///Parallel IO RX clk configuration register
pub mod rx_clk_cfg;
/**TX_CLK_CFG (rw) register accessor: Parallel IO TX clk configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tx_clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_clk_cfg`] module*/
pub type TX_CLK_CFG = crate::Reg<tx_clk_cfg::TX_CLK_CFG_SPEC>;
///Parallel IO TX clk configuration register
pub mod tx_clk_cfg;
/**CLK (rw) register accessor: Parallel IO clk configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk`] module*/
pub type CLK = crate::Reg<clk::CLK_SPEC>;
///Parallel IO clk configuration register
pub mod clk;
/**VERSION (rw) register accessor: Version register.

You can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@version`] module*/
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
///Version register.
pub mod version;
