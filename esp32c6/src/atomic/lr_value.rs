#[doc = "Register `LR_VALUE` reader"]
pub struct R(crate::R<LR_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LR_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LR_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LR_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LR_VALUE` writer"]
pub struct W(crate::W<LR_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LR_VALUE_SPEC>;
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
impl From<crate::W<LR_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LR_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLOABLE_LR_VALUE` reader - backup gloable value"]
pub type GLOABLE_LR_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GLOABLE_LR_VALUE` writer - backup gloable value"]
pub type GLOABLE_LR_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LR_VALUE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - backup gloable value"]
    #[inline(always)]
    pub fn gloable_lr_value(&self) -> GLOABLE_LR_VALUE_R {
        GLOABLE_LR_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - backup gloable value"]
    #[inline(always)]
    #[must_use]
    pub fn gloable_lr_value(&mut self) -> GLOABLE_LR_VALUE_W<0> {
        GLOABLE_LR_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gloable lr value regsiter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lr_value](index.html) module"]
pub struct LR_VALUE_SPEC;
impl crate::RegisterSpec for LR_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lr_value::R](R) reader structure"]
impl crate::Readable for LR_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lr_value::W](W) writer structure"]
impl crate::Writable for LR_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LR_VALUE to value 0"]
impl crate::Resettable for LR_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
