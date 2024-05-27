///Register `APB2OTP_BLK9_W5` reader
pub type R = crate::R<APB2OTP_BLK9_W5_SPEC>;
///Field `APB2OTP_BLOCK9_W5` reader - Otp block9 word5 data.
pub type APB2OTP_BLOCK9_W5_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block9 word5 data.
    #[inline(always)]
    pub fn apb2otp_block9_w5(&self) -> APB2OTP_BLOCK9_W5_R {
        APB2OTP_BLOCK9_W5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK9_W5")
            .field("apb2otp_block9_w5", &self.apb2otp_block9_w5())
            .finish()
    }
}
/**eFuse apb2otp block9 data register5.

You can [`read`](crate::generic::Reg::read) this register and get [`apb2otp_blk9_w5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APB2OTP_BLK9_W5_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK9_W5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`apb2otp_blk9_w5::R`](R) reader structure
impl crate::Readable for APB2OTP_BLK9_W5_SPEC {}
///`reset()` method sets APB2OTP_BLK9_W5 to value 0
impl crate::Resettable for APB2OTP_BLK9_W5_SPEC {
    const RESET_VALUE: u32 = 0;
}
