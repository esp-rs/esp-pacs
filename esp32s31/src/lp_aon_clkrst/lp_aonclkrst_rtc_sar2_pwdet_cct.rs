#[doc = "Register `LP_AONCLKRST_RTC_SAR2_PWDET_CCT` reader"]
pub type R = crate::R<LP_AONCLKRST_RTC_SAR2_PWDET_CCT_SPEC>;
#[doc = "Register `LP_AONCLKRST_RTC_SAR2_PWDET_CCT` writer"]
pub type W = crate::W<LP_AONCLKRST_RTC_SAR2_PWDET_CCT_SPEC>;
#[doc = "Field `LP_AONCLKRST_RTC_SAR2_PWDET_CCT` reader - rtc_sar2_pwdet_cct value"]
pub type LP_AONCLKRST_RTC_SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_RTC_SAR2_PWDET_CCT` writer - rtc_sar2_pwdet_cct value"]
pub type LP_AONCLKRST_RTC_SAR2_PWDET_CCT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - rtc_sar2_pwdet_cct value"]
    #[inline(always)]
    pub fn lp_aonclkrst_rtc_sar2_pwdet_cct(&self) -> LP_AONCLKRST_RTC_SAR2_PWDET_CCT_R {
        LP_AONCLKRST_RTC_SAR2_PWDET_CCT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_RTC_SAR2_PWDET_CCT")
            .field(
                "lp_aonclkrst_rtc_sar2_pwdet_cct",
                &self.lp_aonclkrst_rtc_sar2_pwdet_cct(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - rtc_sar2_pwdet_cct value"]
    #[inline(always)]
    pub fn lp_aonclkrst_rtc_sar2_pwdet_cct(
        &mut self,
    ) -> LP_AONCLKRST_RTC_SAR2_PWDET_CCT_W<'_, LP_AONCLKRST_RTC_SAR2_PWDET_CCT_SPEC> {
        LP_AONCLKRST_RTC_SAR2_PWDET_CCT_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_rtc_sar2_pwdet_cct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_rtc_sar2_pwdet_cct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_RTC_SAR2_PWDET_CCT_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_RTC_SAR2_PWDET_CCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_rtc_sar2_pwdet_cct::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_RTC_SAR2_PWDET_CCT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_rtc_sar2_pwdet_cct::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_RTC_SAR2_PWDET_CCT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_RTC_SAR2_PWDET_CCT to value 0"]
impl crate::Resettable for LP_AONCLKRST_RTC_SAR2_PWDET_CCT_SPEC {}
