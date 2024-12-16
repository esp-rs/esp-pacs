#[doc = "Register `PRO_DCACHE_PRELOAD_ADDR` reader"]
pub type R = crate::R<PRO_DCACHE_PRELOAD_ADDR_SPEC>;
#[doc = "Register `PRO_DCACHE_PRELOAD_ADDR` writer"]
pub type W = crate::W<PRO_DCACHE_PRELOAD_ADDR_SPEC>;
#[doc = "Field `PRO_DCACHE_PRELOAD_ADDR` reader - The bits are used to configure the start virtual address for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_SIZE_REG."]
pub type PRO_DCACHE_PRELOAD_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_DCACHE_PRELOAD_ADDR` writer - The bits are used to configure the start virtual address for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_SIZE_REG."]
pub type PRO_DCACHE_PRELOAD_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_SIZE_REG."]
    #[inline(always)]
    pub fn pro_dcache_preload_addr(&self) -> PRO_DCACHE_PRELOAD_ADDR_R {
        PRO_DCACHE_PRELOAD_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_PRELOAD_ADDR")
            .field("pro_dcache_preload_addr", &self.pro_dcache_preload_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_SIZE_REG."]
    #[inline(always)]
    pub fn pro_dcache_preload_addr(
        &mut self,
    ) -> PRO_DCACHE_PRELOAD_ADDR_W<PRO_DCACHE_PRELOAD_ADDR_SPEC> {
        PRO_DCACHE_PRELOAD_ADDR_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_preload_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_preload_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_PRELOAD_ADDR_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_PRELOAD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_preload_addr::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_PRELOAD_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dcache_preload_addr::W`](W) writer structure"]
impl crate::Writable for PRO_DCACHE_PRELOAD_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DCACHE_PRELOAD_ADDR to value 0"]
impl crate::Resettable for PRO_DCACHE_PRELOAD_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
