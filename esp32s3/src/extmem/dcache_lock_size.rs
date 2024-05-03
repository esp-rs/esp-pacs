#[doc = "Register `DCACHE_LOCK_SIZE` reader"]
pub type R = crate::R<DCACHE_LOCK_SIZE_SPEC>;
#[doc = "Register `DCACHE_LOCK_SIZE` writer"]
pub type W = crate::W<DCACHE_LOCK_SIZE_SPEC>;
#[doc = "Field `DCACHE_LOCK_SIZE` reader - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with DCACHE_LOCK_ADDR_REG."]
pub type DCACHE_LOCK_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `DCACHE_LOCK_SIZE` writer - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with DCACHE_LOCK_ADDR_REG."]
pub type DCACHE_LOCK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with DCACHE_LOCK_ADDR_REG."]
    #[inline(always)]
    pub fn dcache_lock_size(&self) -> DCACHE_LOCK_SIZE_R {
        DCACHE_LOCK_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_LOCK_SIZE")
            .field("dcache_lock_size", &self.dcache_lock_size().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_LOCK_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with DCACHE_LOCK_ADDR_REG."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_lock_size(&mut self) -> DCACHE_LOCK_SIZE_W<DCACHE_LOCK_SIZE_SPEC> {
        DCACHE_LOCK_SIZE_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_lock_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_lock_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_LOCK_SIZE_SPEC;
impl crate::RegisterSpec for DCACHE_LOCK_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_lock_size::R`](R) reader structure"]
impl crate::Readable for DCACHE_LOCK_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_lock_size::W`](W) writer structure"]
impl crate::Writable for DCACHE_LOCK_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_LOCK_SIZE to value 0"]
impl crate::Resettable for DCACHE_LOCK_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
