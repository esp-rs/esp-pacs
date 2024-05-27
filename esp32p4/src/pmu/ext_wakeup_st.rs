///Register `EXT_WAKEUP_ST` reader
pub type R = crate::R<EXT_WAKEUP_ST_SPEC>;
///Field `EXT_WAKEUP_STATUS` reader - need_des
pub type EXT_WAKEUP_STATUS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn ext_wakeup_status(&self) -> EXT_WAKEUP_STATUS_R {
        EXT_WAKEUP_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_ST")
            .field("ext_wakeup_status", &self.ext_wakeup_status())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXT_WAKEUP_ST_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ext_wakeup_st::R`](R) reader structure
impl crate::Readable for EXT_WAKEUP_ST_SPEC {}
///`reset()` method sets EXT_WAKEUP_ST to value 0
impl crate::Resettable for EXT_WAKEUP_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
