///Register `LOCK_SIZE` reader
pub type R = crate::R<LOCK_SIZE_SPEC>;
///Register `LOCK_SIZE` writer
pub type W = crate::W<LOCK_SIZE_SPEC>;
///Field `LOCK_SIZE` reader - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG
pub type LOCK_SIZE_R = crate::FieldReader<u16>;
///Field `LOCK_SIZE` writer - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG
pub type LOCK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG
    #[inline(always)]
    pub fn lock_size(&self) -> LOCK_SIZE_R {
        LOCK_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK_SIZE")
            .field("lock_size", &self.lock_size())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG
    #[inline(always)]
    #[must_use]
    pub fn lock_size(&mut self) -> LOCK_SIZE_W<LOCK_SIZE_SPEC> {
        LOCK_SIZE_W::new(self, 0)
    }
}
/**Lock (manual lock) size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`lock_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOCK_SIZE_SPEC;
impl crate::RegisterSpec for LOCK_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lock_size::R`](R) reader structure
impl crate::Readable for LOCK_SIZE_SPEC {}
///`write(|w| ..)` method takes [`lock_size::W`](W) writer structure
impl crate::Writable for LOCK_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOCK_SIZE to value 0
impl crate::Resettable for LOCK_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
