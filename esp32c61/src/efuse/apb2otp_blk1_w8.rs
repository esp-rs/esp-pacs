#[doc = "Register `APB2OTP_BLK1_W8` reader"]
pub type R = crate::R<APB2OTP_BLK1_W8_SPEC>;
#[doc = "Field `APB2OTP_BLOCK1_W8` reader - Otp block1 word8 data."]
pub type APB2OTP_BLOCK1_W8_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block1 word8 data."]
    #[inline(always)]
    pub fn apb2otp_block1_w8(&self) -> APB2OTP_BLOCK1_W8_R {
        APB2OTP_BLOCK1_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK1_W8")
            .field("apb2otp_block1_w8", &self.apb2otp_block1_w8())
            .finish()
    }
}
#[doc = "eFuse apb2otp block1 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk1_w8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK1_W8_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK1_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk1_w8::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK1_W8_SPEC {}
#[doc = "`reset()` method sets APB2OTP_BLK1_W8 to value 0"]
impl crate::Resettable for APB2OTP_BLK1_W8_SPEC {}
