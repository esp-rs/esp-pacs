#[doc = "Register `EXT_WAKEUP_ST` reader"]
pub type R = crate::R<EXT_WAKEUP_ST_SPEC>;
#[doc = "Field `EXT_WAKEUP_STATUS` reader - need_des"]
pub type EXT_WAKEUP_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_status(&self) -> EXT_WAKEUP_STATUS_R {
        EXT_WAKEUP_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_ST")
            .field(
                "ext_wakeup_status",
                &format_args!("{}", self.ext_wakeup_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP_ST_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_st::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_ST_SPEC {}
#[doc = "`reset()` method sets EXT_WAKEUP_ST to value 0"]
impl crate::Resettable for EXT_WAKEUP_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
