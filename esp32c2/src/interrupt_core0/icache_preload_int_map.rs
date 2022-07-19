#[doc = "Register `ICACHE_PRELOAD_INT_MAP` reader"]
pub struct R(crate::R<ICACHE_PRELOAD_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_PRELOAD_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_PRELOAD_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_PRELOAD_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_PRELOAD_INT_MAP` writer"]
pub struct W(crate::W<ICACHE_PRELOAD_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_PRELOAD_INT_MAP_SPEC>;
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
impl From<crate::W<ICACHE_PRELOAD_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_PRELOAD_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_PRELOAD_INT_MAP` reader - Need add description"]
pub type ICACHE_PRELOAD_INT_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICACHE_PRELOAD_INT_MAP` writer - Need add description"]
pub type ICACHE_PRELOAD_INT_MAP_W<'a> =
    crate::FieldWriter<'a, u32, ICACHE_PRELOAD_INT_MAP_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn icache_preload_int_map(&self) -> ICACHE_PRELOAD_INT_MAP_R {
        ICACHE_PRELOAD_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn icache_preload_int_map(&mut self) -> ICACHE_PRELOAD_INT_MAP_W {
        ICACHE_PRELOAD_INT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_preload_int_map](index.html) module"]
pub struct ICACHE_PRELOAD_INT_MAP_SPEC;
impl crate::RegisterSpec for ICACHE_PRELOAD_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_preload_int_map::R](R) reader structure"]
impl crate::Readable for ICACHE_PRELOAD_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_preload_int_map::W](W) writer structure"]
impl crate::Writable for ICACHE_PRELOAD_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_PRELOAD_INT_MAP to value 0"]
impl crate::Resettable for ICACHE_PRELOAD_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
