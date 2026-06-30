#[doc = "Register `APB2OTP_BLK0_BACKUP3_W4` reader"]
pub type R = crate::R<APB2OTP_BLK0_BACKUP3_W4_SPEC>;
#[doc = "Register `APB2OTP_BLK0_BACKUP3_W4` writer"]
pub type W = crate::W<APB2OTP_BLK0_BACKUP3_W4_SPEC>;
#[doc = "Field `APB2OTP_BLOCK0_BACKUP3_W4` reader - OTP block0 backup3 word4 data."]
pub type APB2OTP_BLOCK0_BACKUP3_W4_R = crate::FieldReader<u32>;
#[doc = "Field `APB2OTP_BLOCK0_BACKUP3_W4` writer - OTP block0 backup3 word4 data."]
pub type APB2OTP_BLOCK0_BACKUP3_W4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block0 backup3 word4 data."]
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
                &self.apb2otp_block0_backup3_w4(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block0 backup3 word4 data."]
    #[inline(always)]
    pub fn apb2otp_block0_backup3_w4(
        &mut self,
    ) -> APB2OTP_BLOCK0_BACKUP3_W4_W<'_, APB2OTP_BLK0_BACKUP3_W4_SPEC> {
        APB2OTP_BLOCK0_BACKUP3_W4_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block0 data register21.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup3_w4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2otp_blk0_backup3_w4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK0_BACKUP3_W4_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK0_BACKUP3_W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk0_backup3_w4::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK0_BACKUP3_W4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2otp_blk0_backup3_w4::W`](W) writer structure"]
impl crate::Writable for APB2OTP_BLK0_BACKUP3_W4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2OTP_BLK0_BACKUP3_W4 to value 0"]
impl crate::Resettable for APB2OTP_BLK0_BACKUP3_W4_SPEC {}
