#[doc = "Register `CONF_W12` reader"]
pub struct R(crate::R<CONF_W12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_W12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_W12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_W12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_W12` writer"]
pub struct W(crate::W<CONF_W12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_W12_SPEC>;
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
impl From<crate::W<CONF_W12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_W12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_CONF48` reader - *******Description***********"]
pub type SLCHOST_CONF48_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF48` writer - *******Description***********"]
pub type SLCHOST_CONF48_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W12_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF49` reader - *******Description***********"]
pub type SLCHOST_CONF49_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF49` writer - *******Description***********"]
pub type SLCHOST_CONF49_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W12_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF50` reader - *******Description***********"]
pub type SLCHOST_CONF50_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF50` writer - *******Description***********"]
pub type SLCHOST_CONF50_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W12_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF51` reader - *******Description***********"]
pub type SLCHOST_CONF51_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF51` writer - *******Description***********"]
pub type SLCHOST_CONF51_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W12_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf48(&self) -> SLCHOST_CONF48_R {
        SLCHOST_CONF48_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf49(&self) -> SLCHOST_CONF49_R {
        SLCHOST_CONF49_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf50(&self) -> SLCHOST_CONF50_R {
        SLCHOST_CONF50_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf51(&self) -> SLCHOST_CONF51_R {
        SLCHOST_CONF51_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W12")
            .field(
                "slchost_conf48",
                &format_args!("{}", self.slchost_conf48().bits()),
            )
            .field(
                "slchost_conf49",
                &format_args!("{}", self.slchost_conf49().bits()),
            )
            .field(
                "slchost_conf50",
                &format_args!("{}", self.slchost_conf50().bits()),
            )
            .field(
                "slchost_conf51",
                &format_args!("{}", self.slchost_conf51().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf48(&mut self) -> SLCHOST_CONF48_W<0> {
        SLCHOST_CONF48_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf49(&mut self) -> SLCHOST_CONF49_W<8> {
        SLCHOST_CONF49_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf50(&mut self) -> SLCHOST_CONF50_W<16> {
        SLCHOST_CONF50_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf51(&mut self) -> SLCHOST_CONF51_W<24> {
        SLCHOST_CONF51_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_w12](index.html) module"]
pub struct CONF_W12_SPEC;
impl crate::RegisterSpec for CONF_W12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_w12::R](R) reader structure"]
impl crate::Readable for CONF_W12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_w12::W](W) writer structure"]
impl crate::Writable for CONF_W12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W12 to value 0"]
impl crate::Resettable for CONF_W12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
