#[doc = "Register `LC_DMA_INT_CLR` writer"]
pub type W = crate::W<LC_DMA_INT_CLR_SPEC>;
#[doc = "Field `LCD_VSYNC_INT_CLR` writer - The clear bit for LCD_CAM_LCD_VSYNC_INT interrupt."]
pub type LCD_VSYNC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_TRANS_DONE_INT_CLR` writer - The clear bit for LCD_CAM_LCD_TRANS_DONE_INT interrupt."]
pub type LCD_TRANS_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_VSYNC_INT_CLR` writer - The clear bit for LCD_CAM_CAM_VSYNC_INT interrupt."]
pub type CAM_VSYNC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_HS_INT_CLR` writer - The clear bit for LCD_CAM_CAM_HS_INT interrupt."]
pub type CAM_HS_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LC_DMA_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The clear bit for LCD_CAM_LCD_VSYNC_INT interrupt."]
    #[inline(always)]
    pub fn lcd_vsync_int_clr(&mut self) -> LCD_VSYNC_INT_CLR_W<'_, LC_DMA_INT_CLR_SPEC> {
        LCD_VSYNC_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The clear bit for LCD_CAM_LCD_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lcd_trans_done_int_clr(&mut self) -> LCD_TRANS_DONE_INT_CLR_W<'_, LC_DMA_INT_CLR_SPEC> {
        LCD_TRANS_DONE_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - The clear bit for LCD_CAM_CAM_VSYNC_INT interrupt."]
    #[inline(always)]
    pub fn cam_vsync_int_clr(&mut self) -> CAM_VSYNC_INT_CLR_W<'_, LC_DMA_INT_CLR_SPEC> {
        CAM_VSYNC_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The clear bit for LCD_CAM_CAM_HS_INT interrupt."]
    #[inline(always)]
    pub fn cam_hs_int_clr(&mut self) -> CAM_HS_INT_CLR_W<'_, LC_DMA_INT_CLR_SPEC> {
        CAM_HS_INT_CLR_W::new(self, 3)
    }
}
#[doc = "LCD_CAM GDMA interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lc_dma_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_DMA_INT_CLR_SPEC;
impl crate::RegisterSpec for LC_DMA_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lc_dma_int_clr::W`](W) writer structure"]
impl crate::Writable for LC_DMA_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LC_DMA_INT_CLR to value 0"]
impl crate::Resettable for LC_DMA_INT_CLR_SPEC {}
