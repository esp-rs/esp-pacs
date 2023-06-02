#[doc = "Register `ADC_INV_PHASE_CONF` reader"]
pub struct R(crate::R<ADC_INV_PHASE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_INV_PHASE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_INV_PHASE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_INV_PHASE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_INV_PHASE_CONF` writer"]
pub struct W(crate::W<ADC_INV_PHASE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_INV_PHASE_CONF_SPEC>;
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
impl From<crate::W<ADC_INV_PHASE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_INV_PHASE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_ADC_INV_PHASE_ENA` reader - xxxx"]
pub type CLK_ADC_INV_PHASE_ENA_R = crate::BitReader;
#[doc = "Field `CLK_ADC_INV_PHASE_ENA` writer - xxxx"]
pub type CLK_ADC_INV_PHASE_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, ADC_INV_PHASE_CONF_SPEC, O>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn clk_adc_inv_phase_ena(&mut self) -> CLK_ADC_INV_PHASE_ENA_W<0> {
        CLK_ADC_INV_PHASE_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xxxx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_inv_phase_conf](index.html) module"]
pub struct ADC_INV_PHASE_CONF_SPEC;
impl crate::RegisterSpec for ADC_INV_PHASE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_inv_phase_conf::R](R) reader structure"]
impl crate::Readable for ADC_INV_PHASE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_inv_phase_conf::W](W) writer structure"]
impl crate::Writable for ADC_INV_PHASE_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_INV_PHASE_CONF to value 0"]
impl crate::Resettable for ADC_INV_PHASE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
