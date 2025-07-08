#[doc = "Register `UNIT1_OP` reader"]
pub type R = crate::R<UNIT1_OP_SPEC>;
#[doc = "Register `UNIT1_OP` writer"]
pub type W = crate::W<UNIT1_OP_SPEC>;
#[doc = "Field `TIMER_UNIT1_VALUE_VALID` reader - Represents UNIT1 value is synchronized and valid."]
pub type TIMER_UNIT1_VALUE_VALID_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_UPDATE` writer - Configures whether or not to update timer UNIT1, i.e., reads the UNIT1 count value to SYSTIMER_TIMER_UNIT1_VALUE_HI and SYSTIMER_TIMER_UNIT1_VALUE_LO. \\\\ 0: No effect \\\\ 1: Update timer UNIT1\\\\"]
pub type TIMER_UNIT1_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29 - Represents UNIT1 value is synchronized and valid."]
    #[inline(always)]
    pub fn timer_unit1_value_valid(&self) -> TIMER_UNIT1_VALUE_VALID_R {
        TIMER_UNIT1_VALUE_VALID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT1_OP")
            .field("timer_unit1_value_valid", &self.timer_unit1_value_valid())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Configures whether or not to update timer UNIT1, i.e., reads the UNIT1 count value to SYSTIMER_TIMER_UNIT1_VALUE_HI and SYSTIMER_TIMER_UNIT1_VALUE_LO. \\\\ 0: No effect \\\\ 1: Update timer UNIT1\\\\"]
    #[inline(always)]
    pub fn timer_unit1_update(&mut self) -> TIMER_UNIT1_UPDATE_W<UNIT1_OP_SPEC> {
        TIMER_UNIT1_UPDATE_W::new(self, 30)
    }
}
#[doc = "Read UNIT1 value to registers\n\nYou can [`read`](crate::Reg::read) this register and get [`unit1_op::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit1_op::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT1_OP_SPEC;
impl crate::RegisterSpec for UNIT1_OP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unit1_op::R`](R) reader structure"]
impl crate::Readable for UNIT1_OP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unit1_op::W`](W) writer structure"]
impl crate::Writable for UNIT1_OP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNIT1_OP to value 0"]
impl crate::Resettable for UNIT1_OP_SPEC {}
