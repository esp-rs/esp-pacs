#[doc = "Register `ICACHE2_AUTOLOAD_SCT0_SIZE` reader"]
pub type R = crate::R<ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "Field `ICACHE2_AUTOLOAD_SCT0_SIZE` reader - Those bits are used to configure the size of the first section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT0_ADDR and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
pub type ICACHE2_AUTOLOAD_SCT0_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the first section for autoload operation on L1-ICache2. Note that it should be used together with L1_ICACHE2_AUTOLOAD_SCT0_ADDR and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn icache2_autoload_sct0_size(&self) -> ICACHE2_AUTOLOAD_SCT0_SIZE_R {
        ICACHE2_AUTOLOAD_SCT0_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_AUTOLOAD_SCT0_SIZE")
            .field(
                "icache2_autoload_sct0_size",
                &self.icache2_autoload_sct0_size(),
            )
            .finish()
    }
}
#[doc = "L1 instruction Cache 2 autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_autoload_sct0_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_autoload_sct0_size::R`](R) reader structure"]
impl crate::Readable for ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC {}
#[doc = "`reset()` method sets ICACHE2_AUTOLOAD_SCT0_SIZE to value 0"]
impl crate::Resettable for ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC {}
