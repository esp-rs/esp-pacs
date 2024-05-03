#[doc = "Register `OUT_CRC_FINAL_RESULT` reader"]
pub type R = crate::R<OUT_CRC_FINAL_RESULT_SPEC>;
#[doc = "Field `OUT_CRC_FINAL_RESULT` reader - This register is used to store result ch0 of tx"]
pub type OUT_CRC_FINAL_RESULT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to store result ch0 of tx"]
    #[inline(always)]
    pub fn out_crc_final_result(&self) -> OUT_CRC_FINAL_RESULT_R {
        OUT_CRC_FINAL_RESULT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CRC_FINAL_RESULT")
            .field("out_crc_final_result", &self.out_crc_final_result().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CRC_FINAL_RESULT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "This register is used to store ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_crc_final_result::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CRC_FINAL_RESULT_SPEC;
impl crate::RegisterSpec for OUT_CRC_FINAL_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_crc_final_result::R`](R) reader structure"]
impl crate::Readable for OUT_CRC_FINAL_RESULT_SPEC {}
#[doc = "`reset()` method sets OUT_CRC_FINAL_RESULT to value 0"]
impl crate::Resettable for OUT_CRC_FINAL_RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
