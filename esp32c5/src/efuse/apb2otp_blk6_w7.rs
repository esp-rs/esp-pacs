#[doc = "Register `APB2OTP_BLK6_W7` reader"]
pub type R = crate::R<APB2OTP_BLK6_W7_SPEC>;
#[doc = "Field `APB2OTP_BLOCK6_W7` reader - Otp block6 word7 data."]
pub type APB2OTP_BLOCK6_W7_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block6 word7 data."]
    #[inline(always)]
    pub fn apb2otp_block6_w7(&self) -> APB2OTP_BLOCK6_W7_R {
        APB2OTP_BLOCK6_W7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK6_W7")
            .field("apb2otp_block6_w7", &self.apb2otp_block6_w7())
            .finish()
    }
}
#[doc = "eFuse apb2otp block6 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk6_w7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK6_W7_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK6_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk6_w7::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK6_W7_SPEC {}
#[doc = "`reset()` method sets APB2OTP_BLK6_W7 to value 0"]
impl crate::Resettable for APB2OTP_BLK6_W7_SPEC {
    const RESET_VALUE: u32 = 0;
}
