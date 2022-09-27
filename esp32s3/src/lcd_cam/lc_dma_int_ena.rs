#[doc = "Register `LC_DMA_INT_ENA` reader"]
pub struct R(crate::R<LC_DMA_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_DMA_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_DMA_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_DMA_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LC_DMA_INT_ENA` writer"]
pub struct W(crate::W<LC_DMA_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LC_DMA_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LC_DMA_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LC_DMA_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCD_VSYNC_INT_ENA` reader - The enable bit for LCD frame end interrupt."]
pub type LCD_VSYNC_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `LCD_VSYNC_INT_ENA` writer - The enable bit for LCD frame end interrupt."]
pub type LCD_VSYNC_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LC_DMA_INT_ENA_SPEC, bool, O>;
#[doc = "Field `LCD_TRANS_DONE_INT_ENA` reader - The enable bit for lcd transfer end interrupt."]
pub type LCD_TRANS_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `LCD_TRANS_DONE_INT_ENA` writer - The enable bit for lcd transfer end interrupt."]
pub type LCD_TRANS_DONE_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LC_DMA_INT_ENA_SPEC, bool, O>;
#[doc = "Field `CAM_VSYNC_INT_ENA` reader - The enable bit for Camera frame end interrupt."]
pub type CAM_VSYNC_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CAM_VSYNC_INT_ENA` writer - The enable bit for Camera frame end interrupt."]
pub type CAM_VSYNC_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LC_DMA_INT_ENA_SPEC, bool, O>;
#[doc = "Field `CAM_HS_INT_ENA` reader - The enable bit for Camera line interrupt."]
pub type CAM_HS_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CAM_HS_INT_ENA` writer - The enable bit for Camera line interrupt."]
pub type CAM_HS_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LC_DMA_INT_ENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The enable bit for LCD frame end interrupt."]
    #[inline(always)]
    pub fn lcd_vsync_int_ena(&self) -> LCD_VSYNC_INT_ENA_R {
        LCD_VSYNC_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for lcd transfer end interrupt."]
    #[inline(always)]
    pub fn lcd_trans_done_int_ena(&self) -> LCD_TRANS_DONE_INT_ENA_R {
        LCD_TRANS_DONE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for Camera frame end interrupt."]
    #[inline(always)]
    pub fn cam_vsync_int_ena(&self) -> CAM_VSYNC_INT_ENA_R {
        CAM_VSYNC_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for Camera line interrupt."]
    #[inline(always)]
    pub fn cam_hs_int_ena(&self) -> CAM_HS_INT_ENA_R {
        CAM_HS_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for LCD frame end interrupt."]
    #[inline(always)]
    pub fn lcd_vsync_int_ena(&mut self) -> LCD_VSYNC_INT_ENA_W<0> {
        LCD_VSYNC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The enable bit for lcd transfer end interrupt."]
    #[inline(always)]
    pub fn lcd_trans_done_int_ena(&mut self) -> LCD_TRANS_DONE_INT_ENA_W<1> {
        LCD_TRANS_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The enable bit for Camera frame end interrupt."]
    #[inline(always)]
    pub fn cam_vsync_int_ena(&mut self) -> CAM_VSYNC_INT_ENA_W<2> {
        CAM_VSYNC_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The enable bit for Camera line interrupt."]
    #[inline(always)]
    pub fn cam_hs_int_ena(&mut self) -> CAM_HS_INT_ENA_W<3> {
        CAM_HS_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_camera DMA inturrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_dma_int_ena](index.html) module"]
pub struct LC_DMA_INT_ENA_SPEC;
impl crate::RegisterSpec for LC_DMA_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_dma_int_ena::R](R) reader structure"]
impl crate::Readable for LC_DMA_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lc_dma_int_ena::W](W) writer structure"]
impl crate::Writable for LC_DMA_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LC_DMA_INT_ENA to value 0"]
impl crate::Resettable for LC_DMA_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
