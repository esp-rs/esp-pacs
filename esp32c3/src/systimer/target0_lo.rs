#[doc = "Register `TARGET0_LO` reader"]
pub type R = crate::R<TARGET0_LO_SPEC>;
#[doc = "Register `TARGET0_LO` writer"]
pub type W = crate::W<TARGET0_LO_SPEC>;
#[doc = "Field `TIMER_TARGET0_LO` reader - timer taget0 low 32 bit"]
pub type TIMER_TARGET0_LO_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_TARGET0_LO` writer - timer taget0 low 32 bit"]
pub type TIMER_TARGET0_LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - timer taget0 low 32 bit"]
    #[inline(always)]
    pub fn timer_target0_lo(&self) -> TIMER_TARGET0_LO_R {
        TIMER_TARGET0_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET0_LO")
            .field(
                "timer_target0_lo",
                &format_args!("{}", self.timer_target0_lo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET0_LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - timer taget0 low 32 bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer_target0_lo(&mut self) -> TIMER_TARGET0_LO_W<TARGET0_LO_SPEC> {
        TIMER_TARGET0_LO_W::new(self, 0)
    }
}
#[doc = "SYSTIMER_TARGET0_LO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target0_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target0_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET0_LO_SPEC;
impl crate::RegisterSpec for TARGET0_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target0_lo::R`](R) reader structure"]
impl crate::Readable for TARGET0_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target0_lo::W`](W) writer structure"]
impl crate::Writable for TARGET0_LO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARGET0_LO to value 0"]
impl crate::Resettable for TARGET0_LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
