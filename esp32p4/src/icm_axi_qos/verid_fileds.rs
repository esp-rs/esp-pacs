#[doc = "Register `VERID_FILEDS` reader"]
pub type R = crate::R<VERID_FILEDS_SPEC>;
#[doc = "Field `VERID` reader - "]
pub type VERID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn verid(&self) -> VERID_R {
        VERID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERID_FILEDS")
            .field("verid", &self.verid())
            .finish()
    }
}
#[doc = "QoS block version ID (typo matches IDF symbol)\n\nYou can [`read`](crate::Reg::read) this register and get [`verid_fileds::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERID_FILEDS_SPEC;
impl crate::RegisterSpec for VERID_FILEDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verid_fileds::R`](R) reader structure"]
impl crate::Readable for VERID_FILEDS_SPEC {}
#[doc = "`reset()` method sets VERID_FILEDS to value 0"]
impl crate::Resettable for VERID_FILEDS_SPEC {}
