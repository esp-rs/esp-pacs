#[doc = "Register `BUS_TIMEOUT_UID` reader"]
pub type R = crate::R<BUS_TIMEOUT_UID_SPEC>;
#[doc = "Field `LP_PERI_TIMEOUT_UID` reader - when bus timeout, this register will record the timeout master device"]
pub type LP_PERI_TIMEOUT_UID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - when bus timeout, this register will record the timeout master device"]
    #[inline(always)]
    pub fn lp_peri_timeout_uid(&self) -> LP_PERI_TIMEOUT_UID_R {
        LP_PERI_TIMEOUT_UID_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_TIMEOUT_UID")
            .field("lp_peri_timeout_uid", &self.lp_peri_timeout_uid())
            .finish()
    }
}
#[doc = "the timeout master id register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_timeout_uid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_TIMEOUT_UID_SPEC;
impl crate::RegisterSpec for BUS_TIMEOUT_UID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_timeout_uid::R`](R) reader structure"]
impl crate::Readable for BUS_TIMEOUT_UID_SPEC {}
#[doc = "`reset()` method sets BUS_TIMEOUT_UID to value 0"]
impl crate::Resettable for BUS_TIMEOUT_UID_SPEC {}
