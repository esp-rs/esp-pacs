#[doc = "Register `ICACHE1_PRELOAD_SIZE` reader"]
pub type R = crate::R<ICACHE1_PRELOAD_SIZE_SPEC>;
#[doc = "Field `ICACHE1_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_ADDR_REG"]
pub type ICACHE1_PRELOAD_SIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn icache1_preload_size(&self) -> ICACHE1_PRELOAD_SIZE_R {
        ICACHE1_PRELOAD_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE1_PRELOAD_SIZE")
            .field("icache1_preload_size", &self.icache1_preload_size())
            .finish()
    }
}
#[doc = "L1 instruction Cache 1 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_preload_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE1_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE1_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache1_preload_size::R`](R) reader structure"]
impl crate::Readable for ICACHE1_PRELOAD_SIZE_SPEC {}
#[doc = "`reset()` method sets ICACHE1_PRELOAD_SIZE to value 0"]
impl crate::Resettable for ICACHE1_PRELOAD_SIZE_SPEC {}
