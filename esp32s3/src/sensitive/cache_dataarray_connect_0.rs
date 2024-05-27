///Register `CACHE_DATAARRAY_CONNECT_0` reader
pub type R = crate::R<CACHE_DATAARRAY_CONNECT_0_SPEC>;
///Register `CACHE_DATAARRAY_CONNECT_0` writer
pub type W = crate::W<CACHE_DATAARRAY_CONNECT_0_SPEC>;
///Field `CACHE_DATAARRAY_CONNECT_LOCK` reader - Set 1 to lock cache data array registers.
pub type CACHE_DATAARRAY_CONNECT_LOCK_R = crate::BitReader;
///Field `CACHE_DATAARRAY_CONNECT_LOCK` writer - Set 1 to lock cache data array registers.
pub type CACHE_DATAARRAY_CONNECT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to lock cache data array registers.
    #[inline(always)]
    pub fn cache_dataarray_connect_lock(&self) -> CACHE_DATAARRAY_CONNECT_LOCK_R {
        CACHE_DATAARRAY_CONNECT_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_DATAARRAY_CONNECT_0")
            .field(
                "cache_dataarray_connect_lock",
                &self.cache_dataarray_connect_lock(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to lock cache data array registers.
    #[inline(always)]
    #[must_use]
    pub fn cache_dataarray_connect_lock(
        &mut self,
    ) -> CACHE_DATAARRAY_CONNECT_LOCK_W<CACHE_DATAARRAY_CONNECT_0_SPEC> {
        CACHE_DATAARRAY_CONNECT_LOCK_W::new(self, 0)
    }
}
/**Cache data array configuration register 0.

You can [`read`](crate::generic::Reg::read) this register and get [`cache_dataarray_connect_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_dataarray_connect_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_DATAARRAY_CONNECT_0_SPEC;
impl crate::RegisterSpec for CACHE_DATAARRAY_CONNECT_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_dataarray_connect_0::R`](R) reader structure
impl crate::Readable for CACHE_DATAARRAY_CONNECT_0_SPEC {}
///`write(|w| ..)` method takes [`cache_dataarray_connect_0::W`](W) writer structure
impl crate::Writable for CACHE_DATAARRAY_CONNECT_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_DATAARRAY_CONNECT_0 to value 0
impl crate::Resettable for CACHE_DATAARRAY_CONNECT_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
