#[doc = "Register `ICACHE3_AUTOLOAD_SCT0_SIZE` reader"]
pub type R = crate::R<ICACHE3_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "Field `ICACHE3_AUTOLOAD_SCT0_SIZE` reader - Those bits are used to configure the size of the first section for autoload operation on L1-ICache3. Note that it should be used together with L1_ICACHE3_AUTOLOAD_SCT0_ADDR and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
pub type ICACHE3_AUTOLOAD_SCT0_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the first section for autoload operation on L1-ICache3. Note that it should be used together with L1_ICACHE3_AUTOLOAD_SCT0_ADDR and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn icache3_autoload_sct0_size(&self) -> ICACHE3_AUTOLOAD_SCT0_SIZE_R {
        ICACHE3_AUTOLOAD_SCT0_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE3_AUTOLOAD_SCT0_SIZE")
            .field(
                "icache3_autoload_sct0_size",
                &self.icache3_autoload_sct0_size(),
            )
            .finish()
    }
}
#[doc = "L1 instruction Cache 3 autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_autoload_sct0_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE3_AUTOLOAD_SCT0_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE3_AUTOLOAD_SCT0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache3_autoload_sct0_size::R`](R) reader structure"]
impl crate::Readable for ICACHE3_AUTOLOAD_SCT0_SIZE_SPEC {}
#[doc = "`reset()` method sets ICACHE3_AUTOLOAD_SCT0_SIZE to value 0"]
impl crate::Resettable for ICACHE3_AUTOLOAD_SCT0_SIZE_SPEC {}
