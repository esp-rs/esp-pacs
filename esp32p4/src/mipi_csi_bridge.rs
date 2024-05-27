#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    clk_en: CLK_EN,
    csi_en: CSI_EN,
    dma_req_cfg: DMA_REQ_CFG,
    buf_flow_ctl: BUF_FLOW_CTL,
    data_type_cfg: DATA_TYPE_CFG,
    frame_cfg: FRAME_CFG,
    endian_mode: ENDIAN_MODE,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_st: INT_ST,
    int_ena: INT_ENA,
    dma_req_interval: DMA_REQ_INTERVAL,
    dmablk_size: DMABLK_SIZE,
    rdn_eco_cs: RDN_ECO_CS,
    rdn_eco_low: RDN_ECO_LOW,
    rdn_eco_high: RDN_ECO_HIGH,
    host_ctrl: HOST_CTRL,
    mem_ctrl: MEM_CTRL,
}
impl RegisterBlock {
    ///0x00 - csi bridge register mapping unit clock gating.
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    ///0x04 - csi bridge enable.
    #[inline(always)]
    pub const fn csi_en(&self) -> &CSI_EN {
        &self.csi_en
    }
    ///0x08 - dma request configuration.
    #[inline(always)]
    pub const fn dma_req_cfg(&self) -> &DMA_REQ_CFG {
        &self.dma_req_cfg
    }
    ///0x0c - csi bridge buffer control.
    #[inline(always)]
    pub const fn buf_flow_ctl(&self) -> &BUF_FLOW_CTL {
        &self.buf_flow_ctl
    }
    ///0x10 - pixel data type configuration.
    #[inline(always)]
    pub const fn data_type_cfg(&self) -> &DATA_TYPE_CFG {
        &self.data_type_cfg
    }
    ///0x14 - frame configuration.
    #[inline(always)]
    pub const fn frame_cfg(&self) -> &FRAME_CFG {
        &self.frame_cfg
    }
    ///0x18 - data endianness order configuration.
    #[inline(always)]
    pub const fn endian_mode(&self) -> &ENDIAN_MODE {
        &self.endian_mode
    }
    ///0x1c - csi bridge interrupt raw.
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x20 - csi bridge interrupt clr.
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x24 - csi bridge interrupt st.
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x28 - csi bridge interrupt enable.
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x2c - DMA interval configuration.
    #[inline(always)]
    pub const fn dma_req_interval(&self) -> &DMA_REQ_INTERVAL {
        &self.dma_req_interval
    }
    ///0x30 - DMA block size configuration.
    #[inline(always)]
    pub const fn dmablk_size(&self) -> &DMABLK_SIZE {
        &self.dmablk_size
    }
    ///0x34 - N/A
    #[inline(always)]
    pub const fn rdn_eco_cs(&self) -> &RDN_ECO_CS {
        &self.rdn_eco_cs
    }
    ///0x38 - N/A
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> &RDN_ECO_LOW {
        &self.rdn_eco_low
    }
    ///0x3c - N/A
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> &RDN_ECO_HIGH {
        &self.rdn_eco_high
    }
    ///0x40 - csi host control by csi bridge.
    #[inline(always)]
    pub const fn host_ctrl(&self) -> &HOST_CTRL {
        &self.host_ctrl
    }
    ///0x44 - csi bridge buffer control.
    #[inline(always)]
    pub const fn mem_ctrl(&self) -> &MEM_CTRL {
        &self.mem_ctrl
    }
}
/**CLK_EN (rw) register accessor: csi bridge register mapping unit clock gating.

You can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_en`] module*/
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
///csi bridge register mapping unit clock gating.
pub mod clk_en;
/**CSI_EN (rw) register accessor: csi bridge enable.

You can [`read`](crate::generic::Reg::read) this register and get [`csi_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csi_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@csi_en`] module*/
pub type CSI_EN = crate::Reg<csi_en::CSI_EN_SPEC>;
///csi bridge enable.
pub mod csi_en;
/**DMA_REQ_CFG (rw) register accessor: dma request configuration.

You can [`read`](crate::generic::Reg::read) this register and get [`dma_req_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_req_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_req_cfg`] module*/
pub type DMA_REQ_CFG = crate::Reg<dma_req_cfg::DMA_REQ_CFG_SPEC>;
///dma request configuration.
pub mod dma_req_cfg;
/**BUF_FLOW_CTL (rw) register accessor: csi bridge buffer control.

You can [`read`](crate::generic::Reg::read) this register and get [`buf_flow_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_flow_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@buf_flow_ctl`] module*/
pub type BUF_FLOW_CTL = crate::Reg<buf_flow_ctl::BUF_FLOW_CTL_SPEC>;
///csi bridge buffer control.
pub mod buf_flow_ctl;
/**DATA_TYPE_CFG (rw) register accessor: pixel data type configuration.

You can [`read`](crate::generic::Reg::read) this register and get [`data_type_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_type_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@data_type_cfg`] module*/
pub type DATA_TYPE_CFG = crate::Reg<data_type_cfg::DATA_TYPE_CFG_SPEC>;
///pixel data type configuration.
pub mod data_type_cfg;
/**FRAME_CFG (rw) register accessor: frame configuration.

You can [`read`](crate::generic::Reg::read) this register and get [`frame_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frame_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frame_cfg`] module*/
pub type FRAME_CFG = crate::Reg<frame_cfg::FRAME_CFG_SPEC>;
///frame configuration.
pub mod frame_cfg;
/**ENDIAN_MODE (rw) register accessor: data endianness order configuration.

You can [`read`](crate::generic::Reg::read) this register and get [`endian_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endian_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@endian_mode`] module*/
pub type ENDIAN_MODE = crate::Reg<endian_mode::ENDIAN_MODE_SPEC>;
///data endianness order configuration.
pub mod endian_mode;
/**INT_RAW (rw) register accessor: csi bridge interrupt raw.

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///csi bridge interrupt raw.
pub mod int_raw;
/**INT_CLR (w) register accessor: csi bridge interrupt clr.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///csi bridge interrupt clr.
pub mod int_clr;
/**INT_ST (r) register accessor: csi bridge interrupt st.

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///csi bridge interrupt st.
pub mod int_st;
/**INT_ENA (rw) register accessor: csi bridge interrupt enable.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///csi bridge interrupt enable.
pub mod int_ena;
/**DMA_REQ_INTERVAL (rw) register accessor: DMA interval configuration.

You can [`read`](crate::generic::Reg::read) this register and get [`dma_req_interval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_req_interval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_req_interval`] module*/
pub type DMA_REQ_INTERVAL = crate::Reg<dma_req_interval::DMA_REQ_INTERVAL_SPEC>;
///DMA interval configuration.
pub mod dma_req_interval;
/**DMABLK_SIZE (rw) register accessor: DMA block size configuration.

You can [`read`](crate::generic::Reg::read) this register and get [`dmablk_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmablk_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmablk_size`] module*/
pub type DMABLK_SIZE = crate::Reg<dmablk_size::DMABLK_SIZE_SPEC>;
///DMA block size configuration.
pub mod dmablk_size;
/**RDN_ECO_CS (rw) register accessor: N/A

You can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_cs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_cs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdn_eco_cs`] module*/
pub type RDN_ECO_CS = crate::Reg<rdn_eco_cs::RDN_ECO_CS_SPEC>;
///N/A
pub mod rdn_eco_cs;
/**RDN_ECO_LOW (rw) register accessor: N/A

You can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdn_eco_low`] module*/
pub type RDN_ECO_LOW = crate::Reg<rdn_eco_low::RDN_ECO_LOW_SPEC>;
///N/A
pub mod rdn_eco_low;
/**RDN_ECO_HIGH (rw) register accessor: N/A

You can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdn_eco_high`] module*/
pub type RDN_ECO_HIGH = crate::Reg<rdn_eco_high::RDN_ECO_HIGH_SPEC>;
///N/A
pub mod rdn_eco_high;
/**HOST_CTRL (rw) register accessor: csi host control by csi bridge.

You can [`read`](crate::generic::Reg::read) this register and get [`host_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_ctrl`] module*/
pub type HOST_CTRL = crate::Reg<host_ctrl::HOST_CTRL_SPEC>;
///csi host control by csi bridge.
pub mod host_ctrl;
/**MEM_CTRL (rw) register accessor: csi bridge buffer control.

You can [`read`](crate::generic::Reg::read) this register and get [`mem_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_ctrl`] module*/
pub type MEM_CTRL = crate::Reg<mem_ctrl::MEM_CTRL_SPEC>;
///csi bridge buffer control.
pub mod mem_ctrl;
