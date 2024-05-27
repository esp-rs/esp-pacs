///Register `L1_BUS0_ACS_MISS_CNT` reader
pub type R = crate::R<L1_BUS0_ACS_MISS_CNT_SPEC>;
///Field `L1_BUS0_MISS_CNT` reader - The register records the number of missing when bus0 accesses L1-Cache.
pub type L1_BUS0_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The register records the number of missing when bus0 accesses L1-Cache.
    #[inline(always)]
    pub fn l1_bus0_miss_cnt(&self) -> L1_BUS0_MISS_CNT_R {
        L1_BUS0_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_BUS0_ACS_MISS_CNT")
            .field("l1_bus0_miss_cnt", &self.l1_bus0_miss_cnt())
            .finish()
    }
}
/**L1-Cache bus0 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_bus0_acs_miss_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_BUS0_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for L1_BUS0_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_bus0_acs_miss_cnt::R`](R) reader structure
impl crate::Readable for L1_BUS0_ACS_MISS_CNT_SPEC {}
///`reset()` method sets L1_BUS0_ACS_MISS_CNT to value 0
impl crate::Resettable for L1_BUS0_ACS_MISS_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
