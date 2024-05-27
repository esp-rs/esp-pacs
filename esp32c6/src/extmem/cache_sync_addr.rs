///Register `CACHE_SYNC_ADDR` reader
pub type R = crate::R<CACHE_SYNC_ADDR_SPEC>;
///Register `CACHE_SYNC_ADDR` writer
pub type W = crate::W<CACHE_SYNC_ADDR_SPEC>;
///Field `CACHE_SYNC_ADDR` reader - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG
pub type CACHE_SYNC_ADDR_R = crate::FieldReader<u32>;
///Field `CACHE_SYNC_ADDR` writer - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG
pub type CACHE_SYNC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG
    #[inline(always)]
    pub fn cache_sync_addr(&self) -> CACHE_SYNC_ADDR_R {
        CACHE_SYNC_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SYNC_ADDR")
            .field("cache_sync_addr", &self.cache_sync_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG
    #[inline(always)]
    #[must_use]
    pub fn cache_sync_addr(&mut self) -> CACHE_SYNC_ADDR_W<CACHE_SYNC_ADDR_SPEC> {
        CACHE_SYNC_ADDR_W::new(self, 0)
    }
}
/**Sync address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_sync_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_sync_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_SYNC_ADDR_SPEC;
impl crate::RegisterSpec for CACHE_SYNC_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_sync_addr::R`](R) reader structure
impl crate::Readable for CACHE_SYNC_ADDR_SPEC {}
///`write(|w| ..)` method takes [`cache_sync_addr::W`](W) writer structure
impl crate::Writable for CACHE_SYNC_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_SYNC_ADDR to value 0
impl crate::Resettable for CACHE_SYNC_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
