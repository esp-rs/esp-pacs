#[doc = "Register `ADC_INV_PHASE_CONF` reader"]
pub type R = crate::R<ADC_INV_PHASE_CONF_SPEC>;
#[doc = "Register `ADC_INV_PHASE_CONF` writer"]
pub type W = crate::W<ADC_INV_PHASE_CONF_SPEC>;
#[doc = "Field `CLK_ADC_INV_PHASE_ENA` reader - xxxx"]
pub type CLK_ADC_INV_PHASE_ENA_R = crate::BitReader;
#[doc = "Field `CLK_ADC_INV_PHASE_ENA` writer - xxxx"]
pub type CLK_ADC_INV_PHASE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn clk_adc_inv_phase_ena(&self) -> CLK_ADC_INV_PHASE_ENA_R {
        CLK_ADC_INV_PHASE_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_INV_PHASE_CONF")
            .field(
                "clk_adc_inv_phase_ena",
                &format_args!("{}", self.clk_adc_inv_phase_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ADC_INV_PHASE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn clk_adc_inv_phase_ena(&mut self) -> CLK_ADC_INV_PHASE_ENA_W<ADC_INV_PHASE_CONF_SPEC> {
        CLK_ADC_INV_PHASE_ENA_W::new(self, 0)
    }
}
#[doc = "xxxx\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_inv_phase_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_inv_phase_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_INV_PHASE_CONF_SPEC;
impl crate::RegisterSpec for ADC_INV_PHASE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_inv_phase_conf::R`](R) reader structure"]
impl crate::Readable for ADC_INV_PHASE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_inv_phase_conf::W`](W) writer structure"]
impl crate::Writable for ADC_INV_PHASE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_INV_PHASE_CONF to value 0"]
impl crate::Resettable for ADC_INV_PHASE_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
