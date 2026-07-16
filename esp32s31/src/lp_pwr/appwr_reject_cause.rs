#[doc = "Register `APPWR_REJECT_CAUSE` reader"]
pub type R = crate::R<APPWR_REJECT_CAUSE_SPEC>;
#[doc = "Field `APPWR_REJECT_CAUSE` reader - reject cause for appwr"]
pub type APPWR_REJECT_CAUSE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - reject cause for appwr"]
    #[inline(always)]
    pub fn appwr_reject_cause(&self) -> APPWR_REJECT_CAUSE_R {
        APPWR_REJECT_CAUSE_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPWR_REJECT_CAUSE")
            .field("appwr_reject_cause", &self.appwr_reject_cause())
            .finish()
    }
}
#[doc = "reject cause register for appwr\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_reject_cause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPWR_REJECT_CAUSE_SPEC;
impl crate::RegisterSpec for APPWR_REJECT_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appwr_reject_cause::R`](R) reader structure"]
impl crate::Readable for APPWR_REJECT_CAUSE_SPEC {}
#[doc = "`reset()` method sets APPWR_REJECT_CAUSE to value 0"]
impl crate::Resettable for APPWR_REJECT_CAUSE_SPEC {}
