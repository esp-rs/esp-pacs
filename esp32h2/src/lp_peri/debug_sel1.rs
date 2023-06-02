#[doc = "Register `DEBUG_SEL1` reader"]
pub struct R(crate::R<DEBUG_SEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_SEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_SEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_SEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_SEL1` writer"]
pub struct W(crate::W<DEBUG_SEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_SEL1_SPEC>;
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
impl From<crate::W<DEBUG_SEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_SEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBUG_SEL4` reader - need des"]
pub type DEBUG_SEL4_R = crate::FieldReader;
#[doc = "Field `DEBUG_SEL4` writer - need des"]
pub type DEBUG_SEL4_W<'a, const O: u8> = crate::FieldWriter<'a, DEBUG_SEL1_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - need des"]
    #[inline(always)]
    pub fn debug_sel4(&self) -> DEBUG_SEL4_R {
        DEBUG_SEL4_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_SEL1")
            .field("debug_sel4", &format_args!("{}", self.debug_sel4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEBUG_SEL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel4(&mut self) -> DEBUG_SEL4_W<0> {
        DEBUG_SEL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_sel1](index.html) module"]
pub struct DEBUG_SEL1_SPEC;
impl crate::RegisterSpec for DEBUG_SEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_sel1::R](R) reader structure"]
impl crate::Readable for DEBUG_SEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_sel1::W](W) writer structure"]
impl crate::Writable for DEBUG_SEL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG_SEL1 to value 0"]
impl crate::Resettable for DEBUG_SEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
