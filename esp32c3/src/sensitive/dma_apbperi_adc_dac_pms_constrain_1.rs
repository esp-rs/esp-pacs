#[doc = "Register `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0` reader - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0` writer - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1` reader - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1` writer - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2` reader - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2` writer - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3` reader - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3` writer - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0` reader - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0` writer - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1` reader - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1` writer - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2` reader - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2` writer - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3` reader - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R = crate::FieldReader;
#[doc = "Field `DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3` writer - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3"]
pub type DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0"]
    #[inline(always)]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0(
        &self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1"]
    #[inline(always)]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1(
        &self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2"]
    #[inline(always)]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2(
        &self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3"]
    #[inline(always)]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3(
        &self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 12:13 - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0"]
    #[inline(always)]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0(
        &self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1"]
    #[inline(always)]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1(
        &self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2"]
    #[inline(always)]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2(
        &self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3"]
    #[inline(always)]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3(
        &self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_R::new(((self.bits >> 18) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1")
            .field(
                "dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0",
                &format_args!(
                    "{}",
                    self.dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0()
                        .bits()
                ),
            )
            .field(
                "dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1",
                &format_args!(
                    "{}",
                    self.dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1()
                        .bits()
                ),
            )
            .field(
                "dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2",
                &format_args!(
                    "{}",
                    self.dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2()
                        .bits()
                ),
            )
            .field(
                "dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3",
                &format_args!(
                    "{}",
                    self.dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3()
                        .bits()
                ),
            )
            .field(
                "dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0",
                &format_args!(
                    "{}",
                    self.dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0()
                        .bits()
                ),
            )
            .field(
                "dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1",
                &format_args!(
                    "{}",
                    self.dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1()
                        .bits()
                ),
            )
            .field(
                "dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2",
                &format_args!(
                    "{}",
                    self.dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2()
                        .bits()
                ),
            )
            .field(
                "dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3",
                &format_args!(
                    "{}",
                    self.dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3()
                        .bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_0(
        &mut self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W<0> {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_0_W::new(self)
    }
    #[doc = "Bits 2:3 - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_1(
        &mut self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W<2> {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_1_W::new(self)
    }
    #[doc = "Bits 4:5 - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_2(
        &mut self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W<4> {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_2_W::new(self)
    }
    #[doc = "Bits 6:7 - dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_0_pms_3(
        &mut self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W<6> {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_0_PMS_3_W::new(self)
    }
    #[doc = "Bits 12:13 - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_0(
        &mut self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W<12> {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_0_W::new(self)
    }
    #[doc = "Bits 14:15 - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_1(
        &mut self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W<14> {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_1_W::new(self)
    }
    #[doc = "Bits 16:17 - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_2(
        &mut self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W<16> {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_2_W::new(self)
    }
    #[doc = "Bits 18:19 - dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_adc_dac_pms_constrain_sram_world_1_pms_3(
        &mut self,
    ) -> DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W<18> {
        DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_SRAM_WORLD_1_PMS_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apbperi_adc_dac_pms_constrain_1](index.html) module"]
pub struct DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apbperi_adc_dac_pms_constrain_1::R](R) reader structure"]
impl crate::Readable for DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_apbperi_adc_dac_pms_constrain_1::W](W) writer structure"]
impl crate::Writable for DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1 to value 0x000f_f0ff"]
impl crate::Resettable for DMA_APBPERI_ADC_DAC_PMS_CONSTRAIN_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_f0ff;
}
