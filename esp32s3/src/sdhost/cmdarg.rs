#[doc = "Register `CMDARG` reader"]
pub struct R(crate::R<CMDARG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDARG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDARG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDARG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDARG` writer"]
pub struct W(crate::W<CMDARG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDARG_SPEC>;
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
impl From<crate::W<CMDARG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDARG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDARG` reader - Value indicates command argument to be passed to the card."]
pub type CMDARG_R = crate::FieldReader<u32>;
#[doc = "Field `CMDARG` writer - Value indicates command argument to be passed to the card."]
pub type CMDARG_W<'a, const O: u8> = crate::FieldWriter<'a, CMDARG_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to the card."]
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDARG")
            .field("cmdarg", &format_args!("{}", self.cmdarg().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMDARG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to the card."]
    #[inline(always)]
    #[must_use]
    pub fn cmdarg(&mut self) -> CMDARG_W<0> {
        CMDARG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command argument data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdarg](index.html) module"]
pub struct CMDARG_SPEC;
impl crate::RegisterSpec for CMDARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdarg::R](R) reader structure"]
impl crate::Readable for CMDARG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdarg::W](W) writer structure"]
impl crate::Writable for CMDARG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDARG to value 0"]
impl crate::Resettable for CMDARG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
