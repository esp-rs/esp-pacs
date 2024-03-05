#[doc = "Register `IN_CRC_FINAL_RESULT` reader"]
pub type R = crate::R<IN_CRC_FINAL_RESULT_SPEC>;
#[doc = "Field `IN_CRC_FINAL_RESULT_CH` reader - This register is used to store result ch0 of rx"]
pub type IN_CRC_FINAL_RESULT_CH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to store result ch0 of rx"]
    #[inline(always)]
    pub fn in_crc_final_result_ch(&self) -> IN_CRC_FINAL_RESULT_CH_R {
        IN_CRC_FINAL_RESULT_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CRC_FINAL_RESULT")
            .field(
                "in_crc_final_result_ch",
                &format_args!("{}", self.in_crc_final_result_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_CRC_FINAL_RESULT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "This register is used to store ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_crc_final_result::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CRC_FINAL_RESULT_SPEC;
impl crate::RegisterSpec for IN_CRC_FINAL_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_crc_final_result::R`](R) reader structure"]
impl crate::Readable for IN_CRC_FINAL_RESULT_SPEC {}
#[doc = "`reset()` method sets IN_CRC_FINAL_RESULT to value 0"]
impl crate::Resettable for IN_CRC_FINAL_RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
