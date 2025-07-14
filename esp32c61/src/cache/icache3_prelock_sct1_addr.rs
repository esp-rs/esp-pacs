#[doc = "Register `ICACHE3_PRELOCK_SCT1_ADDR` reader"]
pub type R = crate::R<ICACHE3_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "Field `ICACHE3_PRELOCK_SCT1_ADDR` reader - Those bits are used to configure the start address of the second section of prelock on L1-ICache3, which should be used together with L1_ICACHE3_PRELOCK_SCT1_SIZE_REG"]
pub type ICACHE3_PRELOCK_SCT1_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of the second section of prelock on L1-ICache3, which should be used together with L1_ICACHE3_PRELOCK_SCT1_SIZE_REG"]
    #[inline(always)]
    pub fn icache3_prelock_sct1_addr(&self) -> ICACHE3_PRELOCK_SCT1_ADDR_R {
        ICACHE3_PRELOCK_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE3_PRELOCK_SCT1_ADDR")
            .field(
                "icache3_prelock_sct1_addr",
                &self.icache3_prelock_sct1_addr(),
            )
            .finish()
    }
}
#[doc = "L1 instruction Cache 3 prelock section1 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_prelock_sct1_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE3_PRELOCK_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE3_PRELOCK_SCT1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache3_prelock_sct1_addr::R`](R) reader structure"]
impl crate::Readable for ICACHE3_PRELOCK_SCT1_ADDR_SPEC {}
#[doc = "`reset()` method sets ICACHE3_PRELOCK_SCT1_ADDR to value 0"]
impl crate::Resettable for ICACHE3_PRELOCK_SCT1_ADDR_SPEC {}
