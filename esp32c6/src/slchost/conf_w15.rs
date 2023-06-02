#[doc = "Register `CONF_W15` reader"]
pub struct R(crate::R<CONF_W15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_W15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_W15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_W15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_W15` writer"]
pub struct W(crate::W<CONF_W15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_W15_SPEC>;
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
impl From<crate::W<CONF_W15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_W15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_CONF60` reader - *******Description***********"]
pub type SLCHOST_CONF60_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF60` writer - *******Description***********"]
pub type SLCHOST_CONF60_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W15_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF61` reader - *******Description***********"]
pub type SLCHOST_CONF61_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF61` writer - *******Description***********"]
pub type SLCHOST_CONF61_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W15_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF62` reader - *******Description***********"]
pub type SLCHOST_CONF62_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF62` writer - *******Description***********"]
pub type SLCHOST_CONF62_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W15_SPEC, 8, O>;
#[doc = "Field `SLCHOST_CONF63` reader - *******Description***********"]
pub type SLCHOST_CONF63_R = crate::FieldReader;
#[doc = "Field `SLCHOST_CONF63` writer - *******Description***********"]
pub type SLCHOST_CONF63_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_W15_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf60(&self) -> SLCHOST_CONF60_R {
        SLCHOST_CONF60_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf61(&self) -> SLCHOST_CONF61_R {
        SLCHOST_CONF61_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf62(&self) -> SLCHOST_CONF62_R {
        SLCHOST_CONF62_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_conf63(&self) -> SLCHOST_CONF63_R {
        SLCHOST_CONF63_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_W15")
            .field(
                "slchost_conf60",
                &format_args!("{}", self.slchost_conf60().bits()),
            )
            .field(
                "slchost_conf61",
                &format_args!("{}", self.slchost_conf61().bits()),
            )
            .field(
                "slchost_conf62",
                &format_args!("{}", self.slchost_conf62().bits()),
            )
            .field(
                "slchost_conf63",
                &format_args!("{}", self.slchost_conf63().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_W15_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf60(&mut self) -> SLCHOST_CONF60_W<0> {
        SLCHOST_CONF60_W::new(self)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf61(&mut self) -> SLCHOST_CONF61_W<8> {
        SLCHOST_CONF61_W::new(self)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf62(&mut self) -> SLCHOST_CONF62_W<16> {
        SLCHOST_CONF62_W::new(self)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_conf63(&mut self) -> SLCHOST_CONF63_W<24> {
        SLCHOST_CONF63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_w15](index.html) module"]
pub struct CONF_W15_SPEC;
impl crate::RegisterSpec for CONF_W15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_w15::R](R) reader structure"]
impl crate::Readable for CONF_W15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_w15::W](W) writer structure"]
impl crate::Writable for CONF_W15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_W15 to value 0"]
impl crate::Resettable for CONF_W15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
