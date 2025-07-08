#[doc = "Register `BUS0_ACS_HIT_CNT` reader"]
pub type R = crate::R<BUS0_ACS_HIT_CNT_SPEC>;
#[doc = "Field `BUS0_HIT_CNT` reader - The register records the number of hits when bus0 accesses L1-Cache."]
pub type BUS0_HIT_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of hits when bus0 accesses L1-Cache."]
    #[inline(always)]
    pub fn bus0_hit_cnt(&self) -> BUS0_HIT_CNT_R {
        BUS0_HIT_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS0_ACS_HIT_CNT")
            .field("bus0_hit_cnt", &self.bus0_hit_cnt())
            .finish()
    }
}
#[doc = "L1-Cache bus0 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus0_acs_hit_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS0_ACS_HIT_CNT_SPEC;
impl crate::RegisterSpec for BUS0_ACS_HIT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus0_acs_hit_cnt::R`](R) reader structure"]
impl crate::Readable for BUS0_ACS_HIT_CNT_SPEC {}
#[doc = "`reset()` method sets BUS0_ACS_HIT_CNT to value 0"]
impl crate::Resettable for BUS0_ACS_HIT_CNT_SPEC {}
