#[doc = "Register `SAR1_STATUS` reader"]
pub type R = crate::R<SAR1_STATUS_SPEC>;
#[doc = "Field `SAR1_STATE` reader - "]
pub type SAR1_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sar1_state(&self) -> SAR1_STATE_R {
        SAR1_STATE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1_STATUS")
            .field("sar1_state", &self.sar1_state())
            .finish()
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR1_STATUS_SPEC;
impl crate::RegisterSpec for SAR1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1_status::R`](R) reader structure"]
impl crate::Readable for SAR1_STATUS_SPEC {}
#[doc = "`reset()` method sets SAR1_STATUS to value 0"]
impl crate::Resettable for SAR1_STATUS_SPEC {}
