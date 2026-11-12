#[doc = "Register `L1_BUS3_ACS_NXTLVL_RD_CNT` reader"]
pub type R = crate::R<L1_BUS3_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "Field `L1_BUS3_NXTLVL_RD_CNT` reader - The register records the number of times that L1-Cache accesses L2-Cache due to bus3 accessing L1-Cache."]
pub type L1_BUS3_NXTLVL_RD_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L1-Cache accesses L2-Cache due to bus3 accessing L1-Cache."]
    #[inline(always)]
    pub fn l1_bus3_nxtlvl_rd_cnt(&self) -> L1_BUS3_NXTLVL_RD_CNT_R {
        L1_BUS3_NXTLVL_RD_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_BUS3_ACS_NXTLVL_RD_CNT")
            .field("l1_bus3_nxtlvl_rd_cnt", &self.l1_bus3_nxtlvl_rd_cnt())
            .finish()
    }
}
#[doc = "L1-Cache bus3 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_bus3_acs_nxtlvl_rd_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_BUS3_ACS_NXTLVL_RD_CNT_SPEC;
impl crate::RegisterSpec for L1_BUS3_ACS_NXTLVL_RD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_bus3_acs_nxtlvl_rd_cnt::R`](R) reader structure"]
impl crate::Readable for L1_BUS3_ACS_NXTLVL_RD_CNT_SPEC {}
#[doc = "`reset()` method sets L1_BUS3_ACS_NXTLVL_RD_CNT to value 0"]
impl crate::Resettable for L1_BUS3_ACS_NXTLVL_RD_CNT_SPEC {}
