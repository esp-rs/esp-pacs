#[doc = "Register `ICACHE0_PRELOAD_ADDR` reader"]
pub type R = crate::R<ICACHE0_PRELOAD_ADDR_SPEC>;
#[doc = "Field `ICACHE0_PRELOAD_ADDR` reader - Those bits are used to configure the start address of preload on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_SIZE_REG"]
pub type ICACHE0_PRELOAD_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of preload on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_SIZE_REG"]
    #[inline(always)]
    pub fn icache0_preload_addr(&self) -> ICACHE0_PRELOAD_ADDR_R {
        ICACHE0_PRELOAD_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE0_PRELOAD_ADDR")
            .field("icache0_preload_addr", &self.icache0_preload_addr())
            .finish()
    }
}
#[doc = "L1 instruction Cache 0 preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_preload_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE0_PRELOAD_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE0_PRELOAD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache0_preload_addr::R`](R) reader structure"]
impl crate::Readable for ICACHE0_PRELOAD_ADDR_SPEC {}
#[doc = "`reset()` method sets ICACHE0_PRELOAD_ADDR to value 0"]
impl crate::Resettable for ICACHE0_PRELOAD_ADDR_SPEC {}
