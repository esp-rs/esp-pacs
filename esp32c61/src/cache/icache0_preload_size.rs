#[doc = "Register `ICACHE0_PRELOAD_SIZE` reader"]
pub type R = crate::R<ICACHE0_PRELOAD_SIZE_SPEC>;
#[doc = "Field `ICACHE0_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_ADDR_REG"]
pub type ICACHE0_PRELOAD_SIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn icache0_preload_size(&self) -> ICACHE0_PRELOAD_SIZE_R {
        ICACHE0_PRELOAD_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE0_PRELOAD_SIZE")
            .field("icache0_preload_size", &self.icache0_preload_size())
            .finish()
    }
}
#[doc = "L1 instruction Cache 0 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache0_preload_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE0_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE0_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache0_preload_size::R`](R) reader structure"]
impl crate::Readable for ICACHE0_PRELOAD_SIZE_SPEC {}
#[doc = "`reset()` method sets ICACHE0_PRELOAD_SIZE to value 0"]
impl crate::Resettable for ICACHE0_PRELOAD_SIZE_SPEC {}
