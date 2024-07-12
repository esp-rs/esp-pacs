#[doc = "Register `APB2OTP_BLK3_W6` reader"]
pub type R = crate::R<APB2OTP_BLK3_W6_SPEC>;
#[doc = "Field `APB2OTP_BLOCK3_W6` reader - Otp block3 word6 data."]
pub type APB2OTP_BLOCK3_W6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block3 word6 data."]
    #[inline(always)]
    pub fn apb2otp_block3_w6(&self) -> APB2OTP_BLOCK3_W6_R {
        APB2OTP_BLOCK3_W6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK3_W6")
            .field("apb2otp_block3_w6", &self.apb2otp_block3_w6())
            .finish()
    }
}
#[doc = "eFuse apb2otp block3 data register6.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk3_w6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK3_W6_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK3_W6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk3_w6::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK3_W6_SPEC {}
#[doc = "`reset()` method sets APB2OTP_BLK3_W6 to value 0"]
impl crate::Resettable for APB2OTP_BLK3_W6_SPEC {
    const RESET_VALUE: u32 = 0;
}
