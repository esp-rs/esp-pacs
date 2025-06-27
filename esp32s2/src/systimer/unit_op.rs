#[doc = "Register `UNIT%s_OP` reader"]
pub type R = crate::R<UNIT_OP_SPEC>;
#[doc = "Register `UNIT%s_OP` writer"]
pub type W = crate::W<UNIT_OP_SPEC>;
#[doc = "Field `VALUE_VALID` reader - Check if it is valid to read out timer value from registers. 0: Not ready to read timer value from registers; 1: Ready to read timer value from registers"]
pub type VALUE_VALID_R = crate::BitReader;
#[doc = "Field `UPDATE` writer - Update system timer value to registers."]
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Check if it is valid to read out timer value from registers. 0: Not ready to read timer value from registers; 1: Ready to read timer value from registers"]
    #[inline(always)]
    pub fn value_valid(&self) -> VALUE_VALID_R {
        VALUE_VALID_R::new(((self.bits >> 30) & 1) != 0)
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
    #[doc = "Bit 31 - Update system timer value to registers."]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<UNIT_OP_SPEC> {
        UPDATE_W::new(self, 31)
    }
}
#[doc = "Read out system timer value\n\nYou can [`read`](crate::Reg::read) this register and get [`unit_op::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit_op::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
