#[doc = "Register `PRO_CACHE_1` reader"]
pub struct R(crate::R<PRO_CACHE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CACHE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CACHE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CACHE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CACHE_1` writer"]
pub struct W(crate::W<PRO_CACHE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CACHE_1_SPEC>;
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
impl From<crate::W<PRO_CACHE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CACHE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_CACHE_CONNECT` reader - Configure which SRAM Block will be occupied by Icache or Dcache."]
pub type PRO_CACHE_CONNECT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRO_CACHE_CONNECT` writer - Configure which SRAM Block will be occupied by Icache or Dcache."]
pub type PRO_CACHE_CONNECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CACHE_1_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configure which SRAM Block will be occupied by Icache or Dcache."]
    #[inline(always)]
    pub fn pro_cache_connect(&self) -> PRO_CACHE_CONNECT_R {
        PRO_CACHE_CONNECT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_1")
            .field(
                "pro_cache_connect",
                &format_args!("{}", self.pro_cache_connect().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configure which SRAM Block will be occupied by Icache or Dcache."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_connect(&mut self) -> PRO_CACHE_CONNECT_W<0> {
        PRO_CACHE_CONNECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache permission control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cache_1](index.html) module"]
pub struct PRO_CACHE_1_SPEC;
impl crate::RegisterSpec for PRO_CACHE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cache_1::R](R) reader structure"]
impl crate::Readable for PRO_CACHE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cache_1::W](W) writer structure"]
impl crate::Writable for PRO_CACHE_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_1 to value 0"]
impl crate::Resettable for PRO_CACHE_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
