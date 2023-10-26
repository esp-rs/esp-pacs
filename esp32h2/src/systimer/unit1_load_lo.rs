#[doc = "Register `UNIT1_LOAD_LO` reader"]
pub type R = crate::R<UNIT1_LOAD_LO_SPEC>;
#[doc = "Register `UNIT1_LOAD_LO` writer"]
pub type W = crate::W<UNIT1_LOAD_LO_SPEC>;
#[doc = "Field `TIMER_UNIT1_LOAD_LO` reader - timer unit1 load low 32 bits"]
pub type TIMER_UNIT1_LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_UNIT1_LOAD_LO` writer - timer unit1 load low 32 bits"]
pub type TIMER_UNIT1_LOAD_LO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - timer unit1 load low 32 bits"]
    #[inline(always)]
    pub fn timer_unit1_load_lo(&self) -> TIMER_UNIT1_LOAD_LO_R {
        TIMER_UNIT1_LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT1_LOAD_LO")
            .field(
                "timer_unit1_load_lo",
                &format_args!("{}", self.timer_unit1_load_lo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UNIT1_LOAD_LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - timer unit1 load low 32 bits"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit1_load_lo(&mut self) -> TIMER_UNIT1_LOAD_LO_W<UNIT1_LOAD_LO_SPEC, 0> {
        TIMER_UNIT1_LOAD_LO_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "system timer unit1 value low load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit1_load_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit1_load_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT1_LOAD_LO_SPEC;
impl crate::RegisterSpec for UNIT1_LOAD_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unit1_load_lo::R`](R) reader structure"]
impl crate::Readable for UNIT1_LOAD_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unit1_load_lo::W`](W) writer structure"]
impl crate::Writable for UNIT1_LOAD_LO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UNIT1_LOAD_LO to value 0"]
impl crate::Resettable for UNIT1_LOAD_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
