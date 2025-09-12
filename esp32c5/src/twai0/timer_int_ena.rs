#[doc = "Register `TIMER_INT_ENA` reader"]
pub type R = crate::R<TIMER_INT_ENA_SPEC>;
#[doc = "Register `TIMER_INT_ENA` writer"]
pub type W = crate::W<TIMER_INT_ENA_SPEC>;
#[doc = "Field `TIMER_OVERFLOW_INT_ENA` reader - The enable signal for read_done interrupt."]
pub type TIMER_OVERFLOW_INT_ENA_R = crate::BitReader;
#[doc = "Field `TIMER_OVERFLOW_INT_ENA` writer - The enable signal for read_done interrupt."]
pub type TIMER_OVERFLOW_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The enable signal for read_done interrupt."]
    #[inline(always)]
    pub fn timer_overflow_int_ena(&self) -> TIMER_OVERFLOW_INT_ENA_R {
        TIMER_OVERFLOW_INT_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_INT_ENA")
            .field("timer_overflow_int_ena", &self.timer_overflow_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The enable signal for read_done interrupt."]
    #[inline(always)]
    pub fn timer_overflow_int_ena(&mut self) -> TIMER_OVERFLOW_INT_ENA_W<'_, TIMER_INT_ENA_SPEC> {
        TIMER_OVERFLOW_INT_ENA_W::new(self, 0)
    }
}
#[doc = "TWAIFD interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_INT_ENA_SPEC;
impl crate::RegisterSpec for TIMER_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_int_ena::R`](R) reader structure"]
impl crate::Readable for TIMER_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_int_ena::W`](W) writer structure"]
impl crate::Writable for TIMER_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_INT_ENA to value 0"]
impl crate::Resettable for TIMER_INT_ENA_SPEC {}
