#[doc = "Register `APB2OTP_BLK8_W4` reader"]
pub type R = crate::R<APB2OTP_BLK8_W4_SPEC>;
#[doc = "Field `APB2OTP_BLOCK8_W4` reader - Otp block8 word4 data."]
pub type APB2OTP_BLOCK8_W4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block8 word4 data."]
    #[inline(always)]
    pub fn apb2otp_block8_w4(&self) -> APB2OTP_BLOCK8_W4_R {
        APB2OTP_BLOCK8_W4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK8_W4")
            .field("apb2otp_block8_w4", &self.apb2otp_block8_w4())
            .finish()
    }
}
#[doc = "eFuse apb2otp block8 data register4.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk8_w4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK8_W4_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK8_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk8_w4::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK8_W4_SPEC {}
#[doc = "`reset()` method sets APB2OTP_BLK8_W4 to value 0"]
impl crate::Resettable for APB2OTP_BLK8_W4_SPEC {
    const RESET_VALUE: u32 = 0;
}
