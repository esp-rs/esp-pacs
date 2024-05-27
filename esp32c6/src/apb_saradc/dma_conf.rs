///Register `DMA_CONF` reader
pub type R = crate::R<DMA_CONF_SPEC>;
///Register `DMA_CONF` writer
pub type W = crate::W<DMA_CONF_SPEC>;
///Field `ADC_EOF_NUM` reader - the dma_in_suc_eof gen when sample cnt = spi_eof_num
pub type ADC_EOF_NUM_R = crate::FieldReader<u16>;
///Field `ADC_EOF_NUM` writer - the dma_in_suc_eof gen when sample cnt = spi_eof_num
pub type ADC_EOF_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ADC_RESET_FSM` reader - reset_apb_adc_state
pub type ADC_RESET_FSM_R = crate::BitReader;
///Field `ADC_RESET_FSM` writer - reset_apb_adc_state
pub type ADC_RESET_FSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC_TRANS` reader - enable apb_adc use spi_dma
pub type ADC_TRANS_R = crate::BitReader;
///Field `ADC_TRANS` writer - enable apb_adc use spi_dma
pub type ADC_TRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - the dma_in_suc_eof gen when sample cnt = spi_eof_num
    #[inline(always)]
    pub fn adc_eof_num(&self) -> ADC_EOF_NUM_R {
        ADC_EOF_NUM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 30 - reset_apb_adc_state
    #[inline(always)]
    pub fn adc_reset_fsm(&self) -> ADC_RESET_FSM_R {
        ADC_RESET_FSM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - enable apb_adc use spi_dma
    #[inline(always)]
    pub fn adc_trans(&self) -> ADC_TRANS_R {
        ADC_TRANS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CONF")
            .field("adc_eof_num", &self.adc_eof_num())
            .field("adc_reset_fsm", &self.adc_reset_fsm())
            .field("adc_trans", &self.adc_trans())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - the dma_in_suc_eof gen when sample cnt = spi_eof_num
    #[inline(always)]
    #[must_use]
    pub fn adc_eof_num(&mut self) -> ADC_EOF_NUM_W<DMA_CONF_SPEC> {
        ADC_EOF_NUM_W::new(self, 0)
    }
    ///Bit 30 - reset_apb_adc_state
    #[inline(always)]
    #[must_use]
    pub fn adc_reset_fsm(&mut self) -> ADC_RESET_FSM_W<DMA_CONF_SPEC> {
        ADC_RESET_FSM_W::new(self, 30)
    }
    ///Bit 31 - enable apb_adc use spi_dma
    #[inline(always)]
    #[must_use]
    pub fn adc_trans(&mut self) -> ADC_TRANS_W<DMA_CONF_SPEC> {
        ADC_TRANS_W::new(self, 31)
    }
}
/**digital saradc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_CONF_SPEC;
impl crate::RegisterSpec for DMA_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_conf::R`](R) reader structure
impl crate::Readable for DMA_CONF_SPEC {}
///`write(|w| ..)` method takes [`dma_conf::W`](W) writer structure
impl crate::Writable for DMA_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_CONF to value 0xff
impl crate::Resettable for DMA_CONF_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
