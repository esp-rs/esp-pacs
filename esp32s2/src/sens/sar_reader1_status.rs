#[doc = "Register `SAR_READER1_STATUS` reader"]
pub type R = crate::R<SAR_READER1_STATUS_SPEC>;
#[doc = "Field `SAR1_READER_STATUS` reader - "]
pub type SAR1_READER_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar1_reader_status(&self) -> SAR1_READER_STATUS_R {
        SAR1_READER_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READER1_STATUS")
            .field("sar1_reader_status", &self.sar1_reader_status())
            .finish()
    }
}
#[doc = "saradc1 status for debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_reader1_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_READER1_STATUS_SPEC;
impl crate::RegisterSpec for SAR_READER1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_reader1_status::R`](R) reader structure"]
impl crate::Readable for SAR_READER1_STATUS_SPEC {}
#[doc = "`reset()` method sets SAR_READER1_STATUS to value 0"]
impl crate::Resettable for SAR_READER1_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
