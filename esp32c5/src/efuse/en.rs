#[doc = "Register `EN` reader"]
pub type R = crate::R<EN_SPEC>;
#[doc = "Register `EN` writer"]
pub type W = crate::W<EN_SPEC>;
#[doc = "Field `APB2OTP_EN` reader - Apb2otp mode enable signal."]
pub type APB2OTP_EN_R = crate::BitReader;
#[doc = "Field `APB2OTP_EN` writer - Apb2otp mode enable signal."]
pub type APB2OTP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Apb2otp mode enable signal."]
    #[inline(always)]
    pub fn apb2otp_en(&self) -> APB2OTP_EN_R {
        APB2OTP_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EN")
            .field("apb2otp_en", &self.apb2otp_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Apb2otp mode enable signal."]
    #[inline(always)]
    pub fn apb2otp_en(&mut self) -> APB2OTP_EN_W<'_, EN_SPEC> {
        APB2OTP_EN_W::new(self, 0)
    }
}
#[doc = "eFuse apb2otp enable configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EN_SPEC;
impl crate::RegisterSpec for EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en::R`](R) reader structure"]
impl crate::Readable for EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`en::W`](W) writer structure"]
impl crate::Writable for EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EN to value 0"]
impl crate::Resettable for EN_SPEC {}
