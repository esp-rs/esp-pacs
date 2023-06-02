#[doc = "Register `CONF_W8` reader"]
pub struct R(crate::R<CONF_W8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_W8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_W8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_W8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_W8` writer"]
pub struct W(crate::W<CONF_W8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_W8_SPEC>;
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
impl From<crate::W<CONF_W8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_W8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_CONF32` reader - *******Description***********"]
pub type SLCHOST_CONF32_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF32` writer - *******Description***********"]
pub type SLCHOST_CONF32_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W8_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF33` reader - *******Description***********"]
pub type SLCHOST_CONF33_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF33` writer - *******Description***********"]
pub type SLCHOST_CONF33_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W8_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF34` reader - *******Description***********"]
pub type SLCHOST_CONF34_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF34` writer - *******Description***********"]
pub type SLCHOST_CONF34_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W8_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF35` reader - *******Description***********"]
pub type SLCHOST_CONF35_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF35` writer - *******Description***********"]
pub type SLCHOST_CONF35_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W8_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf32(&self) -> SLCHOST_CONF32_R {
        SLCHOST_CONF32_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf33(&self) -> SLCHOST_CONF33_R {
        SLCHOST_CONF33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf34(&self) -> SLCHOST_CONF34_R {
        SLCHOST_CONF34_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf35(&self) -> SLCHOST_CONF35_R {
        SLCHOST_CONF35_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W8")
            .field(
                "slchost_conf32",
                &format_args!("{}", self.slchost_conf32().bits()),
            )
            .field(
                "slchost_conf33",
                &format_args!("{}", self.slchost_conf33().bits()),
            )
            .field(
                "slchost_conf34",
                &format_args!("{}", self.slchost_conf34().bits()),
            )
            .field(
                "slchost_conf35",
                &format_args!("{}", self.slchost_conf35().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf32(&mut self) -> SLCHOST_CONF32_W<0> {
        SLCHOST_CONF32_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf33(&mut self) -> SLCHOST_CONF33_W<8> {
        SLCHOST_CONF33_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf34(&mut self) -> SLCHOST_CONF34_W<16> {
        SLCHOST_CONF34_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf35(&mut self) -> SLCHOST_CONF35_W<24> {
        SLCHOST_CONF35_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_w8](index.html) module"]
pub struct CONF_W8_SPEC;
impl crate::RegisterSpec for CONF_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_w8::R](R) reader structure"]
impl crate::Readable for CONF_W8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_w8::W](W) writer structure"]
impl crate::Writable for CONF_W8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W8 to value 0"]
impl crate::Resettable for CONF_W8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
