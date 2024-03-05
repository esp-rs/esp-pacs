#[doc = "Register `TARGET2_HI` reader"]
pub type R = crate::R<TARGET2_HI_SPEC>;
#[doc = "Register `TARGET2_HI` writer"]
pub type W = crate::W<TARGET2_HI_SPEC>;
#[doc = "Field `TIMER_TARGET2_HI` reader - timer taget2 high 32 bit"]
pub type TIMER_TARGET2_HI_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_TARGET2_HI` writer - timer taget2 high 32 bit"]
pub type TIMER_TARGET2_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - timer taget2 high 32 bit"]
    #[inline(always)]
    pub fn timer_target2_hi(&self) -> TIMER_TARGET2_HI_R {
        TIMER_TARGET2_HI_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET2_HI")
            .field(
                "timer_target2_hi",
                &format_args!("{}", self.timer_target2_hi().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET2_HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - timer taget2 high 32 bit"]
    #[inline(always)]
    #[must_use]
    pub fn timer_target2_hi(&mut self) -> TIMER_TARGET2_HI_W<TARGET2_HI_SPEC> {
        TIMER_TARGET2_HI_W::new(self, 0)
    }
}
#[doc = "SYSTIMER_TARGET2_HI.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target2_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target2_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET2_HI_SPEC;
impl crate::RegisterSpec for TARGET2_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target2_hi::R`](R) reader structure"]
impl crate::Readable for TARGET2_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target2_hi::W`](W) writer structure"]
impl crate::Writable for TARGET2_HI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARGET2_HI to value 0"]
impl crate::Resettable for TARGET2_HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
