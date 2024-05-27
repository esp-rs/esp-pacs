#[doc = "Register `ATTEN1` reader"]
pub type R = crate::R<ATTEN1_SPEC>;
#[doc = "Register `ATTEN1` writer"]
pub type W = crate::W<ATTEN1_SPEC>;
#[doc = "Field `SAR1_ATTEN` reader - 2-bit attenuation for each pad."]
pub type SAR1_ATTEN_R = crate::FieldReader<u32>;
#[doc = "Field `SAR1_ATTEN` writer - 2-bit attenuation for each pad."]
pub type SAR1_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad."]
    #[inline(always)]
    pub fn sar1_atten(&self) -> SAR1_ATTEN_R {
        SAR1_ATTEN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATTEN1")
            .field("sar1_atten", &self.sar1_atten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - 2-bit attenuation for each pad."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_atten(&mut self) -> SAR1_ATTEN_W<ATTEN1_SPEC> {
        SAR1_ATTEN_W::new(self, 0)
    }
}
#[doc = "ADC1 attenuation registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atten1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atten1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATTEN1_SPEC;
impl crate::RegisterSpec for ATTEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atten1::R`](R) reader structure"]
impl crate::Readable for ATTEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atten1::W`](W) writer structure"]
impl crate::Writable for ATTEN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATTEN1 to value 0xffff_ffff"]
impl crate::Resettable for ATTEN1_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
