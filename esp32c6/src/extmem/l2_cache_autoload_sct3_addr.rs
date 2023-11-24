#[doc = "Register `L2_CACHE_AUTOLOAD_SCT3_ADDR` reader"]
pub type R = crate::R<L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC>;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT3_ADDR` reader - Those bits are used to configure the start virtual address of the fourth section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT3_SIZE and L2_CACHE_AUTOLOAD_SCT3_ENA."]
pub type L2_CACHE_AUTOLOAD_SCT3_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the fourth section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT3_SIZE and L2_CACHE_AUTOLOAD_SCT3_ENA."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct3_addr(&self) -> L2_CACHE_AUTOLOAD_SCT3_ADDR_R {
        L2_CACHE_AUTOLOAD_SCT3_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_AUTOLOAD_SCT3_ADDR")
            .field(
                "l2_cache_autoload_sct3_addr",
                &format_args!("{}", self.l2_cache_autoload_sct3_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "L2 Cache autoload section 3 address configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_sct3_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC;
impl crate::RegisterSpec for L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_autoload_sct3_addr::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC {}
#[doc = "`reset()` method sets L2_CACHE_AUTOLOAD_SCT3_ADDR to value 0"]
impl crate::Resettable for L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
