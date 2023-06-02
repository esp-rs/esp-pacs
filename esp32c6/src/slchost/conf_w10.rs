#[doc = "Register `CONF_W10` reader"]
pub struct R(crate::R<CONF_W10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_W10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_W10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_W10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_W10` writer"]
pub struct W(crate::W<CONF_W10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_W10_SPEC>;
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
impl From<crate::W<CONF_W10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_W10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_CONF40` reader - *******Description***********"]
pub type SLCHOST_CONF40_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF40` writer - *******Description***********"]
pub type SLCHOST_CONF40_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W10_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF41` reader - *******Description***********"]
pub type SLCHOST_CONF41_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF41` writer - *******Description***********"]
pub type SLCHOST_CONF41_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W10_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF42` reader - *******Description***********"]
pub type SLCHOST_CONF42_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF42` writer - *******Description***********"]
pub type SLCHOST_CONF42_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W10_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF43` reader - *******Description***********"]
pub type SLCHOST_CONF43_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF43` writer - *******Description***********"]
pub type SLCHOST_CONF43_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W10_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf40(&self) -> SLCHOST_CONF40_R {
        SLCHOST_CONF40_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf41(&self) -> SLCHOST_CONF41_R {
        SLCHOST_CONF41_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf42(&self) -> SLCHOST_CONF42_R {
        SLCHOST_CONF42_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf43(&self) -> SLCHOST_CONF43_R {
        SLCHOST_CONF43_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W10")
            .field(
                "slchost_conf40",
                &format_args!("{}", self.slchost_conf40().bits()),
            )
            .field(
                "slchost_conf41",
                &format_args!("{}", self.slchost_conf41().bits()),
            )
            .field(
                "slchost_conf42",
                &format_args!("{}", self.slchost_conf42().bits()),
            )
            .field(
                "slchost_conf43",
                &format_args!("{}", self.slchost_conf43().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf40(&mut self) -> SLCHOST_CONF40_W<0> {
        SLCHOST_CONF40_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf41(&mut self) -> SLCHOST_CONF41_W<8> {
        SLCHOST_CONF41_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf42(&mut self) -> SLCHOST_CONF42_W<16> {
        SLCHOST_CONF42_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf43(&mut self) -> SLCHOST_CONF43_W<24> {
        SLCHOST_CONF43_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_w10](index.html) module"]
pub struct CONF_W10_SPEC;
impl crate::RegisterSpec for CONF_W10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_w10::R](R) reader structure"]
impl crate::Readable for CONF_W10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_w10::W](W) writer structure"]
impl crate::Writable for CONF_W10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W10 to value 0"]
impl crate::Resettable for CONF_W10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
