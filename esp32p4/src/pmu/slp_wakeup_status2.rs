///Register `SLP_WAKEUP_STATUS2` reader
pub type R = crate::R<SLP_WAKEUP_STATUS2_SPEC>;
///Field `LP_LITE_WAKEUP_CAUSE` reader - need_des
pub type LP_LITE_WAKEUP_CAUSE_R = crate::BitReader;
impl R {
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn lp_lite_wakeup_cause(&self) -> LP_LITE_WAKEUP_CAUSE_R {
        LP_LITE_WAKEUP_CAUSE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_STATUS2")
            .field("lp_lite_wakeup_cause", &self.lp_lite_wakeup_cause())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_status2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLP_WAKEUP_STATUS2_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_STATUS2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`slp_wakeup_status2::R`](R) reader structure
impl crate::Readable for SLP_WAKEUP_STATUS2_SPEC {}
///`reset()` method sets SLP_WAKEUP_STATUS2 to value 0
impl crate::Resettable for SLP_WAKEUP_STATUS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
