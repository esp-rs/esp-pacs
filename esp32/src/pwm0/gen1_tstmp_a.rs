#[doc = "Register `GEN1_TSTMP_A` reader"]
pub struct R(crate::R<GEN1_TSTMP_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN1_TSTMP_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN1_TSTMP_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN1_TSTMP_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN1_TSTMP_A` writer"]
pub struct W(crate::W<GEN1_TSTMP_A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN1_TSTMP_A_SPEC>;
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
impl From<crate::W<GEN1_TSTMP_A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN1_TSTMP_A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN1_A` reader - "]
pub type GEN1_A_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GEN1_A` writer - "]
pub type GEN1_A_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GEN1_TSTMP_A_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gen1_a(&self) -> GEN1_A_R {
        GEN1_A_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gen1_a(&mut self) -> GEN1_A_W<0> {
        GEN1_A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen1_tstmp_a](index.html) module"]
pub struct GEN1_TSTMP_A_SPEC;
impl crate::RegisterSpec for GEN1_TSTMP_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen1_tstmp_a::R](R) reader structure"]
impl crate::Readable for GEN1_TSTMP_A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen1_tstmp_a::W](W) writer structure"]
impl crate::Writable for GEN1_TSTMP_A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN1_TSTMP_A to value 0"]
impl crate::Resettable for GEN1_TSTMP_A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
