///Register `IN_CRC_FINAL_RESULT` reader
pub type R = crate::R<IN_CRC_FINAL_RESULT_SPEC>;
///Field `IN_CRC_FINAL_RESULT` reader - This register is used to store result ch0 of rx
pub type IN_CRC_FINAL_RESULT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - This register is used to store result ch0 of rx
    #[inline(always)]
    pub fn in_crc_final_result(&self) -> IN_CRC_FINAL_RESULT_R {
        IN_CRC_FINAL_RESULT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CRC_FINAL_RESULT")
            .field("in_crc_final_result", &self.in_crc_final_result())
            .finish()
    }
}
/**This register is used to store ch0 crc result

You can [`read`](crate::generic::Reg::read) this register and get [`in_crc_final_result::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IN_CRC_FINAL_RESULT_SPEC;
impl crate::RegisterSpec for IN_CRC_FINAL_RESULT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`in_crc_final_result::R`](R) reader structure
impl crate::Readable for IN_CRC_FINAL_RESULT_SPEC {}
///`reset()` method sets IN_CRC_FINAL_RESULT to value 0
impl crate::Resettable for IN_CRC_FINAL_RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
