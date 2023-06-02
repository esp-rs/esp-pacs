#[doc = "Register `CACHE_LOCK_SIZE` reader"]
pub struct R(crate::R<CACHE_LOCK_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_LOCK_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_LOCK_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_LOCK_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_LOCK_SIZE` writer"]
pub struct W(crate::W<CACHE_LOCK_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_LOCK_SIZE_SPEC>;
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
impl From<crate::W<CACHE_LOCK_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_LOCK_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_LOCK_SIZE` reader - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
pub type CACHE_LOCK_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CACHE_LOCK_SIZE` writer - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
pub type CACHE_LOCK_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CACHE_LOCK_SIZE_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
    #[inline(always)]
    pub fn cache_lock_size(&self) -> CACHE_LOCK_SIZE_R {
        CACHE_LOCK_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_LOCK_SIZE")
            .field(
                "cache_lock_size",
                &format_args!("{}", self.cache_lock_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_LOCK_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
    #[inline(always)]
    #[must_use]
    pub fn cache_lock_size(&mut self) -> CACHE_LOCK_SIZE_W<0> {
        CACHE_LOCK_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock (manual lock) size configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_lock_size](index.html) module"]
pub struct CACHE_LOCK_SIZE_SPEC;
impl crate::RegisterSpec for CACHE_LOCK_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_lock_size::R](R) reader structure"]
impl crate::Readable for CACHE_LOCK_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_lock_size::W](W) writer structure"]
impl crate::Writable for CACHE_LOCK_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_LOCK_SIZE to value 0"]
impl crate::Resettable for CACHE_LOCK_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
