#[doc = "Register `SLP_WAKEUP_STATUS0` reader"]
pub struct R(crate::R<SLP_WAKEUP_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_WAKEUP_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_WAKEUP_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_WAKEUP_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WAKEUP_CAUSE` reader - need_des"]
pub type WAKEUP_CAUSE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WAKEUP_CAUSE_R {
        WAKEUP_CAUSE_R::new(self.bits)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wakeup_status0](index.html) module"]
pub struct SLP_WAKEUP_STATUS0_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_wakeup_status0::R](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLP_WAKEUP_STATUS0 to value 0"]
impl crate::Resettable for SLP_WAKEUP_STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
