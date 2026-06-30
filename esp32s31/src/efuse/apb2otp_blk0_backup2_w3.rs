#[doc = "Register `APB2OTP_BLK0_BACKUP2_W3` reader"]
pub type R = crate::R<APB2OTP_BLK0_BACKUP2_W3_SPEC>;
#[doc = "Register `APB2OTP_BLK0_BACKUP2_W3` writer"]
pub type W = crate::W<APB2OTP_BLK0_BACKUP2_W3_SPEC>;
#[doc = "Field `APB2OTP_BLOCK0_BACKUP2_W3` reader - OTP block0 backup2 word3 data."]
pub type APB2OTP_BLOCK0_BACKUP2_W3_R = crate::FieldReader<u32>;
#[doc = "Field `APB2OTP_BLOCK0_BACKUP2_W3` writer - OTP block0 backup2 word3 data."]
pub type APB2OTP_BLOCK0_BACKUP2_W3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block0 backup2 word3 data."]
    #[inline(always)]
    pub fn apb2otp_block0_backup2_w3(&self) -> APB2OTP_BLOCK0_BACKUP2_W3_R {
        APB2OTP_BLOCK0_BACKUP2_W3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK0_BACKUP2_W3")
            .field(
                "apb2otp_block0_backup2_w3",
                &self.apb2otp_block0_backup2_w3(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block0 backup2 word3 data."]
    #[inline(always)]
    pub fn apb2otp_block0_backup2_w3(
        &mut self,
    ) -> APB2OTP_BLOCK0_BACKUP2_W3_W<'_, APB2OTP_BLK0_BACKUP2_W3_SPEC> {
        APB2OTP_BLOCK0_BACKUP2_W3_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block0 data register12.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk0_backup2_w3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2otp_blk0_backup2_w3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK0_BACKUP2_W3_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK0_BACKUP2_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk0_backup2_w3::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK0_BACKUP2_W3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2otp_blk0_backup2_w3::W`](W) writer structure"]
impl crate::Writable for APB2OTP_BLK0_BACKUP2_W3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2OTP_BLK0_BACKUP2_W3 to value 0"]
impl crate::Resettable for APB2OTP_BLK0_BACKUP2_W3_SPEC {}
