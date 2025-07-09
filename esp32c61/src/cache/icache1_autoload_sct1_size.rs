#[doc = "Register `ICACHE1_AUTOLOAD_SCT1_SIZE` reader"]
pub type R = crate::R<ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "Field `ICACHE1_AUTOLOAD_SCT1_SIZE` reader - Those bits are used to configure the size of the second section for autoload operation on L1-ICache1. Note that it should be used together with L1_ICACHE1_AUTOLOAD_SCT1_ADDR and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
pub type ICACHE1_AUTOLOAD_SCT1_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the second section for autoload operation on L1-ICache1. Note that it should be used together with L1_ICACHE1_AUTOLOAD_SCT1_ADDR and L1_ICACHE_AUTOLOAD_SCT1_ENA."]
    #[inline(always)]
    pub fn icache1_autoload_sct1_size(&self) -> ICACHE1_AUTOLOAD_SCT1_SIZE_R {
        ICACHE1_AUTOLOAD_SCT1_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE1_AUTOLOAD_SCT1_SIZE")
            .field(
                "icache1_autoload_sct1_size",
                &self.icache1_autoload_sct1_size(),
            )
            .finish()
    }
}
#[doc = "L1 instruction Cache 1 autoload section 1 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_autoload_sct1_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache1_autoload_sct1_size::R`](R) reader structure"]
impl crate::Readable for ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC {}
#[doc = "`reset()` method sets ICACHE1_AUTOLOAD_SCT1_SIZE to value 0"]
impl crate::Resettable for ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC {}
