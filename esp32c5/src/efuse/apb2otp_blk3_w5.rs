#[doc = "Register `APB2OTP_BLK3_W5` reader"]
pub type R = crate::R<APB2OTP_BLK3_W5_SPEC>;
#[doc = "Field `APB2OTP_BLOCK3_W5` reader - Otp block3 word5 data."]
pub type APB2OTP_BLOCK3_W5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block3 word5 data."]
    #[inline(always)]
    pub fn apb2otp_block3_w5(&self) -> APB2OTP_BLOCK3_W5_R {
        APB2OTP_BLOCK3_W5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK3_W5")
            .field("apb2otp_block3_w5", &self.apb2otp_block3_w5())
            .finish()
    }
}
#[doc = "eFuse apb2otp block3 data register5.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK3_W5_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK3_W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk3_w5::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK3_W5_SPEC {}
#[doc = "`reset()` method sets APB2OTP_BLK3_W5 to value 0"]
impl crate::Resettable for APB2OTP_BLK3_W5_SPEC {
    const RESET_VALUE: u32 = 0;
}
