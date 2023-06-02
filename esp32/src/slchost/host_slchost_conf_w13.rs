#[doc = "Register `HOST_SLCHOST_CONF_W13` reader"]
pub struct R(crate::R<HOST_SLCHOST_CONF_W13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CONF_W13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CONF_W13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CONF_W13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_CONF_W13` writer"]
pub struct W(crate::W<HOST_SLCHOST_CONF_W13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_CONF_W13_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_CONF_W13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_CONF_W13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_CONF52` reader - "]
pub type HOST_SLCHOST_CONF52_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF52` writer - "]
pub type HOST_SLCHOST_CONF52_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W13_SPEC, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF53` reader - "]
pub type HOST_SLCHOST_CONF53_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF53` writer - "]
pub type HOST_SLCHOST_CONF53_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W13_SPEC, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF54` reader - "]
pub type HOST_SLCHOST_CONF54_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF54` writer - "]
pub type HOST_SLCHOST_CONF54_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W13_SPEC, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF55` reader - "]
pub type HOST_SLCHOST_CONF55_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF55` writer - "]
pub type HOST_SLCHOST_CONF55_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W13_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf52(&self) -> HOST_SLCHOST_CONF52_R {
        HOST_SLCHOST_CONF52_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf53(&self) -> HOST_SLCHOST_CONF53_R {
        HOST_SLCHOST_CONF53_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf54(&self) -> HOST_SLCHOST_CONF54_R {
        HOST_SLCHOST_CONF54_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf55(&self) -> HOST_SLCHOST_CONF55_R {
        HOST_SLCHOST_CONF55_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W13")
            .field(
                "host_slchost_conf52",
                &format_args!("{}", self.host_slchost_conf52().bits()),
            )
            .field(
                "host_slchost_conf53",
                &format_args!("{}", self.host_slchost_conf53().bits()),
            )
            .field(
                "host_slchost_conf54",
                &format_args!("{}", self.host_slchost_conf54().bits()),
            )
            .field(
                "host_slchost_conf55",
                &format_args!("{}", self.host_slchost_conf55().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CONF_W13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf52(&mut self) -> HOST_SLCHOST_CONF52_W<0> {
        HOST_SLCHOST_CONF52_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf53(&mut self) -> HOST_SLCHOST_CONF53_W<8> {
        HOST_SLCHOST_CONF53_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf54(&mut self) -> HOST_SLCHOST_CONF54_W<16> {
        HOST_SLCHOST_CONF54_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf55(&mut self) -> HOST_SLCHOST_CONF55_W<24> {
        HOST_SLCHOST_CONF55_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_conf_w13](index.html) module"]
pub struct HOST_SLCHOST_CONF_W13_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_conf_w13::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w13::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W13 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
