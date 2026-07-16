#[doc = "Register `LP_PERI_TIMEOUT_UID` reader"]
pub type R = crate::R<LP_PERI_TIMEOUT_UID_SPEC>;
#[doc = "Field `LP_PERI_TIMEOUT_UID` reader - "]
pub type LP_PERI_TIMEOUT_UID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn lp_peri_timeout_uid(&self) -> LP_PERI_TIMEOUT_UID_R {
        LP_PERI_TIMEOUT_UID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PERI_TIMEOUT_UID")
            .field("lp_peri_timeout_uid", &self.lp_peri_timeout_uid())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri_timeout_uid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_PERI_TIMEOUT_UID_SPEC;
impl crate::RegisterSpec for LP_PERI_TIMEOUT_UID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_peri_timeout_uid::R`](R) reader structure"]
impl crate::Readable for LP_PERI_TIMEOUT_UID_SPEC {}
#[doc = "`reset()` method sets LP_PERI_TIMEOUT_UID to value 0"]
impl crate::Resettable for LP_PERI_TIMEOUT_UID_SPEC {}
