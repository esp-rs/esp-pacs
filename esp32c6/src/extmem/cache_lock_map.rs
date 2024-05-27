///Register `CACHE_LOCK_MAP` reader
pub type R = crate::R<CACHE_LOCK_MAP_SPEC>;
///Register `CACHE_LOCK_MAP` writer
pub type W = crate::W<CACHE_LOCK_MAP_SPEC>;
///Field `CACHE_LOCK_MAP` reader - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \[4\]: L1-Cache
pub type CACHE_LOCK_MAP_R = crate::FieldReader;
///Field `CACHE_LOCK_MAP` writer - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \[4\]: L1-Cache
pub type CACHE_LOCK_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \[4\]: L1-Cache
    #[inline(always)]
    pub fn cache_lock_map(&self) -> CACHE_LOCK_MAP_R {
        CACHE_LOCK_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_LOCK_MAP")
            .field("cache_lock_map", &self.cache_lock_map())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \[4\]: L1-Cache
    #[inline(always)]
    #[must_use]
    pub fn cache_lock_map(&mut self) -> CACHE_LOCK_MAP_W<CACHE_LOCK_MAP_SPEC> {
        CACHE_LOCK_MAP_W::new(self, 0)
    }
}
/**Lock (manual lock) map configure register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_lock_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_lock_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_LOCK_MAP_SPEC;
impl crate::RegisterSpec for CACHE_LOCK_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_lock_map::R`](R) reader structure
impl crate::Readable for CACHE_LOCK_MAP_SPEC {}
///`write(|w| ..)` method takes [`cache_lock_map::W`](W) writer structure
impl crate::Writable for CACHE_LOCK_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_LOCK_MAP to value 0
impl crate::Resettable for CACHE_LOCK_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
