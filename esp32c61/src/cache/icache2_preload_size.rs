#[doc = "Register `ICACHE2_PRELOAD_SIZE` reader"]
pub type R = crate::R<ICACHE2_PRELOAD_SIZE_SPEC>;
#[doc = "Field `ICACHE2_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOAD_ADDR_REG"]
pub type ICACHE2_PRELOAD_SIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn icache2_preload_size(&self) -> ICACHE2_PRELOAD_SIZE_R {
        ICACHE2_PRELOAD_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_PRELOAD_SIZE")
            .field("icache2_preload_size", &self.icache2_preload_size())
            .finish()
    }
}
#[doc = "L1 instruction Cache 2 preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_preload_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE2_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_preload_size::R`](R) reader structure"]
impl crate::Readable for ICACHE2_PRELOAD_SIZE_SPEC {}
#[doc = "`reset()` method sets ICACHE2_PRELOAD_SIZE to value 0"]
impl crate::Resettable for ICACHE2_PRELOAD_SIZE_SPEC {}
