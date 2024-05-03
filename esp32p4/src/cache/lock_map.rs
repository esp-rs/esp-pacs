#[doc = "Register `LOCK_MAP` reader"]
pub type R = crate::R<LOCK_MAP_SPEC>;
#[doc = "Register `LOCK_MAP` writer"]
pub type W = crate::W<LOCK_MAP_SPEC>;
#[doc = "Field `LOCK_MAP` reader - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \\[0\\]: L1-ICache0, \\[1\\]: L1-ICache1, \\[2\\]: L1-ICache2, \\[3\\]: L1-ICache3, \\[4\\]: L1-DCache, \\[5\\]: L2-Cache."]
pub type LOCK_MAP_R = crate::FieldReader;
#[doc = "Field `LOCK_MAP` writer - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \\[0\\]: L1-ICache0, \\[1\\]: L1-ICache1, \\[2\\]: L1-ICache2, \\[3\\]: L1-ICache3, \\[4\\]: L1-DCache, \\[5\\]: L2-Cache."]
pub type LOCK_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \\[0\\]: L1-ICache0, \\[1\\]: L1-ICache1, \\[2\\]: L1-ICache2, \\[3\\]: L1-ICache3, \\[4\\]: L1-DCache, \\[5\\]: L2-Cache."]
    #[inline(always)]
    pub fn lock_map(&self) -> LOCK_MAP_R {
        LOCK_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK_MAP")
            .field("lock_map", &self.lock_map().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOCK_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \\[0\\]: L1-ICache0, \\[1\\]: L1-ICache1, \\[2\\]: L1-ICache2, \\[3\\]: L1-ICache3, \\[4\\]: L1-DCache, \\[5\\]: L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn lock_map(&mut self) -> LOCK_MAP_W<LOCK_MAP_SPEC> {
        LOCK_MAP_W::new(self, 0)
    }
}
#[doc = "Lock (manual lock) map configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCK_MAP_SPEC;
impl crate::RegisterSpec for LOCK_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock_map::R`](R) reader structure"]
impl crate::Readable for LOCK_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lock_map::W`](W) writer structure"]
impl crate::Writable for LOCK_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK_MAP to value 0"]
impl crate::Resettable for LOCK_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
