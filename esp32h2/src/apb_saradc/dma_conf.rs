#[doc = "Register `DMA_CONF` reader"]
pub type R = crate::R<DMA_CONF_SPEC>;
#[doc = "Register `DMA_CONF` writer"]
pub type W = crate::W<DMA_CONF_SPEC>;
#[doc = "Field `APB_ADC_EOF_NUM` reader - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
pub type APB_ADC_EOF_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `APB_ADC_EOF_NUM` writer - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
pub type APB_ADC_EOF_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `APB_ADC_RESET_FSM` reader - reset_apb_adc_state"]
pub type APB_ADC_RESET_FSM_R = crate::BitReader;
#[doc = "Field `APB_ADC_RESET_FSM` writer - reset_apb_adc_state"]
pub type APB_ADC_RESET_FSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_ADC_TRANS` reader - enable apb_adc use spi_dma"]
pub type APB_ADC_TRANS_R = crate::BitReader;
#[doc = "Field `APB_ADC_TRANS` writer - enable apb_adc use spi_dma"]
pub type APB_ADC_TRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CONF")
            .field(
                "apb_adc_eof_num",
                &format_args!("{}", self.apb_adc_eof_num().bits()),
            )
            .field(
                "apb_adc_reset_fsm",
                &format_args!("{}", self.apb_adc_reset_fsm().bit()),
            )
            .field(
                "apb_adc_trans",
                &format_args!("{}", self.apb_adc_trans().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - the dma_in_suc_eof gen when sample cnt = spi_eof_num"]
    #[inline(always)]
    #[must_use]
    pub fn apb_adc_eof_num(&mut self) -> APB_ADC_EOF_NUM_W<DMA_CONF_SPEC> {
        APB_ADC_EOF_NUM_W::new(self, 0)
    }
    #[doc = "Bit 30 - reset_apb_adc_state"]
    #[inline(always)]
    #[must_use]
    pub fn apb_adc_reset_fsm(&mut self) -> APB_ADC_RESET_FSM_W<DMA_CONF_SPEC> {
        APB_ADC_RESET_FSM_W::new(self, 30)
    }
    #[doc = "Bit 31 - enable apb_adc use spi_dma"]
    #[inline(always)]
    #[must_use]
    pub fn apb_adc_trans(&mut self) -> APB_ADC_TRANS_W<DMA_CONF_SPEC> {
        APB_ADC_TRANS_W::new(self, 31)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CONF_SPEC;
impl crate::RegisterSpec for DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_conf::R`](R) reader structure"]
impl crate::Readable for DMA_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_conf::W`](W) writer structure"]
impl crate::Writable for DMA_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CONF to value 0xff"]
impl crate::Resettable for DMA_CONF_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
