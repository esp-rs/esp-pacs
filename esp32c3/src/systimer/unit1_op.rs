#[doc = "Register `UNIT1_OP` reader"]
pub type R = crate::R<UNIT1_OP_SPEC>;
#[doc = "Register `UNIT1_OP` writer"]
pub type W = crate::W<UNIT1_OP_SPEC>;
#[doc = "Field `TIMER_UNIT1_VALUE_VALID` reader - timer value is sync and valid"]
pub type TIMER_UNIT1_VALUE_VALID_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_UPDATE` writer - update timer unit1"]
pub type TIMER_UNIT1_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29 - timer value is sync and valid"]
    #[inline(always)]
    pub fn timer_unit1_value_valid(&self) -> TIMER_UNIT1_VALUE_VALID_R {
        TIMER_UNIT1_VALUE_VALID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT1_OP")
            .field(
                "timer_unit1_value_valid",
                &format_args!("{}", self.timer_unit1_value_valid().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UNIT1_OP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 30 - update timer unit1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit1_update(&mut self) -> TIMER_UNIT1_UPDATE_W<UNIT1_OP_SPEC> {
        TIMER_UNIT1_UPDATE_W::new(self, 30)
    }
}
#[doc = "SYSTIMER_UNIT1_OP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit1_op::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit1_op::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT1_OP_SPEC;
impl crate::RegisterSpec for UNIT1_OP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unit1_op::R`](R) reader structure"]
impl crate::Readable for UNIT1_OP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unit1_op::W`](W) writer structure"]
impl crate::Writable for UNIT1_OP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UNIT1_OP to value 0"]
impl crate::Resettable for UNIT1_OP_SPEC {
    const RESET_VALUE: u32 = 0;
}
