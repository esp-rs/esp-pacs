#[doc = "Register `L2_CACHE_DEBUG_BUS` reader"]
pub type R = crate::R<L2_CACHE_DEBUG_BUS_SPEC>;
#[doc = "Register `L2_CACHE_DEBUG_BUS` writer"]
pub type W = crate::W<L2_CACHE_DEBUG_BUS_SPEC>;
#[doc = "Field `L2_CACHE_DEBUG_BUS` reader - This is a constant place where we can write data to or read data from the tag/data memory on the specified cache."]
pub type L2_CACHE_DEBUG_BUS_R = crate::FieldReader<u32>;
#[doc = "Field `L2_CACHE_DEBUG_BUS` writer - This is a constant place where we can write data to or read data from the tag/data memory on the specified cache."]
pub type L2_CACHE_DEBUG_BUS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is a constant place where we can write data to or read data from the tag/data memory on the specified cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_debug_bus(&mut self) -> L2_CACHE_DEBUG_BUS_W<L2_CACHE_DEBUG_BUS_SPEC> {
        L2_CACHE_DEBUG_BUS_W::new(self, 0)
    }
}
#[doc = "Cache Tag/data memory content register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_debug_bus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_debug_bus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_DEBUG_BUS_SPEC;
impl crate::RegisterSpec for L2_CACHE_DEBUG_BUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_debug_bus::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_DEBUG_BUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_debug_bus::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_DEBUG_BUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_DEBUG_BUS to value 0x03cc"]
impl crate::Resettable for L2_CACHE_DEBUG_BUS_SPEC {
    const RESET_VALUE: u32 = 0x03cc;
}
