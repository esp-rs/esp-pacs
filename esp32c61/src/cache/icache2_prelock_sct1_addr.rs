#[doc = "Register `ICACHE2_PRELOCK_SCT1_ADDR` reader"]
pub type R = crate::R<ICACHE2_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "Field `ICACHE2_PRELOCK_SCT1_ADDR` reader - Those bits are used to configure the start address of the second section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOCK_SCT1_SIZE_REG"]
pub type ICACHE2_PRELOCK_SCT1_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of the second section of prelock on L1-ICache2, which should be used together with L1_ICACHE2_PRELOCK_SCT1_SIZE_REG"]
    #[inline(always)]
    pub fn icache2_prelock_sct1_addr(&self) -> ICACHE2_PRELOCK_SCT1_ADDR_R {
        ICACHE2_PRELOCK_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE2_PRELOCK_SCT1_ADDR")
            .field(
                "icache2_prelock_sct1_addr",
                &self.icache2_prelock_sct1_addr(),
            )
            .finish()
    }
}
#[doc = "L1 instruction Cache 2 prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache2_prelock_sct1_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE2_PRELOCK_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE2_PRELOCK_SCT1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache2_prelock_sct1_addr::R`](R) reader structure"]
impl crate::Readable for ICACHE2_PRELOCK_SCT1_ADDR_SPEC {}
#[doc = "`reset()` method sets ICACHE2_PRELOCK_SCT1_ADDR to value 0"]
impl crate::Resettable for ICACHE2_PRELOCK_SCT1_ADDR_SPEC {}
