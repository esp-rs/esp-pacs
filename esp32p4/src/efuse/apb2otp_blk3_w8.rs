///Register `APB2OTP_BLK3_W8` reader
pub type R = crate::R<APB2OTP_BLK3_W8_SPEC>;
///Field `APB2OTP_BLOCK3_W8` reader - Otp block3 word8 data.
pub type APB2OTP_BLOCK3_W8_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block3 word8 data.
    #[inline(always)]
    pub fn apb2otp_block3_w8(&self) -> APB2OTP_BLOCK3_W8_R {
        APB2OTP_BLOCK3_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK3_W8")
            .field("apb2otp_block3_w8", &self.apb2otp_block3_w8())
            .finish()
    }
}
/**eFuse apb2otp block3 data register8.

You can [`read`](crate::generic::Reg::read) this register and get [`apb2otp_blk3_w8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APB2OTP_BLK3_W8_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK3_W8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`apb2otp_blk3_w8::R`](R) reader structure
impl crate::Readable for APB2OTP_BLK3_W8_SPEC {}
///`reset()` method sets APB2OTP_BLK3_W8 to value 0
impl crate::Resettable for APB2OTP_BLK3_W8_SPEC {
    const RESET_VALUE: u32 = 0;
}
