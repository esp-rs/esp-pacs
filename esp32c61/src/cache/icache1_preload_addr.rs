#[doc = "Register `ICACHE1_PRELOAD_ADDR` reader"]
pub type R = crate::R<ICACHE1_PRELOAD_ADDR_SPEC>;
#[doc = "Field `ICACHE1_PRELOAD_ADDR` reader - Those bits are used to configure the start address of preload on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_SIZE_REG"]
pub type ICACHE1_PRELOAD_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of preload on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_SIZE_REG"]
    #[inline(always)]
    pub fn icache1_preload_addr(&self) -> ICACHE1_PRELOAD_ADDR_R {
        ICACHE1_PRELOAD_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE1_PRELOAD_ADDR")
            .field("icache1_preload_addr", &self.icache1_preload_addr())
            .finish()
    }
}
#[doc = "L1 instruction Cache 1 preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache1_preload_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE1_PRELOAD_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE1_PRELOAD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache1_preload_addr::R`](R) reader structure"]
impl crate::Readable for ICACHE1_PRELOAD_ADDR_SPEC {}
#[doc = "`reset()` method sets ICACHE1_PRELOAD_ADDR to value 0"]
impl crate::Resettable for ICACHE1_PRELOAD_ADDR_SPEC {}
