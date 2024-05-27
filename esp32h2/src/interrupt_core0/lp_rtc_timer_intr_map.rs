#[doc = "Register `LP_RTC_TIMER_INTR_MAP` reader"]
pub type R = crate::R<LP_RTC_TIMER_INTR_MAP_SPEC>;
#[doc = "Register `LP_RTC_TIMER_INTR_MAP` writer"]
pub type W = crate::W<LP_RTC_TIMER_INTR_MAP_SPEC>;
#[doc = "Field `LP_RTC_TIMER_INTR_MAP` reader - CORE0_LP_RTC_TIMER_INTR mapping register"]
pub type LP_RTC_TIMER_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `LP_RTC_TIMER_INTR_MAP` writer - CORE0_LP_RTC_TIMER_INTR mapping register"]
pub type LP_RTC_TIMER_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CORE0_LP_RTC_TIMER_INTR mapping register"]
    #[inline(always)]
    pub fn lp_rtc_timer_intr_map(&self) -> LP_RTC_TIMER_INTR_MAP_R {
        LP_RTC_TIMER_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_RTC_TIMER_INTR_MAP")
            .field("lp_rtc_timer_intr_map", &self.lp_rtc_timer_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_LP_RTC_TIMER_INTR mapping register"]
    #[inline(always)]
    #[must_use]
    pub fn lp_rtc_timer_intr_map(&mut self) -> LP_RTC_TIMER_INTR_MAP_W<LP_RTC_TIMER_INTR_MAP_SPEC> {
        LP_RTC_TIMER_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rtc_timer_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rtc_timer_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_RTC_TIMER_INTR_MAP_SPEC;
impl crate::RegisterSpec for LP_RTC_TIMER_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rtc_timer_intr_map::R`](R) reader structure"]
impl crate::Readable for LP_RTC_TIMER_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_rtc_timer_intr_map::W`](W) writer structure"]
impl crate::Writable for LP_RTC_TIMER_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_RTC_TIMER_INTR_MAP to value 0"]
impl crate::Resettable for LP_RTC_TIMER_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
