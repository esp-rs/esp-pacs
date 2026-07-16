#[doc = "Register `PERI1_TIMEOUT_UID` reader"]
pub type R = crate::R<PERI1_TIMEOUT_UID_SPEC>;
#[doc = "Field `PERI1_TIMEOUT_UID` reader - Represents the master id\\[5:0\\] and master permission\\[6:5\\] when trigger timeout. This register will be cleared after the interrupt is cleared."]
pub type PERI1_TIMEOUT_UID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Represents the master id\\[5:0\\] and master permission\\[6:5\\] when trigger timeout. This register will be cleared after the interrupt is cleared."]
    #[inline(always)]
    pub fn peri1_timeout_uid(&self) -> PERI1_TIMEOUT_UID_R {
        PERI1_TIMEOUT_UID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI1_TIMEOUT_UID")
            .field("peri1_timeout_uid", &self.peri1_timeout_uid())
            .finish()
    }
}
#[doc = "HP_PERI1_TIMEOUT_UID register\n\nYou can [`read`](crate::Reg::read) this register and get [`peri1_timeout_uid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI1_TIMEOUT_UID_SPEC;
impl crate::RegisterSpec for PERI1_TIMEOUT_UID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri1_timeout_uid::R`](R) reader structure"]
impl crate::Readable for PERI1_TIMEOUT_UID_SPEC {}
#[doc = "`reset()` method sets PERI1_TIMEOUT_UID to value 0"]
impl crate::Resettable for PERI1_TIMEOUT_UID_SPEC {}
