#[doc = "Register `CACHE_DATAARRAY_CONNECT_0` reader"]
pub struct R(crate::R<CACHE_DATAARRAY_CONNECT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_DATAARRAY_CONNECT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_DATAARRAY_CONNECT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_DATAARRAY_CONNECT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_DATAARRAY_CONNECT_0` writer"]
pub struct W(crate::W<CACHE_DATAARRAY_CONNECT_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_DATAARRAY_CONNECT_0_SPEC>;
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
impl From<crate::W<CACHE_DATAARRAY_CONNECT_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_DATAARRAY_CONNECT_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_DATAARRAY_CONNECT_LOCK` reader - Set 1 to lock cache data array registers."]
pub type CACHE_DATAARRAY_CONNECT_LOCK_R = crate::BitReader;
#[doc = "Field `CACHE_DATAARRAY_CONNECT_LOCK` writer - Set 1 to lock cache data array registers."]
pub type CACHE_DATAARRAY_CONNECT_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_DATAARRAY_CONNECT_0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to lock cache data array registers."]
    #[inline(always)]
    pub fn cache_dataarray_connect_lock(&self) -> CACHE_DATAARRAY_CONNECT_LOCK_R {
        CACHE_DATAARRAY_CONNECT_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_DATAARRAY_CONNECT_0")
            .field(
                "cache_dataarray_connect_lock",
                &format_args!("{}", self.cache_dataarray_connect_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_DATAARRAY_CONNECT_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to lock cache data array registers."]
    #[inline(always)]
    #[must_use]
    pub fn cache_dataarray_connect_lock(&mut self) -> CACHE_DATAARRAY_CONNECT_LOCK_W<0> {
        CACHE_DATAARRAY_CONNECT_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache data array configuration register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_dataarray_connect_0](index.html) module"]
pub struct CACHE_DATAARRAY_CONNECT_0_SPEC;
impl crate::RegisterSpec for CACHE_DATAARRAY_CONNECT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_dataarray_connect_0::R](R) reader structure"]
impl crate::Readable for CACHE_DATAARRAY_CONNECT_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_dataarray_connect_0::W](W) writer structure"]
impl crate::Writable for CACHE_DATAARRAY_CONNECT_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_DATAARRAY_CONNECT_0 to value 0"]
impl crate::Resettable for CACHE_DATAARRAY_CONNECT_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
