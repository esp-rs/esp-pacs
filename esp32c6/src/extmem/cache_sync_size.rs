///Register `CACHE_SYNC_SIZE` reader
pub type R = crate::R<CACHE_SYNC_SIZE_SPEC>;
///Register `CACHE_SYNC_SIZE` writer
pub type W = crate::W<CACHE_SYNC_SIZE_SPEC>;
///Field `CACHE_SYNC_SIZE` reader - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG
pub type CACHE_SYNC_SIZE_R = crate::FieldReader<u32>;
///Field `CACHE_SYNC_SIZE` writer - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG
pub type CACHE_SYNC_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG
    #[inline(always)]
    pub fn cache_sync_size(&self) -> CACHE_SYNC_SIZE_R {
        CACHE_SYNC_SIZE_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SYNC_SIZE")
            .field("cache_sync_size", &self.cache_sync_size())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG
    #[inline(always)]
    #[must_use]
    pub fn cache_sync_size(&mut self) -> CACHE_SYNC_SIZE_W<CACHE_SYNC_SIZE_SPEC> {
        CACHE_SYNC_SIZE_W::new(self, 0)
    }
}
/**Sync size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_sync_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_sync_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_SYNC_SIZE_SPEC;
impl crate::RegisterSpec for CACHE_SYNC_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_sync_size::R`](R) reader structure
impl crate::Readable for CACHE_SYNC_SIZE_SPEC {}
///`write(|w| ..)` method takes [`cache_sync_size::W`](W) writer structure
impl crate::Writable for CACHE_SYNC_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_SYNC_SIZE to value 0
impl crate::Resettable for CACHE_SYNC_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
