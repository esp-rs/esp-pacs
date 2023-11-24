#[doc = "Register `L1_ICACHE1_AUTOLOAD_SCT1_SIZE` reader"]
pub type R = crate::R<L1_ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "Field `L1_ICACHE1_AUTOLOAD_SCT1_SIZE` reader - Those bits are used to configure the size of the second section for autoload operation on L1-ICache1. Note that it should be used together with L1_ICACHE1_AUTOLOAD_SCT1_ADDR and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
pub type L1_ICACHE1_AUTOLOAD_SCT1_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on L1-ICache1. Note that it should be used together with L1_ICACHE1_AUTOLOAD_SCT1_ADDR and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn l1_icache1_autoload_sct1_size(&self) -> L1_ICACHE1_AUTOLOAD_SCT1_SIZE_R {
        L1_ICACHE1_AUTOLOAD_SCT1_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE1_AUTOLOAD_SCT1_SIZE")
            .field(
                "l1_icache1_autoload_sct1_size",
                &format_args!("{}", self.l1_icache1_autoload_sct1_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "L1 instruction Cache 1 autoload section 1 size configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_autoload_sct1_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC;
impl crate::RegisterSpec for L1_ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache1_autoload_sct1_size::R`](R) reader structure"]
impl crate::Readable for L1_ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC {}
#[doc = "`reset()` method sets L1_ICACHE1_AUTOLOAD_SCT1_SIZE to value 0"]
impl crate::Resettable for L1_ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
