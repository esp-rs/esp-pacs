#[doc = "Register `APB2OTP_WR_DIS` reader"]
pub type R = crate::R<APB2OTP_WR_DIS_SPEC>;
#[doc = "Register `APB2OTP_WR_DIS` writer"]
pub type W = crate::W<APB2OTP_WR_DIS_SPEC>;
#[doc = "Field `APB2OTP_BLOCK0_WR_DIS` reader - OTP block0 write disable data."]
pub type APB2OTP_BLOCK0_WR_DIS_R = crate::FieldReader<u32>;
#[doc = "Field `APB2OTP_BLOCK0_WR_DIS` writer - OTP block0 write disable data."]
pub type APB2OTP_BLOCK0_WR_DIS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block0 write disable data."]
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
impl W {
    #[doc = "Bits 0:31 - OTP block0 write disable data."]
    #[inline(always)]
    pub fn apb2otp_block0_wr_dis(&mut self) -> APB2OTP_BLOCK0_WR_DIS_W<'_, APB2OTP_WR_DIS_SPEC> {
        APB2OTP_BLOCK0_WR_DIS_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block0 data register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_wr_dis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2otp_wr_dis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_WR_DIS_SPEC;
impl crate::RegisterSpec for APB2OTP_WR_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_wr_dis::R`](R) reader structure"]
impl crate::Readable for APB2OTP_WR_DIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2otp_wr_dis::W`](W) writer structure"]
impl crate::Writable for APB2OTP_WR_DIS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2OTP_WR_DIS to value 0"]
impl crate::Resettable for APB2OTP_WR_DIS_SPEC {}
