#[doc = "Register `L2_CACHE_PRELOCK_SCT1_ADDR` reader"]
pub type R = crate::R<L2_CACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "Register `L2_CACHE_PRELOCK_SCT1_ADDR` writer"]
pub type W = crate::W<L2_CACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "Field `L2_CACHE_PRELOCK_SCT1_ADDR` reader - Those bits are used to configure the start virtual address of the second section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT1_SIZE_REG"]
pub type L2_CACHE_PRELOCK_SCT1_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `L2_CACHE_PRELOCK_SCT1_ADDR` writer - Those bits are used to configure the start virtual address of the second section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT1_SIZE_REG"]
pub type L2_CACHE_PRELOCK_SCT1_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the second section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT1_SIZE_REG"]
    #[inline(always)]
    pub fn l2_cache_prelock_sct1_addr(&self) -> L2_CACHE_PRELOCK_SCT1_ADDR_R {
        L2_CACHE_PRELOCK_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_PRELOCK_SCT1_ADDR")
            .field(
                "l2_cache_prelock_sct1_addr",
                &self.l2_cache_prelock_sct1_addr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the second section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT1_SIZE_REG"]
    #[inline(always)]
    pub fn l2_cache_prelock_sct1_addr(
        &mut self,
    ) -> L2_CACHE_PRELOCK_SCT1_ADDR_W<L2_CACHE_PRELOCK_SCT1_ADDR_SPEC> {
        L2_CACHE_PRELOCK_SCT1_ADDR_W::new(self, 0)
    }
}
#[doc = "L2 Cache prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_prelock_sct1_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_prelock_sct1_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_PRELOCK_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for L2_CACHE_PRELOCK_SCT1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_prelock_sct1_addr::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_PRELOCK_SCT1_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_prelock_sct1_addr::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_PRELOCK_SCT1_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_PRELOCK_SCT1_ADDR to value 0"]
impl crate::Resettable for L2_CACHE_PRELOCK_SCT1_ADDR_SPEC {}
