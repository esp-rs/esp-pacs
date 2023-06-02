#[doc = "Register `RS485_CONF` reader"]
pub struct R(crate::R<RS485_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RS485_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RS485_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RS485_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RS485_CONF` writer"]
pub struct W(crate::W<RS485_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RS485_CONF_SPEC>;
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
impl From<crate::W<RS485_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RS485_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DL0_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type DL0_EN_R = crate::BitReader;
#[doc = "Field `DL0_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type DL0_EN_W<'a, const O: u8> = crate::BitWriter<'a, RS485_CONF_SPEC, O>;
#[doc = "Field `DL1_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type DL1_EN_R = crate::BitReader;
#[doc = "Field `DL1_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type DL1_EN_W<'a, const O: u8> = crate::BitWriter<'a, RS485_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&self) -> DL0_EN_R {
        DL0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&self) -> DL1_EN_R {
        DL1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RS485_CONF")
            .field("dl0_en", &format_args!("{}", self.dl0_en().bit()))
            .field("dl1_en", &format_args!("{}", self.dl1_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RS485_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    #[must_use]
    pub fn dl0_en(&mut self) -> DL0_EN_W<1> {
        DL0_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    #[must_use]
    pub fn dl1_en(&mut self) -> DL1_EN_W<2> {
        DL1_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RS485 mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485_conf](index.html) module"]
pub struct RS485_CONF_SPEC;
impl crate::RegisterSpec for RS485_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rs485_conf::R](R) reader structure"]
impl crate::Readable for RS485_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rs485_conf::W](W) writer structure"]
impl crate::Writable for RS485_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RS485_CONF to value 0"]
impl crate::Resettable for RS485_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
