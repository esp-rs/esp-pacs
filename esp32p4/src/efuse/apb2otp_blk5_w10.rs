#[doc = "Register `APB2OTP_BLK5_W10` reader"]
pub type R = crate::R<APB2OTP_BLK5_W10_SPEC>;
#[doc = "Field `APB2OTP_BLOCK5_W10` reader - Otp block5 word10 data."]
pub type APB2OTP_BLOCK5_W10_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block5 word10 data."]
    #[inline(always)]
    pub fn apb2otp_block5_w10(&self) -> APB2OTP_BLOCK5_W10_R {
        APB2OTP_BLOCK5_W10_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK5_W10")
            .field(
                "apb2otp_block5_w10",
                &format_args!("{}", self.apb2otp_block5_w10().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB2OTP_BLK5_W10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "eFuse apb2otp block5 data register10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2otp_blk5_w10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK5_W10_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK5_W10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk5_w10::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK5_W10_SPEC {}
#[doc = "`reset()` method sets APB2OTP_BLK5_W10 to value 0"]
impl crate::Resettable for APB2OTP_BLK5_W10_SPEC {
    const RESET_VALUE: u32 = 0;
}
