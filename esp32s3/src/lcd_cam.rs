#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD clock register"]
    pub lcd_clock: LCD_CLOCK,
    #[doc = "0x04 - Camera configuration register"]
    pub cam_ctrl: CAM_CTRL,
    #[doc = "0x08 - Camera configuration register"]
    pub cam_ctrl1: CAM_CTRL1,
    #[doc = "0x0c - Camera configuration register"]
    pub cam_rgb_yuv: CAM_RGB_YUV,
    #[doc = "0x10 - LCD configuration register"]
    pub lcd_rgb_yuv: LCD_RGB_YUV,
    #[doc = "0x14 - LCD configuration register"]
    pub lcd_user: LCD_USER,
    #[doc = "0x18 - LCD configuration register"]
    pub lcd_misc: LCD_MISC,
    #[doc = "0x1c - LCD configuration register"]
    pub lcd_ctrl: LCD_CTRL,
    #[doc = "0x20 - LCD configuration register"]
    pub lcd_ctrl1: LCD_CTRL1,
    #[doc = "0x24 - LCD configuration register"]
    pub lcd_ctrl2: LCD_CTRL2,
    #[doc = "0x28 - LCD configuration register"]
    pub lcd_cmd_val: LCD_CMD_VAL,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - LCD configuration register"]
    pub lcd_dly_mode: LCD_DLY_MODE,
    _reserved12: [u8; 0x04],
    #[doc = "0x38 - LCD configuration register"]
    pub lcd_data_dout_mode: LCD_DATA_DOUT_MODE,
    _reserved13: [u8; 0x28],
    #[doc = "0x64 - LCD_camera DMA inturrupt enable register"]
    pub lc_dma_int_ena: LC_DMA_INT_ENA,
    #[doc = "0x68 - LCD_camera DMA raw inturrupt status register"]
    pub lc_dma_int_raw: LC_DMA_INT_RAW,
    #[doc = "0x6c - LCD_camera DMA masked inturrupt status register"]
    pub lc_dma_int_st: LC_DMA_INT_ST,
    #[doc = "0x70 - LCD_camera DMA inturrupt clear register"]
    pub lc_dma_int_clr: LC_DMA_INT_CLR,
    _reserved17: [u8; 0x88],
    #[doc = "0xfc - Version register"]
    pub lc_reg_date: LC_REG_DATE,
}
#[doc = "LCD_CLOCK (rw) register accessor: LCD clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_clock`] module"]
pub type LCD_CLOCK = crate::Reg<lcd_clock::LCD_CLOCK_SPEC>;
#[doc = "LCD clock register"]
pub mod lcd_clock;
#[doc = "CAM_CTRL (rw) register accessor: Camera configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cam_ctrl`] module"]
pub type CAM_CTRL = crate::Reg<cam_ctrl::CAM_CTRL_SPEC>;
#[doc = "Camera configuration register"]
pub mod cam_ctrl;
#[doc = "CAM_CTRL1 (rw) register accessor: Camera configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cam_ctrl1`] module"]
pub type CAM_CTRL1 = crate::Reg<cam_ctrl1::CAM_CTRL1_SPEC>;
#[doc = "Camera configuration register"]
pub mod cam_ctrl1;
#[doc = "CAM_RGB_YUV (rw) register accessor: Camera configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_rgb_yuv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_rgb_yuv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cam_rgb_yuv`] module"]
pub type CAM_RGB_YUV = crate::Reg<cam_rgb_yuv::CAM_RGB_YUV_SPEC>;
#[doc = "Camera configuration register"]
pub mod cam_rgb_yuv;
#[doc = "LCD_RGB_YUV (rw) register accessor: LCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_rgb_yuv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_rgb_yuv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_rgb_yuv`] module"]
pub type LCD_RGB_YUV = crate::Reg<lcd_rgb_yuv::LCD_RGB_YUV_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_rgb_yuv;
#[doc = "LCD_USER (rw) register accessor: LCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_user::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_user::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_user`] module"]
pub type LCD_USER = crate::Reg<lcd_user::LCD_USER_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_user;
#[doc = "LCD_MISC (rw) register accessor: LCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_misc`] module"]
pub type LCD_MISC = crate::Reg<lcd_misc::LCD_MISC_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_misc;
#[doc = "LCD_CTRL (rw) register accessor: LCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_ctrl`] module"]
pub type LCD_CTRL = crate::Reg<lcd_ctrl::LCD_CTRL_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_ctrl;
#[doc = "LCD_CTRL1 (rw) register accessor: LCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_ctrl1`] module"]
pub type LCD_CTRL1 = crate::Reg<lcd_ctrl1::LCD_CTRL1_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_ctrl1;
#[doc = "LCD_CTRL2 (rw) register accessor: LCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_ctrl2`] module"]
pub type LCD_CTRL2 = crate::Reg<lcd_ctrl2::LCD_CTRL2_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_ctrl2;
#[doc = "LCD_CMD_VAL (rw) register accessor: LCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cmd_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cmd_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_cmd_val`] module"]
pub type LCD_CMD_VAL = crate::Reg<lcd_cmd_val::LCD_CMD_VAL_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_cmd_val;
#[doc = "LCD_DLY_MODE (rw) register accessor: LCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_dly_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_dly_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_dly_mode`] module"]
pub type LCD_DLY_MODE = crate::Reg<lcd_dly_mode::LCD_DLY_MODE_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_dly_mode;
#[doc = "LCD_DATA_DOUT_MODE (rw) register accessor: LCD configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_data_dout_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_data_dout_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lcd_data_dout_mode`] module"]
pub type LCD_DATA_DOUT_MODE = crate::Reg<lcd_data_dout_mode::LCD_DATA_DOUT_MODE_SPEC>;
#[doc = "LCD configuration register"]
pub mod lcd_data_dout_mode;
#[doc = "LC_DMA_INT_ENA (rw) register accessor: LCD_camera DMA inturrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_dma_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lc_dma_int_ena`] module"]
pub type LC_DMA_INT_ENA = crate::Reg<lc_dma_int_ena::LC_DMA_INT_ENA_SPEC>;
#[doc = "LCD_camera DMA inturrupt enable register"]
pub mod lc_dma_int_ena;
#[doc = "LC_DMA_INT_RAW (r) register accessor: LCD_camera DMA raw inturrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lc_dma_int_raw`] module"]
pub type LC_DMA_INT_RAW = crate::Reg<lc_dma_int_raw::LC_DMA_INT_RAW_SPEC>;
#[doc = "LCD_camera DMA raw inturrupt status register"]
pub mod lc_dma_int_raw;
#[doc = "LC_DMA_INT_ST (r) register accessor: LCD_camera DMA masked inturrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lc_dma_int_st`] module"]
pub type LC_DMA_INT_ST = crate::Reg<lc_dma_int_st::LC_DMA_INT_ST_SPEC>;
#[doc = "LCD_camera DMA masked inturrupt status register"]
pub mod lc_dma_int_st;
#[doc = "LC_DMA_INT_CLR (w) register accessor: LCD_camera DMA inturrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_dma_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lc_dma_int_clr`] module"]
pub type LC_DMA_INT_CLR = crate::Reg<lc_dma_int_clr::LC_DMA_INT_CLR_SPEC>;
#[doc = "LCD_camera DMA inturrupt clear register"]
pub mod lc_dma_int_clr;
#[doc = "LC_REG_DATE (rw) register accessor: Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lc_reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lc_reg_date`] module"]
pub type LC_REG_DATE = crate::Reg<lc_reg_date::LC_REG_DATE_SPEC>;
#[doc = "Version register"]
pub mod lc_reg_date;
