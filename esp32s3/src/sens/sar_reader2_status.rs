#[doc = "Register `SAR_READER2_STATUS` reader"]
pub type R = crate::R<SAR_READER2_STATUS_SPEC>;
#[doc = "Field `SAR_SAR2_READER_STATUS` reader - get saradc1 reader controller status"]
pub type SAR_SAR2_READER_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - get saradc1 reader controller status"]
    #[inline(always)]
    pub fn sar_sar2_reader_status(&self) -> SAR_SAR2_READER_STATUS_R {
        SAR_SAR2_READER_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READER2_STATUS")
            .field(
                "sar_sar2_reader_status",
                &self.sar_sar2_reader_status().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_READER2_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "get saradc1 reader controller status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_reader2_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_READER2_STATUS_SPEC;
impl crate::RegisterSpec for SAR_READER2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_reader2_status::R`](R) reader structure"]
impl crate::Readable for SAR_READER2_STATUS_SPEC {}
#[doc = "`reset()` method sets SAR_READER2_STATUS to value 0"]
impl crate::Resettable for SAR_READER2_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
