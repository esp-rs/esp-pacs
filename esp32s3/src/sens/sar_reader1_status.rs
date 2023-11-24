#[doc = "Register `SAR_READER1_STATUS` reader"]
pub type R = crate::R<SAR_READER1_STATUS_SPEC>;
#[doc = "Field `SAR_SAR1_READER_STATUS` reader - get saradc1 reader controller status"]
pub type SAR_SAR1_READER_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - get saradc1 reader controller status"]
    #[inline(always)]
    pub fn sar_sar1_reader_status(&self) -> SAR_SAR1_READER_STATUS_R {
        SAR_SAR1_READER_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READER1_STATUS")
            .field(
                "sar_sar1_reader_status",
                &format_args!("{}", self.sar_sar1_reader_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_READER1_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "get saradc1 reader controller status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_reader1_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_READER1_STATUS_SPEC;
impl crate::RegisterSpec for SAR_READER1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_reader1_status::R`](R) reader structure"]
impl crate::Readable for SAR_READER1_STATUS_SPEC {}
#[doc = "`reset()` method sets SAR_READER1_STATUS to value 0"]
impl crate::Resettable for SAR_READER1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
