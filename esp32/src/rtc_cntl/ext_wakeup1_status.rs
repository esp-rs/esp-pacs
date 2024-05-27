#[doc = "Register `EXT_WAKEUP1_STATUS` reader"]
pub type R = crate::R<EXT_WAKEUP1_STATUS_SPEC>;
#[doc = "Field `EXT_WAKEUP1_STATUS` reader - ext wakeup1 status"]
pub type EXT_WAKEUP1_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - ext wakeup1 status"]
    #[inline(always)]
    pub fn ext_wakeup1_status(&self) -> EXT_WAKEUP1_STATUS_R {
        EXT_WAKEUP1_STATUS_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP1_STATUS")
            .field("ext_wakeup1_status", &self.ext_wakeup1_status())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup1_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP1_STATUS_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup1_status::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP1_STATUS_SPEC {}
#[doc = "`reset()` method sets EXT_WAKEUP1_STATUS to value 0"]
impl crate::Resettable for EXT_WAKEUP1_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
