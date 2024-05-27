///Register `L1_IBUS1_ACS_CONFLICT_CNT` reader
pub type R = crate::R<L1_IBUS1_ACS_CONFLICT_CNT_SPEC>;
///Field `L1_IBUS1_CONFLICT_CNT` reader - The register records the number of access-conflicts when bus1 accesses L1-ICache1.
pub type L1_IBUS1_CONFLICT_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The register records the number of access-conflicts when bus1 accesses L1-ICache1.
    #[inline(always)]
    pub fn l1_ibus1_conflict_cnt(&self) -> L1_IBUS1_CONFLICT_CNT_R {
        L1_IBUS1_CONFLICT_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_IBUS1_ACS_CONFLICT_CNT")
            .field("l1_ibus1_conflict_cnt", &self.l1_ibus1_conflict_cnt())
            .finish()
    }
}
/**L1-ICache bus1 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus1_acs_conflict_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_IBUS1_ACS_CONFLICT_CNT_SPEC;
impl crate::RegisterSpec for L1_IBUS1_ACS_CONFLICT_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_ibus1_acs_conflict_cnt::R`](R) reader structure
impl crate::Readable for L1_IBUS1_ACS_CONFLICT_CNT_SPEC {}
///`reset()` method sets L1_IBUS1_ACS_CONFLICT_CNT to value 0
impl crate::Resettable for L1_IBUS1_ACS_CONFLICT_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
