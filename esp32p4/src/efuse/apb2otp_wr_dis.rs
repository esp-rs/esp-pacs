#[doc = "Register `APB2OTP_WR_DIS` reader"]
pub type R = crate::R<APB2OTP_WR_DIS_SPEC>;
#[doc = "Field `APB2OTP_BLOCK0_WR_DIS` reader - Otp block0 write disable data."]
pub type APB2OTP_BLOCK0_WR_DIS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 write disable data."]
    #[inline(always)]
    pub fn apb2otp_block0_wr_dis(&self) -> APB2OTP_BLOCK0_WR_DIS_R {
        APB2OTP_BLOCK0_WR_DIS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_WR_DIS")
            .field("apb2otp_block0_wr_dis", &self.apb2otp_block0_wr_dis())
            .finish()
    }
}
#[doc = "eFuse apb2otp block0 data register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2otp_wr_dis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_WR_DIS_SPEC;
impl crate::RegisterSpec for APB2OTP_WR_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_wr_dis::R`](R) reader structure"]
impl crate::Readable for APB2OTP_WR_DIS_SPEC {}
#[doc = "`reset()` method sets APB2OTP_WR_DIS to value 0"]
impl crate::Resettable for APB2OTP_WR_DIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
