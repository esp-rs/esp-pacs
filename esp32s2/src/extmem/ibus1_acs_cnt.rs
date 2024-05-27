///Register `IBUS1_ACS_CNT` reader
pub type R = crate::R<IBUS1_ACS_CNT_SPEC>;
///Field `IBUS1_ACS_CNT` reader - The bits are used to count the number of ibus1 access icache.
pub type IBUS1_ACS_CNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The bits are used to count the number of ibus1 access icache.
    #[inline(always)]
    pub fn ibus1_acs_cnt(&self) -> IBUS1_ACS_CNT_R {
        IBUS1_ACS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS1_ACS_CNT")
            .field("ibus1_acs_cnt", &self.ibus1_acs_cnt())
            .finish()
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`ibus1_acs_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IBUS1_ACS_CNT_SPEC;
impl crate::RegisterSpec for IBUS1_ACS_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ibus1_acs_cnt::R`](R) reader structure
impl crate::Readable for IBUS1_ACS_CNT_SPEC {}
///`reset()` method sets IBUS1_ACS_CNT to value 0
impl crate::Resettable for IBUS1_ACS_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
