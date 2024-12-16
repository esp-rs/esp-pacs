#[doc = "Register `APB_ADC_INTR_MAP` reader"]
pub type R = crate::R<APB_ADC_INTR_MAP_SPEC>;
#[doc = "Register `APB_ADC_INTR_MAP` writer"]
pub type W = crate::W<APB_ADC_INTR_MAP_SPEC>;
#[doc = "Field `APB_ADC_INTR_MAP` reader - Need add description"]
pub type APB_ADC_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `APB_ADC_INTR_MAP` writer - Need add description"]
pub type APB_ADC_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn apb_adc_intr_map(&self) -> APB_ADC_INTR_MAP_R {
        APB_ADC_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_ADC_INTR_MAP")
            .field("apb_adc_intr_map", &self.apb_adc_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn apb_adc_intr_map(&mut self) -> APB_ADC_INTR_MAP_W<APB_ADC_INTR_MAP_SPEC> {
        APB_ADC_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_adc_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_adc_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_ADC_INTR_MAP_SPEC;
impl crate::RegisterSpec for APB_ADC_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_adc_intr_map::R`](R) reader structure"]
impl crate::Readable for APB_ADC_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_adc_intr_map::W`](W) writer structure"]
impl crate::Writable for APB_ADC_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_ADC_INTR_MAP to value 0"]
impl crate::Resettable for APB_ADC_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
