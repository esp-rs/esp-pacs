#[doc = "Register `DMA_CONF` reader"]
pub struct R(crate::R<DMA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CONF` writer"]
pub struct W(crate::W<DMA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CONF_SPEC>;
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
impl From<crate::W<DMA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_ADC_EOF_NUM` reader - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
pub type APB_ADC_EOF_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `APB_ADC_EOF_NUM` writer - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
pub type APB_ADC_EOF_NUM_W<'a> = crate::FieldWriter<'a, u32, DMA_CONF_SPEC, u16, u16, 16, 0>;
#[doc = "Field `APB_ADC_RESET_FSM` reader - reset_apb_adc_state"]
pub type APB_ADC_RESET_FSM_R = crate::BitReader<bool>;
#[doc = "Field `APB_ADC_RESET_FSM` writer - reset_apb_adc_state"]
pub type APB_ADC_RESET_FSM_W<'a> = crate::BitWriter<'a, u32, DMA_CONF_SPEC, bool, 30>;
#[doc = "Field `APB_ADC_TRANS` reader - enable apb_adc use spi_dma"]
pub type APB_ADC_TRANS_R = crate::BitReader<bool>;
#[doc = "Field `APB_ADC_TRANS` writer - enable apb_adc use spi_dma"]
pub type APB_ADC_TRANS_W<'a> = crate::BitWriter<'a, u32, DMA_CONF_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:15 - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
    #[inline(always)]
    pub fn apb_adc_eof_num(&self) -> APB_ADC_EOF_NUM_R {
        APB_ADC_EOF_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - reset_apb_adc_state"]
    #[inline(always)]
    pub fn apb_adc_reset_fsm(&self) -> APB_ADC_RESET_FSM_R {
        APB_ADC_RESET_FSM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable apb_adc use spi_dma"]
    #[inline(always)]
    pub fn apb_adc_trans(&self) -> APB_ADC_TRANS_R {
        APB_ADC_TRANS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
    #[inline(always)]
    pub fn apb_adc_eof_num(&mut self) -> APB_ADC_EOF_NUM_W {
        APB_ADC_EOF_NUM_W::new(self)
    }
    #[doc = "Bit 30 - reset_apb_adc_state"]
    #[inline(always)]
    pub fn apb_adc_reset_fsm(&mut self) -> APB_ADC_RESET_FSM_W {
        APB_ADC_RESET_FSM_W::new(self)
    }
    #[doc = "Bit 31 - enable apb_adc use spi_dma"]
    #[inline(always)]
    pub fn apb_adc_trans(&mut self) -> APB_ADC_TRANS_W {
        APB_ADC_TRANS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_conf](index.html) module"]
pub struct DMA_CONF_SPEC;
impl crate::RegisterSpec for DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_conf::R](R) reader structure"]
impl crate::Readable for DMA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_conf::W](W) writer structure"]
impl crate::Writable for DMA_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CONF to value 0xff"]
impl crate::Resettable for DMA_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
