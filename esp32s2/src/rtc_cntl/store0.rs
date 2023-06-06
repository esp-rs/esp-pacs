#[doc = "Register `STORE0` reader"]
pub struct R(crate::R<STORE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE0` writer"]
pub struct W(crate::W<STORE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE0_SPEC>;
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
impl From<crate::W<STORE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRATCH0` reader - Reservation register 0"]
pub type SCRATCH0_R = crate::FieldReader<u32>;
#[doc = "Field `SCRATCH0` writer - Reservation register 0"]
pub type SCRATCH0_W<'a, const O: u8> = crate::FieldWriter<'a, STORE0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Reservation register 0"]
    #[inline(always)]
    pub fn scratch0(&self) -> SCRATCH0_R {
        SCRATCH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE0")
            .field("scratch0", &format_args!("{}", self.scratch0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STORE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reservation register 0"]
    #[inline(always)]
    #[must_use]
    pub fn scratch0(&mut self) -> SCRATCH0_W<0> {
        SCRATCH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reservation register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store0](index.html) module"]
pub struct STORE0_SPEC;
impl crate::RegisterSpec for STORE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store0::R](R) reader structure"]
impl crate::Readable for STORE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store0::W](W) writer structure"]
impl crate::Writable for STORE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE0 to value 0"]
impl crate::Resettable for STORE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
