#[doc = "Register `CMPR0_VALUE0` reader"]
pub struct R(crate::R<CMPR0_VALUE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPR0_VALUE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPR0_VALUE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPR0_VALUE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPR0_VALUE0` writer"]
pub struct W(crate::W<CMPR0_VALUE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPR0_VALUE0_SPEC>;
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
impl From<crate::W<CMPR0_VALUE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPR0_VALUE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR0_A` reader - PWM generator 0 time stamp A's shadow register"]
pub type CMPR0_A_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMPR0_A` writer - PWM generator 0 time stamp A's shadow register"]
pub type CMPR0_A_W<'a, const O: u8> = crate::FieldWriter<'a, CMPR0_VALUE0_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp A's shadow register"]
    #[inline(always)]
    pub fn cmpr0_a(&self) -> CMPR0_A_R {
        CMPR0_A_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPR0_VALUE0")
            .field("cmpr0_a", &format_args!("{}", self.cmpr0_a().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMPR0_VALUE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp A's shadow register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_a(&mut self) -> CMPR0_A_W<0> {
        CMPR0_A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow register for register A.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpr0_value0](index.html) module"]
pub struct CMPR0_VALUE0_SPEC;
impl crate::RegisterSpec for CMPR0_VALUE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpr0_value0::R](R) reader structure"]
impl crate::Readable for CMPR0_VALUE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpr0_value0::W](W) writer structure"]
impl crate::Writable for CMPR0_VALUE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPR0_VALUE0 to value 0"]
impl crate::Resettable for CMPR0_VALUE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
