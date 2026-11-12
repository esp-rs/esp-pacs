#[doc = "Register `L1_CACHE_AUTOLOAD_SCT1_ADDR` reader"]
pub type R = crate::R<L1_CACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "Register `L1_CACHE_AUTOLOAD_SCT1_ADDR` writer"]
pub type W = crate::W<L1_CACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "Field `L1_CACHE_AUTOLOAD_SCT1_ADDR` reader - Configures the starting virtual address of Section 1 for the autoloading operation in the L1 cache. Note that it should be used together with CACHE_L1_CACHE_AUTOLOAD_SCT1_SIZE and CACHE_L1_CACHE_AUTOLOAD_SCT1_ENA."]
pub type L1_CACHE_AUTOLOAD_SCT1_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `L1_CACHE_AUTOLOAD_SCT1_ADDR` writer - Configures the starting virtual address of Section 1 for the autoloading operation in the L1 cache. Note that it should be used together with CACHE_L1_CACHE_AUTOLOAD_SCT1_SIZE and CACHE_L1_CACHE_AUTOLOAD_SCT1_ENA."]
pub type L1_CACHE_AUTOLOAD_SCT1_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the starting virtual address of Section 1 for the autoloading operation in the L1 cache. Note that it should be used together with CACHE_L1_CACHE_AUTOLOAD_SCT1_SIZE and CACHE_L1_CACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_cache_autoload_sct1_addr(&self) -> L1_CACHE_AUTOLOAD_SCT1_ADDR_R {
        L1_CACHE_AUTOLOAD_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_AUTOLOAD_SCT1_ADDR")
            .field(
                "l1_cache_autoload_sct1_addr",
                &self.l1_cache_autoload_sct1_addr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the starting virtual address of Section 1 for the autoloading operation in the L1 cache. Note that it should be used together with CACHE_L1_CACHE_AUTOLOAD_SCT1_SIZE and CACHE_L1_CACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_cache_autoload_sct1_addr(
        &mut self,
    ) -> L1_CACHE_AUTOLOAD_SCT1_ADDR_W<'_, L1_CACHE_AUTOLOAD_SCT1_ADDR_SPEC> {
        L1_CACHE_AUTOLOAD_SCT1_ADDR_W::new(self, 0)
    }
}
#[doc = "L1 cache autoloading Section 1 address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_autoload_sct1_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_autoload_sct1_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_AUTOLOAD_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for L1_CACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_autoload_sct1_addr::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_AUTOLOAD_SCT1_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_autoload_sct1_addr::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_AUTOLOAD_SCT1_ADDR to value 0"]
impl crate::Resettable for L1_CACHE_AUTOLOAD_SCT1_ADDR_SPEC {}
