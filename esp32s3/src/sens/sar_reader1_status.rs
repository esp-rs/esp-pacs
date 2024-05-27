///Register `SAR_READER1_STATUS` reader
pub type R = crate::R<SAR_READER1_STATUS_SPEC>;
///Field `SAR_SAR1_READER_STATUS` reader - get saradc1 reader controller status
pub type SAR_SAR1_READER_STATUS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - get saradc1 reader controller status
    #[inline(always)]
    pub fn sar_sar1_reader_status(&self) -> SAR_SAR1_READER_STATUS_R {
        SAR_SAR1_READER_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READER1_STATUS")
            .field("sar_sar1_reader_status", &self.sar_sar1_reader_status())
            .finish()
    }
}
/**get saradc1 reader controller status

You can [`read`](crate::generic::Reg::read) this register and get [`sar_reader1_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_READER1_STATUS_SPEC;
impl crate::RegisterSpec for SAR_READER1_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_reader1_status::R`](R) reader structure
impl crate::Readable for SAR_READER1_STATUS_SPEC {}
///`reset()` method sets SAR_READER1_STATUS to value 0
impl crate::Resettable for SAR_READER1_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
