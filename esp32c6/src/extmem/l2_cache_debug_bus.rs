#[doc = "Register `L2_CACHE_DEBUG_BUS` reader"]
pub struct R(crate::R<L2_CACHE_DEBUG_BUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_DEBUG_BUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_DEBUG_BUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_DEBUG_BUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_DEBUG_BUS` reader - This is a constant place where we can write data to or read data from the tag/data memory on the specified cache."]
pub type L2_CACHE_DEBUG_BUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This is a constant place where we can write data to or read data from the tag/data memory on the specified cache."]
    #[inline(always)]
    pub fn l2_cache_debug_bus(&self) -> L2_CACHE_DEBUG_BUS_R {
        L2_CACHE_DEBUG_BUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_DEBUG_BUS")
            .field(
                "l2_cache_debug_bus",
                &format_args!("{}", self.l2_cache_debug_bus().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_DEBUG_BUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Tag/data memory content register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_debug_bus](index.html) module"]
pub struct L2_CACHE_DEBUG_BUS_SPEC;
impl crate::RegisterSpec for L2_CACHE_DEBUG_BUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_debug_bus::R](R) reader structure"]
impl crate::Readable for L2_CACHE_DEBUG_BUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_DEBUG_BUS to value 0x03a4"]
impl crate::Resettable for L2_CACHE_DEBUG_BUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x03a4;
}
