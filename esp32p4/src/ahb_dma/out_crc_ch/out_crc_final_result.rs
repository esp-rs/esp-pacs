///Register `OUT_CRC_FINAL_RESULT` reader
pub type R = crate::R<OUT_CRC_FINAL_RESULT_SPEC>;
///Field `OUT_CRC_FINAL_RESULT` reader - This register is used to store result ch0 of tx
pub type OUT_CRC_FINAL_RESULT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - This register is used to store result ch0 of tx
    #[inline(always)]
    pub fn out_crc_final_result(&self) -> OUT_CRC_FINAL_RESULT_R {
        OUT_CRC_FINAL_RESULT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CRC_FINAL_RESULT")
            .field("out_crc_final_result", &self.out_crc_final_result())
            .finish()
    }
}
/**This register is used to store ch0 crc result

You can [`read`](crate::generic::Reg::read) this register and get [`out_crc_final_result::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_CRC_FINAL_RESULT_SPEC;
impl crate::RegisterSpec for OUT_CRC_FINAL_RESULT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_crc_final_result::R`](R) reader structure
impl crate::Readable for OUT_CRC_FINAL_RESULT_SPEC {}
///`reset()` method sets OUT_CRC_FINAL_RESULT to value 0
impl crate::Resettable for OUT_CRC_FINAL_RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
