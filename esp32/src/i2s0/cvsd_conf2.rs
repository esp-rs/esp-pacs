#[doc = "Register `CVSD_CONF2` reader"]
pub struct R(crate::R<CVSD_CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CVSD_CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CVSD_CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CVSD_CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CVSD_CONF2` writer"]
pub struct W(crate::W<CVSD_CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CVSD_CONF2_SPEC>;
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
impl From<crate::W<CVSD_CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CVSD_CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVSD_K` reader - "]
pub type CVSD_K_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CVSD_K` writer - "]
pub type CVSD_K_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CVSD_CONF2_SPEC, u8, u8, 3, O>;
#[doc = "Field `CVSD_J` reader - "]
pub type CVSD_J_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CVSD_J` writer - "]
pub type CVSD_J_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CVSD_CONF2_SPEC, u8, u8, 3, O>;
#[doc = "Field `CVSD_BETA` reader - "]
pub type CVSD_BETA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CVSD_BETA` writer - "]
pub type CVSD_BETA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CVSD_CONF2_SPEC, u16, u16, 10, O>;
#[doc = "Field `CVSD_H` reader - "]
pub type CVSD_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CVSD_H` writer - "]
pub type CVSD_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CVSD_CONF2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cvsd_k(&self) -> CVSD_K_R {
        CVSD_K_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn cvsd_j(&self) -> CVSD_J_R {
        CVSD_J_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn cvsd_beta(&self) -> CVSD_BETA_R {
        CVSD_BETA_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cvsd_h(&self) -> CVSD_H_R {
        CVSD_H_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cvsd_k(&mut self) -> CVSD_K_W<0> {
        CVSD_K_W::new(self)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn cvsd_j(&mut self) -> CVSD_J_W<3> {
        CVSD_J_W::new(self)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn cvsd_beta(&mut self) -> CVSD_BETA_W<6> {
        CVSD_BETA_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cvsd_h(&mut self) -> CVSD_H_W<16> {
        CVSD_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvsd_conf2](index.html) module"]
pub struct CVSD_CONF2_SPEC;
impl crate::RegisterSpec for CVSD_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cvsd_conf2::R](R) reader structure"]
impl crate::Readable for CVSD_CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cvsd_conf2::W](W) writer structure"]
impl crate::Writable for CVSD_CONF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CVSD_CONF2 to value 0x0005_02a4"]
impl crate::Resettable for CVSD_CONF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_02a4
    }
}
