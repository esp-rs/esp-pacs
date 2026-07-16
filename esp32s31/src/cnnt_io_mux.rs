#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    gmac_rmii_clk: GMAC_RMII_CLK,
    gmac_rx_clk: GMAC_RX_CLK,
    gmac_phy_rxdv: GMAC_PHY_RXDV,
    gmac_phy_rxd3: GMAC_PHY_RXD3,
    gmac_phy_rxd2: GMAC_PHY_RXD2,
    gmac_phy_rxd1: GMAC_PHY_RXD1,
    gmac_phy_rxd0: GMAC_PHY_RXD0,
    sdio_data0: SDIO_DATA0,
    sdio_data1: SDIO_DATA1,
    sdio_data2: SDIO_DATA2,
    sdio_data3: SDIO_DATA3,
    sdio_clk: SDIO_CLK,
    sdio_cmd: SDIO_CMD,
    _reserved13: [u8; 0x03c0],
    ctrl: CTRL,
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - GMAC IO MUX configuration register for GMAC_RMII_CLK"]
    #[inline(always)]
    pub const fn gmac_rmii_clk(&self) -> &GMAC_RMII_CLK {
        &self.gmac_rmii_clk
    }
    #[doc = "0x04 - GMAC IO MUX configuration register for GMAC_RX_CLK"]
    #[inline(always)]
    pub const fn gmac_rx_clk(&self) -> &GMAC_RX_CLK {
        &self.gmac_rx_clk
    }
    #[doc = "0x08 - GMAC IO MUX configuration register for GMAC_PHY_RXDV"]
    #[inline(always)]
    pub const fn gmac_phy_rxdv(&self) -> &GMAC_PHY_RXDV {
        &self.gmac_phy_rxdv
    }
    #[doc = "0x0c - GMAC IO MUX configuration register for GMAC_PHY_RXD3"]
    #[inline(always)]
    pub const fn gmac_phy_rxd3(&self) -> &GMAC_PHY_RXD3 {
        &self.gmac_phy_rxd3
    }
    #[doc = "0x10 - GMAC IO MUX configuration register for GMAC_PHY_RXD2"]
    #[inline(always)]
    pub const fn gmac_phy_rxd2(&self) -> &GMAC_PHY_RXD2 {
        &self.gmac_phy_rxd2
    }
    #[doc = "0x14 - GMAC IO MUX configuration register for GMAC_PHY_RXD1"]
    #[inline(always)]
    pub const fn gmac_phy_rxd1(&self) -> &GMAC_PHY_RXD1 {
        &self.gmac_phy_rxd1
    }
    #[doc = "0x18 - GMAC IO MUX configuration register for GMAC_PHY_RXD0"]
    #[inline(always)]
    pub const fn gmac_phy_rxd0(&self) -> &GMAC_PHY_RXD0 {
        &self.gmac_phy_rxd0
    }
    #[doc = "0x1c - SDIO IO MUX configuration register for SDIO_DATA0"]
    #[inline(always)]
    pub const fn sdio_data0(&self) -> &SDIO_DATA0 {
        &self.sdio_data0
    }
    #[doc = "0x20 - SDIO IO MUX configuration register for SDIO_DATA1"]
    #[inline(always)]
    pub const fn sdio_data1(&self) -> &SDIO_DATA1 {
        &self.sdio_data1
    }
    #[doc = "0x24 - SDIO IO MUX configuration register for SDIO_DATA2"]
    #[inline(always)]
    pub const fn sdio_data2(&self) -> &SDIO_DATA2 {
        &self.sdio_data2
    }
    #[doc = "0x28 - SDIO IO MUX configuration register for SDIO_DATA3"]
    #[inline(always)]
    pub const fn sdio_data3(&self) -> &SDIO_DATA3 {
        &self.sdio_data3
    }
    #[doc = "0x2c - SDIO IO MUX configuration register for SDIO_CLK"]
    #[inline(always)]
    pub const fn sdio_clk(&self) -> &SDIO_CLK {
        &self.sdio_clk
    }
    #[doc = "0x30 - SDIO IO MUX configuration register for SDIO_CMD"]
    #[inline(always)]
    pub const fn sdio_cmd(&self) -> &SDIO_CMD {
        &self.sdio_cmd
    }
    #[doc = "0x3f4 - SDIO IO MUX configuration register for ctrl sel"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x3f8 - CNNT_IO_MUX clock gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0x3fc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "GMAC_RMII_CLK (rw) register accessor: GMAC IO MUX configuration register for GMAC_RMII_CLK\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_rmii_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_rmii_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_rmii_clk`] module"]
pub type GMAC_RMII_CLK = crate::Reg<gmac_rmii_clk::GMAC_RMII_CLK_SPEC>;
#[doc = "GMAC IO MUX configuration register for GMAC_RMII_CLK"]
pub mod gmac_rmii_clk;
#[doc = "GMAC_RX_CLK (rw) register accessor: GMAC IO MUX configuration register for GMAC_RX_CLK\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_rx_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_rx_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_rx_clk`] module"]
pub type GMAC_RX_CLK = crate::Reg<gmac_rx_clk::GMAC_RX_CLK_SPEC>;
#[doc = "GMAC IO MUX configuration register for GMAC_RX_CLK"]
pub mod gmac_rx_clk;
#[doc = "GMAC_PHY_RXDV (rw) register accessor: GMAC IO MUX configuration register for GMAC_PHY_RXDV\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_phy_rxdv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_phy_rxdv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_phy_rxdv`] module"]
pub type GMAC_PHY_RXDV = crate::Reg<gmac_phy_rxdv::GMAC_PHY_RXDV_SPEC>;
#[doc = "GMAC IO MUX configuration register for GMAC_PHY_RXDV"]
pub mod gmac_phy_rxdv;
#[doc = "GMAC_PHY_RXD3 (rw) register accessor: GMAC IO MUX configuration register for GMAC_PHY_RXD3\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_phy_rxd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_phy_rxd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_phy_rxd3`] module"]
pub type GMAC_PHY_RXD3 = crate::Reg<gmac_phy_rxd3::GMAC_PHY_RXD3_SPEC>;
#[doc = "GMAC IO MUX configuration register for GMAC_PHY_RXD3"]
pub mod gmac_phy_rxd3;
#[doc = "GMAC_PHY_RXD2 (rw) register accessor: GMAC IO MUX configuration register for GMAC_PHY_RXD2\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_phy_rxd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_phy_rxd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_phy_rxd2`] module"]
pub type GMAC_PHY_RXD2 = crate::Reg<gmac_phy_rxd2::GMAC_PHY_RXD2_SPEC>;
#[doc = "GMAC IO MUX configuration register for GMAC_PHY_RXD2"]
pub mod gmac_phy_rxd2;
#[doc = "GMAC_PHY_RXD1 (rw) register accessor: GMAC IO MUX configuration register for GMAC_PHY_RXD1\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_phy_rxd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_phy_rxd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_phy_rxd1`] module"]
pub type GMAC_PHY_RXD1 = crate::Reg<gmac_phy_rxd1::GMAC_PHY_RXD1_SPEC>;
#[doc = "GMAC IO MUX configuration register for GMAC_PHY_RXD1"]
pub mod gmac_phy_rxd1;
#[doc = "GMAC_PHY_RXD0 (rw) register accessor: GMAC IO MUX configuration register for GMAC_PHY_RXD0\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_phy_rxd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_phy_rxd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_phy_rxd0`] module"]
pub type GMAC_PHY_RXD0 = crate::Reg<gmac_phy_rxd0::GMAC_PHY_RXD0_SPEC>;
#[doc = "GMAC IO MUX configuration register for GMAC_PHY_RXD0"]
pub mod gmac_phy_rxd0;
#[doc = "SDIO_DATA0 (rw) register accessor: SDIO IO MUX configuration register for SDIO_DATA0\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_data0`] module"]
pub type SDIO_DATA0 = crate::Reg<sdio_data0::SDIO_DATA0_SPEC>;
#[doc = "SDIO IO MUX configuration register for SDIO_DATA0"]
pub mod sdio_data0;
#[doc = "SDIO_DATA1 (rw) register accessor: SDIO IO MUX configuration register for SDIO_DATA1\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_data1`] module"]
pub type SDIO_DATA1 = crate::Reg<sdio_data1::SDIO_DATA1_SPEC>;
#[doc = "SDIO IO MUX configuration register for SDIO_DATA1"]
pub mod sdio_data1;
#[doc = "SDIO_DATA2 (rw) register accessor: SDIO IO MUX configuration register for SDIO_DATA2\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_data2`] module"]
pub type SDIO_DATA2 = crate::Reg<sdio_data2::SDIO_DATA2_SPEC>;
#[doc = "SDIO IO MUX configuration register for SDIO_DATA2"]
pub mod sdio_data2;
#[doc = "SDIO_DATA3 (rw) register accessor: SDIO IO MUX configuration register for SDIO_DATA3\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_data3`] module"]
pub type SDIO_DATA3 = crate::Reg<sdio_data3::SDIO_DATA3_SPEC>;
#[doc = "SDIO IO MUX configuration register for SDIO_DATA3"]
pub mod sdio_data3;
#[doc = "SDIO_CLK (rw) register accessor: SDIO IO MUX configuration register for SDIO_CLK\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_clk`] module"]
pub type SDIO_CLK = crate::Reg<sdio_clk::SDIO_CLK_SPEC>;
#[doc = "SDIO IO MUX configuration register for SDIO_CLK"]
pub mod sdio_clk;
#[doc = "SDIO_CMD (rw) register accessor: SDIO IO MUX configuration register for SDIO_CMD\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_cmd`] module"]
pub type SDIO_CMD = crate::Reg<sdio_cmd::SDIO_CMD_SPEC>;
#[doc = "SDIO IO MUX configuration register for SDIO_CMD"]
pub mod sdio_cmd;
#[doc = "CTRL (rw) register accessor: SDIO IO MUX configuration register for ctrl sel\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SDIO IO MUX configuration register for ctrl sel"]
pub mod ctrl;
#[doc = "CLOCK_GATE (rw) register accessor: CNNT_IO_MUX clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "CNNT_IO_MUX clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
