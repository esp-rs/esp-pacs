///Register `SAR_READ_STATUS1` reader
pub type R = crate::R<SAR_READ_STATUS1_SPEC>;
///Field `SAR1_READER_STATUS` reader -
pub type SAR1_READER_STATUS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn sar1_reader_status(&self) -> SAR1_READER_STATUS_R {
        SAR1_READER_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READ_STATUS1")
            .field("sar1_reader_status", &self.sar1_reader_status())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sar_read_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_READ_STATUS1_SPEC;
impl crate::RegisterSpec for SAR_READ_STATUS1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_read_status1::R`](R) reader structure
impl crate::Readable for SAR_READ_STATUS1_SPEC {}
///`reset()` method sets SAR_READ_STATUS1 to value 0
impl crate::Resettable for SAR_READ_STATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
