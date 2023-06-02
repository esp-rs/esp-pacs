#[doc = "Register `SLP_REJECT_CONF` reader"]
pub struct R(crate::R<SLP_REJECT_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_REJECT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_REJECT_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_REJECT_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_REJECT_CONF` writer"]
pub struct W(crate::W<SLP_REJECT_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_REJECT_CONF_SPEC>;
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
impl From<crate::W<SLP_REJECT_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_REJECT_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_REJECT_EN` reader - enable GPIO reject"]
pub type GPIO_REJECT_EN_R = crate::BitReader;
#[doc = "Field `GPIO_REJECT_EN` writer - enable GPIO reject"]
pub type GPIO_REJECT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLP_REJECT_CONF_SPEC, O>;
#[doc = "Field `SDIO_REJECT_EN` reader - enable SDIO reject"]
pub type SDIO_REJECT_EN_R = crate::BitReader;
#[doc = "Field `SDIO_REJECT_EN` writer - enable SDIO reject"]
pub type SDIO_REJECT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLP_REJECT_CONF_SPEC, O>;
#[doc = "Field `LIGHT_SLP_REJECT_EN` reader - enable reject for light sleep"]
pub type LIGHT_SLP_REJECT_EN_R = crate::BitReader;
#[doc = "Field `LIGHT_SLP_REJECT_EN` writer - enable reject for light sleep"]
pub type LIGHT_SLP_REJECT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLP_REJECT_CONF_SPEC, O>;
#[doc = "Field `DEEP_SLP_REJECT_EN` reader - enable reject for deep sleep"]
pub type DEEP_SLP_REJECT_EN_R = crate::BitReader;
#[doc = "Field `DEEP_SLP_REJECT_EN` writer - enable reject for deep sleep"]
pub type DEEP_SLP_REJECT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLP_REJECT_CONF_SPEC, O>;
#[doc = "Field `REJECT_CAUSE` reader - sleep reject cause"]
pub type REJECT_CAUSE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 24 - enable GPIO reject"]
    #[inline(always)]
    pub fn gpio_reject_en(&self) -> GPIO_REJECT_EN_R {
        GPIO_REJECT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - enable SDIO reject"]
    #[inline(always)]
    pub fn sdio_reject_en(&self) -> SDIO_REJECT_EN_R {
        SDIO_REJECT_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - enable reject for light sleep"]
    #[inline(always)]
    pub fn light_slp_reject_en(&self) -> LIGHT_SLP_REJECT_EN_R {
        LIGHT_SLP_REJECT_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - enable reject for deep sleep"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&self) -> DEEP_SLP_REJECT_EN_R {
        DEEP_SLP_REJECT_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - sleep reject cause"]
    #[inline(always)]
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_REJECT_CONF")
            .field(
                "gpio_reject_en",
                &format_args!("{}", self.gpio_reject_en().bit()),
            )
            .field(
                "sdio_reject_en",
                &format_args!("{}", self.sdio_reject_en().bit()),
            )
            .field(
                "light_slp_reject_en",
                &format_args!("{}", self.light_slp_reject_en().bit()),
            )
            .field(
                "deep_slp_reject_en",
                &format_args!("{}", self.deep_slp_reject_en().bit()),
            )
            .field(
                "reject_cause",
                &format_args!("{}", self.reject_cause().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_REJECT_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 24 - enable GPIO reject"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_reject_en(&mut self) -> GPIO_REJECT_EN_W<24> {
        GPIO_REJECT_EN_W::new(self)
    }
    #[doc = "Bit 25 - enable SDIO reject"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_reject_en(&mut self) -> SDIO_REJECT_EN_W<25> {
        SDIO_REJECT_EN_W::new(self)
    }
    #[doc = "Bit 26 - enable reject for light sleep"]
    #[inline(always)]
    #[must_use]
    pub fn light_slp_reject_en(&mut self) -> LIGHT_SLP_REJECT_EN_W<26> {
        LIGHT_SLP_REJECT_EN_W::new(self)
    }
    #[doc = "Bit 27 - enable reject for deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn deep_slp_reject_en(&mut self) -> DEEP_SLP_REJECT_EN_W<27> {
        DEEP_SLP_REJECT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_reject_conf](index.html) module"]
pub struct SLP_REJECT_CONF_SPEC;
impl crate::RegisterSpec for SLP_REJECT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_reject_conf::R](R) reader structure"]
impl crate::Readable for SLP_REJECT_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_reject_conf::W](W) writer structure"]
impl crate::Writable for SLP_REJECT_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_REJECT_CONF to value 0"]
impl crate::Resettable for SLP_REJECT_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
