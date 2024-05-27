#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    lcd_cmd_val: LCD_CMD_VAL,
    _reserved11: [u8; 0x04],
    lcd_dly_mode: LCD_DLY_MODE,
    _reserved12: [u8; 0x04],
    lcd_data_dout_mode: LCD_DATA_DOUT_MODE,
    _reserved13: [u8; 0x28],
    lc_dma_int_ena: LC_DMA_INT_ENA,
    lc_dma_int_raw: LC_DMA_INT_RAW,
    lc_dma_int_st: LC_DMA_INT_ST,
    lc_dma_int_clr: LC_DMA_INT_CLR,
    _reserved17: [u8; 0x88],
    lc_reg_date: LC_REG_DATE,
}
impl RegisterBlock {
    ///0x00 - LCD clock configuration register
    #[inline(always)]
    pub const fn lcd_clock(&self) -> &LCD_CLOCK {
        &self.lcd_clock
    }
    ///0x04 - Camera clock configuration register
    #[inline(always)]
    pub const fn cam_ctrl(&self) -> &CAM_CTRL {
        &self.cam_ctrl
    }
    ///0x08 - Camera control register
    #[inline(always)]
    pub const fn cam_ctrl1(&self) -> &CAM_CTRL1 {
        &self.cam_ctrl1
    }
    ///0x0c - Camera data format conversion register
    #[inline(always)]
    pub const fn cam_rgb_yuv(&self) -> &CAM_RGB_YUV {
        &self.cam_rgb_yuv
    }
    ///0x10 - LCD data format conversion register
    #[inline(always)]
    pub const fn lcd_rgb_yuv(&self) -> &LCD_RGB_YUV {
        &self.lcd_rgb_yuv
    }
    ///0x14 - LCD user configuration register
    #[inline(always)]
    pub const fn lcd_user(&self) -> &LCD_USER {
        &self.lcd_user
    }
    ///0x18 - LCD MISC configuration register
    #[inline(always)]
    pub const fn lcd_misc(&self) -> &LCD_MISC {
        &self.lcd_misc
    }
    ///0x1c - LCD signal configuration register
    #[inline(always)]
    pub const fn lcd_ctrl(&self) -> &LCD_CTRL {
        &self.lcd_ctrl
    }
    ///0x20 - LCD signal configuration register 1
    #[inline(always)]
    pub const fn lcd_ctrl1(&self) -> &LCD_CTRL1 {
        &self.lcd_ctrl1
    }
    ///0x24 - LCD signal configuration register 2
    #[inline(always)]
    pub const fn lcd_ctrl2(&self) -> &LCD_CTRL2 {
        &self.lcd_ctrl2
    }
    ///0x28 - LCD command value configuration register
    #[inline(always)]
    pub const fn lcd_cmd_val(&self) -> &LCD_CMD_VAL {
        &self.lcd_cmd_val
    }
    ///0x30 - LCD signal delay configuration register
    #[inline(always)]
    pub const fn lcd_dly_mode(&self) -> &LCD_DLY_MODE {
        &self.lcd_dly_mode
    }
    ///0x38 - LCD data delay configuration register
    #[inline(always)]
    pub const fn lcd_data_dout_mode(&self) -> &LCD_DATA_DOUT_MODE {
        &self.lcd_data_dout_mode
    }
    ///0x64 - LCD_CAM GDMA interrupt enable register
    #[inline(always)]
    pub const fn lc_dma_int_ena(&self) -> &LC_DMA_INT_ENA {
        &self.lc_dma_int_ena
    }
    ///0x68 - LCD_CAM GDMA raw interrupt status register
    #[inline(always)]
    pub const fn lc_dma_int_raw(&self) -> &LC_DMA_INT_RAW {
        &self.lc_dma_int_raw
    }
    ///0x6c - LCD_CAM GDMA masked interrupt status register
    #[inline(always)]
    pub const fn lc_dma_int_st(&self) -> &LC_DMA_INT_ST {
        &self.lc_dma_int_st
    }
    ///0x70 - LCD_CAM GDMA interrupt clear register
    #[inline(always)]
    pub const fn lc_dma_int_clr(&self) -> &LC_DMA_INT_CLR {
        &self.lc_dma_int_clr
    }
    ///0xfc - Version control register
    #[inline(always)]
    pub const fn lc_reg_date(&self) -> &LC_REG_DATE {
        &self.lc_reg_date
    }
}
/**LCD_CLOCK (rw) register accessor: LCD clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_clock`] module*/
pub type LCD_CLOCK = crate::Reg<lcd_clock::LCD_CLOCK_SPEC>;
///LCD clock configuration register
pub mod lcd_clock;
/**CAM_CTRL (rw) register accessor: Camera clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cam_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cam_ctrl`] module*/
pub type CAM_CTRL = crate::Reg<cam_ctrl::CAM_CTRL_SPEC>;
///Camera clock configuration register
pub mod cam_ctrl;
/**CAM_CTRL1 (rw) register accessor: Camera control register

You can [`read`](crate::generic::Reg::read) this register and get [`cam_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cam_ctrl1`] module*/
pub type CAM_CTRL1 = crate::Reg<cam_ctrl1::CAM_CTRL1_SPEC>;
///Camera control register
pub mod cam_ctrl1;
/**CAM_RGB_YUV (rw) register accessor: Camera data format conversion register

You can [`read`](crate::generic::Reg::read) this register and get [`cam_rgb_yuv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_rgb_yuv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cam_rgb_yuv`] module*/
pub type CAM_RGB_YUV = crate::Reg<cam_rgb_yuv::CAM_RGB_YUV_SPEC>;
///Camera data format conversion register
pub mod cam_rgb_yuv;
/**LCD_RGB_YUV (rw) register accessor: LCD data format conversion register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_rgb_yuv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_rgb_yuv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_rgb_yuv`] module*/
pub type LCD_RGB_YUV = crate::Reg<lcd_rgb_yuv::LCD_RGB_YUV_SPEC>;
///LCD data format conversion register
pub mod lcd_rgb_yuv;
/**LCD_USER (rw) register accessor: LCD user configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_user`] module*/
pub type LCD_USER = crate::Reg<lcd_user::LCD_USER_SPEC>;
///LCD user configuration register
pub mod lcd_user;
/**LCD_MISC (rw) register accessor: LCD MISC configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_misc`] module*/
pub type LCD_MISC = crate::Reg<lcd_misc::LCD_MISC_SPEC>;
///LCD MISC configuration register
pub mod lcd_misc;
/**LCD_CTRL (rw) register accessor: LCD signal configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_ctrl`] module*/
pub type LCD_CTRL = crate::Reg<lcd_ctrl::LCD_CTRL_SPEC>;
///LCD signal configuration register
pub mod lcd_ctrl;
/**LCD_CTRL1 (rw) register accessor: LCD signal configuration register 1

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_ctrl1`] module*/
pub type LCD_CTRL1 = crate::Reg<lcd_ctrl1::LCD_CTRL1_SPEC>;
///LCD signal configuration register 1
pub mod lcd_ctrl1;
/**LCD_CTRL2 (rw) register accessor: LCD signal configuration register 2

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_ctrl2`] module*/
pub type LCD_CTRL2 = crate::Reg<lcd_ctrl2::LCD_CTRL2_SPEC>;
///LCD signal configuration register 2
pub mod lcd_ctrl2;
/**LCD_CMD_VAL (rw) register accessor: LCD command value configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_cmd_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cmd_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_cmd_val`] module*/
pub type LCD_CMD_VAL = crate::Reg<lcd_cmd_val::LCD_CMD_VAL_SPEC>;
///LCD command value configuration register
pub mod lcd_cmd_val;
/**LCD_DLY_MODE (rw) register accessor: LCD signal delay configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_dly_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_dly_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_dly_mode`] module*/
pub type LCD_DLY_MODE = crate::Reg<lcd_dly_mode::LCD_DLY_MODE_SPEC>;
///LCD signal delay configuration register
pub mod lcd_dly_mode;
/**LCD_DATA_DOUT_MODE (rw) register accessor: LCD data delay configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_data_dout_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_data_dout_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lcd_data_dout_mode`] module*/
pub type LCD_DATA_DOUT_MODE = crate::Reg<lcd_data_dout_mode::LCD_DATA_DOUT_MODE_SPEC>;
///LCD data delay configuration register
pub mod lcd_data_dout_mode;
/**LC_DMA_INT_ENA (rw) register accessor: LCD_CAM GDMA interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_dma_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lc_dma_int_ena`] module*/
pub type LC_DMA_INT_ENA = crate::Reg<lc_dma_int_ena::LC_DMA_INT_ENA_SPEC>;
///LCD_CAM GDMA interrupt enable register
pub mod lc_dma_int_ena;
/**LC_DMA_INT_RAW (r) register accessor: LCD_CAM GDMA raw interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lc_dma_int_raw`] module*/
pub type LC_DMA_INT_RAW = crate::Reg<lc_dma_int_raw::LC_DMA_INT_RAW_SPEC>;
///LCD_CAM GDMA raw interrupt status register
pub mod lc_dma_int_raw;
/**LC_DMA_INT_ST (r) register accessor: LCD_CAM GDMA masked interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lc_dma_int_st`] module*/
pub type LC_DMA_INT_ST = crate::Reg<lc_dma_int_st::LC_DMA_INT_ST_SPEC>;
///LCD_CAM GDMA masked interrupt status register
pub mod lc_dma_int_st;
/**LC_DMA_INT_CLR (w) register accessor: LCD_CAM GDMA interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_dma_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lc_dma_int_clr`] module*/
pub type LC_DMA_INT_CLR = crate::Reg<lc_dma_int_clr::LC_DMA_INT_CLR_SPEC>;
///LCD_CAM GDMA interrupt clear register
pub mod lc_dma_int_clr;
/**LC_REG_DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`lc_reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lc_reg_date`] module*/
pub type LC_REG_DATE = crate::Reg<lc_reg_date::LC_REG_DATE_SPEC>;
///Version control register
pub mod lc_reg_date;
