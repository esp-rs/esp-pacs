#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    blend0_clut_data: BLEND0_CLUT_DATA,
    blend1_clut_data: BLEND1_CLUT_DATA,
    _reserved2: [u8; 0x04],
    clut_conf: CLUT_CONF,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    sr_color_mode: SR_COLOR_MODE,
    blend_color_mode: BLEND_COLOR_MODE,
    sr_byte_order: SR_BYTE_ORDER,
    blend_byte_order: BLEND_BYTE_ORDER,
    _reserved11: [u8; 0x04],
    blend_trans_mode: BLEND_TRANS_MODE,
    sr_fix_alpha: SR_FIX_ALPHA,
    blend_tx_size: BLEND_TX_SIZE,
    blend_fix_alpha: BLEND_FIX_ALPHA,
    _reserved15: [u8; 0x04],
    blend_rgb: BLEND_RGB,
    blend_fix_pixel: BLEND_FIX_PIXEL,
    ck_fg_low: CK_FG_LOW,
    ck_fg_high: CK_FG_HIGH,
    ck_bg_low: CK_BG_LOW,
    ck_bg_high: CK_BG_HIGH,
    ck_default: CK_DEFAULT,
    sr_scal_rotate: SR_SCAL_ROTATE,
    sr_mem_pd: SR_MEM_PD,
    reg_conf: REG_CONF,
    clut_cnt: CLUT_CNT,
    blend_st: BLEND_ST,
    sr_param_err_st: SR_PARAM_ERR_ST,
    sr_status: SR_STATUS,
    eco_low: ECO_LOW,
    eco_high: ECO_HIGH,
    eco_cell_ctrl: ECO_CELL_CTRL,
    sram_ctrl: SRAM_CTRL,
    _reserved33: [u8; 0x70],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - CLUT sram data read/write register in background plane of blender
    #[inline(always)]
    pub const fn blend0_clut_data(&self) -> &BLEND0_CLUT_DATA {
        &self.blend0_clut_data
    }
    ///0x04 - CLUT sram data read/write register in foreground plane of blender
    #[inline(always)]
    pub const fn blend1_clut_data(&self) -> &BLEND1_CLUT_DATA {
        &self.blend1_clut_data
    }
    ///0x0c - CLUT configure register
    #[inline(always)]
    pub const fn clut_conf(&self) -> &CLUT_CONF {
        &self.clut_conf
    }
    ///0x10 - Raw status interrupt
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x14 - Masked interrupt
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x18 - Interrupt enable bits
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x1c - Interrupt clear bits
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x20 - Scaling and rotating engine color mode register
    #[inline(always)]
    pub const fn sr_color_mode(&self) -> &SR_COLOR_MODE {
        &self.sr_color_mode
    }
    ///0x24 - blending engine color mode register
    #[inline(always)]
    pub const fn blend_color_mode(&self) -> &BLEND_COLOR_MODE {
        &self.blend_color_mode
    }
    ///0x28 - Scaling and rotating engine byte order register
    #[inline(always)]
    pub const fn sr_byte_order(&self) -> &SR_BYTE_ORDER {
        &self.sr_byte_order
    }
    ///0x2c - Blending engine byte order register
    #[inline(always)]
    pub const fn blend_byte_order(&self) -> &BLEND_BYTE_ORDER {
        &self.blend_byte_order
    }
    ///0x34 - Blending engine mode configure register
    #[inline(always)]
    pub const fn blend_trans_mode(&self) -> &BLEND_TRANS_MODE {
        &self.blend_trans_mode
    }
    ///0x38 - Scaling and rotating engine alpha override register
    #[inline(always)]
    pub const fn sr_fix_alpha(&self) -> &SR_FIX_ALPHA {
        &self.sr_fix_alpha
    }
    ///0x3c - Fix pixel filling mode image size register
    #[inline(always)]
    pub const fn blend_tx_size(&self) -> &BLEND_TX_SIZE {
        &self.blend_tx_size
    }
    ///0x40 - Blending engine alpha override register
    #[inline(always)]
    pub const fn blend_fix_alpha(&self) -> &BLEND_FIX_ALPHA {
        &self.blend_fix_alpha
    }
    ///0x48 - RGB color register
    #[inline(always)]
    pub const fn blend_rgb(&self) -> &BLEND_RGB {
        &self.blend_rgb
    }
    ///0x4c - Blending engine fix pixel register
    #[inline(always)]
    pub const fn blend_fix_pixel(&self) -> &BLEND_FIX_PIXEL {
        &self.blend_fix_pixel
    }
    ///0x50 - foreground color key lower threshold
    #[inline(always)]
    pub const fn ck_fg_low(&self) -> &CK_FG_LOW {
        &self.ck_fg_low
    }
    ///0x54 - foreground color key higher threshold
    #[inline(always)]
    pub const fn ck_fg_high(&self) -> &CK_FG_HIGH {
        &self.ck_fg_high
    }
    ///0x58 - background color key lower threshold
    #[inline(always)]
    pub const fn ck_bg_low(&self) -> &CK_BG_LOW {
        &self.ck_bg_low
    }
    ///0x5c - background color key higher threshold
    #[inline(always)]
    pub const fn ck_bg_high(&self) -> &CK_BG_HIGH {
        &self.ck_bg_high
    }
    ///0x60 - default value when foreground and background both in color key range
    #[inline(always)]
    pub const fn ck_default(&self) -> &CK_DEFAULT {
        &self.ck_default
    }
    ///0x64 - Scaling and rotating coefficient register
    #[inline(always)]
    pub const fn sr_scal_rotate(&self) -> &SR_SCAL_ROTATE {
        &self.sr_scal_rotate
    }
    ///0x68 - SR memory power done register
    #[inline(always)]
    pub const fn sr_mem_pd(&self) -> &SR_MEM_PD {
        &self.sr_mem_pd
    }
    ///0x6c - Register clock enable register
    #[inline(always)]
    pub const fn reg_conf(&self) -> &REG_CONF {
        &self.reg_conf
    }
    ///0x70 - BLEND CLUT write counter register
    #[inline(always)]
    pub const fn clut_cnt(&self) -> &CLUT_CNT {
        &self.clut_cnt
    }
    ///0x74 - Blending engine status register
    #[inline(always)]
    pub const fn blend_st(&self) -> &BLEND_ST {
        &self.blend_st
    }
    ///0x78 - Scaling and rotating coefficient error register
    #[inline(always)]
    pub const fn sr_param_err_st(&self) -> &SR_PARAM_ERR_ST {
        &self.sr_param_err_st
    }
    ///0x7c - SR FSM register
    #[inline(always)]
    pub const fn sr_status(&self) -> &SR_STATUS {
        &self.sr_status
    }
    ///0x80 - Reserved.
    #[inline(always)]
    pub const fn eco_low(&self) -> &ECO_LOW {
        &self.eco_low
    }
    ///0x84 - Reserved.
    #[inline(always)]
    pub const fn eco_high(&self) -> &ECO_HIGH {
        &self.eco_high
    }
    ///0x88 - Reserved.
    #[inline(always)]
    pub const fn eco_cell_ctrl(&self) -> &ECO_CELL_CTRL {
        &self.eco_cell_ctrl
    }
    ///0x8c - PPA SRAM Control Register
    #[inline(always)]
    pub const fn sram_ctrl(&self) -> &SRAM_CTRL {
        &self.sram_ctrl
    }
    ///0x100 - PPA Version register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**BLEND0_CLUT_DATA (rw) register accessor: CLUT sram data read/write register in background plane of blender

You can [`read`](crate::generic::Reg::read) this register and get [`blend0_clut_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend0_clut_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blend0_clut_data`] module*/
pub type BLEND0_CLUT_DATA = crate::Reg<blend0_clut_data::BLEND0_CLUT_DATA_SPEC>;
///CLUT sram data read/write register in background plane of blender
pub mod blend0_clut_data;
/**BLEND1_CLUT_DATA (rw) register accessor: CLUT sram data read/write register in foreground plane of blender

You can [`read`](crate::generic::Reg::read) this register and get [`blend1_clut_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend1_clut_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blend1_clut_data`] module*/
pub type BLEND1_CLUT_DATA = crate::Reg<blend1_clut_data::BLEND1_CLUT_DATA_SPEC>;
///CLUT sram data read/write register in foreground plane of blender
pub mod blend1_clut_data;
/**CLUT_CONF (rw) register accessor: CLUT configure register

You can [`read`](crate::generic::Reg::read) this register and get [`clut_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clut_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clut_conf`] module*/
pub type CLUT_CONF = crate::Reg<clut_conf::CLUT_CONF_SPEC>;
///CLUT configure register
pub mod clut_conf;
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
/**SR_COLOR_MODE (rw) register accessor: Scaling and rotating engine color mode register

You can [`read`](crate::generic::Reg::read) this register and get [`sr_color_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_color_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr_color_mode`] module*/
pub type SR_COLOR_MODE = crate::Reg<sr_color_mode::SR_COLOR_MODE_SPEC>;
///Scaling and rotating engine color mode register
pub mod sr_color_mode;
/**BLEND_COLOR_MODE (rw) register accessor: blending engine color mode register

You can [`read`](crate::generic::Reg::read) this register and get [`blend_color_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend_color_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blend_color_mode`] module*/
pub type BLEND_COLOR_MODE = crate::Reg<blend_color_mode::BLEND_COLOR_MODE_SPEC>;
///blending engine color mode register
pub mod blend_color_mode;
/**SR_BYTE_ORDER (rw) register accessor: Scaling and rotating engine byte order register

You can [`read`](crate::generic::Reg::read) this register and get [`sr_byte_order::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_byte_order::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr_byte_order`] module*/
pub type SR_BYTE_ORDER = crate::Reg<sr_byte_order::SR_BYTE_ORDER_SPEC>;
///Scaling and rotating engine byte order register
pub mod sr_byte_order;
/**BLEND_BYTE_ORDER (rw) register accessor: Blending engine byte order register

You can [`read`](crate::generic::Reg::read) this register and get [`blend_byte_order::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend_byte_order::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blend_byte_order`] module*/
pub type BLEND_BYTE_ORDER = crate::Reg<blend_byte_order::BLEND_BYTE_ORDER_SPEC>;
///Blending engine byte order register
pub mod blend_byte_order;
/**BLEND_TRANS_MODE (rw) register accessor: Blending engine mode configure register

You can [`read`](crate::generic::Reg::read) this register and get [`blend_trans_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend_trans_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blend_trans_mode`] module*/
pub type BLEND_TRANS_MODE = crate::Reg<blend_trans_mode::BLEND_TRANS_MODE_SPEC>;
///Blending engine mode configure register
pub mod blend_trans_mode;
/**SR_FIX_ALPHA (rw) register accessor: Scaling and rotating engine alpha override register

You can [`read`](crate::generic::Reg::read) this register and get [`sr_fix_alpha::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_fix_alpha::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr_fix_alpha`] module*/
pub type SR_FIX_ALPHA = crate::Reg<sr_fix_alpha::SR_FIX_ALPHA_SPEC>;
///Scaling and rotating engine alpha override register
pub mod sr_fix_alpha;
/**BLEND_TX_SIZE (rw) register accessor: Fix pixel filling mode image size register

You can [`read`](crate::generic::Reg::read) this register and get [`blend_tx_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend_tx_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blend_tx_size`] module*/
pub type BLEND_TX_SIZE = crate::Reg<blend_tx_size::BLEND_TX_SIZE_SPEC>;
///Fix pixel filling mode image size register
pub mod blend_tx_size;
/**BLEND_FIX_ALPHA (rw) register accessor: Blending engine alpha override register

You can [`read`](crate::generic::Reg::read) this register and get [`blend_fix_alpha::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend_fix_alpha::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blend_fix_alpha`] module*/
pub type BLEND_FIX_ALPHA = crate::Reg<blend_fix_alpha::BLEND_FIX_ALPHA_SPEC>;
///Blending engine alpha override register
pub mod blend_fix_alpha;
/**BLEND_RGB (rw) register accessor: RGB color register

You can [`read`](crate::generic::Reg::read) this register and get [`blend_rgb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend_rgb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blend_rgb`] module*/
pub type BLEND_RGB = crate::Reg<blend_rgb::BLEND_RGB_SPEC>;
///RGB color register
pub mod blend_rgb;
/**BLEND_FIX_PIXEL (rw) register accessor: Blending engine fix pixel register

You can [`read`](crate::generic::Reg::read) this register and get [`blend_fix_pixel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend_fix_pixel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blend_fix_pixel`] module*/
pub type BLEND_FIX_PIXEL = crate::Reg<blend_fix_pixel::BLEND_FIX_PIXEL_SPEC>;
///Blending engine fix pixel register
pub mod blend_fix_pixel;
/**CK_FG_LOW (rw) register accessor: foreground color key lower threshold

You can [`read`](crate::generic::Reg::read) this register and get [`ck_fg_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ck_fg_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ck_fg_low`] module*/
pub type CK_FG_LOW = crate::Reg<ck_fg_low::CK_FG_LOW_SPEC>;
///foreground color key lower threshold
pub mod ck_fg_low;
/**CK_FG_HIGH (rw) register accessor: foreground color key higher threshold

You can [`read`](crate::generic::Reg::read) this register and get [`ck_fg_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ck_fg_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ck_fg_high`] module*/
pub type CK_FG_HIGH = crate::Reg<ck_fg_high::CK_FG_HIGH_SPEC>;
///foreground color key higher threshold
pub mod ck_fg_high;
/**CK_BG_LOW (rw) register accessor: background color key lower threshold

You can [`read`](crate::generic::Reg::read) this register and get [`ck_bg_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ck_bg_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ck_bg_low`] module*/
pub type CK_BG_LOW = crate::Reg<ck_bg_low::CK_BG_LOW_SPEC>;
///background color key lower threshold
pub mod ck_bg_low;
/**CK_BG_HIGH (rw) register accessor: background color key higher threshold

You can [`read`](crate::generic::Reg::read) this register and get [`ck_bg_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ck_bg_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ck_bg_high`] module*/
pub type CK_BG_HIGH = crate::Reg<ck_bg_high::CK_BG_HIGH_SPEC>;
///background color key higher threshold
pub mod ck_bg_high;
/**CK_DEFAULT (rw) register accessor: default value when foreground and background both in color key range

You can [`read`](crate::generic::Reg::read) this register and get [`ck_default::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ck_default::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ck_default`] module*/
pub type CK_DEFAULT = crate::Reg<ck_default::CK_DEFAULT_SPEC>;
///default value when foreground and background both in color key range
pub mod ck_default;
/**SR_SCAL_ROTATE (rw) register accessor: Scaling and rotating coefficient register

You can [`read`](crate::generic::Reg::read) this register and get [`sr_scal_rotate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_scal_rotate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr_scal_rotate`] module*/
pub type SR_SCAL_ROTATE = crate::Reg<sr_scal_rotate::SR_SCAL_ROTATE_SPEC>;
///Scaling and rotating coefficient register
pub mod sr_scal_rotate;
/**SR_MEM_PD (rw) register accessor: SR memory power done register

You can [`read`](crate::generic::Reg::read) this register and get [`sr_mem_pd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_mem_pd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr_mem_pd`] module*/
pub type SR_MEM_PD = crate::Reg<sr_mem_pd::SR_MEM_PD_SPEC>;
///SR memory power done register
pub mod sr_mem_pd;
/**REG_CONF (rw) register accessor: Register clock enable register

You can [`read`](crate::generic::Reg::read) this register and get [`reg_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reg_conf`] module*/
pub type REG_CONF = crate::Reg<reg_conf::REG_CONF_SPEC>;
///Register clock enable register
pub mod reg_conf;
/**CLUT_CNT (r) register accessor: BLEND CLUT write counter register

You can [`read`](crate::generic::Reg::read) this register and get [`clut_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clut_cnt`] module*/
pub type CLUT_CNT = crate::Reg<clut_cnt::CLUT_CNT_SPEC>;
///BLEND CLUT write counter register
pub mod clut_cnt;
/**BLEND_ST (r) register accessor: Blending engine status register

You can [`read`](crate::generic::Reg::read) this register and get [`blend_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@blend_st`] module*/
pub type BLEND_ST = crate::Reg<blend_st::BLEND_ST_SPEC>;
///Blending engine status register
pub mod blend_st;
/**SR_PARAM_ERR_ST (r) register accessor: Scaling and rotating coefficient error register

You can [`read`](crate::generic::Reg::read) this register and get [`sr_param_err_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr_param_err_st`] module*/
pub type SR_PARAM_ERR_ST = crate::Reg<sr_param_err_st::SR_PARAM_ERR_ST_SPEC>;
///Scaling and rotating coefficient error register
pub mod sr_param_err_st;
/**SR_STATUS (r) register accessor: SR FSM register

You can [`read`](crate::generic::Reg::read) this register and get [`sr_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sr_status`] module*/
pub type SR_STATUS = crate::Reg<sr_status::SR_STATUS_SPEC>;
///SR FSM register
pub mod sr_status;
/**ECO_LOW (rw) register accessor: Reserved.

You can [`read`](crate::generic::Reg::read) this register and get [`eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eco_low`] module*/
pub type ECO_LOW = crate::Reg<eco_low::ECO_LOW_SPEC>;
///Reserved.
pub mod eco_low;
/**ECO_HIGH (rw) register accessor: Reserved.

You can [`read`](crate::generic::Reg::read) this register and get [`eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eco_high`] module*/
pub type ECO_HIGH = crate::Reg<eco_high::ECO_HIGH_SPEC>;
///Reserved.
pub mod eco_high;
/**ECO_CELL_CTRL (rw) register accessor: Reserved.

You can [`read`](crate::generic::Reg::read) this register and get [`eco_cell_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_cell_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eco_cell_ctrl`] module*/
pub type ECO_CELL_CTRL = crate::Reg<eco_cell_ctrl::ECO_CELL_CTRL_SPEC>;
///Reserved.
pub mod eco_cell_ctrl;
/**SRAM_CTRL (rw) register accessor: PPA SRAM Control Register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_ctrl`] module*/
pub type SRAM_CTRL = crate::Reg<sram_ctrl::SRAM_CTRL_SPEC>;
///PPA SRAM Control Register
pub mod sram_ctrl;
/**DATE (rw) register accessor: PPA Version register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///PPA Version register
pub mod date;
