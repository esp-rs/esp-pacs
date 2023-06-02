#[doc = "Register `CONF_W2` reader"]
pub struct R(crate::R<CONF_W2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_W2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_W2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_W2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_W2` writer"]
pub struct W(crate::W<CONF_W2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_W2_SPEC>;
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
impl From<crate::W<CONF_W2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_W2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_CONF8` reader - *******Description***********"]
pub type SLCHOST_CONF8_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF8` writer - *******Description***********"]
pub type SLCHOST_CONF8_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W2_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF9` reader - *******Description***********"]
pub type SLCHOST_CONF9_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF9` writer - *******Description***********"]
pub type SLCHOST_CONF9_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W2_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF10` reader - *******Description***********"]
pub type SLCHOST_CONF10_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF10` writer - *******Description***********"]
pub type SLCHOST_CONF10_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W2_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF11` reader - *******Description***********"]
pub type SLCHOST_CONF11_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF11` writer - *******Description***********"]
pub type SLCHOST_CONF11_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W2_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf8(&self) -> SLCHOST_CONF8_R {
        SLCHOST_CONF8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf9(&self) -> SLCHOST_CONF9_R {
        SLCHOST_CONF9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf10(&self) -> SLCHOST_CONF10_R {
        SLCHOST_CONF10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf11(&self) -> SLCHOST_CONF11_R {
        SLCHOST_CONF11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W2")
            .field(
                "slchost_conf8",
                &format_args!("{}", self.slchost_conf8().bits()),
            )
            .field(
                "slchost_conf9",
                &format_args!("{}", self.slchost_conf9().bits()),
            )
            .field(
                "slchost_conf10",
                &format_args!("{}", self.slchost_conf10().bits()),
            )
            .field(
                "slchost_conf11",
                &format_args!("{}", self.slchost_conf11().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf8(&mut self) -> SLCHOST_CONF8_W<0> {
        SLCHOST_CONF8_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf9(&mut self) -> SLCHOST_CONF9_W<8> {
        SLCHOST_CONF9_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf10(&mut self) -> SLCHOST_CONF10_W<16> {
        SLCHOST_CONF10_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf11(&mut self) -> SLCHOST_CONF11_W<24> {
        SLCHOST_CONF11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_w2](index.html) module"]
pub struct CONF_W2_SPEC;
impl crate::RegisterSpec for CONF_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_w2::R](R) reader structure"]
impl crate::Readable for CONF_W2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_w2::W](W) writer structure"]
impl crate::Writable for CONF_W2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W2 to value 0"]
impl crate::Resettable for CONF_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
