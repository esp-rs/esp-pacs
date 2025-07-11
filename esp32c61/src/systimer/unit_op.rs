#[doc = "Register `UNIT%s_OP` reader"]
pub type R = crate::R<UNIT_OP_SPEC>;
#[doc = "Register `UNIT%s_OP` writer"]
pub type W = crate::W<UNIT_OP_SPEC>;
#[doc = "Field `VALUE_VALID` reader - Represents UNIT0 value is synchronized and valid."]
pub type VALUE_VALID_R = crate::BitReader;
#[doc = "Field `UPDATE` writer - Configures whether or not to update timer UNIT0, i.e., reads the UNIT0 count value to SYSTIMER_TIMER_UNIT0_VALUE_HI and SYSTIMER_TIMER_UNIT0_VALUE_LO. \\\\ 0: No effect\\\\ 1: Update timer UNIT0 \\\\"]
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29 - Represents UNIT0 value is synchronized and valid."]
    #[inline(always)]
    pub fn value_valid(&self) -> VALUE_VALID_R {
        VALUE_VALID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT_OP")
            .field("value_valid", &self.value_valid())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Configures whether or not to update timer UNIT0, i.e., reads the UNIT0 count value to SYSTIMER_TIMER_UNIT0_VALUE_HI and SYSTIMER_TIMER_UNIT0_VALUE_LO. \\\\ 0: No effect\\\\ 1: Update timer UNIT0 \\\\"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<UNIT_OP_SPEC> {
        UPDATE_W::new(self, 30)
    }
}
#[doc = "Read UNIT%s value to registers\n\nYou can [`read`](crate::Reg::read) this register and get [`unit_op::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit_op::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT_OP_SPEC;
impl crate::RegisterSpec for UNIT_OP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unit_op::R`](R) reader structure"]
impl crate::Readable for UNIT_OP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unit_op::W`](W) writer structure"]
impl crate::Writable for UNIT_OP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNIT%s_OP to value 0"]
impl crate::Resettable for UNIT_OP_SPEC {}
