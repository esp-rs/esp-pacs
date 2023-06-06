#[doc = "Register `UNIT1_LOAD_LO` reader"]
pub struct R(crate::R<UNIT1_LOAD_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNIT1_LOAD_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNIT1_LOAD_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNIT1_LOAD_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNIT1_LOAD_LO` writer"]
pub struct W(crate::W<UNIT1_LOAD_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNIT1_LOAD_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UNIT1_LOAD_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNIT1_LOAD_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_UNIT1_LOAD_LO` reader - timer unit1 load low 32 bits"]
pub type TIMER_UNIT1_LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_UNIT1_LOAD_LO` writer - timer unit1 load low 32 bits"]
pub type TIMER_UNIT1_LOAD_LO_W<'a, const O: u8> =
    crate::FieldWriter<'a, UNIT1_LOAD_LO_SPEC, 32, O, u32>;
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
    pub fn timer_unit1_load_lo(&mut self) -> TIMER_UNIT1_LOAD_LO_W<0> {
        TIMER_UNIT1_LOAD_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system timer unit1 value low load register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unit1_load_lo](index.html) module"]
pub struct UNIT1_LOAD_LO_SPEC;
impl crate::RegisterSpec for UNIT1_LOAD_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unit1_load_lo::R](R) reader structure"]
impl crate::Readable for UNIT1_LOAD_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [unit1_load_lo::W](W) writer structure"]
impl crate::Writable for UNIT1_LOAD_LO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UNIT1_LOAD_LO to value 0"]
impl crate::Resettable for UNIT1_LOAD_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
