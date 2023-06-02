#[doc = "Register `CACHE_DATAARRAY_CONNECT_1` reader"]
pub struct R(crate::R<CACHE_DATAARRAY_CONNECT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_DATAARRAY_CONNECT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_DATAARRAY_CONNECT_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_DATAARRAY_CONNECT_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_DATAARRAY_CONNECT_1` writer"]
pub struct W(crate::W<CACHE_DATAARRAY_CONNECT_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_DATAARRAY_CONNECT_1_SPEC>;
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
impl From<crate::W<CACHE_DATAARRAY_CONNECT_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_DATAARRAY_CONNECT_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_DATAARRAY_CONNECT_FLATTEN` reader - Cache data array connection configuration."]
pub type CACHE_DATAARRAY_CONNECT_FLATTEN_R = crate::FieldReader;
#[doc = "Field `CACHE_DATAARRAY_CONNECT_FLATTEN` writer - Cache data array connection configuration."]
pub type CACHE_DATAARRAY_CONNECT_FLATTEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, CACHE_DATAARRAY_CONNECT_1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Cache data array connection configuration."]
    #[inline(always)]
    pub fn cache_dataarray_connect_flatten(&self) -> CACHE_DATAARRAY_CONNECT_FLATTEN_R {
        CACHE_DATAARRAY_CONNECT_FLATTEN_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_DATAARRAY_CONNECT_1")
            .field(
                "cache_dataarray_connect_flatten",
                &format_args!("{}", self.cache_dataarray_connect_flatten().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_DATAARRAY_CONNECT_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cache data array connection configuration."]
    #[inline(always)]
    #[must_use]
    pub fn cache_dataarray_connect_flatten(&mut self) -> CACHE_DATAARRAY_CONNECT_FLATTEN_W<0> {
        CACHE_DATAARRAY_CONNECT_FLATTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache data array configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_dataarray_connect_1](index.html) module"]
pub struct CACHE_DATAARRAY_CONNECT_1_SPEC;
impl crate::RegisterSpec for CACHE_DATAARRAY_CONNECT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_dataarray_connect_1::R](R) reader structure"]
impl crate::Readable for CACHE_DATAARRAY_CONNECT_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_dataarray_connect_1::W](W) writer structure"]
impl crate::Writable for CACHE_DATAARRAY_CONNECT_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_DATAARRAY_CONNECT_1 to value 0xff"]
impl crate::Resettable for CACHE_DATAARRAY_CONNECT_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
