#[doc = "Register `SAR_READ_STATUS2` reader"]
pub type R = crate::R<SAR_READ_STATUS2_SPEC>;
#[doc = "Field `SAR2_READER_STATUS` reader - "]
pub type SAR2_READER_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar2_reader_status(&self) -> SAR2_READER_STATUS_R {
        SAR2_READER_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READ_STATUS2")
            .field("sar2_reader_status", &self.sar2_reader_status())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_read_status2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_READ_STATUS2_SPEC;
impl crate::RegisterSpec for SAR_READ_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_read_status2::R`](R) reader structure"]
impl crate::Readable for SAR_READ_STATUS2_SPEC {}
#[doc = "`reset()` method sets SAR_READ_STATUS2 to value 0"]
impl crate::Resettable for SAR_READ_STATUS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
