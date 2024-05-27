///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `ADC2_THRES` writer - Clear bit of APB_SARADC_ADC2_THRES_INT interrupt.
pub type ADC2_THRES_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ADC1_THRES` writer - Clear bit of APB_SARADC_ADC1_THRES_INT interrupt.
pub type ADC1_THRES_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ADC2_DONE` writer - Clear bit of APB_SARADC_ADC2_DONE_INT interrupt.
pub type ADC2_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ADC1_DONE` writer - Clear bit of APB_SARADC_ADC1_DONE_INT interrupt.
pub type ADC1_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 28 - Clear bit of APB_SARADC_ADC2_THRES_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn adc2_thres(&mut self) -> ADC2_THRES_W<INT_CLR_SPEC> {
        ADC2_THRES_W::new(self, 28)
    }
    ///Bit 29 - Clear bit of APB_SARADC_ADC1_THRES_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn adc1_thres(&mut self) -> ADC1_THRES_W<INT_CLR_SPEC> {
        ADC1_THRES_W::new(self, 29)
    }
    ///Bit 30 - Clear bit of APB_SARADC_ADC2_DONE_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn adc2_done(&mut self) -> ADC2_DONE_W<INT_CLR_SPEC> {
        ADC2_DONE_W::new(self, 30)
    }
    ///Bit 31 - Clear bit of APB_SARADC_ADC1_DONE_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn adc1_done(&mut self) -> ADC1_DONE_W<INT_CLR_SPEC> {
        ADC1_DONE_W::new(self, 31)
    }
}
/**Clear DIG ADC interrupts

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf000_0000;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
