#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    clk_en: CLK_EN,
    en: EN,
    dma_req_cfg: DMA_REQ_CFG,
    raw_num_cfg: RAW_NUM_CFG,
    raw_buf_credit_ctl: RAW_BUF_CREDIT_CTL,
    fifo_flow_status: FIFO_FLOW_STATUS,
    pixel_type: PIXEL_TYPE,
    dma_block_interval: DMA_BLOCK_INTERVAL,
    dma_req_interval: DMA_REQ_INTERVAL,
    dpi_lcd_ctl: DPI_LCD_CTL,
    dpi_rsv_dpi_data: DPI_RSV_DPI_DATA,
    _reserved11: [u8; 0x04],
    dpi_v_cfg0: DPI_V_CFG0,
    dpi_v_cfg1: DPI_V_CFG1,
    dpi_h_cfg0: DPI_H_CFG0,
    dpi_h_cfg1: DPI_H_CFG1,
    dpi_misc_config: DPI_MISC_CONFIG,
    dpi_config_update: DPI_CONFIG_UPDATE,
    _reserved17: [u8; 0x08],
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    int_raw: INT_RAW,
    int_st: INT_ST,
    host_bist_ctl: HOST_BIST_CTL,
    host_trigger_rev: HOST_TRIGGER_REV,
    blk_raw_num_cfg: BLK_RAW_NUM_CFG,
    dma_frame_interval: DMA_FRAME_INTERVAL,
    mem_aux_ctrl: MEM_AUX_CTRL,
    rdn_eco_cs: RDN_ECO_CS,
    rdn_eco_low: RDN_ECO_LOW,
    rdn_eco_high: RDN_ECO_HIGH,
    host_ctrl: HOST_CTRL,
    mem_clk_ctrl: MEM_CLK_CTRL,
    dma_flow_ctrl: DMA_FLOW_CTRL,
    raw_buf_almost_empty_thrd: RAW_BUF_ALMOST_EMPTY_THRD,
    yuv_cfg: YUV_CFG,
    phy_lp_loopback_ctrl: PHY_LP_LOOPBACK_CTRL,
    phy_hs_loopback_ctrl: PHY_HS_LOOPBACK_CTRL,
    phy_loopback_cnt: PHY_LOOPBACK_CNT,
}
impl RegisterBlock {
    #[doc = "0x00 - dsi bridge clk control register"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x04 - dsi bridge en register"]
    #[inline(always)]
    pub const fn en(&self) -> &EN {
        &self.en
    }
    #[doc = "0x08 - dsi bridge dma burst len register"]
    #[inline(always)]
    pub const fn dma_req_cfg(&self) -> &DMA_REQ_CFG {
        &self.dma_req_cfg
    }
    #[doc = "0x0c - dsi bridge raw number control register"]
    #[inline(always)]
    pub const fn raw_num_cfg(&self) -> &RAW_NUM_CFG {
        &self.raw_num_cfg
    }
    #[doc = "0x10 - dsi bridge credit register"]
    #[inline(always)]
    pub const fn raw_buf_credit_ctl(&self) -> &RAW_BUF_CREDIT_CTL {
        &self.raw_buf_credit_ctl
    }
    #[doc = "0x14 - dsi bridge raw buffer depth register"]
    #[inline(always)]
    pub const fn fifo_flow_status(&self) -> &FIFO_FLOW_STATUS {
        &self.fifo_flow_status
    }
    #[doc = "0x18 - dsi bridge dpi type control register"]
    #[inline(always)]
    pub const fn pixel_type(&self) -> &PIXEL_TYPE {
        &self.pixel_type
    }
    #[doc = "0x1c - dsi bridge dma block interval control register"]
    #[inline(always)]
    pub const fn dma_block_interval(&self) -> &DMA_BLOCK_INTERVAL {
        &self.dma_block_interval
    }
    #[doc = "0x20 - dsi bridge dma req interval control register"]
    #[inline(always)]
    pub const fn dma_req_interval(&self) -> &DMA_REQ_INTERVAL {
        &self.dma_req_interval
    }
    #[doc = "0x24 - dsi bridge dpi signal control register"]
    #[inline(always)]
    pub const fn dpi_lcd_ctl(&self) -> &DPI_LCD_CTL {
        &self.dpi_lcd_ctl
    }
    #[doc = "0x28 - dsi bridge dpi reserved data register"]
    #[inline(always)]
    pub const fn dpi_rsv_dpi_data(&self) -> &DPI_RSV_DPI_DATA {
        &self.dpi_rsv_dpi_data
    }
    #[doc = "0x30 - dsi bridge dpi v config register 0"]
    #[inline(always)]
    pub const fn dpi_v_cfg0(&self) -> &DPI_V_CFG0 {
        &self.dpi_v_cfg0
    }
    #[doc = "0x34 - dsi bridge dpi v config register 1"]
    #[inline(always)]
    pub const fn dpi_v_cfg1(&self) -> &DPI_V_CFG1 {
        &self.dpi_v_cfg1
    }
    #[doc = "0x38 - dsi bridge dpi h config register 0"]
    #[inline(always)]
    pub const fn dpi_h_cfg0(&self) -> &DPI_H_CFG0 {
        &self.dpi_h_cfg0
    }
    #[doc = "0x3c - dsi bridge dpi h config register 1"]
    #[inline(always)]
    pub const fn dpi_h_cfg1(&self) -> &DPI_H_CFG1 {
        &self.dpi_h_cfg1
    }
    #[doc = "0x40 - dsi_bridge dpi misc config register"]
    #[inline(always)]
    pub const fn dpi_misc_config(&self) -> &DPI_MISC_CONFIG {
        &self.dpi_misc_config
    }
    #[doc = "0x44 - dsi_bridge dpi config update register"]
    #[inline(always)]
    pub const fn dpi_config_update(&self) -> &DPI_CONFIG_UPDATE {
        &self.dpi_config_update
    }
    #[doc = "0x50 - dsi_bridge interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x54 - dsi_bridge interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x58 - dsi_bridge raw interrupt register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x5c - dsi_bridge masked interrupt register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x60 - dsi_bridge host bist control register"]
    #[inline(always)]
    pub const fn host_bist_ctl(&self) -> &HOST_BIST_CTL {
        &self.host_bist_ctl
    }
    #[doc = "0x64 - dsi_bridge host trigger reverse control register"]
    #[inline(always)]
    pub const fn host_trigger_rev(&self) -> &HOST_TRIGGER_REV {
        &self.host_trigger_rev
    }
    #[doc = "0x68 - dsi_bridge block raw number control register"]
    #[inline(always)]
    pub const fn blk_raw_num_cfg(&self) -> &BLK_RAW_NUM_CFG {
        &self.blk_raw_num_cfg
    }
    #[doc = "0x6c - dsi_bridge dam frame interval control register"]
    #[inline(always)]
    pub const fn dma_frame_interval(&self) -> &DMA_FRAME_INTERVAL {
        &self.dma_frame_interval
    }
    #[doc = "0x70 - dsi_bridge mem aux control register"]
    #[inline(always)]
    pub const fn mem_aux_ctrl(&self) -> &MEM_AUX_CTRL {
        &self.mem_aux_ctrl
    }
    #[doc = "0x74 - dsi_bridge rdn eco cs register"]
    #[inline(always)]
    pub const fn rdn_eco_cs(&self) -> &RDN_ECO_CS {
        &self.rdn_eco_cs
    }
    #[doc = "0x78 - dsi_bridge rdn eco all low register"]
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> &RDN_ECO_LOW {
        &self.rdn_eco_low
    }
    #[doc = "0x7c - dsi_bridge rdn eco all high register"]
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> &RDN_ECO_HIGH {
        &self.rdn_eco_high
    }
    #[doc = "0x80 - dsi_bridge host control register"]
    #[inline(always)]
    pub const fn host_ctrl(&self) -> &HOST_CTRL {
        &self.host_ctrl
    }
    #[doc = "0x84 - dsi_bridge mem force on control register"]
    #[inline(always)]
    pub const fn mem_clk_ctrl(&self) -> &MEM_CLK_CTRL {
        &self.mem_clk_ctrl
    }
    #[doc = "0x88 - dsi_bridge dma flow controller register"]
    #[inline(always)]
    pub const fn dma_flow_ctrl(&self) -> &DMA_FLOW_CTRL {
        &self.dma_flow_ctrl
    }
    #[doc = "0x8c - dsi_bridge buffer empty threshold register"]
    #[inline(always)]
    pub const fn raw_buf_almost_empty_thrd(&self) -> &RAW_BUF_ALMOST_EMPTY_THRD {
        &self.raw_buf_almost_empty_thrd
    }
    #[doc = "0x90 - dsi_bridge yuv format config register"]
    #[inline(always)]
    pub const fn yuv_cfg(&self) -> &YUV_CFG {
        &self.yuv_cfg
    }
    #[doc = "0x94 - dsi phy lp_loopback test ctrl"]
    #[inline(always)]
    pub const fn phy_lp_loopback_ctrl(&self) -> &PHY_LP_LOOPBACK_CTRL {
        &self.phy_lp_loopback_ctrl
    }
    #[doc = "0x98 - dsi phy hp_loopback test ctrl"]
    #[inline(always)]
    pub const fn phy_hs_loopback_ctrl(&self) -> &PHY_HS_LOOPBACK_CTRL {
        &self.phy_hs_loopback_ctrl
    }
    #[doc = "0x9c - loopback test cnt"]
    #[inline(always)]
    pub const fn phy_loopback_cnt(&self) -> &PHY_LOOPBACK_CNT {
        &self.phy_loopback_cnt
    }
}
#[doc = "CLK_EN (rw) register accessor: dsi bridge clk control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "dsi bridge clk control register"]
pub mod clk_en;
#[doc = "EN (rw) register accessor: dsi bridge en register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "dsi bridge en register"]
pub mod en;
#[doc = "DMA_REQ_CFG (rw) register accessor: dsi bridge dma burst len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_req_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_req_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_req_cfg`] module"]
pub type DMA_REQ_CFG = crate::Reg<dma_req_cfg::DMA_REQ_CFG_SPEC>;
#[doc = "dsi bridge dma burst len register"]
pub mod dma_req_cfg;
#[doc = "RAW_NUM_CFG (rw) register accessor: dsi bridge raw number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_num_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw_num_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_num_cfg`] module"]
pub type RAW_NUM_CFG = crate::Reg<raw_num_cfg::RAW_NUM_CFG_SPEC>;
#[doc = "dsi bridge raw number control register"]
pub mod raw_num_cfg;
#[doc = "RAW_BUF_CREDIT_CTL (rw) register accessor: dsi bridge credit register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_buf_credit_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw_buf_credit_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_buf_credit_ctl`] module"]
pub type RAW_BUF_CREDIT_CTL = crate::Reg<raw_buf_credit_ctl::RAW_BUF_CREDIT_CTL_SPEC>;
#[doc = "dsi bridge credit register"]
pub mod raw_buf_credit_ctl;
#[doc = "FIFO_FLOW_STATUS (r) register accessor: dsi bridge raw buffer depth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_flow_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_flow_status`] module"]
pub type FIFO_FLOW_STATUS = crate::Reg<fifo_flow_status::FIFO_FLOW_STATUS_SPEC>;
#[doc = "dsi bridge raw buffer depth register"]
pub mod fifo_flow_status;
#[doc = "PIXEL_TYPE (rw) register accessor: dsi bridge dpi type control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pixel_type::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pixel_type::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pixel_type`] module"]
pub type PIXEL_TYPE = crate::Reg<pixel_type::PIXEL_TYPE_SPEC>;
#[doc = "dsi bridge dpi type control register"]
pub mod pixel_type;
#[doc = "DMA_BLOCK_INTERVAL (rw) register accessor: dsi bridge dma block interval control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_block_interval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_block_interval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_block_interval`] module"]
pub type DMA_BLOCK_INTERVAL = crate::Reg<dma_block_interval::DMA_BLOCK_INTERVAL_SPEC>;
#[doc = "dsi bridge dma block interval control register"]
pub mod dma_block_interval;
#[doc = "DMA_REQ_INTERVAL (rw) register accessor: dsi bridge dma req interval control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_req_interval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_req_interval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_req_interval`] module"]
pub type DMA_REQ_INTERVAL = crate::Reg<dma_req_interval::DMA_REQ_INTERVAL_SPEC>;
#[doc = "dsi bridge dma req interval control register"]
pub mod dma_req_interval;
#[doc = "DPI_LCD_CTL (rw) register accessor: dsi bridge dpi signal control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_lcd_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_lcd_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_lcd_ctl`] module"]
pub type DPI_LCD_CTL = crate::Reg<dpi_lcd_ctl::DPI_LCD_CTL_SPEC>;
#[doc = "dsi bridge dpi signal control register"]
pub mod dpi_lcd_ctl;
#[doc = "DPI_RSV_DPI_DATA (rw) register accessor: dsi bridge dpi reserved data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_rsv_dpi_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_rsv_dpi_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_rsv_dpi_data`] module"]
pub type DPI_RSV_DPI_DATA = crate::Reg<dpi_rsv_dpi_data::DPI_RSV_DPI_DATA_SPEC>;
#[doc = "dsi bridge dpi reserved data register"]
pub mod dpi_rsv_dpi_data;
#[doc = "DPI_V_CFG0 (rw) register accessor: dsi bridge dpi v config register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_v_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_v_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_v_cfg0`] module"]
pub type DPI_V_CFG0 = crate::Reg<dpi_v_cfg0::DPI_V_CFG0_SPEC>;
#[doc = "dsi bridge dpi v config register 0"]
pub mod dpi_v_cfg0;
#[doc = "DPI_V_CFG1 (rw) register accessor: dsi bridge dpi v config register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_v_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_v_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_v_cfg1`] module"]
pub type DPI_V_CFG1 = crate::Reg<dpi_v_cfg1::DPI_V_CFG1_SPEC>;
#[doc = "dsi bridge dpi v config register 1"]
pub mod dpi_v_cfg1;
#[doc = "DPI_H_CFG0 (rw) register accessor: dsi bridge dpi h config register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_h_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_h_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_h_cfg0`] module"]
pub type DPI_H_CFG0 = crate::Reg<dpi_h_cfg0::DPI_H_CFG0_SPEC>;
#[doc = "dsi bridge dpi h config register 0"]
pub mod dpi_h_cfg0;
#[doc = "DPI_H_CFG1 (rw) register accessor: dsi bridge dpi h config register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_h_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_h_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_h_cfg1`] module"]
pub type DPI_H_CFG1 = crate::Reg<dpi_h_cfg1::DPI_H_CFG1_SPEC>;
#[doc = "dsi bridge dpi h config register 1"]
pub mod dpi_h_cfg1;
#[doc = "DPI_MISC_CONFIG (rw) register accessor: dsi_bridge dpi misc config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_misc_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_misc_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_misc_config`] module"]
pub type DPI_MISC_CONFIG = crate::Reg<dpi_misc_config::DPI_MISC_CONFIG_SPEC>;
#[doc = "dsi_bridge dpi misc config register"]
pub mod dpi_misc_config;
#[doc = "DPI_CONFIG_UPDATE (w) register accessor: dsi_bridge dpi config update register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_config_update::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi_config_update`] module"]
pub type DPI_CONFIG_UPDATE = crate::Reg<dpi_config_update::DPI_CONFIG_UPDATE_SPEC>;
#[doc = "dsi_bridge dpi config update register"]
pub mod dpi_config_update;
#[doc = "INT_ENA (rw) register accessor: dsi_bridge interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "dsi_bridge interrupt enable register"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: dsi_bridge interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "dsi_bridge interrupt clear register"]
pub mod int_clr;
#[doc = "INT_RAW (rw) register accessor: dsi_bridge raw interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "dsi_bridge raw interrupt register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: dsi_bridge masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "dsi_bridge masked interrupt register"]
pub mod int_st;
#[doc = "HOST_BIST_CTL (rw) register accessor: dsi_bridge host bist control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_bist_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_bist_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_bist_ctl`] module"]
pub type HOST_BIST_CTL = crate::Reg<host_bist_ctl::HOST_BIST_CTL_SPEC>;
#[doc = "dsi_bridge host bist control register"]
pub mod host_bist_ctl;
#[doc = "HOST_TRIGGER_REV (rw) register accessor: dsi_bridge host trigger reverse control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_trigger_rev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_trigger_rev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_trigger_rev`] module"]
pub type HOST_TRIGGER_REV = crate::Reg<host_trigger_rev::HOST_TRIGGER_REV_SPEC>;
#[doc = "dsi_bridge host trigger reverse control register"]
pub mod host_trigger_rev;
#[doc = "BLK_RAW_NUM_CFG (rw) register accessor: dsi_bridge block raw number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk_raw_num_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk_raw_num_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_raw_num_cfg`] module"]
pub type BLK_RAW_NUM_CFG = crate::Reg<blk_raw_num_cfg::BLK_RAW_NUM_CFG_SPEC>;
#[doc = "dsi_bridge block raw number control register"]
pub mod blk_raw_num_cfg;
#[doc = "DMA_FRAME_INTERVAL (rw) register accessor: dsi_bridge dam frame interval control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_frame_interval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_frame_interval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_frame_interval`] module"]
pub type DMA_FRAME_INTERVAL = crate::Reg<dma_frame_interval::DMA_FRAME_INTERVAL_SPEC>;
#[doc = "dsi_bridge dam frame interval control register"]
pub mod dma_frame_interval;
#[doc = "MEM_AUX_CTRL (rw) register accessor: dsi_bridge mem aux control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_aux_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_aux_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_aux_ctrl`] module"]
pub type MEM_AUX_CTRL = crate::Reg<mem_aux_ctrl::MEM_AUX_CTRL_SPEC>;
#[doc = "dsi_bridge mem aux control register"]
pub mod mem_aux_ctrl;
#[doc = "RDN_ECO_CS (rw) register accessor: dsi_bridge rdn eco cs register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_cs`] module"]
pub type RDN_ECO_CS = crate::Reg<rdn_eco_cs::RDN_ECO_CS_SPEC>;
#[doc = "dsi_bridge rdn eco cs register"]
pub mod rdn_eco_cs;
#[doc = "RDN_ECO_LOW (rw) register accessor: dsi_bridge rdn eco all low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_low`] module"]
pub type RDN_ECO_LOW = crate::Reg<rdn_eco_low::RDN_ECO_LOW_SPEC>;
#[doc = "dsi_bridge rdn eco all low register"]
pub mod rdn_eco_low;
#[doc = "RDN_ECO_HIGH (rw) register accessor: dsi_bridge rdn eco all high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_high`] module"]
pub type RDN_ECO_HIGH = crate::Reg<rdn_eco_high::RDN_ECO_HIGH_SPEC>;
#[doc = "dsi_bridge rdn eco all high register"]
pub mod rdn_eco_high;
#[doc = "HOST_CTRL (rw) register accessor: dsi_bridge host control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctrl`] module"]
pub type HOST_CTRL = crate::Reg<host_ctrl::HOST_CTRL_SPEC>;
#[doc = "dsi_bridge host control register"]
pub mod host_ctrl;
#[doc = "MEM_CLK_CTRL (rw) register accessor: dsi_bridge mem force on control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_clk_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_clk_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_clk_ctrl`] module"]
pub type MEM_CLK_CTRL = crate::Reg<mem_clk_ctrl::MEM_CLK_CTRL_SPEC>;
#[doc = "dsi_bridge mem force on control register"]
pub mod mem_clk_ctrl;
#[doc = "DMA_FLOW_CTRL (rw) register accessor: dsi_bridge dma flow controller register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_flow_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_flow_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_flow_ctrl`] module"]
pub type DMA_FLOW_CTRL = crate::Reg<dma_flow_ctrl::DMA_FLOW_CTRL_SPEC>;
#[doc = "dsi_bridge dma flow controller register"]
pub mod dma_flow_ctrl;
#[doc = "RAW_BUF_ALMOST_EMPTY_THRD (rw) register accessor: dsi_bridge buffer empty threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_buf_almost_empty_thrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw_buf_almost_empty_thrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_buf_almost_empty_thrd`] module"]
pub type RAW_BUF_ALMOST_EMPTY_THRD =
    crate::Reg<raw_buf_almost_empty_thrd::RAW_BUF_ALMOST_EMPTY_THRD_SPEC>;
#[doc = "dsi_bridge buffer empty threshold register"]
pub mod raw_buf_almost_empty_thrd;
#[doc = "YUV_CFG (rw) register accessor: dsi_bridge yuv format config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`yuv_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`yuv_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@yuv_cfg`] module"]
pub type YUV_CFG = crate::Reg<yuv_cfg::YUV_CFG_SPEC>;
#[doc = "dsi_bridge yuv format config register"]
pub mod yuv_cfg;
#[doc = "PHY_LP_LOOPBACK_CTRL (rw) register accessor: dsi phy lp_loopback test ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_lp_loopback_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_lp_loopback_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_lp_loopback_ctrl`] module"]
pub type PHY_LP_LOOPBACK_CTRL = crate::Reg<phy_lp_loopback_ctrl::PHY_LP_LOOPBACK_CTRL_SPEC>;
#[doc = "dsi phy lp_loopback test ctrl"]
pub mod phy_lp_loopback_ctrl;
#[doc = "PHY_HS_LOOPBACK_CTRL (rw) register accessor: dsi phy hp_loopback test ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_hs_loopback_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_hs_loopback_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_hs_loopback_ctrl`] module"]
pub type PHY_HS_LOOPBACK_CTRL = crate::Reg<phy_hs_loopback_ctrl::PHY_HS_LOOPBACK_CTRL_SPEC>;
#[doc = "dsi phy hp_loopback test ctrl"]
pub mod phy_hs_loopback_ctrl;
#[doc = "PHY_LOOPBACK_CNT (rw) register accessor: loopback test cnt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_loopback_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_loopback_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_loopback_cnt`] module"]
pub type PHY_LOOPBACK_CNT = crate::Reg<phy_loopback_cnt::PHY_LOOPBACK_CNT_SPEC>;
#[doc = "loopback test cnt"]
pub mod phy_loopback_cnt;
