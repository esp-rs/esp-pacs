#[doc = "Register `LOCK_ADDR` reader"]
pub type R = crate::R<LOCK_ADDR_SPEC>;
#[doc = "Register `LOCK_ADDR` writer"]
pub type W = crate::W<LOCK_ADDR_SPEC>;
#[doc = "Field `LOCK_ADDR` reader - Those bits are used to configure the start virtual address of the lock/unlock operation, which should be used together with CACHE_LOCK_SIZE_REG"]
pub type LOCK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `LOCK_ADDR` writer - Those bits are used to configure the start virtual address of the lock/unlock operation, which should be used together with CACHE_LOCK_SIZE_REG"]
pub type LOCK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the lock/unlock operation, which should be used together with CACHE_LOCK_SIZE_REG"]
    #[inline(always)]
    pub fn lock_addr(&self) -> LOCK_ADDR_R {
        LOCK_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK_ADDR")
            .field("lock_addr", &self.lock_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the lock/unlock operation, which should be used together with CACHE_LOCK_SIZE_REG"]
    #[inline(always)]
    #[must_use]
    pub fn lock_addr(&mut self) -> LOCK_ADDR_W<LOCK_ADDR_SPEC> {
        LOCK_ADDR_W::new(self, 0)
    }
}
#[doc = "Lock (manual lock) address configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCK_ADDR_SPEC;
impl crate::RegisterSpec for LOCK_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock_addr::R`](R) reader structure"]
impl crate::Readable for LOCK_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lock_addr::W`](W) writer structure"]
impl crate::Writable for LOCK_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK_ADDR to value 0"]
impl crate::Resettable for LOCK_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
