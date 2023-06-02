#[doc = "Register `MODEXP_MODE` reader"]
pub struct R(crate::R<MODEXP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODEXP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODEXP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODEXP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODEXP_MODE` writer"]
pub struct W(crate::W<MODEXP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODEXP_MODE_SPEC>;
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
impl From<crate::W<MODEXP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODEXP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODEXP_MODE` reader - This register contains the mode of modular exponentiation."]
pub type MODEXP_MODE_R = crate::FieldReader;
#[doc = "Field `MODEXP_MODE` writer - This register contains the mode of modular exponentiation."]
pub type MODEXP_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, MODEXP_MODE_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - This register contains the mode of modular exponentiation."]
    #[inline(always)]
    pub fn modexp_mode(&self) -> MODEXP_MODE_R {
        MODEXP_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEXP_MODE")
            .field(
                "modexp_mode",
                &format_args!("{}", self.modexp_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEXP_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - This register contains the mode of modular exponentiation."]
    #[inline(always)]
    #[must_use]
    pub fn modexp_mode(&mut self) -> MODEXP_MODE_W<0> {
        MODEXP_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modexp_mode](index.html) module"]
pub struct MODEXP_MODE_SPEC;
impl crate::RegisterSpec for MODEXP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modexp_mode::R](R) reader structure"]
impl crate::Readable for MODEXP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modexp_mode::W](W) writer structure"]
impl crate::Writable for MODEXP_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEXP_MODE to value 0"]
impl crate::Resettable for MODEXP_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
