#[doc = "Register `ASSIST_DEBUG_INTR_MAP` reader"]
pub struct R(crate::R<ASSIST_DEBUG_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASSIST_DEBUG_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASSIST_DEBUG_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASSIST_DEBUG_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASSIST_DEBUG_INTR_MAP` writer"]
pub struct W(crate::W<ASSIST_DEBUG_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASSIST_DEBUG_INTR_MAP_SPEC>;
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
impl From<crate::W<ASSIST_DEBUG_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASSIST_DEBUG_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASSIST_DEBUG_INTR_MAP` reader - Need add description"]
pub type ASSIST_DEBUG_INTR_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ASSIST_DEBUG_INTR_MAP` writer - Need add description"]
pub type ASSIST_DEBUG_INTR_MAP_W<'a> =
    crate::FieldWriter<'a, u32, ASSIST_DEBUG_INTR_MAP_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn assist_debug_intr_map(&self) -> ASSIST_DEBUG_INTR_MAP_R {
        ASSIST_DEBUG_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn assist_debug_intr_map(&mut self) -> ASSIST_DEBUG_INTR_MAP_W {
        ASSIST_DEBUG_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assist_debug_intr_map](index.html) module"]
pub struct ASSIST_DEBUG_INTR_MAP_SPEC;
impl crate::RegisterSpec for ASSIST_DEBUG_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [assist_debug_intr_map::R](R) reader structure"]
impl crate::Readable for ASSIST_DEBUG_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [assist_debug_intr_map::W](W) writer structure"]
impl crate::Writable for ASSIST_DEBUG_INTR_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASSIST_DEBUG_INTR_MAP to value 0"]
impl crate::Resettable for ASSIST_DEBUG_INTR_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
