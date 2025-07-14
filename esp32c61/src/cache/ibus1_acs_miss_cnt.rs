#[doc = "Register `IBUS1_ACS_MISS_CNT` reader"]
pub type R = crate::R<IBUS1_ACS_MISS_CNT_SPEC>;
#[doc = "Field `IBUS1_MISS_CNT` reader - The register records the number of missing when bus1 accesses L1-ICache1."]
pub type IBUS1_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn ibus1_miss_cnt(&self) -> IBUS1_MISS_CNT_R {
        IBUS1_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS1_ACS_MISS_CNT")
            .field("ibus1_miss_cnt", &self.ibus1_miss_cnt())
            .finish()
    }
}
#[doc = "L1-ICache bus1 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus1_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBUS1_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for IBUS1_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibus1_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for IBUS1_ACS_MISS_CNT_SPEC {}
#[doc = "`reset()` method sets IBUS1_ACS_MISS_CNT to value 0"]
impl crate::Resettable for IBUS1_ACS_MISS_CNT_SPEC {}
