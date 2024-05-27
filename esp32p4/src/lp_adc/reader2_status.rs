#[doc = "Register `READER2_STATUS` reader"]
pub type R = crate::R<READER2_STATUS_SPEC>;
#[doc = "Field `SAR2_READER_STATUS` reader - N/A"]
pub type SAR2_READER_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn sar2_reader_status(&self) -> SAR2_READER_STATUS_R {
        SAR2_READER_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("READER2_STATUS")
            .field("sar2_reader_status", &self.sar2_reader_status())
            .finish()
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reader2_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READER2_STATUS_SPEC;
impl crate::RegisterSpec for READER2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reader2_status::R`](R) reader structure"]
impl crate::Readable for READER2_STATUS_SPEC {}
#[doc = "`reset()` method sets READER2_STATUS to value 0"]
impl crate::Resettable for READER2_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
