#[doc = "Register `APB2OTP_BLK0_BACKUP3_W4` reader"]
pub type R = crate::R<APB2OTP_BLK0_BACKUP3_W4_SPEC>;
#[doc = "Field `APB2OTP_BLOCK0_BACKUP3_W4` reader - Otp block0 backup3 word4 data."]
pub type APB2OTP_BLOCK0_BACKUP3_W4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup3 word4 data."]
    #[inline(always)]
    pub fn apb2otp_block0_backup3_w4(&self) -> APB2OTP_BLOCK0_BACKUP3_W4_R {
        APB2OTP_BLOCK0_BACKUP3_W4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK0_BACKUP3_W4")
            .field(
                "apb2otp_block0_backup3_w4",
                &format_args!("{}", self.apb2otp_block0_backup3_w4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB2OTP_BLK0_BACKUP3_W4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "eFuse apb2otp block0 data register15.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2otp_blk0_backup3_w4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK0_BACKUP3_W4_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK0_BACKUP3_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk0_backup3_w4::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK0_BACKUP3_W4_SPEC {}
#[doc = "`reset()` method sets APB2OTP_BLK0_BACKUP3_W4 to value 0"]
impl crate::Resettable for APB2OTP_BLK0_BACKUP3_W4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
