#[doc = "Register `ICACHE3_PRELOCK_SCT_SIZE` reader"]
pub type R = crate::R<ICACHE3_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Field `ICACHE3_PRELOCK_SCT0_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache3, which should be used together with L1_ICACHE3_PRELOCK_SCT0_ADDR_REG"]
pub type ICACHE3_PRELOCK_SCT0_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `ICACHE3_PRELOCK_SCT1_SIZE` reader - Those bits are used to configure the size of the second section of prelock on L1-ICache3, which should be used together with L1_ICACHE3_PRELOCK_SCT1_ADDR_REG"]
pub type ICACHE3_PRELOCK_SCT1_SIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache3, which should be used together with L1_ICACHE3_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn icache3_prelock_sct0_size(&self) -> ICACHE3_PRELOCK_SCT0_SIZE_R {
        ICACHE3_PRELOCK_SCT0_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Those bits are used to configure the size of the second section of prelock on L1-ICache3, which should be used together with L1_ICACHE3_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn icache3_prelock_sct1_size(&self) -> ICACHE3_PRELOCK_SCT1_SIZE_R {
        ICACHE3_PRELOCK_SCT1_SIZE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE3_PRELOCK_SCT_SIZE")
            .field(
                "icache3_prelock_sct0_size",
                &self.icache3_prelock_sct0_size(),
            )
            .field(
                "icache3_prelock_sct1_size",
                &self.icache3_prelock_sct1_size(),
            )
            .finish()
    }
}
#[doc = "L1 instruction Cache 3 prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache3_prelock_sct_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE3_PRELOCK_SCT_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE3_PRELOCK_SCT_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache3_prelock_sct_size::R`](R) reader structure"]
impl crate::Readable for ICACHE3_PRELOCK_SCT_SIZE_SPEC {}
#[doc = "`reset()` method sets ICACHE3_PRELOCK_SCT_SIZE to value 0x3fff_3fff"]
impl crate::Resettable for ICACHE3_PRELOCK_SCT_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x3fff_3fff;
}
