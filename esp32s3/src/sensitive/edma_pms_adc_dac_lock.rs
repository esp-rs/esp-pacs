#[doc = "Register `EDMA_PMS_ADC_DAC_LOCK` reader"]
pub struct R(crate::R<EDMA_PMS_ADC_DAC_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDMA_PMS_ADC_DAC_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDMA_PMS_ADC_DAC_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDMA_PMS_ADC_DAC_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDMA_PMS_ADC_DAC_LOCK` writer"]
pub struct W(crate::W<EDMA_PMS_ADC_DAC_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDMA_PMS_ADC_DAC_LOCK_SPEC>;
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
impl From<crate::W<EDMA_PMS_ADC_DAC_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDMA_PMS_ADC_DAC_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDMA_PMS_ADC_DAC_LOCK` reader - Set 1 to lock EDMA-ADC/DAC permission control registers."]
pub type EDMA_PMS_ADC_DAC_LOCK_R = crate::BitReader;
#[doc = "Field `EDMA_PMS_ADC_DAC_LOCK` writer - Set 1 to lock EDMA-ADC/DAC permission control registers."]
pub type EDMA_PMS_ADC_DAC_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, EDMA_PMS_ADC_DAC_LOCK_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to lock EDMA-ADC/DAC permission control registers."]
    #[inline(always)]
    pub fn edma_pms_adc_dac_lock(&self) -> EDMA_PMS_ADC_DAC_LOCK_R {
        EDMA_PMS_ADC_DAC_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDMA_PMS_ADC_DAC_LOCK")
            .field(
                "edma_pms_adc_dac_lock",
                &format_args!("{}", self.edma_pms_adc_dac_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EDMA_PMS_ADC_DAC_LOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to lock EDMA-ADC/DAC permission control registers."]
    #[inline(always)]
    #[must_use]
    pub fn edma_pms_adc_dac_lock(&mut self) -> EDMA_PMS_ADC_DAC_LOCK_W<0> {
        EDMA_PMS_ADC_DAC_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMA-ADC/DAC permission lock register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edma_pms_adc_dac_lock](index.html) module"]
pub struct EDMA_PMS_ADC_DAC_LOCK_SPEC;
impl crate::RegisterSpec for EDMA_PMS_ADC_DAC_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edma_pms_adc_dac_lock::R](R) reader structure"]
impl crate::Readable for EDMA_PMS_ADC_DAC_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edma_pms_adc_dac_lock::W](W) writer structure"]
impl crate::Writable for EDMA_PMS_ADC_DAC_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDMA_PMS_ADC_DAC_LOCK to value 0"]
impl crate::Resettable for EDMA_PMS_ADC_DAC_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
