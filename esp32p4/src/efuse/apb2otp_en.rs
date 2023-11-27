#[doc = "Register `APB2OTP_EN` reader"]
pub type R = crate::R<APB2OTP_EN_SPEC>;
#[doc = "Register `APB2OTP_EN` writer"]
pub type W = crate::W<APB2OTP_EN_SPEC>;
#[doc = "Field `APB2OTP_APB2OTP_EN` reader - Apb2otp mode enable signal."]
pub type APB2OTP_APB2OTP_EN_R = crate::BitReader;
#[doc = "Field `APB2OTP_APB2OTP_EN` writer - Apb2otp mode enable signal."]
pub type APB2OTP_APB2OTP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Apb2otp mode enable signal."]
    #[inline(always)]
    pub fn apb2otp_apb2otp_en(&self) -> APB2OTP_APB2OTP_EN_R {
        APB2OTP_APB2OTP_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_EN")
            .field(
                "apb2otp_apb2otp_en",
                &format_args!("{}", self.apb2otp_apb2otp_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB2OTP_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Apb2otp mode enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn apb2otp_apb2otp_en(&mut self) -> APB2OTP_APB2OTP_EN_W<APB2OTP_EN_SPEC> {
        APB2OTP_APB2OTP_EN_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "eFuse apb2otp enable configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2otp_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2otp_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2OTP_EN_SPEC;
impl crate::RegisterSpec for APB2OTP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2otp_en::R`](R) reader structure"]
impl crate::Readable for APB2OTP_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2otp_en::W`](W) writer structure"]
impl crate::Writable for APB2OTP_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2OTP_EN to value 0"]
impl crate::Resettable for APB2OTP_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
