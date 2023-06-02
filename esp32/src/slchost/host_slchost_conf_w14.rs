#[doc = "Register `HOST_SLCHOST_CONF_W14` reader"]
pub struct R(crate::R<HOST_SLCHOST_CONF_W14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CONF_W14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CONF_W14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CONF_W14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_CONF_W14` writer"]
pub struct W(crate::W<HOST_SLCHOST_CONF_W14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_CONF_W14_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_CONF_W14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_CONF_W14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_CONF56` reader - "]
pub type HOST_SLCHOST_CONF56_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF56` writer - "]
pub type HOST_SLCHOST_CONF56_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W14_SPEC, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF57` reader - "]
pub type HOST_SLCHOST_CONF57_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF57` writer - "]
pub type HOST_SLCHOST_CONF57_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W14_SPEC, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF58` reader - "]
pub type HOST_SLCHOST_CONF58_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF58` writer - "]
pub type HOST_SLCHOST_CONF58_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W14_SPEC, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF59` reader - "]
pub type HOST_SLCHOST_CONF59_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF59` writer - "]
pub type HOST_SLCHOST_CONF59_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W14_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf56(&self) -> HOST_SLCHOST_CONF56_R {
        HOST_SLCHOST_CONF56_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf57(&self) -> HOST_SLCHOST_CONF57_R {
        HOST_SLCHOST_CONF57_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf58(&self) -> HOST_SLCHOST_CONF58_R {
        HOST_SLCHOST_CONF58_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf59(&self) -> HOST_SLCHOST_CONF59_R {
        HOST_SLCHOST_CONF59_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W14")
            .field(
                "host_slchost_conf56",
                &format_args!("{}", self.host_slchost_conf56().bits()),
            )
            .field(
                "host_slchost_conf57",
                &format_args!("{}", self.host_slchost_conf57().bits()),
            )
            .field(
                "host_slchost_conf58",
                &format_args!("{}", self.host_slchost_conf58().bits()),
            )
            .field(
                "host_slchost_conf59",
                &format_args!("{}", self.host_slchost_conf59().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CONF_W14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf56(&mut self) -> HOST_SLCHOST_CONF56_W<0> {
        HOST_SLCHOST_CONF56_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf57(&mut self) -> HOST_SLCHOST_CONF57_W<8> {
        HOST_SLCHOST_CONF57_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf58(&mut self) -> HOST_SLCHOST_CONF58_W<16> {
        HOST_SLCHOST_CONF58_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf59(&mut self) -> HOST_SLCHOST_CONF59_W<24> {
        HOST_SLCHOST_CONF59_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_conf_w14](index.html) module"]
pub struct HOST_SLCHOST_CONF_W14_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_conf_w14::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w14::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W14 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
