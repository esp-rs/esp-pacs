#[doc = "Register `APB2OTP_BLK4_W8` reader"]
pub type R = crate::R<APB2OTP_BLK4_W8_SPEC>;
#[doc = "Register `APB2OTP_BLK4_W8` writer"]
pub type W = crate::W<APB2OTP_BLK4_W8_SPEC>;
#[doc = "Field `APB2OTP_BLOCK4_W8` reader - OTP block4 word8 data."]
pub type APB2OTP_BLOCK4_W8_R = crate::FieldReader<u32>;
#[doc = "Field `APB2OTP_BLOCK4_W8` writer - OTP block4 word8 data."]
pub type APB2OTP_BLOCK4_W8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block4 word8 data."]
    #[inline(always)]
    pub fn apb2otp_block4_w8(&self) -> APB2OTP_BLOCK4_W8_R {
        APB2OTP_BLOCK4_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_BLK4_W8")
            .field("apb2otp_block4_w8", &self.apb2otp_block4_w8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - OTP block4 word8 data."]
    #[inline(always)]
    pub fn apb2otp_block4_w8(&mut self) -> APB2OTP_BLOCK4_W8_W<'_, APB2OTP_BLK4_W8_SPEC> {
        APB2OTP_BLOCK4_W8_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp block4 data register8.\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2otp_blk4_w8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2otp_blk4_w8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_BLK4_W8_SPEC;
impl crate::RegisterSpec for APB2OTP_BLK4_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_blk4_w8::R`](R) reader structure"]
impl crate::Readable for APB2OTP_BLK4_W8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2otp_blk4_w8::W`](W) writer structure"]
impl crate::Writable for APB2OTP_BLK4_W8_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2OTP_BLK4_W8 to value 0"]
impl crate::Resettable for APB2OTP_BLK4_W8_SPEC {}
