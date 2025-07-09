#[doc = "Register `LP_INT_ENA` reader"]
pub type R = crate::R<LP_INT_ENA_SPEC>;
#[doc = "Register `LP_INT_ENA` writer"]
pub type W = crate::W<LP_INT_ENA_SPEC>;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_ENA` reader - Enable the RTC main timer overflow interrupt..\\\\0 : Disable \\\\1: Enable"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_ENA_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_ENA` writer - Enable the RTC main timer overflow interrupt..\\\\0 : Disable \\\\1: Enable"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_LP_INT_ENA` reader - Enable the RTC main timer interrupt..\\\\0 : Disable \\\\1: Enable"]
pub type MAIN_TIMER_LP_INT_ENA_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_LP_INT_ENA` writer - Enable the RTC main timer interrupt..\\\\0 : Disable \\\\1: Enable"]
pub type MAIN_TIMER_LP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Enable the RTC main timer overflow interrupt..\\\\0 : Disable \\\\1: Enable"]
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_ena(&self) -> MAIN_TIMER_OVERFLOW_LP_INT_ENA_R {
        MAIN_TIMER_OVERFLOW_LP_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the RTC main timer interrupt..\\\\0 : Disable \\\\1: Enable"]
    #[inline(always)]
    pub fn main_timer_lp_int_ena(&self) -> MAIN_TIMER_LP_INT_ENA_R {
        MAIN_TIMER_LP_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ENA")
            .field(
                "main_timer_overflow_lp_int_ena",
                &self.main_timer_overflow_lp_int_ena(),
            )
            .field("main_timer_lp_int_ena", &self.main_timer_lp_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Enable the RTC main timer overflow interrupt..\\\\0 : Disable \\\\1: Enable"]
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_ena(
        &mut self,
    ) -> MAIN_TIMER_OVERFLOW_LP_INT_ENA_W<LP_INT_ENA_SPEC> {
        MAIN_TIMER_OVERFLOW_LP_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable the RTC main timer interrupt..\\\\0 : Disable \\\\1: Enable"]
    #[inline(always)]
    pub fn main_timer_lp_int_ena(&mut self) -> MAIN_TIMER_LP_INT_ENA_W<LP_INT_ENA_SPEC> {
        MAIN_TIMER_LP_INT_ENA_W::new(self, 31)
    }
}
#[doc = "RTC timer interrupt enable register(For ULP)\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_ENA_SPEC;
impl crate::RegisterSpec for LP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_ena::R`](R) reader structure"]
impl crate::Readable for LP_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_int_ena::W`](W) writer structure"]
impl crate::Writable for LP_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_INT_ENA to value 0"]
impl crate::Resettable for LP_INT_ENA_SPEC {}
