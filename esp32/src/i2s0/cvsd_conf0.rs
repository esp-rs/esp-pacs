#[doc = "Register `CVSD_CONF0` reader"]
pub struct R(crate::R<CVSD_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CVSD_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CVSD_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CVSD_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CVSD_CONF0` writer"]
pub struct W(crate::W<CVSD_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CVSD_CONF0_SPEC>;
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
impl From<crate::W<CVSD_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CVSD_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVSD_Y_MAX` reader - "]
pub type CVSD_Y_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CVSD_Y_MAX` writer - "]
pub type CVSD_Y_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CVSD_CONF0_SPEC, u16, u16, 16, O>;
#[doc = "Field `CVSD_Y_MIN` reader - "]
pub type CVSD_Y_MIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CVSD_Y_MIN` writer - "]
pub type CVSD_Y_MIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CVSD_CONF0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cvsd_y_max(&self) -> CVSD_Y_MAX_R {
        CVSD_Y_MAX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cvsd_y_min(&self) -> CVSD_Y_MIN_R {
        CVSD_Y_MIN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cvsd_y_max(&mut self) -> CVSD_Y_MAX_W<0> {
        CVSD_Y_MAX_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cvsd_y_min(&mut self) -> CVSD_Y_MIN_W<16> {
        CVSD_Y_MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvsd_conf0](index.html) module"]
pub struct CVSD_CONF0_SPEC;
impl crate::RegisterSpec for CVSD_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cvsd_conf0::R](R) reader structure"]
impl crate::Readable for CVSD_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cvsd_conf0::W](W) writer structure"]
impl crate::Writable for CVSD_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CVSD_CONF0 to value 0x8000_7fff"]
impl crate::Resettable for CVSD_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_7fff
    }
}
