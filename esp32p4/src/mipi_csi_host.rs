#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    version: VERSION,
    n_lanes: N_LANES,
    csi2_resetn: CSI2_RESETN,
    int_st_main: INT_ST_MAIN,
    _reserved4: [u8; 0x30],
    phy_shutdownz: PHY_SHUTDOWNZ,
    dphy_rstz: DPHY_RSTZ,
    phy_rx: PHY_RX,
    phy_stopstate: PHY_STOPSTATE,
    phy_test_ctrl0: PHY_TEST_CTRL0,
    phy_test_ctrl1: PHY_TEST_CTRL1,
    _reserved10: [u8; 0x70],
    vc_extension: VC_EXTENSION,
    phy_cal: PHY_CAL,
    _reserved12: [u8; 0x10],
    int_st_phy_fatal: INT_ST_PHY_FATAL,
    int_msk_phy_fatal: INT_MSK_PHY_FATAL,
    int_force_phy_fatal: INT_FORCE_PHY_FATAL,
    _reserved15: [u8; 0x04],
    int_st_pkt_fatal: INT_ST_PKT_FATAL,
    int_msk_pkt_fatal: INT_MSK_PKT_FATAL,
    int_force_pkt_fatal: INT_FORCE_PKT_FATAL,
    _reserved18: [u8; 0x14],
    int_st_phy: INT_ST_PHY,
    int_msk_phy: INT_MSK_PHY,
    int_force_phy: INT_FORCE_PHY,
    _reserved21: [u8; 0x0164],
    int_st_bndry_frame_fatal: INT_ST_BNDRY_FRAME_FATAL,
    int_msk_bndry_frame_fatal: INT_MSK_BNDRY_FRAME_FATAL,
    int_force_bndry_frame_fatal: INT_FORCE_BNDRY_FRAME_FATAL,
    _reserved24: [u8; 0x04],
    int_st_seq_frame_fatal: INT_ST_SEQ_FRAME_FATAL,
    int_msk_seq_frame_fatal: INT_MSK_SEQ_FRAME_FATAL,
    int_force_seq_frame_fatal: INT_FORCE_SEQ_FRAME_FATAL,
    _reserved27: [u8; 0x04],
    int_st_crc_frame_fatal: INT_ST_CRC_FRAME_FATAL,
    int_msk_crc_frame_fatal: INT_MSK_CRC_FRAME_FATAL,
    int_force_crc_frame_fatal: INT_FORCE_CRC_FRAME_FATAL,
    _reserved30: [u8; 0x04],
    int_st_pld_crc_fatal: INT_ST_PLD_CRC_FATAL,
    int_msk_pld_crc_fatal: INT_MSK_PLD_CRC_FATAL,
    int_force_pld_crc_fatal: INT_FORCE_PLD_CRC_FATAL,
    _reserved33: [u8; 0x04],
    int_st_data_id: INT_ST_DATA_ID,
    int_msk_data_id: INT_MSK_DATA_ID,
    int_force_data_id: INT_FORCE_DATA_ID,
    _reserved36: [u8; 0x04],
    int_st_ecc_corrected: INT_ST_ECC_CORRECTED,
    int_msk_ecc_corrected: INT_MSK_ECC_CORRECTED,
    int_force_ecc_corrected: INT_FORCE_ECC_CORRECTED,
    _reserved39: [u8; 0x24],
    scrambling: SCRAMBLING,
    scrambling_seed1: SCRAMBLING_SEED1,
    scrambling_seed2: SCRAMBLING_SEED2,
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn n_lanes(&self) -> &N_LANES {
        &self.n_lanes
    }
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn csi2_resetn(&self) -> &CSI2_RESETN {
        &self.csi2_resetn
    }
    #[doc = "0x0c - NA"]
    #[inline(always)]
    pub const fn int_st_main(&self) -> &INT_ST_MAIN {
        &self.int_st_main
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn phy_shutdownz(&self) -> &PHY_SHUTDOWNZ {
        &self.phy_shutdownz
    }
    #[doc = "0x44 - NA"]
    #[inline(always)]
    pub const fn dphy_rstz(&self) -> &DPHY_RSTZ {
        &self.dphy_rstz
    }
    #[doc = "0x48 - NA"]
    #[inline(always)]
    pub const fn phy_rx(&self) -> &PHY_RX {
        &self.phy_rx
    }
    #[doc = "0x4c - NA"]
    #[inline(always)]
    pub const fn phy_stopstate(&self) -> &PHY_STOPSTATE {
        &self.phy_stopstate
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn phy_test_ctrl0(&self) -> &PHY_TEST_CTRL0 {
        &self.phy_test_ctrl0
    }
    #[doc = "0x54 - NA"]
    #[inline(always)]
    pub const fn phy_test_ctrl1(&self) -> &PHY_TEST_CTRL1 {
        &self.phy_test_ctrl1
    }
    #[doc = "0xc8 - NA"]
    #[inline(always)]
    pub const fn vc_extension(&self) -> &VC_EXTENSION {
        &self.vc_extension
    }
    #[doc = "0xcc - NA"]
    #[inline(always)]
    pub const fn phy_cal(&self) -> &PHY_CAL {
        &self.phy_cal
    }
    #[doc = "0xe0 - NA"]
    #[inline(always)]
    pub const fn int_st_phy_fatal(&self) -> &INT_ST_PHY_FATAL {
        &self.int_st_phy_fatal
    }
    #[doc = "0xe4 - NA"]
    #[inline(always)]
    pub const fn int_msk_phy_fatal(&self) -> &INT_MSK_PHY_FATAL {
        &self.int_msk_phy_fatal
    }
    #[doc = "0xe8 - NA"]
    #[inline(always)]
    pub const fn int_force_phy_fatal(&self) -> &INT_FORCE_PHY_FATAL {
        &self.int_force_phy_fatal
    }
    #[doc = "0xf0 - NA"]
    #[inline(always)]
    pub const fn int_st_pkt_fatal(&self) -> &INT_ST_PKT_FATAL {
        &self.int_st_pkt_fatal
    }
    #[doc = "0xf4 - NA"]
    #[inline(always)]
    pub const fn int_msk_pkt_fatal(&self) -> &INT_MSK_PKT_FATAL {
        &self.int_msk_pkt_fatal
    }
    #[doc = "0xf8 - NA"]
    #[inline(always)]
    pub const fn int_force_pkt_fatal(&self) -> &INT_FORCE_PKT_FATAL {
        &self.int_force_pkt_fatal
    }
    #[doc = "0x110 - NA"]
    #[inline(always)]
    pub const fn int_st_phy(&self) -> &INT_ST_PHY {
        &self.int_st_phy
    }
    #[doc = "0x114 - NA"]
    #[inline(always)]
    pub const fn int_msk_phy(&self) -> &INT_MSK_PHY {
        &self.int_msk_phy
    }
    #[doc = "0x118 - NA"]
    #[inline(always)]
    pub const fn int_force_phy(&self) -> &INT_FORCE_PHY {
        &self.int_force_phy
    }
    #[doc = "0x280 - NA"]
    #[inline(always)]
    pub const fn int_st_bndry_frame_fatal(&self) -> &INT_ST_BNDRY_FRAME_FATAL {
        &self.int_st_bndry_frame_fatal
    }
    #[doc = "0x284 - NA"]
    #[inline(always)]
    pub const fn int_msk_bndry_frame_fatal(&self) -> &INT_MSK_BNDRY_FRAME_FATAL {
        &self.int_msk_bndry_frame_fatal
    }
    #[doc = "0x288 - NA"]
    #[inline(always)]
    pub const fn int_force_bndry_frame_fatal(&self) -> &INT_FORCE_BNDRY_FRAME_FATAL {
        &self.int_force_bndry_frame_fatal
    }
    #[doc = "0x290 - NA"]
    #[inline(always)]
    pub const fn int_st_seq_frame_fatal(&self) -> &INT_ST_SEQ_FRAME_FATAL {
        &self.int_st_seq_frame_fatal
    }
    #[doc = "0x294 - NA"]
    #[inline(always)]
    pub const fn int_msk_seq_frame_fatal(&self) -> &INT_MSK_SEQ_FRAME_FATAL {
        &self.int_msk_seq_frame_fatal
    }
    #[doc = "0x298 - NA"]
    #[inline(always)]
    pub const fn int_force_seq_frame_fatal(&self) -> &INT_FORCE_SEQ_FRAME_FATAL {
        &self.int_force_seq_frame_fatal
    }
    #[doc = "0x2a0 - NA"]
    #[inline(always)]
    pub const fn int_st_crc_frame_fatal(&self) -> &INT_ST_CRC_FRAME_FATAL {
        &self.int_st_crc_frame_fatal
    }
    #[doc = "0x2a4 - NA"]
    #[inline(always)]
    pub const fn int_msk_crc_frame_fatal(&self) -> &INT_MSK_CRC_FRAME_FATAL {
        &self.int_msk_crc_frame_fatal
    }
    #[doc = "0x2a8 - NA"]
    #[inline(always)]
    pub const fn int_force_crc_frame_fatal(&self) -> &INT_FORCE_CRC_FRAME_FATAL {
        &self.int_force_crc_frame_fatal
    }
    #[doc = "0x2b0 - NA"]
    #[inline(always)]
    pub const fn int_st_pld_crc_fatal(&self) -> &INT_ST_PLD_CRC_FATAL {
        &self.int_st_pld_crc_fatal
    }
    #[doc = "0x2b4 - NA"]
    #[inline(always)]
    pub const fn int_msk_pld_crc_fatal(&self) -> &INT_MSK_PLD_CRC_FATAL {
        &self.int_msk_pld_crc_fatal
    }
    #[doc = "0x2b8 - NA"]
    #[inline(always)]
    pub const fn int_force_pld_crc_fatal(&self) -> &INT_FORCE_PLD_CRC_FATAL {
        &self.int_force_pld_crc_fatal
    }
    #[doc = "0x2c0 - NA"]
    #[inline(always)]
    pub const fn int_st_data_id(&self) -> &INT_ST_DATA_ID {
        &self.int_st_data_id
    }
    #[doc = "0x2c4 - NA"]
    #[inline(always)]
    pub const fn int_msk_data_id(&self) -> &INT_MSK_DATA_ID {
        &self.int_msk_data_id
    }
    #[doc = "0x2c8 - NA"]
    #[inline(always)]
    pub const fn int_force_data_id(&self) -> &INT_FORCE_DATA_ID {
        &self.int_force_data_id
    }
    #[doc = "0x2d0 - NA"]
    #[inline(always)]
    pub const fn int_st_ecc_corrected(&self) -> &INT_ST_ECC_CORRECTED {
        &self.int_st_ecc_corrected
    }
    #[doc = "0x2d4 - NA"]
    #[inline(always)]
    pub const fn int_msk_ecc_corrected(&self) -> &INT_MSK_ECC_CORRECTED {
        &self.int_msk_ecc_corrected
    }
    #[doc = "0x2d8 - NA"]
    #[inline(always)]
    pub const fn int_force_ecc_corrected(&self) -> &INT_FORCE_ECC_CORRECTED {
        &self.int_force_ecc_corrected
    }
    #[doc = "0x300 - NA"]
    #[inline(always)]
    pub const fn scrambling(&self) -> &SCRAMBLING {
        &self.scrambling
    }
    #[doc = "0x304 - NA"]
    #[inline(always)]
    pub const fn scrambling_seed1(&self) -> &SCRAMBLING_SEED1 {
        &self.scrambling_seed1
    }
    #[doc = "0x308 - NA"]
    #[inline(always)]
    pub const fn scrambling_seed2(&self) -> &SCRAMBLING_SEED2 {
        &self.scrambling_seed2
    }
}
#[doc = "VERSION (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "NA"]
pub mod version;
#[doc = "N_LANES (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_lanes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_lanes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@n_lanes`] module"]
pub type N_LANES = crate::Reg<n_lanes::N_LANES_SPEC>;
#[doc = "NA"]
pub mod n_lanes;
#[doc = "CSI2_RESETN (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csi2_resetn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csi2_resetn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csi2_resetn`] module"]
pub type CSI2_RESETN = crate::Reg<csi2_resetn::CSI2_RESETN_SPEC>;
#[doc = "NA"]
pub mod csi2_resetn;
#[doc = "INT_ST_MAIN (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_main::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_main`] module"]
pub type INT_ST_MAIN = crate::Reg<int_st_main::INT_ST_MAIN_SPEC>;
#[doc = "NA"]
pub mod int_st_main;
#[doc = "PHY_SHUTDOWNZ (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_shutdownz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_shutdownz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_shutdownz`] module"]
pub type PHY_SHUTDOWNZ = crate::Reg<phy_shutdownz::PHY_SHUTDOWNZ_SPEC>;
#[doc = "NA"]
pub mod phy_shutdownz;
#[doc = "DPHY_RSTZ (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dphy_rstz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dphy_rstz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_rstz`] module"]
pub type DPHY_RSTZ = crate::Reg<dphy_rstz::DPHY_RSTZ_SPEC>;
#[doc = "NA"]
pub mod dphy_rstz;
#[doc = "PHY_RX (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_rx::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_rx`] module"]
pub type PHY_RX = crate::Reg<phy_rx::PHY_RX_SPEC>;
#[doc = "NA"]
pub mod phy_rx;
#[doc = "PHY_STOPSTATE (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_stopstate::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_stopstate`] module"]
pub type PHY_STOPSTATE = crate::Reg<phy_stopstate::PHY_STOPSTATE_SPEC>;
#[doc = "NA"]
pub mod phy_stopstate;
#[doc = "PHY_TEST_CTRL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_test_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_test_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_test_ctrl0`] module"]
pub type PHY_TEST_CTRL0 = crate::Reg<phy_test_ctrl0::PHY_TEST_CTRL0_SPEC>;
#[doc = "NA"]
pub mod phy_test_ctrl0;
#[doc = "PHY_TEST_CTRL1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_test_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_test_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_test_ctrl1`] module"]
pub type PHY_TEST_CTRL1 = crate::Reg<phy_test_ctrl1::PHY_TEST_CTRL1_SPEC>;
#[doc = "NA"]
pub mod phy_test_ctrl1;
#[doc = "VC_EXTENSION (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vc_extension::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vc_extension::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vc_extension`] module"]
pub type VC_EXTENSION = crate::Reg<vc_extension::VC_EXTENSION_SPEC>;
#[doc = "NA"]
pub mod vc_extension;
#[doc = "PHY_CAL (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_cal::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cal`] module"]
pub type PHY_CAL = crate::Reg<phy_cal::PHY_CAL_SPEC>;
#[doc = "NA"]
pub mod phy_cal;
#[doc = "INT_ST_PHY_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_phy_fatal::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_phy_fatal`] module"]
pub type INT_ST_PHY_FATAL = crate::Reg<int_st_phy_fatal::INT_ST_PHY_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_st_phy_fatal;
#[doc = "INT_MSK_PHY_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk_phy_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk_phy_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_phy_fatal`] module"]
pub type INT_MSK_PHY_FATAL = crate::Reg<int_msk_phy_fatal::INT_MSK_PHY_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_msk_phy_fatal;
#[doc = "INT_FORCE_PHY_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_phy_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_phy_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_phy_fatal`] module"]
pub type INT_FORCE_PHY_FATAL = crate::Reg<int_force_phy_fatal::INT_FORCE_PHY_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_force_phy_fatal;
#[doc = "INT_ST_PKT_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_pkt_fatal::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_pkt_fatal`] module"]
pub type INT_ST_PKT_FATAL = crate::Reg<int_st_pkt_fatal::INT_ST_PKT_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_st_pkt_fatal;
#[doc = "INT_MSK_PKT_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk_pkt_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk_pkt_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_pkt_fatal`] module"]
pub type INT_MSK_PKT_FATAL = crate::Reg<int_msk_pkt_fatal::INT_MSK_PKT_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_msk_pkt_fatal;
#[doc = "INT_FORCE_PKT_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_pkt_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_pkt_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_pkt_fatal`] module"]
pub type INT_FORCE_PKT_FATAL = crate::Reg<int_force_pkt_fatal::INT_FORCE_PKT_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_force_pkt_fatal;
#[doc = "INT_ST_PHY (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_phy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_phy`] module"]
pub type INT_ST_PHY = crate::Reg<int_st_phy::INT_ST_PHY_SPEC>;
#[doc = "NA"]
pub mod int_st_phy;
#[doc = "INT_MSK_PHY (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk_phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk_phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_phy`] module"]
pub type INT_MSK_PHY = crate::Reg<int_msk_phy::INT_MSK_PHY_SPEC>;
#[doc = "NA"]
pub mod int_msk_phy;
#[doc = "INT_FORCE_PHY (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_phy`] module"]
pub type INT_FORCE_PHY = crate::Reg<int_force_phy::INT_FORCE_PHY_SPEC>;
#[doc = "NA"]
pub mod int_force_phy;
#[doc = "INT_ST_BNDRY_FRAME_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_bndry_frame_fatal::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_bndry_frame_fatal`] module"]
pub type INT_ST_BNDRY_FRAME_FATAL =
    crate::Reg<int_st_bndry_frame_fatal::INT_ST_BNDRY_FRAME_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_st_bndry_frame_fatal;
#[doc = "INT_MSK_BNDRY_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk_bndry_frame_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk_bndry_frame_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_bndry_frame_fatal`] module"]
pub type INT_MSK_BNDRY_FRAME_FATAL =
    crate::Reg<int_msk_bndry_frame_fatal::INT_MSK_BNDRY_FRAME_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_msk_bndry_frame_fatal;
#[doc = "INT_FORCE_BNDRY_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_bndry_frame_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_bndry_frame_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_bndry_frame_fatal`] module"]
pub type INT_FORCE_BNDRY_FRAME_FATAL =
    crate::Reg<int_force_bndry_frame_fatal::INT_FORCE_BNDRY_FRAME_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_force_bndry_frame_fatal;
#[doc = "INT_ST_SEQ_FRAME_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_seq_frame_fatal::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_seq_frame_fatal`] module"]
pub type INT_ST_SEQ_FRAME_FATAL = crate::Reg<int_st_seq_frame_fatal::INT_ST_SEQ_FRAME_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_st_seq_frame_fatal;
#[doc = "INT_MSK_SEQ_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk_seq_frame_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk_seq_frame_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_seq_frame_fatal`] module"]
pub type INT_MSK_SEQ_FRAME_FATAL =
    crate::Reg<int_msk_seq_frame_fatal::INT_MSK_SEQ_FRAME_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_msk_seq_frame_fatal;
#[doc = "INT_FORCE_SEQ_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_seq_frame_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_seq_frame_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_seq_frame_fatal`] module"]
pub type INT_FORCE_SEQ_FRAME_FATAL =
    crate::Reg<int_force_seq_frame_fatal::INT_FORCE_SEQ_FRAME_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_force_seq_frame_fatal;
#[doc = "INT_ST_CRC_FRAME_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_crc_frame_fatal::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_crc_frame_fatal`] module"]
pub type INT_ST_CRC_FRAME_FATAL = crate::Reg<int_st_crc_frame_fatal::INT_ST_CRC_FRAME_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_st_crc_frame_fatal;
#[doc = "INT_MSK_CRC_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk_crc_frame_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk_crc_frame_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_crc_frame_fatal`] module"]
pub type INT_MSK_CRC_FRAME_FATAL =
    crate::Reg<int_msk_crc_frame_fatal::INT_MSK_CRC_FRAME_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_msk_crc_frame_fatal;
#[doc = "INT_FORCE_CRC_FRAME_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_crc_frame_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_crc_frame_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_crc_frame_fatal`] module"]
pub type INT_FORCE_CRC_FRAME_FATAL =
    crate::Reg<int_force_crc_frame_fatal::INT_FORCE_CRC_FRAME_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_force_crc_frame_fatal;
#[doc = "INT_ST_PLD_CRC_FATAL (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_pld_crc_fatal::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_pld_crc_fatal`] module"]
pub type INT_ST_PLD_CRC_FATAL = crate::Reg<int_st_pld_crc_fatal::INT_ST_PLD_CRC_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_st_pld_crc_fatal;
#[doc = "INT_MSK_PLD_CRC_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk_pld_crc_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk_pld_crc_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_pld_crc_fatal`] module"]
pub type INT_MSK_PLD_CRC_FATAL = crate::Reg<int_msk_pld_crc_fatal::INT_MSK_PLD_CRC_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_msk_pld_crc_fatal;
#[doc = "INT_FORCE_PLD_CRC_FATAL (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_pld_crc_fatal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_pld_crc_fatal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_pld_crc_fatal`] module"]
pub type INT_FORCE_PLD_CRC_FATAL =
    crate::Reg<int_force_pld_crc_fatal::INT_FORCE_PLD_CRC_FATAL_SPEC>;
#[doc = "NA"]
pub mod int_force_pld_crc_fatal;
#[doc = "INT_ST_DATA_ID (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_data_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_data_id`] module"]
pub type INT_ST_DATA_ID = crate::Reg<int_st_data_id::INT_ST_DATA_ID_SPEC>;
#[doc = "NA"]
pub mod int_st_data_id;
#[doc = "INT_MSK_DATA_ID (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk_data_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk_data_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_data_id`] module"]
pub type INT_MSK_DATA_ID = crate::Reg<int_msk_data_id::INT_MSK_DATA_ID_SPEC>;
#[doc = "NA"]
pub mod int_msk_data_id;
#[doc = "INT_FORCE_DATA_ID (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_data_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_data_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_data_id`] module"]
pub type INT_FORCE_DATA_ID = crate::Reg<int_force_data_id::INT_FORCE_DATA_ID_SPEC>;
#[doc = "NA"]
pub mod int_force_data_id;
#[doc = "INT_ST_ECC_CORRECTED (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st_ecc_corrected::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st_ecc_corrected`] module"]
pub type INT_ST_ECC_CORRECTED = crate::Reg<int_st_ecc_corrected::INT_ST_ECC_CORRECTED_SPEC>;
#[doc = "NA"]
pub mod int_st_ecc_corrected;
#[doc = "INT_MSK_ECC_CORRECTED (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_msk_ecc_corrected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk_ecc_corrected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_msk_ecc_corrected`] module"]
pub type INT_MSK_ECC_CORRECTED = crate::Reg<int_msk_ecc_corrected::INT_MSK_ECC_CORRECTED_SPEC>;
#[doc = "NA"]
pub mod int_msk_ecc_corrected;
#[doc = "INT_FORCE_ECC_CORRECTED (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force_ecc_corrected::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force_ecc_corrected::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_force_ecc_corrected`] module"]
pub type INT_FORCE_ECC_CORRECTED =
    crate::Reg<int_force_ecc_corrected::INT_FORCE_ECC_CORRECTED_SPEC>;
#[doc = "NA"]
pub mod int_force_ecc_corrected;
#[doc = "SCRAMBLING (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scrambling::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scrambling::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scrambling`] module"]
pub type SCRAMBLING = crate::Reg<scrambling::SCRAMBLING_SPEC>;
#[doc = "NA"]
pub mod scrambling;
#[doc = "SCRAMBLING_SEED1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scrambling_seed1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scrambling_seed1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scrambling_seed1`] module"]
pub type SCRAMBLING_SEED1 = crate::Reg<scrambling_seed1::SCRAMBLING_SEED1_SPEC>;
#[doc = "NA"]
pub mod scrambling_seed1;
#[doc = "SCRAMBLING_SEED2 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scrambling_seed2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scrambling_seed2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scrambling_seed2`] module"]
pub type SCRAMBLING_SEED2 = crate::Reg<scrambling_seed2::SCRAMBLING_SEED2_SPEC>;
#[doc = "NA"]
pub mod scrambling_seed2;
