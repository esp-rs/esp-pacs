#[doc = "Register `BUS1_ACS_MISS_CNT` reader"]
pub type R = crate::R<BUS1_ACS_MISS_CNT_SPEC>;
#[doc = "Field `BUS1_MISS_CNT` reader - The register records the number of missing when bus1 accesses L1-Cache."]
pub type BUS1_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when bus1 accesses L1-Cache."]
    #[inline(always)]
    pub fn bus1_miss_cnt(&self) -> BUS1_MISS_CNT_R {
        BUS1_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS1_ACS_MISS_CNT")
            .field("bus1_miss_cnt", &self.bus1_miss_cnt())
            .finish()
    }
}
#[doc = "L1-Cache bus1 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus1_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS1_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for BUS1_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus1_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for BUS1_ACS_MISS_CNT_SPEC {}
#[doc = "`reset()` method sets BUS1_ACS_MISS_CNT to value 0"]
impl crate::Resettable for BUS1_ACS_MISS_CNT_SPEC {}
