#[doc = "Register `COCPU_DISABLE` reader"]
pub type R = crate::R<COCPU_DISABLE_SPEC>;
#[doc = "Register `COCPU_DISABLE` writer"]
pub type W = crate::W<COCPU_DISABLE_SPEC>;
#[doc = "Field `DISABLE_RTC_CPU` reader - configure ulp diable"]
pub type DISABLE_RTC_CPU_R = crate::BitReader;
#[doc = "Field `DISABLE_RTC_CPU` writer - configure ulp diable"]
pub type DISABLE_RTC_CPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - configure ulp diable"]
    #[inline(always)]
    pub fn disable_rtc_cpu(&self) -> DISABLE_RTC_CPU_R {
        DISABLE_RTC_CPU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COCPU_DISABLE")
            .field(
                "disable_rtc_cpu",
                &format_args!("{}", self.disable_rtc_cpu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COCPU_DISABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - configure ulp diable"]
    #[inline(always)]
    #[must_use]
    pub fn disable_rtc_cpu(&mut self) -> DISABLE_RTC_CPU_W<COCPU_DISABLE_SPEC, 31> {
        DISABLE_RTC_CPU_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configure ulp diable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cocpu_disable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cocpu_disable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COCPU_DISABLE_SPEC;
impl crate::RegisterSpec for COCPU_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cocpu_disable::R`](R) reader structure"]
impl crate::Readable for COCPU_DISABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cocpu_disable::W`](W) writer structure"]
impl crate::Writable for COCPU_DISABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COCPU_DISABLE to value 0"]
impl crate::Resettable for COCPU_DISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
