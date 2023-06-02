#[doc = "Register `HOST_SLCHOST_CONF_W5` reader"]
pub struct R(crate::R<HOST_SLCHOST_CONF_W5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CONF_W5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CONF_W5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CONF_W5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_CONF_W5` writer"]
pub struct W(crate::W<HOST_SLCHOST_CONF_W5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_CONF_W5_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_CONF_W5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_CONF_W5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_CONF20` reader - "]
pub type HOST_SLCHOST_CONF20_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF20` writer - "]
pub type HOST_SLCHOST_CONF20_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W5_SPEC, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF21` reader - "]
pub type HOST_SLCHOST_CONF21_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF21` writer - "]
pub type HOST_SLCHOST_CONF21_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W5_SPEC, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF22` reader - "]
pub type HOST_SLCHOST_CONF22_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF22` writer - "]
pub type HOST_SLCHOST_CONF22_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W5_SPEC, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF23` reader - "]
pub type HOST_SLCHOST_CONF23_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF23` writer - "]
pub type HOST_SLCHOST_CONF23_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLCHOST_CONF_W5_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf20(&self) -> HOST_SLCHOST_CONF20_R {
        HOST_SLCHOST_CONF20_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf21(&self) -> HOST_SLCHOST_CONF21_R {
        HOST_SLCHOST_CONF21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf22(&self) -> HOST_SLCHOST_CONF22_R {
        HOST_SLCHOST_CONF22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf23(&self) -> HOST_SLCHOST_CONF23_R {
        HOST_SLCHOST_CONF23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W5")
            .field(
                "host_slchost_conf20",
                &format_args!("{}", self.host_slchost_conf20().bits()),
            )
            .field(
                "host_slchost_conf21",
                &format_args!("{}", self.host_slchost_conf21().bits()),
            )
            .field(
                "host_slchost_conf22",
                &format_args!("{}", self.host_slchost_conf22().bits()),
            )
            .field(
                "host_slchost_conf23",
                &format_args!("{}", self.host_slchost_conf23().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CONF_W5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf20(&mut self) -> HOST_SLCHOST_CONF20_W<0> {
        HOST_SLCHOST_CONF20_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf21(&mut self) -> HOST_SLCHOST_CONF21_W<8> {
        HOST_SLCHOST_CONF21_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf22(&mut self) -> HOST_SLCHOST_CONF22_W<16> {
        HOST_SLCHOST_CONF22_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf23(&mut self) -> HOST_SLCHOST_CONF23_W<24> {
        HOST_SLCHOST_CONF23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_conf_w5](index.html) module"]
pub struct HOST_SLCHOST_CONF_W5_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_conf_w5::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w5::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W5 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
