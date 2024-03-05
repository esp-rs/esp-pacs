#[doc = "Register `TIMER2` reader"]
pub type R = crate::R<TIMER2_SPEC>;
#[doc = "Register `TIMER2` writer"]
pub type W = crate::W<TIMER2_SPEC>;
#[doc = "Field `MIN_TIME_CK8M_OFF` reader - minimal cycles in slow_clk_rtc for CK8M in power down state"]
pub type MIN_TIME_CK8M_OFF_R = crate::FieldReader;
#[doc = "Field `MIN_TIME_CK8M_OFF` writer - minimal cycles in slow_clk_rtc for CK8M in power down state"]
pub type MIN_TIME_CK8M_OFF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 24:31 - minimal cycles in slow_clk_rtc for CK8M in power down state"]
    #[inline(always)]
    pub fn min_time_ck8m_off(&self) -> MIN_TIME_CK8M_OFF_R {
        MIN_TIME_CK8M_OFF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER2")
            .field(
                "min_time_ck8m_off",
                &format_args!("{}", self.min_time_ck8m_off().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 24:31 - minimal cycles in slow_clk_rtc for CK8M in power down state"]
    #[inline(always)]
    #[must_use]
    pub fn min_time_ck8m_off(&mut self) -> MIN_TIME_CK8M_OFF_W<TIMER2_SPEC> {
        MIN_TIME_CK8M_OFF_W::new(self, 24)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER2_SPEC;
impl crate::RegisterSpec for TIMER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2::R`](R) reader structure"]
impl crate::Readable for TIMER2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer2::W`](W) writer structure"]
impl crate::Writable for TIMER2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2 to value 0x0100_0000"]
impl crate::Resettable for TIMER2_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
