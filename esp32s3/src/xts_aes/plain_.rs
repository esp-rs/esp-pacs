#[doc = "Register `PLAIN_%s` reader"]
pub struct R(crate::R<PLAIN__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLAIN__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLAIN__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLAIN__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLAIN_%s` writer"]
pub struct W(crate::W<PLAIN__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLAIN__SPEC>;
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
impl From<crate::W<PLAIN__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLAIN__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLAIN_0` reader - Stores the nth 32-bit piece of plaintext."]
pub type PLAIN_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PLAIN_0` writer - Stores the nth 32-bit piece of plaintext."]
pub type PLAIN_0_W<'a, const O: u8> = crate::FieldWriter<'a, PLAIN__SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the nth 32-bit piece of plaintext."]
    #[inline(always)]
    pub fn plain_0(&self) -> PLAIN_0_R {
        PLAIN_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLAIN_")
            .field("plain_0", &format_args!("{}", self.plain_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PLAIN__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the nth 32-bit piece of plaintext."]
    #[inline(always)]
    #[must_use]
    pub fn plain_0(&mut self) -> PLAIN_0_W<0> {
        PLAIN_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Plaintext register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plain_](index.html) module"]
pub struct PLAIN__SPEC;
impl crate::RegisterSpec for PLAIN__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plain_::R](R) reader structure"]
impl crate::Readable for PLAIN__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plain_::W](W) writer structure"]
impl crate::Writable for PLAIN__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLAIN_%s to value 0"]
impl crate::Resettable for PLAIN__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
