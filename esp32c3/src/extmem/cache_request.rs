#[doc = "Register `CACHE_REQUEST` reader"]
pub struct R(crate::R<CACHE_REQUEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_REQUEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_REQUEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_REQUEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_REQUEST` writer"]
pub struct W(crate::W<CACHE_REQUEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_REQUEST_SPEC>;
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
impl From<crate::W<CACHE_REQUEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_REQUEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - The bit is used to disable request recording which could cause performance issue"]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - The bit is used to disable request recording which could cause performance issue"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_REQUEST_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable request recording which could cause performance issue"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_REQUEST")
            .field("bypass", &format_args!("{}", self.bypass().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_REQUEST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable request recording which could cause performance issue"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<0> {
        BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_request](index.html) module"]
pub struct CACHE_REQUEST_SPEC;
impl crate::RegisterSpec for CACHE_REQUEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_request::R](R) reader structure"]
impl crate::Readable for CACHE_REQUEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_request::W](W) writer structure"]
impl crate::Writable for CACHE_REQUEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_REQUEST to value 0"]
impl crate::Resettable for CACHE_REQUEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
