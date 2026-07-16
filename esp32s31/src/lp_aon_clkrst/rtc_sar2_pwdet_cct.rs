#[doc = "Register `RTC_SAR2_PWDET_CCT` reader"]
pub type R = crate::R<RTC_SAR2_PWDET_CCT_SPEC>;
#[doc = "Register `RTC_SAR2_PWDET_CCT` writer"]
pub type W = crate::W<RTC_SAR2_PWDET_CCT_SPEC>;
#[doc = "Field `RTC_SAR2_PWDET_CCT` reader - rtc_sar2_pwdet_cct value"]
pub type RTC_SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `RTC_SAR2_PWDET_CCT` writer - rtc_sar2_pwdet_cct value"]
pub type RTC_SAR2_PWDET_CCT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - rtc_sar2_pwdet_cct value"]
    #[inline(always)]
    pub fn rtc_sar2_pwdet_cct(&self) -> RTC_SAR2_PWDET_CCT_R {
        RTC_SAR2_PWDET_CCT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_SAR2_PWDET_CCT")
            .field("rtc_sar2_pwdet_cct", &self.rtc_sar2_pwdet_cct())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - rtc_sar2_pwdet_cct value"]
    #[inline(always)]
    pub fn rtc_sar2_pwdet_cct(&mut self) -> RTC_SAR2_PWDET_CCT_W<'_, RTC_SAR2_PWDET_CCT_SPEC> {
        RTC_SAR2_PWDET_CCT_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_sar2_pwdet_cct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_sar2_pwdet_cct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_SAR2_PWDET_CCT_SPEC;
impl crate::RegisterSpec for RTC_SAR2_PWDET_CCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_sar2_pwdet_cct::R`](R) reader structure"]
impl crate::Readable for RTC_SAR2_PWDET_CCT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_sar2_pwdet_cct::W`](W) writer structure"]
impl crate::Writable for RTC_SAR2_PWDET_CCT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_SAR2_PWDET_CCT to value 0"]
impl crate::Resettable for RTC_SAR2_PWDET_CCT_SPEC {}
