#[doc = "Register `RND_ECO_LOW` reader"]
pub struct R(crate::R<RND_ECO_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RND_ECO_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RND_ECO_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RND_ECO_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RND_ECO_LOW` writer"]
pub struct W(crate::W<RND_ECO_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RND_ECO_LOW_SPEC>;
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
impl From<crate::W<RND_ECO_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RND_ECO_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDCY_LOW` reader - Only reserved for ECO."]
pub type REDCY_LOW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REDCY_LOW` writer - Only reserved for ECO."]
pub type REDCY_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RND_ECO_LOW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Only reserved for ECO."]
    #[inline(always)]
    pub fn redcy_low(&self) -> REDCY_LOW_R {
        REDCY_LOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Only reserved for ECO."]
    #[inline(always)]
    #[must_use]
    pub fn redcy_low(&mut self) -> REDCY_LOW_W<0> {
        REDCY_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "redcy eco low register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnd_eco_low](index.html) module"]
pub struct RND_ECO_LOW_SPEC;
impl crate::RegisterSpec for RND_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rnd_eco_low::R](R) reader structure"]
impl crate::Readable for RND_ECO_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rnd_eco_low::W](W) writer structure"]
impl crate::Writable for RND_ECO_LOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RND_ECO_LOW to value 0"]
impl crate::Resettable for RND_ECO_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
