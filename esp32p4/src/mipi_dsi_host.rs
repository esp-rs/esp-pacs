#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    version: VERSION,
    pwr_up: PWR_UP,
    clkmgr_cfg: CLKMGR_CFG,
    dpi_vcid: DPI_VCID,
    dpi_color_coding: DPI_COLOR_CODING,
    dpi_cfg_pol: DPI_CFG_POL,
    dpi_lp_cmd_tim: DPI_LP_CMD_TIM,
    dbi_vcid: DBI_VCID,
    dbi_cfg: DBI_CFG,
    dbi_partitioning_en: DBI_PARTITIONING_EN,
    dbi_cmdsize: DBI_CMDSIZE,
    pckhdl_cfg: PCKHDL_CFG,
    gen_vcid: GEN_VCID,
    mode_cfg: MODE_CFG,
    vid_mode_cfg: VID_MODE_CFG,
    vid_pkt_size: VID_PKT_SIZE,
    vid_num_chunks: VID_NUM_CHUNKS,
    vid_null_size: VID_NULL_SIZE,
    vid_hsa_time: VID_HSA_TIME,
    vid_hbp_time: VID_HBP_TIME,
    vid_hline_time: VID_HLINE_TIME,
    vid_vsa_lines: VID_VSA_LINES,
    vid_vbp_lines: VID_VBP_LINES,
    vid_vfp_lines: VID_VFP_LINES,
    vid_vactive_lines: VID_VACTIVE_LINES,
    edpi_cmd_size: EDPI_CMD_SIZE,
    cmd_mode_cfg: CMD_MODE_CFG,
    gen_hdr: GEN_HDR,
    gen_pld_data: GEN_PLD_DATA,
    cmd_pkt_status: CMD_PKT_STATUS,
    to_cnt_cfg: TO_CNT_CFG,
    hs_rd_to_cnt: HS_RD_TO_CNT,
    lp_rd_to_cnt: LP_RD_TO_CNT,
    hs_wr_to_cnt: HS_WR_TO_CNT,
    lp_wr_to_cnt: LP_WR_TO_CNT,
    bta_to_cnt: BTA_TO_CNT,
    sdf_3d: SDF_3D,
    lpclk_ctrl: LPCLK_CTRL,
    phy_tmr_lpclk_cfg: PHY_TMR_LPCLK_CFG,
    phy_tmr_cfg: PHY_TMR_CFG,
    phy_rstz: PHY_RSTZ,
    phy_if_cfg: PHY_IF_CFG,
    phy_ulps_ctrl: PHY_ULPS_CTRL,
    phy_tx_triggers: PHY_TX_TRIGGERS,
    phy_status: PHY_STATUS,
    phy_tst_ctrl0: PHY_TST_CTRL0,
    phy_tst_ctrl1: PHY_TST_CTRL1,
    int_st0: INT_ST0,
    int_st1: INT_ST1,
    int_msk0: INT_MSK0,
    int_msk1: INT_MSK1,
    phy_cal: PHY_CAL,
    _reserved52: [u8; 0x08],
    int_force0: INT_FORCE0,
    int_force1: INT_FORCE1,
    _reserved54: [u8; 0x10],
    dsc_parameter: DSC_PARAMETER,
    phy_tmr_rd_cfg: PHY_TMR_RD_CFG,
    _reserved56: [u8; 0x08],
    vid_shadow_ctrl: VID_SHADOW_CTRL,
    _reserved57: [u8; 0x08],
    dpi_vcid_act: DPI_VCID_ACT,
    dpi_color_coding_act: DPI_COLOR_CODING_ACT,
    _reserved59: [u8; 0x04],
    dpi_lp_cmd_tim_act: DPI_LP_CMD_TIM_ACT,
    edpi_te_hw_cfg: EDPI_TE_HW_CFG,
    _reserved61: [u8; 0x18],
    vid_mode_cfg_act: VID_MODE_CFG_ACT,
    vid_pkt_size_act: VID_PKT_SIZE_ACT,
    vid_num_chunks_act: VID_NUM_CHUNKS_ACT,
    vid_null_size_act: VID_NULL_SIZE_ACT,
    vid_hsa_time_act: VID_HSA_TIME_ACT,
    vid_hbp_time_act: VID_HBP_TIME_ACT,
    vid_hline_time_act: VID_HLINE_TIME_ACT,
    vid_vsa_lines_act: VID_VSA_LINES_ACT,
    vid_vbp_lines_act: VID_VBP_LINES_ACT,
    vid_vfp_lines_act: VID_VFP_LINES_ACT,
    vid_vactive_lines_act: VID_VACTIVE_LINES_ACT,
    _reserved72: [u8; 0x04],
    vid_pkt_status: VID_PKT_STATUS,
    _reserved73: [u8; 0x24],
    sdf_3d_act: SDF_3D_ACT,
}
impl RegisterBlock {
    ///0x00 - NA
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
    ///0x04 - NA
    #[inline(always)]
    pub const fn pwr_up(&self) -> &PWR_UP {
        &self.pwr_up
    }
    ///0x08 - NA
    #[inline(always)]
    pub const fn clkmgr_cfg(&self) -> &CLKMGR_CFG {
        &self.clkmgr_cfg
    }
    ///0x0c - NA
    #[inline(always)]
    pub const fn dpi_vcid(&self) -> &DPI_VCID {
        &self.dpi_vcid
    }
    ///0x10 - NA
    #[inline(always)]
    pub const fn dpi_color_coding(&self) -> &DPI_COLOR_CODING {
        &self.dpi_color_coding
    }
    ///0x14 - NA
    #[inline(always)]
    pub const fn dpi_cfg_pol(&self) -> &DPI_CFG_POL {
        &self.dpi_cfg_pol
    }
    ///0x18 - NA
    #[inline(always)]
    pub const fn dpi_lp_cmd_tim(&self) -> &DPI_LP_CMD_TIM {
        &self.dpi_lp_cmd_tim
    }
    ///0x1c - NA
    #[inline(always)]
    pub const fn dbi_vcid(&self) -> &DBI_VCID {
        &self.dbi_vcid
    }
    ///0x20 - NA
    #[inline(always)]
    pub const fn dbi_cfg(&self) -> &DBI_CFG {
        &self.dbi_cfg
    }
    ///0x24 - NA
    #[inline(always)]
    pub const fn dbi_partitioning_en(&self) -> &DBI_PARTITIONING_EN {
        &self.dbi_partitioning_en
    }
    ///0x28 - NA
    #[inline(always)]
    pub const fn dbi_cmdsize(&self) -> &DBI_CMDSIZE {
        &self.dbi_cmdsize
    }
    ///0x2c - NA
    #[inline(always)]
    pub const fn pckhdl_cfg(&self) -> &PCKHDL_CFG {
        &self.pckhdl_cfg
    }
    ///0x30 - NA
    #[inline(always)]
    pub const fn gen_vcid(&self) -> &GEN_VCID {
        &self.gen_vcid
    }
    ///0x34 - NA
    #[inline(always)]
    pub const fn mode_cfg(&self) -> &MODE_CFG {
        &self.mode_cfg
    }
    ///0x38 - NA
    #[inline(always)]
    pub const fn vid_mode_cfg(&self) -> &VID_MODE_CFG {
        &self.vid_mode_cfg
    }
    ///0x3c - NA
    #[inline(always)]
    pub const fn vid_pkt_size(&self) -> &VID_PKT_SIZE {
        &self.vid_pkt_size
    }
    ///0x40 - NA
    #[inline(always)]
    pub const fn vid_num_chunks(&self) -> &VID_NUM_CHUNKS {
        &self.vid_num_chunks
    }
    ///0x44 - NA
    #[inline(always)]
    pub const fn vid_null_size(&self) -> &VID_NULL_SIZE {
        &self.vid_null_size
    }
    ///0x48 - NA
    #[inline(always)]
    pub const fn vid_hsa_time(&self) -> &VID_HSA_TIME {
        &self.vid_hsa_time
    }
    ///0x4c - NA
    #[inline(always)]
    pub const fn vid_hbp_time(&self) -> &VID_HBP_TIME {
        &self.vid_hbp_time
    }
    ///0x50 - NA
    #[inline(always)]
    pub const fn vid_hline_time(&self) -> &VID_HLINE_TIME {
        &self.vid_hline_time
    }
    ///0x54 - NA
    #[inline(always)]
    pub const fn vid_vsa_lines(&self) -> &VID_VSA_LINES {
        &self.vid_vsa_lines
    }
    ///0x58 - NA
    #[inline(always)]
    pub const fn vid_vbp_lines(&self) -> &VID_VBP_LINES {
        &self.vid_vbp_lines
    }
    ///0x5c - NA
    #[inline(always)]
    pub const fn vid_vfp_lines(&self) -> &VID_VFP_LINES {
        &self.vid_vfp_lines
    }
    ///0x60 - NA
    #[inline(always)]
    pub const fn vid_vactive_lines(&self) -> &VID_VACTIVE_LINES {
        &self.vid_vactive_lines
    }
    ///0x64 - NA
    #[inline(always)]
    pub const fn edpi_cmd_size(&self) -> &EDPI_CMD_SIZE {
        &self.edpi_cmd_size
    }
    ///0x68 - NA
    #[inline(always)]
    pub const fn cmd_mode_cfg(&self) -> &CMD_MODE_CFG {
        &self.cmd_mode_cfg
    }
    ///0x6c - NA
    #[inline(always)]
    pub const fn gen_hdr(&self) -> &GEN_HDR {
        &self.gen_hdr
    }
    ///0x70 - NA
    #[inline(always)]
    pub const fn gen_pld_data(&self) -> &GEN_PLD_DATA {
        &self.gen_pld_data
    }
    ///0x74 - NA
    #[inline(always)]
    pub const fn cmd_pkt_status(&self) -> &CMD_PKT_STATUS {
        &self.cmd_pkt_status
    }
    ///0x78 - NA
    #[inline(always)]
    pub const fn to_cnt_cfg(&self) -> &TO_CNT_CFG {
        &self.to_cnt_cfg
    }
    ///0x7c - NA
    #[inline(always)]
    pub const fn hs_rd_to_cnt(&self) -> &HS_RD_TO_CNT {
        &self.hs_rd_to_cnt
    }
    ///0x80 - NA
    #[inline(always)]
    pub const fn lp_rd_to_cnt(&self) -> &LP_RD_TO_CNT {
        &self.lp_rd_to_cnt
    }
    ///0x84 - NA
    #[inline(always)]
    pub const fn hs_wr_to_cnt(&self) -> &HS_WR_TO_CNT {
        &self.hs_wr_to_cnt
    }
    ///0x88 - NA
    #[inline(always)]
    pub const fn lp_wr_to_cnt(&self) -> &LP_WR_TO_CNT {
        &self.lp_wr_to_cnt
    }
    ///0x8c - NA
    #[inline(always)]
    pub const fn bta_to_cnt(&self) -> &BTA_TO_CNT {
        &self.bta_to_cnt
    }
    ///0x90 - NA
    #[inline(always)]
    pub const fn sdf_3d(&self) -> &SDF_3D {
        &self.sdf_3d
    }
    ///0x94 - NA
    #[inline(always)]
    pub const fn lpclk_ctrl(&self) -> &LPCLK_CTRL {
        &self.lpclk_ctrl
    }
    ///0x98 - NA
    #[inline(always)]
    pub const fn phy_tmr_lpclk_cfg(&self) -> &PHY_TMR_LPCLK_CFG {
        &self.phy_tmr_lpclk_cfg
    }
    ///0x9c - NA
    #[inline(always)]
    pub const fn phy_tmr_cfg(&self) -> &PHY_TMR_CFG {
        &self.phy_tmr_cfg
    }
    ///0xa0 - NA
    #[inline(always)]
    pub const fn phy_rstz(&self) -> &PHY_RSTZ {
        &self.phy_rstz
    }
    ///0xa4 - NA
    #[inline(always)]
    pub const fn phy_if_cfg(&self) -> &PHY_IF_CFG {
        &self.phy_if_cfg
    }
    ///0xa8 - NA
    #[inline(always)]
    pub const fn phy_ulps_ctrl(&self) -> &PHY_ULPS_CTRL {
        &self.phy_ulps_ctrl
    }
    ///0xac - NA
    #[inline(always)]
    pub const fn phy_tx_triggers(&self) -> &PHY_TX_TRIGGERS {
        &self.phy_tx_triggers
    }
    ///0xb0 - NA
    #[inline(always)]
    pub const fn phy_status(&self) -> &PHY_STATUS {
        &self.phy_status
    }
    ///0xb4 - NA
    #[inline(always)]
    pub const fn phy_tst_ctrl0(&self) -> &PHY_TST_CTRL0 {
        &self.phy_tst_ctrl0
    }
    ///0xb8 - NA
    #[inline(always)]
    pub const fn phy_tst_ctrl1(&self) -> &PHY_TST_CTRL1 {
        &self.phy_tst_ctrl1
    }
    ///0xbc - NA
    #[inline(always)]
    pub const fn int_st0(&self) -> &INT_ST0 {
        &self.int_st0
    }
    ///0xc0 - NA
    #[inline(always)]
    pub const fn int_st1(&self) -> &INT_ST1 {
        &self.int_st1
    }
    ///0xc4 - NA
    #[inline(always)]
    pub const fn int_msk0(&self) -> &INT_MSK0 {
        &self.int_msk0
    }
    ///0xc8 - NA
    #[inline(always)]
    pub const fn int_msk1(&self) -> &INT_MSK1 {
        &self.int_msk1
    }
    ///0xcc - NA
    #[inline(always)]
    pub const fn phy_cal(&self) -> &PHY_CAL {
        &self.phy_cal
    }
    ///0xd8 - NA
    #[inline(always)]
    pub const fn int_force0(&self) -> &INT_FORCE0 {
        &self.int_force0
    }
    ///0xdc - NA
    #[inline(always)]
    pub const fn int_force1(&self) -> &INT_FORCE1 {
        &self.int_force1
    }
    ///0xf0 - NA
    #[inline(always)]
    pub const fn dsc_parameter(&self) -> &DSC_PARAMETER {
        &self.dsc_parameter
    }
    ///0xf4 - NA
    #[inline(always)]
    pub const fn phy_tmr_rd_cfg(&self) -> &PHY_TMR_RD_CFG {
        &self.phy_tmr_rd_cfg
    }
    ///0x100 - NA
    #[inline(always)]
    pub const fn vid_shadow_ctrl(&self) -> &VID_SHADOW_CTRL {
        &self.vid_shadow_ctrl
    }
    ///0x10c - NA
    #[inline(always)]
    pub const fn dpi_vcid_act(&self) -> &DPI_VCID_ACT {
        &self.dpi_vcid_act
    }
    ///0x110 - NA
    #[inline(always)]
    pub const fn dpi_color_coding_act(&self) -> &DPI_COLOR_CODING_ACT {
        &self.dpi_color_coding_act
    }
    ///0x118 - NA
    #[inline(always)]
    pub const fn dpi_lp_cmd_tim_act(&self) -> &DPI_LP_CMD_TIM_ACT {
        &self.dpi_lp_cmd_tim_act
    }
    ///0x11c - NA
    #[inline(always)]
    pub const fn edpi_te_hw_cfg(&self) -> &EDPI_TE_HW_CFG {
        &self.edpi_te_hw_cfg
    }
    ///0x138 - NA
    #[inline(always)]
    pub const fn vid_mode_cfg_act(&self) -> &VID_MODE_CFG_ACT {
        &self.vid_mode_cfg_act
    }
    ///0x13c - NA
    #[inline(always)]
    pub const fn vid_pkt_size_act(&self) -> &VID_PKT_SIZE_ACT {
        &self.vid_pkt_size_act
    }
    ///0x140 - NA
    #[inline(always)]
    pub const fn vid_num_chunks_act(&self) -> &VID_NUM_CHUNKS_ACT {
        &self.vid_num_chunks_act
    }
    ///0x144 - NA
    #[inline(always)]
    pub const fn vid_null_size_act(&self) -> &VID_NULL_SIZE_ACT {
        &self.vid_null_size_act
    }
    ///0x148 - NA
    #[inline(always)]
    pub const fn vid_hsa_time_act(&self) -> &VID_HSA_TIME_ACT {
        &self.vid_hsa_time_act
    }
    ///0x14c - NA
    #[inline(always)]
    pub const fn vid_hbp_time_act(&self) -> &VID_HBP_TIME_ACT {
        &self.vid_hbp_time_act
    }
    ///0x150 - NA
    #[inline(always)]
    pub const fn vid_hline_time_act(&self) -> &VID_HLINE_TIME_ACT {
        &self.vid_hline_time_act
    }
    ///0x154 - NA
    #[inline(always)]
    pub const fn vid_vsa_lines_act(&self) -> &VID_VSA_LINES_ACT {
        &self.vid_vsa_lines_act
    }
    ///0x158 - NA
    #[inline(always)]
    pub const fn vid_vbp_lines_act(&self) -> &VID_VBP_LINES_ACT {
        &self.vid_vbp_lines_act
    }
    ///0x15c - NA
    #[inline(always)]
    pub const fn vid_vfp_lines_act(&self) -> &VID_VFP_LINES_ACT {
        &self.vid_vfp_lines_act
    }
    ///0x160 - NA
    #[inline(always)]
    pub const fn vid_vactive_lines_act(&self) -> &VID_VACTIVE_LINES_ACT {
        &self.vid_vactive_lines_act
    }
    ///0x168 - NA
    #[inline(always)]
    pub const fn vid_pkt_status(&self) -> &VID_PKT_STATUS {
        &self.vid_pkt_status
    }
    ///0x190 - NA
    #[inline(always)]
    pub const fn sdf_3d_act(&self) -> &SDF_3D_ACT {
        &self.sdf_3d_act
    }
}
/**VERSION (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@version`] module*/
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
///NA
pub mod version;
/**PWR_UP (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`pwr_up::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_up::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwr_up`] module*/
pub type PWR_UP = crate::Reg<pwr_up::PWR_UP_SPEC>;
///NA
pub mod pwr_up;
/**CLKMGR_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`clkmgr_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkmgr_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clkmgr_cfg`] module*/
pub type CLKMGR_CFG = crate::Reg<clkmgr_cfg::CLKMGR_CFG_SPEC>;
///NA
pub mod clkmgr_cfg;
/**DPI_VCID (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dpi_vcid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_vcid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpi_vcid`] module*/
pub type DPI_VCID = crate::Reg<dpi_vcid::DPI_VCID_SPEC>;
///NA
pub mod dpi_vcid;
/**DPI_COLOR_CODING (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dpi_color_coding::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_color_coding::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpi_color_coding`] module*/
pub type DPI_COLOR_CODING = crate::Reg<dpi_color_coding::DPI_COLOR_CODING_SPEC>;
///NA
pub mod dpi_color_coding;
/**DPI_CFG_POL (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dpi_cfg_pol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_cfg_pol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpi_cfg_pol`] module*/
pub type DPI_CFG_POL = crate::Reg<dpi_cfg_pol::DPI_CFG_POL_SPEC>;
///NA
pub mod dpi_cfg_pol;
/**DPI_LP_CMD_TIM (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dpi_lp_cmd_tim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_lp_cmd_tim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpi_lp_cmd_tim`] module*/
pub type DPI_LP_CMD_TIM = crate::Reg<dpi_lp_cmd_tim::DPI_LP_CMD_TIM_SPEC>;
///NA
pub mod dpi_lp_cmd_tim;
/**DBI_VCID (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dbi_vcid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_vcid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbi_vcid`] module*/
pub type DBI_VCID = crate::Reg<dbi_vcid::DBI_VCID_SPEC>;
///NA
pub mod dbi_vcid;
/**DBI_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dbi_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbi_cfg`] module*/
pub type DBI_CFG = crate::Reg<dbi_cfg::DBI_CFG_SPEC>;
///NA
pub mod dbi_cfg;
/**DBI_PARTITIONING_EN (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dbi_partitioning_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_partitioning_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbi_partitioning_en`] module*/
pub type DBI_PARTITIONING_EN = crate::Reg<dbi_partitioning_en::DBI_PARTITIONING_EN_SPEC>;
///NA
pub mod dbi_partitioning_en;
/**DBI_CMDSIZE (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dbi_cmdsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_cmdsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbi_cmdsize`] module*/
pub type DBI_CMDSIZE = crate::Reg<dbi_cmdsize::DBI_CMDSIZE_SPEC>;
///NA
pub mod dbi_cmdsize;
/**PCKHDL_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`pckhdl_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pckhdl_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pckhdl_cfg`] module*/
pub type PCKHDL_CFG = crate::Reg<pckhdl_cfg::PCKHDL_CFG_SPEC>;
///NA
pub mod pckhdl_cfg;
/**GEN_VCID (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`gen_vcid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_vcid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gen_vcid`] module*/
pub type GEN_VCID = crate::Reg<gen_vcid::GEN_VCID_SPEC>;
///NA
pub mod gen_vcid;
/**MODE_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`mode_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mode_cfg`] module*/
pub type MODE_CFG = crate::Reg<mode_cfg::MODE_CFG_SPEC>;
///NA
pub mod mode_cfg;
/**VID_MODE_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_mode_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_mode_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_mode_cfg`] module*/
pub type VID_MODE_CFG = crate::Reg<vid_mode_cfg::VID_MODE_CFG_SPEC>;
///NA
pub mod vid_mode_cfg;
/**VID_PKT_SIZE (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_pkt_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_pkt_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_pkt_size`] module*/
pub type VID_PKT_SIZE = crate::Reg<vid_pkt_size::VID_PKT_SIZE_SPEC>;
///NA
pub mod vid_pkt_size;
/**VID_NUM_CHUNKS (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_num_chunks::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_num_chunks::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_num_chunks`] module*/
pub type VID_NUM_CHUNKS = crate::Reg<vid_num_chunks::VID_NUM_CHUNKS_SPEC>;
///NA
pub mod vid_num_chunks;
/**VID_NULL_SIZE (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_null_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_null_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_null_size`] module*/
pub type VID_NULL_SIZE = crate::Reg<vid_null_size::VID_NULL_SIZE_SPEC>;
///NA
pub mod vid_null_size;
/**VID_HSA_TIME (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_hsa_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_hsa_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_hsa_time`] module*/
pub type VID_HSA_TIME = crate::Reg<vid_hsa_time::VID_HSA_TIME_SPEC>;
///NA
pub mod vid_hsa_time;
/**VID_HBP_TIME (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_hbp_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_hbp_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_hbp_time`] module*/
pub type VID_HBP_TIME = crate::Reg<vid_hbp_time::VID_HBP_TIME_SPEC>;
///NA
pub mod vid_hbp_time;
/**VID_HLINE_TIME (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_hline_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_hline_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_hline_time`] module*/
pub type VID_HLINE_TIME = crate::Reg<vid_hline_time::VID_HLINE_TIME_SPEC>;
///NA
pub mod vid_hline_time;
/**VID_VSA_LINES (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_vsa_lines::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vsa_lines::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_vsa_lines`] module*/
pub type VID_VSA_LINES = crate::Reg<vid_vsa_lines::VID_VSA_LINES_SPEC>;
///NA
pub mod vid_vsa_lines;
/**VID_VBP_LINES (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_vbp_lines::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vbp_lines::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_vbp_lines`] module*/
pub type VID_VBP_LINES = crate::Reg<vid_vbp_lines::VID_VBP_LINES_SPEC>;
///NA
pub mod vid_vbp_lines;
/**VID_VFP_LINES (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_vfp_lines::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vfp_lines::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_vfp_lines`] module*/
pub type VID_VFP_LINES = crate::Reg<vid_vfp_lines::VID_VFP_LINES_SPEC>;
///NA
pub mod vid_vfp_lines;
/**VID_VACTIVE_LINES (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_vactive_lines::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_vactive_lines::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_vactive_lines`] module*/
pub type VID_VACTIVE_LINES = crate::Reg<vid_vactive_lines::VID_VACTIVE_LINES_SPEC>;
///NA
pub mod vid_vactive_lines;
/**EDPI_CMD_SIZE (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`edpi_cmd_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edpi_cmd_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@edpi_cmd_size`] module*/
pub type EDPI_CMD_SIZE = crate::Reg<edpi_cmd_size::EDPI_CMD_SIZE_SPEC>;
///NA
pub mod edpi_cmd_size;
/**CMD_MODE_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`cmd_mode_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_mode_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd_mode_cfg`] module*/
pub type CMD_MODE_CFG = crate::Reg<cmd_mode_cfg::CMD_MODE_CFG_SPEC>;
///NA
pub mod cmd_mode_cfg;
/**GEN_HDR (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`gen_hdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_hdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gen_hdr`] module*/
pub type GEN_HDR = crate::Reg<gen_hdr::GEN_HDR_SPEC>;
///NA
pub mod gen_hdr;
/**GEN_PLD_DATA (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`gen_pld_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_pld_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gen_pld_data`] module*/
pub type GEN_PLD_DATA = crate::Reg<gen_pld_data::GEN_PLD_DATA_SPEC>;
///NA
pub mod gen_pld_data;
/**CMD_PKT_STATUS (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`cmd_pkt_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd_pkt_status`] module*/
pub type CMD_PKT_STATUS = crate::Reg<cmd_pkt_status::CMD_PKT_STATUS_SPEC>;
///NA
pub mod cmd_pkt_status;
/**TO_CNT_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`to_cnt_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`to_cnt_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@to_cnt_cfg`] module*/
pub type TO_CNT_CFG = crate::Reg<to_cnt_cfg::TO_CNT_CFG_SPEC>;
///NA
pub mod to_cnt_cfg;
/**HS_RD_TO_CNT (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`hs_rd_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_rd_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hs_rd_to_cnt`] module*/
pub type HS_RD_TO_CNT = crate::Reg<hs_rd_to_cnt::HS_RD_TO_CNT_SPEC>;
///NA
pub mod hs_rd_to_cnt;
/**LP_RD_TO_CNT (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`lp_rd_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rd_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_rd_to_cnt`] module*/
pub type LP_RD_TO_CNT = crate::Reg<lp_rd_to_cnt::LP_RD_TO_CNT_SPEC>;
///NA
pub mod lp_rd_to_cnt;
/**HS_WR_TO_CNT (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`hs_wr_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_wr_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hs_wr_to_cnt`] module*/
pub type HS_WR_TO_CNT = crate::Reg<hs_wr_to_cnt::HS_WR_TO_CNT_SPEC>;
///NA
pub mod hs_wr_to_cnt;
/**LP_WR_TO_CNT (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`lp_wr_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_wr_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_wr_to_cnt`] module*/
pub type LP_WR_TO_CNT = crate::Reg<lp_wr_to_cnt::LP_WR_TO_CNT_SPEC>;
///NA
pub mod lp_wr_to_cnt;
/**BTA_TO_CNT (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`bta_to_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bta_to_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bta_to_cnt`] module*/
pub type BTA_TO_CNT = crate::Reg<bta_to_cnt::BTA_TO_CNT_SPEC>;
///NA
pub mod bta_to_cnt;
/**SDF_3D (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`sdf_3d::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdf_3d::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdf_3d`] module*/
pub type SDF_3D = crate::Reg<sdf_3d::SDF_3D_SPEC>;
///NA
pub mod sdf_3d;
/**LPCLK_CTRL (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`lpclk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpclk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpclk_ctrl`] module*/
pub type LPCLK_CTRL = crate::Reg<lpclk_ctrl::LPCLK_CTRL_SPEC>;
///NA
pub mod lpclk_ctrl;
/**PHY_TMR_LPCLK_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_tmr_lpclk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tmr_lpclk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_tmr_lpclk_cfg`] module*/
pub type PHY_TMR_LPCLK_CFG = crate::Reg<phy_tmr_lpclk_cfg::PHY_TMR_LPCLK_CFG_SPEC>;
///NA
pub mod phy_tmr_lpclk_cfg;
/**PHY_TMR_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_tmr_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tmr_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_tmr_cfg`] module*/
pub type PHY_TMR_CFG = crate::Reg<phy_tmr_cfg::PHY_TMR_CFG_SPEC>;
///NA
pub mod phy_tmr_cfg;
/**PHY_RSTZ (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_rstz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_rstz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_rstz`] module*/
pub type PHY_RSTZ = crate::Reg<phy_rstz::PHY_RSTZ_SPEC>;
///NA
pub mod phy_rstz;
/**PHY_IF_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_if_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_if_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_if_cfg`] module*/
pub type PHY_IF_CFG = crate::Reg<phy_if_cfg::PHY_IF_CFG_SPEC>;
///NA
pub mod phy_if_cfg;
/**PHY_ULPS_CTRL (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_ulps_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_ulps_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_ulps_ctrl`] module*/
pub type PHY_ULPS_CTRL = crate::Reg<phy_ulps_ctrl::PHY_ULPS_CTRL_SPEC>;
///NA
pub mod phy_ulps_ctrl;
/**PHY_TX_TRIGGERS (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_tx_triggers::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tx_triggers::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_tx_triggers`] module*/
pub type PHY_TX_TRIGGERS = crate::Reg<phy_tx_triggers::PHY_TX_TRIGGERS_SPEC>;
///NA
pub mod phy_tx_triggers;
/**PHY_STATUS (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_status`] module*/
pub type PHY_STATUS = crate::Reg<phy_status::PHY_STATUS_SPEC>;
///NA
pub mod phy_status;
/**PHY_TST_CTRL0 (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_tst_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_tst_ctrl0`] module*/
pub type PHY_TST_CTRL0 = crate::Reg<phy_tst_ctrl0::PHY_TST_CTRL0_SPEC>;
///NA
pub mod phy_tst_ctrl0;
/**PHY_TST_CTRL1 (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_tst_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_tst_ctrl1`] module*/
pub type PHY_TST_CTRL1 = crate::Reg<phy_tst_ctrl1::PHY_TST_CTRL1_SPEC>;
///NA
pub mod phy_tst_ctrl1;
/**INT_ST0 (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`int_st0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st0`] module*/
pub type INT_ST0 = crate::Reg<int_st0::INT_ST0_SPEC>;
///NA
pub mod int_st0;
/**INT_ST1 (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`int_st1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st1`] module*/
pub type INT_ST1 = crate::Reg<int_st1::INT_ST1_SPEC>;
///NA
pub mod int_st1;
/**INT_MSK0 (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`int_msk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_msk0`] module*/
pub type INT_MSK0 = crate::Reg<int_msk0::INT_MSK0_SPEC>;
///NA
pub mod int_msk0;
/**INT_MSK1 (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`int_msk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_msk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_msk1`] module*/
pub type INT_MSK1 = crate::Reg<int_msk1::INT_MSK1_SPEC>;
///NA
pub mod int_msk1;
/**PHY_CAL (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_cal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_cal`] module*/
pub type PHY_CAL = crate::Reg<phy_cal::PHY_CAL_SPEC>;
///NA
pub mod phy_cal;
/**INT_FORCE0 (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`int_force0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_force0`] module*/
pub type INT_FORCE0 = crate::Reg<int_force0::INT_FORCE0_SPEC>;
///NA
pub mod int_force0;
/**INT_FORCE1 (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`int_force1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_force1`] module*/
pub type INT_FORCE1 = crate::Reg<int_force1::INT_FORCE1_SPEC>;
///NA
pub mod int_force1;
/**DSC_PARAMETER (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dsc_parameter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsc_parameter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dsc_parameter`] module*/
pub type DSC_PARAMETER = crate::Reg<dsc_parameter::DSC_PARAMETER_SPEC>;
///NA
pub mod dsc_parameter;
/**PHY_TMR_RD_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_tmr_rd_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tmr_rd_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@phy_tmr_rd_cfg`] module*/
pub type PHY_TMR_RD_CFG = crate::Reg<phy_tmr_rd_cfg::PHY_TMR_RD_CFG_SPEC>;
///NA
pub mod phy_tmr_rd_cfg;
/**VID_SHADOW_CTRL (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_shadow_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_shadow_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_shadow_ctrl`] module*/
pub type VID_SHADOW_CTRL = crate::Reg<vid_shadow_ctrl::VID_SHADOW_CTRL_SPEC>;
///NA
pub mod vid_shadow_ctrl;
/**DPI_VCID_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dpi_vcid_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpi_vcid_act`] module*/
pub type DPI_VCID_ACT = crate::Reg<dpi_vcid_act::DPI_VCID_ACT_SPEC>;
///NA
pub mod dpi_vcid_act;
/**DPI_COLOR_CODING_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dpi_color_coding_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpi_color_coding_act`] module*/
pub type DPI_COLOR_CODING_ACT = crate::Reg<dpi_color_coding_act::DPI_COLOR_CODING_ACT_SPEC>;
///NA
pub mod dpi_color_coding_act;
/**DPI_LP_CMD_TIM_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`dpi_lp_cmd_tim_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpi_lp_cmd_tim_act`] module*/
pub type DPI_LP_CMD_TIM_ACT = crate::Reg<dpi_lp_cmd_tim_act::DPI_LP_CMD_TIM_ACT_SPEC>;
///NA
pub mod dpi_lp_cmd_tim_act;
/**EDPI_TE_HW_CFG (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`edpi_te_hw_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edpi_te_hw_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@edpi_te_hw_cfg`] module*/
pub type EDPI_TE_HW_CFG = crate::Reg<edpi_te_hw_cfg::EDPI_TE_HW_CFG_SPEC>;
///NA
pub mod edpi_te_hw_cfg;
/**VID_MODE_CFG_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_mode_cfg_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_mode_cfg_act`] module*/
pub type VID_MODE_CFG_ACT = crate::Reg<vid_mode_cfg_act::VID_MODE_CFG_ACT_SPEC>;
///NA
pub mod vid_mode_cfg_act;
/**VID_PKT_SIZE_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_pkt_size_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_pkt_size_act`] module*/
pub type VID_PKT_SIZE_ACT = crate::Reg<vid_pkt_size_act::VID_PKT_SIZE_ACT_SPEC>;
///NA
pub mod vid_pkt_size_act;
/**VID_NUM_CHUNKS_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_num_chunks_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_num_chunks_act`] module*/
pub type VID_NUM_CHUNKS_ACT = crate::Reg<vid_num_chunks_act::VID_NUM_CHUNKS_ACT_SPEC>;
///NA
pub mod vid_num_chunks_act;
/**VID_NULL_SIZE_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_null_size_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_null_size_act`] module*/
pub type VID_NULL_SIZE_ACT = crate::Reg<vid_null_size_act::VID_NULL_SIZE_ACT_SPEC>;
///NA
pub mod vid_null_size_act;
/**VID_HSA_TIME_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_hsa_time_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_hsa_time_act`] module*/
pub type VID_HSA_TIME_ACT = crate::Reg<vid_hsa_time_act::VID_HSA_TIME_ACT_SPEC>;
///NA
pub mod vid_hsa_time_act;
/**VID_HBP_TIME_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_hbp_time_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_hbp_time_act`] module*/
pub type VID_HBP_TIME_ACT = crate::Reg<vid_hbp_time_act::VID_HBP_TIME_ACT_SPEC>;
///NA
pub mod vid_hbp_time_act;
/**VID_HLINE_TIME_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_hline_time_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_hline_time_act`] module*/
pub type VID_HLINE_TIME_ACT = crate::Reg<vid_hline_time_act::VID_HLINE_TIME_ACT_SPEC>;
///NA
pub mod vid_hline_time_act;
/**VID_VSA_LINES_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_vsa_lines_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_vsa_lines_act`] module*/
pub type VID_VSA_LINES_ACT = crate::Reg<vid_vsa_lines_act::VID_VSA_LINES_ACT_SPEC>;
///NA
pub mod vid_vsa_lines_act;
/**VID_VBP_LINES_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_vbp_lines_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_vbp_lines_act`] module*/
pub type VID_VBP_LINES_ACT = crate::Reg<vid_vbp_lines_act::VID_VBP_LINES_ACT_SPEC>;
///NA
pub mod vid_vbp_lines_act;
/**VID_VFP_LINES_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_vfp_lines_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_vfp_lines_act`] module*/
pub type VID_VFP_LINES_ACT = crate::Reg<vid_vfp_lines_act::VID_VFP_LINES_ACT_SPEC>;
///NA
pub mod vid_vfp_lines_act;
/**VID_VACTIVE_LINES_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_vactive_lines_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_vactive_lines_act`] module*/
pub type VID_VACTIVE_LINES_ACT = crate::Reg<vid_vactive_lines_act::VID_VACTIVE_LINES_ACT_SPEC>;
///NA
pub mod vid_vactive_lines_act;
/**VID_PKT_STATUS (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_pkt_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vid_pkt_status`] module*/
pub type VID_PKT_STATUS = crate::Reg<vid_pkt_status::VID_PKT_STATUS_SPEC>;
///NA
pub mod vid_pkt_status;
/**SDF_3D_ACT (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`sdf_3d_act::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdf_3d_act`] module*/
pub type SDF_3D_ACT = crate::Reg<sdf_3d_act::SDF_3D_ACT_SPEC>;
///NA
pub mod sdf_3d_act;
