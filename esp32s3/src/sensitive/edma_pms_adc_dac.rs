#[doc = "Register `EDMA_PMS_ADC_DAC` reader"]
pub type R = crate::R<EDMA_PMS_ADC_DAC_SPEC>;
#[doc = "Register `EDMA_PMS_ADC_DAC` writer"]
pub type W = crate::W<EDMA_PMS_ADC_DAC_SPEC>;
#[doc = "Field `ATTR1` reader - This field is used to configure the permission of ADC/DAC accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR1_R = crate::FieldReader;
#[doc = "Field `ATTR1` writer - This field is used to configure the permission of ADC/DAC accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ATTR2` reader - This field is used to configure the permission of ADC/DAC accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR2_R = crate::FieldReader;
#[doc = "Field `ATTR2` writer - This field is used to configure the permission of ADC/DAC accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - This field is used to configure the permission of ADC/DAC accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr1(&self) -> ATTR1_R {
        ATTR1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - This field is used to configure the permission of ADC/DAC accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr2(&self) -> ATTR2_R {
        ATTR2_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDMA_PMS_ADC_DAC")
            .field("attr1", &self.attr1())
            .field("attr2", &self.attr2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to configure the permission of ADC/DAC accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    #[must_use]
    pub fn attr1(&mut self) -> ATTR1_W<EDMA_PMS_ADC_DAC_SPEC> {
        ATTR1_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - This field is used to configure the permission of ADC/DAC accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    #[must_use]
    pub fn attr2(&mut self) -> ATTR2_W<EDMA_PMS_ADC_DAC_SPEC> {
        ATTR2_W::new(self, 2)
    }
}
#[doc = "EDMA-ADC/DAC permission control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`edma_pms_adc_dac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edma_pms_adc_dac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EDMA_PMS_ADC_DAC_SPEC;
impl crate::RegisterSpec for EDMA_PMS_ADC_DAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edma_pms_adc_dac::R`](R) reader structure"]
impl crate::Readable for EDMA_PMS_ADC_DAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`edma_pms_adc_dac::W`](W) writer structure"]
impl crate::Writable for EDMA_PMS_ADC_DAC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EDMA_PMS_ADC_DAC to value 0x0f"]
impl crate::Resettable for EDMA_PMS_ADC_DAC_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
