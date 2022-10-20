#[doc = "Register `GDFIFOCFG` reader"]
pub struct R(crate::R<GDFIFOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GDFIFOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GDFIFOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GDFIFOCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GDFIFOCFG` writer"]
pub struct W(crate::W<GDFIFOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GDFIFOCFG_SPEC>;
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
impl From<crate::W<GDFIFOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GDFIFOCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GDFIFOCFG` reader - "]
pub type GDFIFOCFG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GDFIFOCFG` writer - "]
pub type GDFIFOCFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GDFIFOCFG_SPEC, u16, u16, 16, O>;
#[doc = "Field `EPINFOBASEADDR` reader - "]
pub type EPINFOBASEADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EPINFOBASEADDR` writer - "]
pub type EPINFOBASEADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GDFIFOCFG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gdfifocfg(&self) -> GDFIFOCFG_R {
        GDFIFOCFG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn epinfobaseaddr(&self) -> EPINFOBASEADDR_R {
        EPINFOBASEADDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gdfifocfg(&mut self) -> GDFIFOCFG_W<0> {
        GDFIFOCFG_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn epinfobaseaddr(&mut self) -> EPINFOBASEADDR_W<16> {
        EPINFOBASEADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gdfifocfg](index.html) module"]
pub struct GDFIFOCFG_SPEC;
impl crate::RegisterSpec for GDFIFOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gdfifocfg::R](R) reader structure"]
impl crate::Readable for GDFIFOCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gdfifocfg::W](W) writer structure"]
impl crate::Writable for GDFIFOCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GDFIFOCFG to value 0"]
impl crate::Resettable for GDFIFOCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
