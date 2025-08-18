#[doc = "Register `DCACHE_SYNC_ADDR` reader"]
pub type R = crate::R<DCACHE_SYNC_ADDR_SPEC>;
#[doc = "Register `DCACHE_SYNC_ADDR` writer"]
pub type W = crate::W<DCACHE_SYNC_ADDR_SPEC>;
#[doc = "Field `DCACHE_SYNC_ADDR` reader - The bits are used to configure the start virtual address for clean operations. It should be combined with DCACHE_SYNC_SIZE_REG."]
pub type DCACHE_SYNC_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DCACHE_SYNC_ADDR` writer - The bits are used to configure the start virtual address for clean operations. It should be combined with DCACHE_SYNC_SIZE_REG."]
pub type DCACHE_SYNC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for clean operations. It should be combined with DCACHE_SYNC_SIZE_REG."]
    #[inline(always)]
    pub fn dcache_sync_addr(&self) -> DCACHE_SYNC_ADDR_R {
        DCACHE_SYNC_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_SYNC_ADDR")
            .field("dcache_sync_addr", &self.dcache_sync_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for clean operations. It should be combined with DCACHE_SYNC_SIZE_REG."]
    #[inline(always)]
    pub fn dcache_sync_addr(&mut self) -> DCACHE_SYNC_ADDR_W<'_, DCACHE_SYNC_ADDR_SPEC> {
        DCACHE_SYNC_ADDR_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_sync_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_sync_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_SYNC_ADDR_SPEC;
impl crate::RegisterSpec for DCACHE_SYNC_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_sync_addr::R`](R) reader structure"]
impl crate::Readable for DCACHE_SYNC_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_sync_addr::W`](W) writer structure"]
impl crate::Writable for DCACHE_SYNC_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCACHE_SYNC_ADDR to value 0"]
impl crate::Resettable for DCACHE_SYNC_ADDR_SPEC {}
