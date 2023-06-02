#[doc = "Register `CMPR2_VALUE1` reader"]
pub struct R(crate::R<CMPR2_VALUE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPR2_VALUE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPR2_VALUE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPR2_VALUE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPR2_VALUE1` writer"]
pub struct W(crate::W<CMPR2_VALUE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPR2_VALUE1_SPEC>;
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
impl From<crate::W<CMPR2_VALUE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPR2_VALUE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR2_B` reader - PWM generator 2 time stamp B's shadow register"]
pub type CMPR2_B_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMPR2_B` writer - PWM generator 2 time stamp B's shadow register"]
pub type CMPR2_B_W<'a, const O: u8> = crate::FieldWriter<'a, CMPR2_VALUE1_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15 - PWM generator 2 time stamp B's shadow register"]
    #[inline(always)]
    pub fn cmpr2_b(&self) -> CMPR2_B_R {
        CMPR2_B_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPR2_VALUE1")
            .field("cmpr2_b", &format_args!("{}", self.cmpr2_b().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMPR2_VALUE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 2 time stamp B's shadow register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_b(&mut self) -> CMPR2_B_W<0> {
        CMPR2_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow register for register B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpr2_value1](index.html) module"]
pub struct CMPR2_VALUE1_SPEC;
impl crate::RegisterSpec for CMPR2_VALUE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpr2_value1::R](R) reader structure"]
impl crate::Readable for CMPR2_VALUE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpr2_value1::W](W) writer structure"]
impl crate::Writable for CMPR2_VALUE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPR2_VALUE1 to value 0"]
impl crate::Resettable for CMPR2_VALUE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
