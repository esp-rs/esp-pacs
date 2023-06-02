#[doc = "Register `CONF_W11` reader"]
pub struct R(crate::R<CONF_W11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_W11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_W11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_W11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_W11` writer"]
pub struct W(crate::W<CONF_W11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_W11_SPEC>;
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
impl From<crate::W<CONF_W11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_W11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_CONF44` reader - *******Description***********"]
pub type SLCHOST_CONF44_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF44` writer - *******Description***********"]
pub type SLCHOST_CONF44_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W11_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF45` reader - *******Description***********"]
pub type SLCHOST_CONF45_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF45` writer - *******Description***********"]
pub type SLCHOST_CONF45_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W11_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF46` reader - *******Description***********"]
pub type SLCHOST_CONF46_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF46` writer - *******Description***********"]
pub type SLCHOST_CONF46_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W11_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF47` reader - *******Description***********"]
pub type SLCHOST_CONF47_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF47` writer - *******Description***********"]
pub type SLCHOST_CONF47_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W11_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf44(&self) -> SLCHOST_CONF44_R {
        SLCHOST_CONF44_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf45(&self) -> SLCHOST_CONF45_R {
        SLCHOST_CONF45_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf46(&self) -> SLCHOST_CONF46_R {
        SLCHOST_CONF46_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf47(&self) -> SLCHOST_CONF47_R {
        SLCHOST_CONF47_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W11")
            .field(
                "slchost_conf44",
                &format_args!("{}", self.slchost_conf44().bits()),
            )
            .field(
                "slchost_conf45",
                &format_args!("{}", self.slchost_conf45().bits()),
            )
            .field(
                "slchost_conf46",
                &format_args!("{}", self.slchost_conf46().bits()),
            )
            .field(
                "slchost_conf47",
                &format_args!("{}", self.slchost_conf47().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf44(&mut self) -> SLCHOST_CONF44_W<0> {
        SLCHOST_CONF44_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf45(&mut self) -> SLCHOST_CONF45_W<8> {
        SLCHOST_CONF45_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf46(&mut self) -> SLCHOST_CONF46_W<16> {
        SLCHOST_CONF46_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf47(&mut self) -> SLCHOST_CONF47_W<24> {
        SLCHOST_CONF47_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_w11](index.html) module"]
pub struct CONF_W11_SPEC;
impl crate::RegisterSpec for CONF_W11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_w11::R](R) reader structure"]
impl crate::Readable for CONF_W11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_w11::W](W) writer structure"]
impl crate::Writable for CONF_W11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W11 to value 0"]
impl crate::Resettable for CONF_W11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
