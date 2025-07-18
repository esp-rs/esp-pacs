#[doc = "Register `CACHE_LOCK_SIZE` reader"]
pub type R = crate::R<CACHE_LOCK_SIZE_SPEC>;
#[doc = "Register `CACHE_LOCK_SIZE` writer"]
pub type W = crate::W<CACHE_LOCK_SIZE_SPEC>;
#[doc = "Field `CACHE_LOCK_SIZE` reader - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
pub type CACHE_LOCK_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `CACHE_LOCK_SIZE` writer - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
pub type CACHE_LOCK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
            .field("cache_lock_size", &self.cache_lock_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
    #[inline(always)]
    pub fn cache_lock_size(&mut self) -> CACHE_LOCK_SIZE_W<CACHE_LOCK_SIZE_SPEC> {
        CACHE_LOCK_SIZE_W::new(self, 0)
    }
}
#[doc = "Lock (manual lock) size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_lock_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_lock_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_LOCK_SIZE_SPEC;
impl crate::RegisterSpec for CACHE_LOCK_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_lock_size::R`](R) reader structure"]
impl crate::Readable for CACHE_LOCK_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_lock_size::W`](W) writer structure"]
impl crate::Writable for CACHE_LOCK_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_LOCK_SIZE to value 0"]
impl crate::Resettable for CACHE_LOCK_SIZE_SPEC {}
