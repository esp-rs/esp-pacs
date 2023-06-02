#[doc = "Register `CONF_W0` reader"]
pub struct R(crate::R<CONF_W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_W0` writer"]
pub struct W(crate::W<CONF_W0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_W0_SPEC>;
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
impl From<crate::W<CONF_W0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_W0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_CONF0` reader - *******Description***********"]
pub type SLCHOST_CONF0_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF0` writer - *******Description***********"]
pub type SLCHOST_CONF0_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W0_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF1` reader - *******Description***********"]
pub type SLCHOST_CONF1_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF1` writer - *******Description***********"]
pub type SLCHOST_CONF1_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W0_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF2` reader - *******Description***********"]
pub type SLCHOST_CONF2_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF2` writer - *******Description***********"]
pub type SLCHOST_CONF2_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W0_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF3` reader - *******Description***********"]
pub type SLCHOST_CONF3_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF3` writer - *******Description***********"]
pub type SLCHOST_CONF3_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W0_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf0(&self) -> SLCHOST_CONF0_R {
        SLCHOST_CONF0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf1(&self) -> SLCHOST_CONF1_R {
        SLCHOST_CONF1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf2(&self) -> SLCHOST_CONF2_R {
        SLCHOST_CONF2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf3(&self) -> SLCHOST_CONF3_R {
        SLCHOST_CONF3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W0")
            .field(
                "slchost_conf0",
                &format_args!("{}", self.slchost_conf0().bits()),
            )
            .field(
                "slchost_conf1",
                &format_args!("{}", self.slchost_conf1().bits()),
            )
            .field(
                "slchost_conf2",
                &format_args!("{}", self.slchost_conf2().bits()),
            )
            .field(
                "slchost_conf3",
                &format_args!("{}", self.slchost_conf3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf0(&mut self) -> SLCHOST_CONF0_W<0> {
        SLCHOST_CONF0_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf1(&mut self) -> SLCHOST_CONF1_W<8> {
        SLCHOST_CONF1_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf2(&mut self) -> SLCHOST_CONF2_W<16> {
        SLCHOST_CONF2_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf3(&mut self) -> SLCHOST_CONF3_W<24> {
        SLCHOST_CONF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_w0](index.html) module"]
pub struct CONF_W0_SPEC;
impl crate::RegisterSpec for CONF_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_w0::R](R) reader structure"]
impl crate::Readable for CONF_W0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_w0::W](W) writer structure"]
impl crate::Writable for CONF_W0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W0 to value 0"]
impl crate::Resettable for CONF_W0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
