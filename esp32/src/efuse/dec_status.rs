#[doc = "Register `DEC_STATUS` reader"]
pub type R = crate::R<DEC_STATUS_SPEC>;
#[doc = "Field `DEC_WARNINGS` reader - "]
pub type DEC_WARNINGS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dec_warnings(&self) -> DEC_WARNINGS_R {
        DEC_WARNINGS_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEC_STATUS")
            .field("dec_warnings", &self.dec_warnings())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dec_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEC_STATUS_SPEC;
impl crate::RegisterSpec for DEC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dec_status::R`](R) reader structure"]
impl crate::Readable for DEC_STATUS_SPEC {}
#[doc = "`reset()` method sets DEC_STATUS to value 0"]
impl crate::Resettable for DEC_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
