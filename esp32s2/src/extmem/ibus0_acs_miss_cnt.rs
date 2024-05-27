///Register `IBUS0_ACS_MISS_CNT` reader
pub type R = crate::R<IBUS0_ACS_MISS_CNT_SPEC>;
///Field `IBUS0_ACS_MISS_CNT` reader - The bits are used to count the number of the cache miss caused by ibus0 access.
pub type IBUS0_ACS_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The bits are used to count the number of the cache miss caused by ibus0 access.
    #[inline(always)]
    pub fn ibus0_acs_miss_cnt(&self) -> IBUS0_ACS_MISS_CNT_R {
        IBUS0_ACS_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS0_ACS_MISS_CNT")
            .field("ibus0_acs_miss_cnt", &self.ibus0_acs_miss_cnt())
            .finish()
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`ibus0_acs_miss_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IBUS0_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for IBUS0_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ibus0_acs_miss_cnt::R`](R) reader structure
impl crate::Readable for IBUS0_ACS_MISS_CNT_SPEC {}
///`reset()` method sets IBUS0_ACS_MISS_CNT to value 0
impl crate::Resettable for IBUS0_ACS_MISS_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
