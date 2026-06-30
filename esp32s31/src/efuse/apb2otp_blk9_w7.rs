#[doc = "Register `APB2OTP_BLK9_W7` reader"]
pub type R = crate::R<APB2OTP_BLK9_W7_SPEC>;
#[doc = "Register `APB2OTP_BLK9_W7` writer"]
pub type W = crate::W<APB2OTP_BLK9_W7_SPEC>;
#[doc = "Field `APB2OTP_BLOCK9_W7` reader - OTP block9 word7 data."]
pub type APB2OTP_BLOCK9_W7_R = crate::FieldReader<u32>;
#[doc = "Field `APB2OTP_BLOCK9_W7` writer - OTP block9 word7 data."]
pub type APB2OTP_BLOCK9_W7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block9 word7 data."]
    #[inline(always)]
    pub fn apb2otp_block9_w7(&self) -> APB2OTP_BLOCK9_W7_R {
        APB2OTP_BLOCK9_W7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK9_W7")
            .field("apb2otp_block9_w7", &self.apb2otp_block9_w7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block9 word7 data."]
    #[inline(always)]
    pub fn apb2otp_block9_w7(&mut self) -> APB2OTP_BLOCK9_W7_W<'_, APB2OTP_BLK9_W7_SPEC> {
        APB2OTP_BLOCK9_W7_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block9 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk9_w7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2otp_blk9_w7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK9_W7_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK9_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk9_w7::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK9_W7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2otp_blk9_w7::W`](W) writer structure"]
impl crate::Writable for APB2OTP_BLK9_W7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2OTP_BLK9_W7 to value 0"]
impl crate::Resettable for APB2OTP_BLK9_W7_SPEC {}
