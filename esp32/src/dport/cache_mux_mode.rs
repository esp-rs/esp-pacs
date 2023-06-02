#[doc = "Register `CACHE_MUX_MODE` reader"]
pub struct R(crate::R<CACHE_MUX_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MUX_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MUX_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MUX_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_MUX_MODE` writer"]
pub struct W(crate::W<CACHE_MUX_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_MUX_MODE_SPEC>;
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
impl From<crate::W<CACHE_MUX_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_MUX_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_MUX_MODE` reader - "]
pub type CACHE_MUX_MODE_R = crate::FieldReader;
#[doc = "Field `CACHE_MUX_MODE` writer - "]
pub type CACHE_MUX_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, CACHE_MUX_MODE_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cache_mux_mode(&self) -> CACHE_MUX_MODE_R {
        CACHE_MUX_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MUX_MODE")
            .field(
                "cache_mux_mode",
                &format_args!("{}", self.cache_mux_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_MUX_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cache_mux_mode(&mut self) -> CACHE_MUX_MODE_W<0> {
        CACHE_MUX_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mux_mode](index.html) module"]
pub struct CACHE_MUX_MODE_SPEC;
impl crate::RegisterSpec for CACHE_MUX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mux_mode::R](R) reader structure"]
impl crate::Readable for CACHE_MUX_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_mux_mode::W](W) writer structure"]
impl crate::Writable for CACHE_MUX_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_MUX_MODE to value 0"]
impl crate::Resettable for CACHE_MUX_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
