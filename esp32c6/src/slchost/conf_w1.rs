#[doc = "Register `CONF_W1` reader"]
pub struct R(crate::R<CONF_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_W1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_W1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_W1` writer"]
pub struct W(crate::W<CONF_W1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_W1_SPEC>;
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
impl From<crate::W<CONF_W1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_W1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_CONF4` reader - *******Description***********"]
pub type SLCHOST_CONF4_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF4` writer - *******Description***********"]
pub type SLCHOST_CONF4_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W1_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF5` reader - *******Description***********"]
pub type SLCHOST_CONF5_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF5` writer - *******Description***********"]
pub type SLCHOST_CONF5_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W1_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF6` reader - *******Description***********"]
pub type SLCHOST_CONF6_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF6` writer - *******Description***********"]
pub type SLCHOST_CONF6_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W1_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF7` reader - *******Description***********"]
pub type SLCHOST_CONF7_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF7` writer - *******Description***********"]
pub type SLCHOST_CONF7_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf4(&self) -> SLCHOST_CONF4_R {
        SLCHOST_CONF4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf5(&self) -> SLCHOST_CONF5_R {
        SLCHOST_CONF5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf6(&self) -> SLCHOST_CONF6_R {
        SLCHOST_CONF6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf7(&self) -> SLCHOST_CONF7_R {
        SLCHOST_CONF7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W1")
            .field(
                "slchost_conf4",
                &format_args!("{}", self.slchost_conf4().bits()),
            )
            .field(
                "slchost_conf5",
                &format_args!("{}", self.slchost_conf5().bits()),
            )
            .field(
                "slchost_conf6",
                &format_args!("{}", self.slchost_conf6().bits()),
            )
            .field(
                "slchost_conf7",
                &format_args!("{}", self.slchost_conf7().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf4(&mut self) -> SLCHOST_CONF4_W<0> {
        SLCHOST_CONF4_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf5(&mut self) -> SLCHOST_CONF5_W<8> {
        SLCHOST_CONF5_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf6(&mut self) -> SLCHOST_CONF6_W<16> {
        SLCHOST_CONF6_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf7(&mut self) -> SLCHOST_CONF7_W<24> {
        SLCHOST_CONF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_w1](index.html) module"]
pub struct CONF_W1_SPEC;
impl crate::RegisterSpec for CONF_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_w1::R](R) reader structure"]
impl crate::Readable for CONF_W1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_w1::W](W) writer structure"]
impl crate::Writable for CONF_W1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W1 to value 0"]
impl crate::Resettable for CONF_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
