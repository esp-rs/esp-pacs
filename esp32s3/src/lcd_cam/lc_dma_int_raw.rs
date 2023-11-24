#[doc = "Register `LC_DMA_INT_RAW` reader"]
pub type R = crate::R<LC_DMA_INT_RAW_SPEC>;
#[doc = "Field `LCD_VSYNC_INT_RAW` reader - The raw bit for LCD frame end interrupt."]
pub type LCD_VSYNC_INT_RAW_R = crate::BitReader;
#[doc = "Field `LCD_TRANS_DONE_INT_RAW` reader - The raw bit for lcd transfer end interrupt."]
pub type LCD_TRANS_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `CAM_VSYNC_INT_RAW` reader - The raw bit for Camera frame end interrupt."]
pub type CAM_VSYNC_INT_RAW_R = crate::BitReader;
#[doc = "Field `CAM_HS_INT_RAW` reader - The raw bit for Camera line interrupt."]
pub type CAM_HS_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw bit for LCD frame end interrupt."]
    #[inline(always)]
    pub fn lcd_vsync_int_raw(&self) -> LCD_VSYNC_INT_RAW_R {
        LCD_VSYNC_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit for lcd transfer end interrupt."]
    #[inline(always)]
    pub fn lcd_trans_done_int_raw(&self) -> LCD_TRANS_DONE_INT_RAW_R {
        LCD_TRANS_DONE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit for Camera frame end interrupt."]
    #[inline(always)]
    pub fn cam_vsync_int_raw(&self) -> CAM_VSYNC_INT_RAW_R {
        CAM_VSYNC_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit for Camera line interrupt."]
    #[inline(always)]
    pub fn cam_hs_int_raw(&self) -> CAM_HS_INT_RAW_R {
        CAM_HS_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC_DMA_INT_RAW")
            .field(
                "lcd_vsync_int_raw",
                &format_args!("{}", self.lcd_vsync_int_raw().bit()),
            )
            .field(
                "lcd_trans_done_int_raw",
                &format_args!("{}", self.lcd_trans_done_int_raw().bit()),
            )
            .field(
                "cam_vsync_int_raw",
                &format_args!("{}", self.cam_vsync_int_raw().bit()),
            )
            .field(
                "cam_hs_int_raw",
                &format_args!("{}", self.cam_hs_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LC_DMA_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "LCD_camera DMA raw inturrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_DMA_INT_RAW_SPEC;
impl crate::RegisterSpec for LC_DMA_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_dma_int_raw::R`](R) reader structure"]
impl crate::Readable for LC_DMA_INT_RAW_SPEC {}
#[doc = "`reset()` method sets LC_DMA_INT_RAW to value 0"]
impl crate::Resettable for LC_DMA_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
