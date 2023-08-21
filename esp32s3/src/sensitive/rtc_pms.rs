#[doc = "Register `RTC_PMS` reader"]
pub type R = crate::R<RTC_PMS_SPEC>;
#[doc = "Register `RTC_PMS` writer"]
pub type W = crate::W<RTC_PMS_SPEC>;
#[doc = "Field `DIS_RTC_CPU` reader - Set 1 to disable rtc coprocessor."]
pub type DIS_RTC_CPU_R = crate::BitReader;
#[doc = "Field `DIS_RTC_CPU` writer - Set 1 to disable rtc coprocessor."]
pub type DIS_RTC_CPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to disable rtc coprocessor."]
    #[inline(always)]
    pub fn dis_rtc_cpu(&self) -> DIS_RTC_CPU_R {
        DIS_RTC_CPU_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_PMS")
            .field("dis_rtc_cpu", &format_args!("{}", self.dis_rtc_cpu().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_PMS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to disable rtc coprocessor."]
    #[inline(always)]
    #[must_use]
    pub fn dis_rtc_cpu(&mut self) -> DIS_RTC_CPU_W<RTC_PMS_SPEC, 0> {
        DIS_RTC_CPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC coprocessor permission register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_pms::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pms::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_PMS_SPEC;
impl crate::RegisterSpec for RTC_PMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_pms::R`](R) reader structure"]
impl crate::Readable for RTC_PMS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_pms::W`](W) writer structure"]
impl crate::Writable for RTC_PMS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_PMS to value 0"]
impl crate::Resettable for RTC_PMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
