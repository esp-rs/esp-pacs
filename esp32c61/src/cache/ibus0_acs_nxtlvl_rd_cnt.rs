#[doc = "Register `IBUS0_ACS_NXTLVL_RD_CNT` reader"]
pub type R = crate::R<IBUS0_ACS_NXTLVL_RD_CNT_SPEC>;
#[doc = "Field `IBUS0_NXTLVL_RD_CNT` reader - The register records the number of times that L1-ICache accesses L2-Cache due to bus0 accessing L1-ICache0."]
pub type IBUS0_NXTLVL_RD_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L1-ICache accesses L2-Cache due to bus0 accessing L1-ICache0."]
    #[inline(always)]
    pub fn ibus0_nxtlvl_rd_cnt(&self) -> IBUS0_NXTLVL_RD_CNT_R {
        IBUS0_NXTLVL_RD_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS0_ACS_NXTLVL_RD_CNT")
            .field("ibus0_nxtlvl_rd_cnt", &self.ibus0_nxtlvl_rd_cnt())
            .finish()
    }
}
#[doc = "L1-ICache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibus0_acs_nxtlvl_rd_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBUS0_ACS_NXTLVL_RD_CNT_SPEC;
impl crate::RegisterSpec for IBUS0_ACS_NXTLVL_RD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibus0_acs_nxtlvl_rd_cnt::R`](R) reader structure"]
impl crate::Readable for IBUS0_ACS_NXTLVL_RD_CNT_SPEC {}
#[doc = "`reset()` method sets IBUS0_ACS_NXTLVL_RD_CNT to value 0"]
impl crate::Resettable for IBUS0_ACS_NXTLVL_RD_CNT_SPEC {}
