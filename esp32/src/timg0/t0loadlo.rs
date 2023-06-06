#[doc = "Register `T0LOADLO` reader"]
pub struct R(crate::R<T0LOADLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0LOADLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0LOADLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0LOADLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0LOADLO` writer"]
pub struct W(crate::W<T0LOADLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0LOADLO_SPEC>;
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
impl From<crate::W<T0LOADLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0LOADLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD_LO` reader - Lower 32 bits of the value that will load into timer 0 time-base counter"]
pub type LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `LOAD_LO` writer - Lower 32 bits of the value that will load into timer 0 time-base counter"]
pub type LOAD_LO_W<'a, const O: u8> = crate::FieldWriter<'a, T0LOADLO_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Lower 32 bits of the value that will load into timer 0 time-base counter"]
    #[inline(always)]
    pub fn load_lo(&self) -> LOAD_LO_R {
        LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0LOADLO")
            .field("load_lo", &format_args!("{}", self.load_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0LOADLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lower 32 bits of the value that will load into timer 0 time-base counter"]
    #[inline(always)]
    #[must_use]
    pub fn load_lo(&mut self) -> LOAD_LO_W<0> {
        LOAD_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0loadlo](index.html) module"]
pub struct T0LOADLO_SPEC;
impl crate::RegisterSpec for T0LOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0loadlo::R](R) reader structure"]
impl crate::Readable for T0LOADLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0loadlo::W](W) writer structure"]
impl crate::Writable for T0LOADLO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0LOADLO to value 0"]
impl crate::Resettable for T0LOADLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
