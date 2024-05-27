#[doc = "Register `CAP_TIMER_PHASE` reader"]
pub type R = crate::R<CAP_TIMER_PHASE_SPEC>;
#[doc = "Register `CAP_TIMER_PHASE` writer"]
pub type W = crate::W<CAP_TIMER_PHASE_SPEC>;
#[doc = "Field `CAP_TIMER_PHASE` reader - "]
pub type CAP_TIMER_PHASE_R = crate::FieldReader<u32>;
#[doc = "Field `CAP_TIMER_PHASE` writer - "]
pub type CAP_TIMER_PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cap_timer_phase(&self) -> CAP_TIMER_PHASE_R {
        CAP_TIMER_PHASE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_TIMER_PHASE")
            .field("cap_timer_phase", &self.cap_timer_phase())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cap_timer_phase(&mut self) -> CAP_TIMER_PHASE_W<CAP_TIMER_PHASE_SPEC> {
        CAP_TIMER_PHASE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_phase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_phase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_TIMER_PHASE_SPEC;
impl crate::RegisterSpec for CAP_TIMER_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_timer_phase::R`](R) reader structure"]
impl crate::Readable for CAP_TIMER_PHASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cap_timer_phase::W`](W) writer structure"]
impl crate::Writable for CAP_TIMER_PHASE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP_TIMER_PHASE to value 0"]
impl crate::Resettable for CAP_TIMER_PHASE_SPEC {
    const RESET_VALUE: u32 = 0;
}
