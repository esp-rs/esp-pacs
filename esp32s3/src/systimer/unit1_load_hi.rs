#[doc = "Register `UNIT1_LOAD_HI` reader"]
pub type R = crate::R<UNIT1_LOAD_HI_SPEC>;
#[doc = "Register `UNIT1_LOAD_HI` writer"]
pub type W = crate::W<UNIT1_LOAD_HI_SPEC>;
#[doc = "Field `TIMER_UNIT1_LOAD_HI` reader - timer unit1 load high 20 bits"]
pub type TIMER_UNIT1_LOAD_HI_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_UNIT1_LOAD_HI` writer - timer unit1 load high 20 bits"]
pub type TIMER_UNIT1_LOAD_HI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl R {
    #[doc = "Bits 0:19 - timer unit1 load high 20 bits"]
    #[inline(always)]
    pub fn timer_unit1_load_hi(&self) -> TIMER_UNIT1_LOAD_HI_R {
        TIMER_UNIT1_LOAD_HI_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT1_LOAD_HI")
            .field(
                "timer_unit1_load_hi",
                &format_args!("{}", self.timer_unit1_load_hi().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UNIT1_LOAD_HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - timer unit1 load high 20 bits"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit1_load_hi(&mut self) -> TIMER_UNIT1_LOAD_HI_W<UNIT1_LOAD_HI_SPEC, 0> {
        TIMER_UNIT1_LOAD_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "system timer unit1 value high load register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit1_load_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit1_load_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT1_LOAD_HI_SPEC;
impl crate::RegisterSpec for UNIT1_LOAD_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unit1_load_hi::R`](R) reader structure"]
impl crate::Readable for UNIT1_LOAD_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unit1_load_hi::W`](W) writer structure"]
impl crate::Writable for UNIT1_LOAD_HI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UNIT1_LOAD_HI to value 0"]
impl crate::Resettable for UNIT1_LOAD_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
