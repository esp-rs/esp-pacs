#[doc = "Register `APB2OTP_BLK10_W3` reader"]
pub type R = crate::R<APB2OTP_BLK10_W3_SPEC>;
#[doc = "Field `APB2OTP_BLOCK10_W3` reader - Otp block10 word3 data."]
pub type APB2OTP_BLOCK10_W3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block10 word3 data."]
    #[inline(always)]
    pub fn apb2otp_block10_w3(&self) -> APB2OTP_BLOCK10_W3_R {
        APB2OTP_BLOCK10_W3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK10_W3")
            .field("apb2otp_block10_w3", &self.apb2otp_block10_w3())
            .finish()
    }
}
#[doc = "eFuse apb2otp block10 data register3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2otp_blk10_w3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK10_W3_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK10_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk10_w3::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK10_W3_SPEC {}
#[doc = "`reset()` method sets APB2OTP_BLK10_W3 to value 0"]
impl crate::Resettable for APB2OTP_BLK10_W3_SPEC {
    const RESET_VALUE: u32 = 0;
}
