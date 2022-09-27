#[doc = "Register `HOST_SLCHOST_CONF_W0` reader"]
pub struct R(crate::R<HOST_SLCHOST_CONF_W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CONF_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CONF_W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CONF_W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_CONF_W0` writer"]
pub struct W(crate::W<HOST_SLCHOST_CONF_W0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_CONF_W0_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_CONF_W0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_CONF_W0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLCHOST_CONF0` reader - "]
pub type HOST_SLCHOST_CONF0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF0` writer - "]
pub type HOST_SLCHOST_CONF0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W0_SPEC, u8, u8, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF1` reader - "]
pub type HOST_SLCHOST_CONF1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF1` writer - "]
pub type HOST_SLCHOST_CONF1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W0_SPEC, u8, u8, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF2` reader - "]
pub type HOST_SLCHOST_CONF2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF2` writer - "]
pub type HOST_SLCHOST_CONF2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W0_SPEC, u8, u8, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF3` reader - "]
pub type HOST_SLCHOST_CONF3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOST_SLCHOST_CONF3` writer - "]
pub type HOST_SLCHOST_CONF3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_SLCHOST_CONF_W0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf0(&self) -> HOST_SLCHOST_CONF0_R {
        HOST_SLCHOST_CONF0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf1(&self) -> HOST_SLCHOST_CONF1_R {
        HOST_SLCHOST_CONF1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf2(&self) -> HOST_SLCHOST_CONF2_R {
        HOST_SLCHOST_CONF2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf3(&self) -> HOST_SLCHOST_CONF3_R {
        HOST_SLCHOST_CONF3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf0(&mut self) -> HOST_SLCHOST_CONF0_W<0> {
        HOST_SLCHOST_CONF0_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf1(&mut self) -> HOST_SLCHOST_CONF1_W<8> {
        HOST_SLCHOST_CONF1_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf2(&mut self) -> HOST_SLCHOST_CONF2_W<16> {
        HOST_SLCHOST_CONF2_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf3(&mut self) -> HOST_SLCHOST_CONF3_W<24> {
        HOST_SLCHOST_CONF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_conf_w0](index.html) module"]
pub struct HOST_SLCHOST_CONF_W0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_conf_w0::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w0::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W0 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
