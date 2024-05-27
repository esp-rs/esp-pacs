#[doc = "Register `LC_DMA_INT_ST` reader"]
pub type R = crate::R<LC_DMA_INT_ST_SPEC>;
#[doc = "Field `LCD_VSYNC_INT_ST` reader - The status bit for LCD frame end interrupt."]
pub type LCD_VSYNC_INT_ST_R = crate::BitReader;
#[doc = "Field `LCD_TRANS_DONE_INT_ST` reader - The status bit for lcd transfer end interrupt."]
pub type LCD_TRANS_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `CAM_VSYNC_INT_ST` reader - The status bit for Camera frame end interrupt."]
pub type CAM_VSYNC_INT_ST_R = crate::BitReader;
#[doc = "Field `CAM_HS_INT_ST` reader - The status bit for Camera transfer end interrupt."]
pub type CAM_HS_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status bit for LCD frame end interrupt."]
    #[inline(always)]
    pub fn lcd_vsync_int_st(&self) -> LCD_VSYNC_INT_ST_R {
        LCD_VSYNC_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for lcd transfer end interrupt."]
    #[inline(always)]
    pub fn lcd_trans_done_int_st(&self) -> LCD_TRANS_DONE_INT_ST_R {
        LCD_TRANS_DONE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for Camera frame end interrupt."]
    #[inline(always)]
    pub fn cam_vsync_int_st(&self) -> CAM_VSYNC_INT_ST_R {
        CAM_VSYNC_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for Camera transfer end interrupt."]
    #[inline(always)]
    pub fn cam_hs_int_st(&self) -> CAM_HS_INT_ST_R {
        CAM_HS_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC_DMA_INT_ST")
            .field("lcd_vsync_int_st", &self.lcd_vsync_int_st())
            .field("lcd_trans_done_int_st", &self.lcd_trans_done_int_st())
            .field("cam_vsync_int_st", &self.cam_vsync_int_st())
            .field("cam_hs_int_st", &self.cam_hs_int_st())
            .finish()
    }
}
#[doc = "LCDCAM interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_dma_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_DMA_INT_ST_SPEC;
impl crate::RegisterSpec for LC_DMA_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_dma_int_st::R`](R) reader structure"]
impl crate::Readable for LC_DMA_INT_ST_SPEC {}
#[doc = "`reset()` method sets LC_DMA_INT_ST to value 0"]
impl crate::Resettable for LC_DMA_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
