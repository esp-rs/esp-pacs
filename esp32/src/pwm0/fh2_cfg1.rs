#[doc = "Register `FH2_CFG1` reader"]
pub struct R(crate::R<FH2_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FH2_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FH2_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FH2_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FH2_CFG1` writer"]
pub struct W(crate::W<FH2_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FH2_CFG1_SPEC>;
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
impl From<crate::W<FH2_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FH2_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FH2_CLR_OST` reader - "]
pub type FH2_CLR_OST_R = crate::BitReader<bool>;
#[doc = "Field `FH2_CLR_OST` writer - "]
pub type FH2_CLR_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG1_SPEC, bool, O>;
#[doc = "Field `FH2_CBCPULSE` reader - "]
pub type FH2_CBCPULSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FH2_CBCPULSE` writer - "]
pub type FH2_CBCPULSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FH2_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `FH2_FORCE_CBC` reader - "]
pub type FH2_FORCE_CBC_R = crate::BitReader<bool>;
#[doc = "Field `FH2_FORCE_CBC` writer - "]
pub type FH2_FORCE_CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG1_SPEC, bool, O>;
#[doc = "Field `FH2_FORCE_OST` reader - "]
pub type FH2_FORCE_OST_R = crate::BitReader<bool>;
#[doc = "Field `FH2_FORCE_OST` writer - "]
pub type FH2_FORCE_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FH2_CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh2_clr_ost(&self) -> FH2_CLR_OST_R {
        FH2_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fh2_cbcpulse(&self) -> FH2_CBCPULSE_R {
        FH2_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh2_force_cbc(&self) -> FH2_FORCE_CBC_R {
        FH2_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh2_force_ost(&self) -> FH2_FORCE_OST_R {
        FH2_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fh2_clr_ost(&mut self) -> FH2_CLR_OST_W<0> {
        FH2_CLR_OST_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fh2_cbcpulse(&mut self) -> FH2_CBCPULSE_W<1> {
        FH2_CBCPULSE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fh2_force_cbc(&mut self) -> FH2_FORCE_CBC_W<3> {
        FH2_FORCE_CBC_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fh2_force_ost(&mut self) -> FH2_FORCE_OST_W<4> {
        FH2_FORCE_OST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fh2_cfg1](index.html) module"]
pub struct FH2_CFG1_SPEC;
impl crate::RegisterSpec for FH2_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fh2_cfg1::R](R) reader structure"]
impl crate::Readable for FH2_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fh2_cfg1::W](W) writer structure"]
impl crate::Writable for FH2_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FH2_CFG1 to value 0"]
impl crate::Resettable for FH2_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
