#[doc = "Register `L1_CACHE_DEBUG_BUS` reader"]
pub struct R(crate::R<L1_CACHE_DEBUG_BUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_DEBUG_BUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_DEBUG_BUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_DEBUG_BUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_DEBUG_BUS` writer"]
pub struct W(crate::W<L1_CACHE_DEBUG_BUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_DEBUG_BUS_SPEC>;
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
impl From<crate::W<L1_CACHE_DEBUG_BUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_DEBUG_BUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_CACHE_DEBUG_BUS` reader - This is a constant place where we can write data to or read data from the tag/data memory on the specified cache."]
pub type L1_CACHE_DEBUG_BUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `L1_CACHE_DEBUG_BUS` writer - This is a constant place where we can write data to or read data from the tag/data memory on the specified cache."]
pub type L1_CACHE_DEBUG_BUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, L1_CACHE_DEBUG_BUS_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This is a constant place where we can write data to or read data from the tag/data memory on the specified cache."]
    #[inline(always)]
    pub fn l1_cache_debug_bus(&self) -> L1_CACHE_DEBUG_BUS_R {
        L1_CACHE_DEBUG_BUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_DEBUG_BUS")
            .field(
                "l1_cache_debug_bus",
                &format_args!("{}", self.l1_cache_debug_bus().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_DEBUG_BUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is a constant place where we can write data to or read data from the tag/data memory on the specified cache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_debug_bus(&mut self) -> L1_CACHE_DEBUG_BUS_W<0> {
        L1_CACHE_DEBUG_BUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Tag/data memory content register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_debug_bus](index.html) module"]
pub struct L1_CACHE_DEBUG_BUS_SPEC;
impl crate::RegisterSpec for L1_CACHE_DEBUG_BUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_debug_bus::R](R) reader structure"]
impl crate::Readable for L1_CACHE_DEBUG_BUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_debug_bus::W](W) writer structure"]
impl crate::Writable for L1_CACHE_DEBUG_BUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_DEBUG_BUS to value 0x0254"]
impl crate::Resettable for L1_CACHE_DEBUG_BUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0254;
}
