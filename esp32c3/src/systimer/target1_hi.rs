#[doc = "Register `TARGET1_HI` reader"]
pub type R = crate::R<TARGET1_HI_SPEC>;
#[doc = "Register `TARGET1_HI` writer"]
pub type W = crate::W<TARGET1_HI_SPEC>;
#[doc = "Field `TIMER_TARGET1_HI` reader - timer taget1 high 32 bit"]
pub type TIMER_TARGET1_HI_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_TARGET1_HI` writer - timer taget1 high 32 bit"]
pub type TIMER_TARGET1_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - timer taget1 high 32 bit"]
    #[inline(always)]
    pub fn timer_target1_hi(&self) -> TIMER_TARGET1_HI_R {
        TIMER_TARGET1_HI_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET1_HI")
            .field(
                "timer_target1_hi",
                &format_args!("{}", self.timer_target1_hi().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET1_HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - timer taget1 high 32 bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer_target1_hi(&mut self) -> TIMER_TARGET1_HI_W<TARGET1_HI_SPEC> {
        TIMER_TARGET1_HI_W::new(self, 0)
    }
}
#[doc = "SYSTIMER_TARGET1_HI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target1_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target1_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET1_HI_SPEC;
impl crate::RegisterSpec for TARGET1_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target1_hi::R`](R) reader structure"]
impl crate::Readable for TARGET1_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target1_hi::W`](W) writer structure"]
impl crate::Writable for TARGET1_HI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARGET1_HI to value 0"]
impl crate::Resettable for TARGET1_HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
