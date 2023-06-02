#[doc = "Register `APB2OTP_EN` reader"]
pub struct R(crate::R<APB2OTP_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2OTP_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2OTP_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2OTP_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2OTP_EN` writer"]
pub struct W(crate::W<APB2OTP_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2OTP_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APB2OTP_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2OTP_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB2OTP_EN` reader - Debug mode enable signal."]
pub type APB2OTP_EN_R = crate::BitReader;
#[doc = "Field `APB2OTP_EN` writer - Debug mode enable signal."]
pub type APB2OTP_EN_W<'a, const O: u8> = crate::BitWriter<'a, APB2OTP_EN_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Debug mode enable signal."]
    #[inline(always)]
    pub fn apb2otp_en(&self) -> APB2OTP_EN_R {
        APB2OTP_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2OTP_EN")
            .field("apb2otp_en", &format_args!("{}", self.apb2otp_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB2OTP_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Debug mode enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn apb2otp_en(&mut self) -> APB2OTP_EN_W<0> {
        APB2OTP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Otp_debuger apb2otp enable configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2otp_en](index.html) module"]
pub struct APB2OTP_EN_SPEC;
impl crate::RegisterSpec for APB2OTP_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2otp_en::R](R) reader structure"]
impl crate::Readable for APB2OTP_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2otp_en::W](W) writer structure"]
impl crate::Writable for APB2OTP_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2OTP_EN to value 0"]
impl crate::Resettable for APB2OTP_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
