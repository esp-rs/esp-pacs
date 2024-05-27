///Register `TIMESTAMP_DATA` reader
pub type R = crate::R<TIMESTAMP_DATA_SPEC>;
///Field `TIMESTAMP_DATA` reader - Data of timestamp of a CAN frame.
pub type TIMESTAMP_DATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Data of timestamp of a CAN frame.
    #[inline(always)]
    pub fn timestamp_data(&self) -> TIMESTAMP_DATA_R {
        TIMESTAMP_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMESTAMP_DATA")
            .field("timestamp_data", &self.timestamp_data())
            .finish()
    }
}
/**Timestamp data register

You can [`read`](crate::generic::Reg::read) this register and get [`timestamp_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIMESTAMP_DATA_SPEC;
impl crate::RegisterSpec for TIMESTAMP_DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`timestamp_data::R`](R) reader structure
impl crate::Readable for TIMESTAMP_DATA_SPEC {}
///`reset()` method sets TIMESTAMP_DATA to value 0
impl crate::Resettable for TIMESTAMP_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
