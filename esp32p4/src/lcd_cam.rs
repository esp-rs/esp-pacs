#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    lcd_clock: LCD_CLOCK,
    cam_ctrl: CAM_CTRL,
    cam_ctrl1: CAM_CTRL1,
    cam_rgb_yuv: CAM_RGB_YUV,
    lcd_rgb_yuv: LCD_RGB_YUV,
    lcd_user: LCD_USER,
    lcd_misc: LCD_MISC,
    lcd_ctrl: LCD_CTRL,
    lcd_ctrl1: LCD_CTRL1,
    lcd_ctrl2: LCD_CTRL2,
    lcd_first_cmd_val: LCD_FIRST_CMD_VAL,
    lcd_latter_cmd_val: LCD_LATTER_CMD_VAL,
    lcd_dly_mode_cfg1: LCD_DLY_MODE_CFG1,
    _reserved13: [u8; 0x04],
    lcd_dly_mode_cfg2: LCD_DLY_MODE_CFG2,
    _reserved14: [u8; 0x28],
    lc_dma_int_ena: LC_DMA_INT_ENA,
    lc_dma_int_raw: LC_DMA_INT_RAW,
    lc_dma_int_st: LC_DMA_INT_ST,
    lc_dma_int_clr: LC_DMA_INT_CLR,
    _reserved18: [u8; 0x88],
    lc_reg_date: LC_REG_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - LCD clock config register."]
    #[inline(always)]
    pub const fn lcd_clock(&self) -> &LCD_CLOCK {
        &self.lcd_clock
    }
    #[doc = "0x04 - CAM config register."]
    #[inline(always)]
    pub const fn cam_ctrl(&self) -> &CAM_CTRL {
        &self.cam_ctrl
    }
    #[doc = "0x08 - CAM config register."]
    #[inline(always)]
    pub const fn cam_ctrl1(&self) -> &CAM_CTRL1 {
        &self.cam_ctrl1
    }
    #[doc = "0x0c - CAM YUV/RGB converter configuration register."]
    #[inline(always)]
    pub const fn cam_rgb_yuv(&self) -> &CAM_RGB_YUV {
        &self.cam_rgb_yuv
    }
    #[doc = "0x10 - LCD YUV/RGB converter configuration register."]
    #[inline(always)]
    pub const fn lcd_rgb_yuv(&self) -> &LCD_RGB_YUV {
        &self.lcd_rgb_yuv
    }
    #[doc = "0x14 - LCD config register."]
    #[inline(always)]
    pub const fn lcd_user(&self) -> &LCD_USER {
        &self.lcd_user
    }
    #[doc = "0x18 - LCD config register."]
    #[inline(always)]
    pub const fn lcd_misc(&self) -> &LCD_MISC {
        &self.lcd_misc
    }
    #[doc = "0x1c - LCD config register."]
    #[inline(always)]
    pub const fn lcd_ctrl(&self) -> &LCD_CTRL {
        &self.lcd_ctrl
    }
    #[doc = "0x20 - LCD config register."]
    #[inline(always)]
    pub const fn lcd_ctrl1(&self) -> &LCD_CTRL1 {
        &self.lcd_ctrl1
    }
    #[doc = "0x24 - LCD config register."]
    #[inline(always)]
    pub const fn lcd_ctrl2(&self) -> &LCD_CTRL2 {
        &self.lcd_ctrl2
    }
    #[doc = "0x28 - LCD config register."]
    #[inline(always)]
    pub const fn lcd_first_cmd_val(&self) -> &LCD_FIRST_CMD_VAL {
        &self.lcd_first_cmd_val
    }
    #[doc = "0x2c - LCD config register."]
    #[inline(always)]
    pub const fn lcd_latter_cmd_val(&self) -> &LCD_LATTER_CMD_VAL {
        &self.lcd_latter_cmd_val
    }
    #[doc = "0x30 - LCD config register."]
    #[inline(always)]
    pub const fn lcd_dly_mode_cfg1(&self) -> &LCD_DLY_MODE_CFG1 {
        &self.lcd_dly_mode_cfg1
    }
    #[doc = "0x38 - LCD config register."]
    #[inline(always)]
    pub const fn lcd_dly_mode_cfg2(&self) -> &LCD_DLY_MODE_CFG2 {
        &self.lcd_dly_mode_cfg2
    }
    #[doc = "0x64 - LCDCAM interrupt enable register."]
    #[inline(always)]
    pub const fn lc_dma_int_ena(&self) -> &LC_DMA_INT_ENA {
        &self.lc_dma_int_ena
    }
    #[doc = "0x68 - LCDCAM interrupt raw register, valid in level."]
    #[inline(always)]
    pub const fn lc_dma_int_raw(&self) -> &LC_DMA_INT_RAW {
        &self.lc_dma_int_raw
    }
    #[doc = "0x6c - LCDCAM interrupt status register."]
    #[inline(always)]
    pub const fn lc_dma_int_st(&self) -> &LC_DMA_INT_ST {
        &self.lc_dma_int_st
    }
    #[doc = "0x70 - LCDCAM interrupt clear register."]
    #[inline(always)]
    pub const fn lc_dma_int_clr(&self) -> &LC_DMA_INT_CLR {
        &self.lc_dma_int_clr
    }
    #[doc = "0xfc - Version register"]
    #[inline(always)]
    pub const fn lc_reg_date(&self) -> &LC_REG_DATE {
        &self.lc_reg_date
    }
}
#[doc = "LCD_CLOCK (rw) register accessor: LCD clock config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_clock`] module"]
pub type LCD_CLOCK = crate::Reg<lcd_clock::LCD_CLOCK_SPEC>;
#[doc = "LCD clock config register."]
pub mod lcd_clock;
#[doc = "CAM_CTRL (rw) register accessor: CAM config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cam_ctrl`] module"]
pub type CAM_CTRL = crate::Reg<cam_ctrl::CAM_CTRL_SPEC>;
#[doc = "CAM config register."]
pub mod cam_ctrl;
#[doc = "CAM_CTRL1 (rw) register accessor: CAM config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cam_ctrl1`] module"]
pub type CAM_CTRL1 = crate::Reg<cam_ctrl1::CAM_CTRL1_SPEC>;
#[doc = "CAM config register."]
pub mod cam_ctrl1;
#[doc = "CAM_RGB_YUV (rw) register accessor: CAM YUV/RGB converter configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_rgb_yuv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_rgb_yuv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cam_rgb_yuv`] module"]
pub type CAM_RGB_YUV = crate::Reg<cam_rgb_yuv::CAM_RGB_YUV_SPEC>;
#[doc = "CAM YUV/RGB converter configuration register."]
pub mod cam_rgb_yuv;
#[doc = "LCD_RGB_YUV (rw) register accessor: LCD YUV/RGB converter configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_rgb_yuv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_rgb_yuv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_rgb_yuv`] module"]
pub type LCD_RGB_YUV = crate::Reg<lcd_rgb_yuv::LCD_RGB_YUV_SPEC>;
#[doc = "LCD YUV/RGB converter configuration register."]
pub mod lcd_rgb_yuv;
#[doc = "LCD_USER (rw) register accessor: LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_user`] module"]
pub type LCD_USER = crate::Reg<lcd_user::LCD_USER_SPEC>;
#[doc = "LCD config register."]
pub mod lcd_user;
#[doc = "LCD_MISC (rw) register accessor: LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_misc`] module"]
pub type LCD_MISC = crate::Reg<lcd_misc::LCD_MISC_SPEC>;
#[doc = "LCD config register."]
pub mod lcd_misc;
#[doc = "LCD_CTRL (rw) register accessor: LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ctrl`] module"]
pub type LCD_CTRL = crate::Reg<lcd_ctrl::LCD_CTRL_SPEC>;
#[doc = "LCD config register."]
pub mod lcd_ctrl;
#[doc = "LCD_CTRL1 (rw) register accessor: LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ctrl1`] module"]
pub type LCD_CTRL1 = crate::Reg<lcd_ctrl1::LCD_CTRL1_SPEC>;
#[doc = "LCD config register."]
pub mod lcd_ctrl1;
#[doc = "LCD_CTRL2 (rw) register accessor: LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_ctrl2`] module"]
pub type LCD_CTRL2 = crate::Reg<lcd_ctrl2::LCD_CTRL2_SPEC>;
#[doc = "LCD config register."]
pub mod lcd_ctrl2;
#[doc = "LCD_FIRST_CMD_VAL (rw) register accessor: LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_first_cmd_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_first_cmd_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_first_cmd_val`] module"]
pub type LCD_FIRST_CMD_VAL = crate::Reg<lcd_first_cmd_val::LCD_FIRST_CMD_VAL_SPEC>;
#[doc = "LCD config register."]
pub mod lcd_first_cmd_val;
#[doc = "LCD_LATTER_CMD_VAL (rw) register accessor: LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_latter_cmd_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_latter_cmd_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_latter_cmd_val`] module"]
pub type LCD_LATTER_CMD_VAL = crate::Reg<lcd_latter_cmd_val::LCD_LATTER_CMD_VAL_SPEC>;
#[doc = "LCD config register."]
pub mod lcd_latter_cmd_val;
#[doc = "LCD_DLY_MODE_CFG1 (rw) register accessor: LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_dly_mode_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_dly_mode_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_dly_mode_cfg1`] module"]
pub type LCD_DLY_MODE_CFG1 = crate::Reg<lcd_dly_mode_cfg1::LCD_DLY_MODE_CFG1_SPEC>;
#[doc = "LCD config register."]
pub mod lcd_dly_mode_cfg1;
#[doc = "LCD_DLY_MODE_CFG2 (rw) register accessor: LCD config register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_dly_mode_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_dly_mode_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_dly_mode_cfg2`] module"]
pub type LCD_DLY_MODE_CFG2 = crate::Reg<lcd_dly_mode_cfg2::LCD_DLY_MODE_CFG2_SPEC>;
#[doc = "LCD config register."]
pub mod lcd_dly_mode_cfg2;
#[doc = "LC_DMA_INT_ENA (rw) register accessor: LCDCAM interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_dma_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_dma_int_ena`] module"]
pub type LC_DMA_INT_ENA = crate::Reg<lc_dma_int_ena::LC_DMA_INT_ENA_SPEC>;
#[doc = "LCDCAM interrupt enable register."]
pub mod lc_dma_int_ena;
#[doc = "LC_DMA_INT_RAW (r) register accessor: LCDCAM interrupt raw register, valid in level.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_dma_int_raw`] module"]
pub type LC_DMA_INT_RAW = crate::Reg<lc_dma_int_raw::LC_DMA_INT_RAW_SPEC>;
#[doc = "LCDCAM interrupt raw register, valid in level."]
pub mod lc_dma_int_raw;
#[doc = "LC_DMA_INT_ST (r) register accessor: LCDCAM interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_dma_int_st`] module"]
pub type LC_DMA_INT_ST = crate::Reg<lc_dma_int_st::LC_DMA_INT_ST_SPEC>;
#[doc = "LCDCAM interrupt status register."]
pub mod lc_dma_int_st;
#[doc = "LC_DMA_INT_CLR (w) register accessor: LCDCAM interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_dma_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_dma_int_clr`] module"]
pub type LC_DMA_INT_CLR = crate::Reg<lc_dma_int_clr::LC_DMA_INT_CLR_SPEC>;
#[doc = "LCDCAM interrupt clear register."]
pub mod lc_dma_int_clr;
#[doc = "LC_REG_DATE (rw) register accessor: Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lc_reg_date`] module"]
pub type LC_REG_DATE = crate::Reg<lc_reg_date::LC_REG_DATE_SPEC>;
#[doc = "Version register"]
pub mod lc_reg_date;
