#[doc = "Register `DCACHE_SYNC_SIZE` reader"]
pub type R = crate::R<DCACHE_SYNC_SIZE_SPEC>;
#[doc = "Register `DCACHE_SYNC_SIZE` writer"]
pub type W = crate::W<DCACHE_SYNC_SIZE_SPEC>;
#[doc = "Field `DCACHE_SYNC_SIZE` reader - The bits are used to configure the length for sync operations. The bits are the counts of cache block. It should be combined with DCACHE_SYNC_ADDR_REG."]
pub type DCACHE_SYNC_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `DCACHE_SYNC_SIZE` writer - The bits are used to configure the length for sync operations. The bits are the counts of cache block. It should be combined with DCACHE_SYNC_ADDR_REG."]
pub type DCACHE_SYNC_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - The bits are used to configure the length for sync operations. The bits are the counts of cache block. It should be combined with DCACHE_SYNC_ADDR_REG."]
    #[inline(always)]
    pub fn dcache_sync_size(&self) -> DCACHE_SYNC_SIZE_R {
        DCACHE_SYNC_SIZE_R::new(self.bits & 0x007f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_SYNC_SIZE")
            .field("dcache_sync_size", &self.dcache_sync_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:22 - The bits are used to configure the length for sync operations. The bits are the counts of cache block. It should be combined with DCACHE_SYNC_ADDR_REG."]
    #[inline(always)]
    pub fn dcache_sync_size(&mut self) -> DCACHE_SYNC_SIZE_W<DCACHE_SYNC_SIZE_SPEC> {
        DCACHE_SYNC_SIZE_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_sync_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_sync_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_SYNC_SIZE_SPEC;
impl crate::RegisterSpec for DCACHE_SYNC_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_sync_size::R`](R) reader structure"]
impl crate::Readable for DCACHE_SYNC_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_sync_size::W`](W) writer structure"]
impl crate::Writable for DCACHE_SYNC_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCACHE_SYNC_SIZE to value 0"]
impl crate::Resettable for DCACHE_SYNC_SIZE_SPEC {}
