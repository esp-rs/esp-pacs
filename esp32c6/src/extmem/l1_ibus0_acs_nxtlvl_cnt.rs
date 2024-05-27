#[doc = "Register `L1_IBUS0_ACS_NXTLVL_CNT` reader"]
pub type R = crate::R<L1_IBUS0_ACS_NXTLVL_CNT_SPEC>;
#[doc = "Field `L1_IBUS0_NXTLVL_CNT` reader - The register records the number of times that L1-ICache accesses L2-Cache due to bus0 accessing L1-ICache0."]
pub type L1_IBUS0_NXTLVL_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L1-ICache accesses L2-Cache due to bus0 accessing L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_nxtlvl_cnt(&self) -> L1_IBUS0_NXTLVL_CNT_R {
        L1_IBUS0_NXTLVL_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_IBUS0_ACS_NXTLVL_CNT")
            .field("l1_ibus0_nxtlvl_cnt", &self.l1_ibus0_nxtlvl_cnt())
            .finish()
    }
}
#[doc = "L1-ICache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus0_acs_nxtlvl_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_IBUS0_ACS_NXTLVL_CNT_SPEC;
impl crate::RegisterSpec for L1_IBUS0_ACS_NXTLVL_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_ibus0_acs_nxtlvl_cnt::R`](R) reader structure"]
impl crate::Readable for L1_IBUS0_ACS_NXTLVL_CNT_SPEC {}
#[doc = "`reset()` method sets L1_IBUS0_ACS_NXTLVL_CNT to value 0"]
impl crate::Resettable for L1_IBUS0_ACS_NXTLVL_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
