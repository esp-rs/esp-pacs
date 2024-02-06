#[doc = "Register `SR_PARAM_ERR_ST` reader"]
pub type R = crate::R<SR_PARAM_ERR_ST_SPEC>;
#[doc = "Field `TX_DSCR_VB_ERR_ST` reader - The error is that the scaled VB plus the offset of Y coordinate in 2DDMA receive descriptor is larger than VA in 2DDMA receive descriptor."]
pub type TX_DSCR_VB_ERR_ST_R = crate::BitReader;
#[doc = "Field `TX_DSCR_HB_ERR_ST` reader - The error is that the scaled HB plus the offset of X coordinate in 2DDMA receive descriptor is larger than HA in 2DDMA receive descriptor."]
pub type TX_DSCR_HB_ERR_ST_R = crate::BitReader;
#[doc = "Field `Y_RX_SCAL_EQUAL_0_ERR_ST` reader - The error is that the PPA_SR_SCAL_Y_INT and PPA_SR_CAL_Y_FRAG both are 0."]
pub type Y_RX_SCAL_EQUAL_0_ERR_ST_R = crate::BitReader;
#[doc = "Field `RX_DSCR_VB_ERR_ST` reader - The error is that VB in 2DDMA receive descriptor plus the offset of Y coordinate in 2DDMA transmit descriptor is larger than VA in 2DDMA transmit descriptor"]
pub type RX_DSCR_VB_ERR_ST_R = crate::BitReader;
#[doc = "Field `YDST_LEN_TOO_SAMLL_ERR_ST` reader - The error is that the scaled image width is 0. For example. when source width is 14. scaled value is 1/16. and no rotate operation. then scaled width would be 0 as the result would be floored."]
pub type YDST_LEN_TOO_SAMLL_ERR_ST_R = crate::BitReader;
#[doc = "Field `YDST_LEN_TOO_LARGE_ERR_ST` reader - The error is that the scaled width is larger than (2^13 - 1)."]
pub type YDST_LEN_TOO_LARGE_ERR_ST_R = crate::BitReader;
#[doc = "Field `X_RX_SCAL_EQUAL_0_ERR_ST` reader - The error is that the scaled image height is 0."]
pub type X_RX_SCAL_EQUAL_0_ERR_ST_R = crate::BitReader;
#[doc = "Field `RX_DSCR_HB_ERR_ST` reader - The error is that the HB in 2DDMA transmit descriptor plus the offset of X coordinate in 2DDMA transmit descriptor is larger than HA in 2DDMA transmit descriptor."]
pub type RX_DSCR_HB_ERR_ST_R = crate::BitReader;
#[doc = "Field `XDST_LEN_TOO_SAMLL_ERR_ST` reader - The error is that the scaled image height is 0. For example. when source height is 14. scaled value is 1/16. and no rotate operation. then scaled height would be 0 as the result would be floored."]
pub type XDST_LEN_TOO_SAMLL_ERR_ST_R = crate::BitReader;
#[doc = "Field `XDST_LEN_TOO_LARGE_ERR_ST` reader - The error is that the scaled image height is larger than (2^13 - 1)."]
pub type XDST_LEN_TOO_LARGE_ERR_ST_R = crate::BitReader;
#[doc = "Field `X_YUV420_RX_SCALE_ERR_ST` reader - The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 rx"]
pub type X_YUV420_RX_SCALE_ERR_ST_R = crate::BitReader;
#[doc = "Field `Y_YUV420_RX_SCALE_ERR_ST` reader - The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 rx"]
pub type Y_YUV420_RX_SCALE_ERR_ST_R = crate::BitReader;
#[doc = "Field `X_YUV420_TX_SCALE_ERR_ST` reader - The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 tx"]
pub type X_YUV420_TX_SCALE_ERR_ST_R = crate::BitReader;
#[doc = "Field `Y_YUV420_TX_SCALE_ERR_ST` reader - The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 tx"]
pub type Y_YUV420_TX_SCALE_ERR_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The error is that the scaled VB plus the offset of Y coordinate in 2DDMA receive descriptor is larger than VA in 2DDMA receive descriptor."]
    #[inline(always)]
    pub fn tx_dscr_vb_err_st(&self) -> TX_DSCR_VB_ERR_ST_R {
        TX_DSCR_VB_ERR_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The error is that the scaled HB plus the offset of X coordinate in 2DDMA receive descriptor is larger than HA in 2DDMA receive descriptor."]
    #[inline(always)]
    pub fn tx_dscr_hb_err_st(&self) -> TX_DSCR_HB_ERR_ST_R {
        TX_DSCR_HB_ERR_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The error is that the PPA_SR_SCAL_Y_INT and PPA_SR_CAL_Y_FRAG both are 0."]
    #[inline(always)]
    pub fn y_rx_scal_equal_0_err_st(&self) -> Y_RX_SCAL_EQUAL_0_ERR_ST_R {
        Y_RX_SCAL_EQUAL_0_ERR_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The error is that VB in 2DDMA receive descriptor plus the offset of Y coordinate in 2DDMA transmit descriptor is larger than VA in 2DDMA transmit descriptor"]
    #[inline(always)]
    pub fn rx_dscr_vb_err_st(&self) -> RX_DSCR_VB_ERR_ST_R {
        RX_DSCR_VB_ERR_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The error is that the scaled image width is 0. For example. when source width is 14. scaled value is 1/16. and no rotate operation. then scaled width would be 0 as the result would be floored."]
    #[inline(always)]
    pub fn ydst_len_too_samll_err_st(&self) -> YDST_LEN_TOO_SAMLL_ERR_ST_R {
        YDST_LEN_TOO_SAMLL_ERR_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The error is that the scaled width is larger than (2^13 - 1)."]
    #[inline(always)]
    pub fn ydst_len_too_large_err_st(&self) -> YDST_LEN_TOO_LARGE_ERR_ST_R {
        YDST_LEN_TOO_LARGE_ERR_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The error is that the scaled image height is 0."]
    #[inline(always)]
    pub fn x_rx_scal_equal_0_err_st(&self) -> X_RX_SCAL_EQUAL_0_ERR_ST_R {
        X_RX_SCAL_EQUAL_0_ERR_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The error is that the HB in 2DDMA transmit descriptor plus the offset of X coordinate in 2DDMA transmit descriptor is larger than HA in 2DDMA transmit descriptor."]
    #[inline(always)]
    pub fn rx_dscr_hb_err_st(&self) -> RX_DSCR_HB_ERR_ST_R {
        RX_DSCR_HB_ERR_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The error is that the scaled image height is 0. For example. when source height is 14. scaled value is 1/16. and no rotate operation. then scaled height would be 0 as the result would be floored."]
    #[inline(always)]
    pub fn xdst_len_too_samll_err_st(&self) -> XDST_LEN_TOO_SAMLL_ERR_ST_R {
        XDST_LEN_TOO_SAMLL_ERR_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The error is that the scaled image height is larger than (2^13 - 1)."]
    #[inline(always)]
    pub fn xdst_len_too_large_err_st(&self) -> XDST_LEN_TOO_LARGE_ERR_ST_R {
        XDST_LEN_TOO_LARGE_ERR_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 rx"]
    #[inline(always)]
    pub fn x_yuv420_rx_scale_err_st(&self) -> X_YUV420_RX_SCALE_ERR_ST_R {
        X_YUV420_RX_SCALE_ERR_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 rx"]
    #[inline(always)]
    pub fn y_yuv420_rx_scale_err_st(&self) -> Y_YUV420_RX_SCALE_ERR_ST_R {
        Y_YUV420_RX_SCALE_ERR_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The error is that the ha/hb/x param in dma2d descriptor is an odd num when enable yuv420 tx"]
    #[inline(always)]
    pub fn x_yuv420_tx_scale_err_st(&self) -> X_YUV420_TX_SCALE_ERR_ST_R {
        X_YUV420_TX_SCALE_ERR_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The error is that the va/vb/y param in dma2d descriptor is an odd num when enable yuv420 tx"]
    #[inline(always)]
    pub fn y_yuv420_tx_scale_err_st(&self) -> Y_YUV420_TX_SCALE_ERR_ST_R {
        Y_YUV420_TX_SCALE_ERR_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_PARAM_ERR_ST")
            .field(
                "tx_dscr_vb_err_st",
                &format_args!("{}", self.tx_dscr_vb_err_st().bit()),
            )
            .field(
                "tx_dscr_hb_err_st",
                &format_args!("{}", self.tx_dscr_hb_err_st().bit()),
            )
            .field(
                "y_rx_scal_equal_0_err_st",
                &format_args!("{}", self.y_rx_scal_equal_0_err_st().bit()),
            )
            .field(
                "rx_dscr_vb_err_st",
                &format_args!("{}", self.rx_dscr_vb_err_st().bit()),
            )
            .field(
                "ydst_len_too_samll_err_st",
                &format_args!("{}", self.ydst_len_too_samll_err_st().bit()),
            )
            .field(
                "ydst_len_too_large_err_st",
                &format_args!("{}", self.ydst_len_too_large_err_st().bit()),
            )
            .field(
                "x_rx_scal_equal_0_err_st",
                &format_args!("{}", self.x_rx_scal_equal_0_err_st().bit()),
            )
            .field(
                "rx_dscr_hb_err_st",
                &format_args!("{}", self.rx_dscr_hb_err_st().bit()),
            )
            .field(
                "xdst_len_too_samll_err_st",
                &format_args!("{}", self.xdst_len_too_samll_err_st().bit()),
            )
            .field(
                "xdst_len_too_large_err_st",
                &format_args!("{}", self.xdst_len_too_large_err_st().bit()),
            )
            .field(
                "x_yuv420_rx_scale_err_st",
                &format_args!("{}", self.x_yuv420_rx_scale_err_st().bit()),
            )
            .field(
                "y_yuv420_rx_scale_err_st",
                &format_args!("{}", self.y_yuv420_rx_scale_err_st().bit()),
            )
            .field(
                "x_yuv420_tx_scale_err_st",
                &format_args!("{}", self.x_yuv420_tx_scale_err_st().bit()),
            )
            .field(
                "y_yuv420_tx_scale_err_st",
                &format_args!("{}", self.y_yuv420_tx_scale_err_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SR_PARAM_ERR_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Scaling and rotating coefficient error register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr_param_err_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_PARAM_ERR_ST_SPEC;
impl crate::RegisterSpec for SR_PARAM_ERR_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_param_err_st::R`](R) reader structure"]
impl crate::Readable for SR_PARAM_ERR_ST_SPEC {}
#[doc = "`reset()` method sets SR_PARAM_ERR_ST to value 0"]
impl crate::Resettable for SR_PARAM_ERR_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
