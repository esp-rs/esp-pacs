#[doc = "Register `L1_DCACHE_AUTOLOAD_SCT1_SIZE` reader"]
pub type R = crate::R<L1_DCACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "Register `L1_DCACHE_AUTOLOAD_SCT1_SIZE` writer"]
pub type W = crate::W<L1_DCACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT1_SIZE` reader - Those bits are used to configure the size of the second section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT1_ADDR and L1_DCACHE_AUTOLOAD_SCT1_ENA."]
pub type L1_DCACHE_AUTOLOAD_SCT1_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `L1_DCACHE_AUTOLOAD_SCT1_SIZE` writer - Those bits are used to configure the size of the second section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT1_ADDR and L1_DCACHE_AUTOLOAD_SCT1_ENA."]
pub type L1_DCACHE_AUTOLOAD_SCT1_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT1_ADDR and L1_DCACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_dcache_autoload_sct1_size(&self) -> L1_DCACHE_AUTOLOAD_SCT1_SIZE_R {
        L1_DCACHE_AUTOLOAD_SCT1_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_DCACHE_AUTOLOAD_SCT1_SIZE")
            .field(
                "l1_dcache_autoload_sct1_size",
                &format_args!("{}", self.l1_dcache_autoload_sct1_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_DCACHE_AUTOLOAD_SCT1_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on L1-DCache. Note that it should be used together with L1_DCACHE_AUTOLOAD_SCT1_ADDR and L1_DCACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_autoload_sct1_size(
        &mut self,
    ) -> L1_DCACHE_AUTOLOAD_SCT1_SIZE_W<L1_DCACHE_AUTOLOAD_SCT1_SIZE_SPEC> {
        L1_DCACHE_AUTOLOAD_SCT1_SIZE_W::new(self, 0)
    }
}
#[doc = "L1 data Cache autoload section 1 size configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_sct1_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct1_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_DCACHE_AUTOLOAD_SCT1_SIZE_SPEC;
impl crate::RegisterSpec for L1_DCACHE_AUTOLOAD_SCT1_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_autoload_sct1_size::R`](R) reader structure"]
impl crate::Readable for L1_DCACHE_AUTOLOAD_SCT1_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_autoload_sct1_size::W`](W) writer structure"]
impl crate::Writable for L1_DCACHE_AUTOLOAD_SCT1_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_DCACHE_AUTOLOAD_SCT1_SIZE to value 0"]
impl crate::Resettable for L1_DCACHE_AUTOLOAD_SCT1_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
