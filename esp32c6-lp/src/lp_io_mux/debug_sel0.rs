#[doc = "Register `DEBUG_SEL0` reader"]
pub struct R(crate::R<DEBUG_SEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_SEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_SEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_SEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_SEL0` writer"]
pub struct W(crate::W<DEBUG_SEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_SEL0_SPEC>;
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
impl From<crate::W<DEBUG_SEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_SEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_DEBUG_SEL0` reader - need des"]
pub type LP_DEBUG_SEL0_R = crate::FieldReader;
#[doc = "Field `LP_DEBUG_SEL0` writer - need des"]
pub type LP_DEBUG_SEL0_W<'a, const O: u8> = crate::FieldWriter<'a, DEBUG_SEL0_SPEC, 7, O>;
#[doc = "Field `LP_DEBUG_SEL1` reader - need des"]
pub type LP_DEBUG_SEL1_R = crate::FieldReader;
#[doc = "Field `LP_DEBUG_SEL1` writer - need des"]
pub type LP_DEBUG_SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, DEBUG_SEL0_SPEC, 7, O>;
#[doc = "Field `LP_DEBUG_SEL2` reader - need des"]
pub type LP_DEBUG_SEL2_R = crate::FieldReader;
#[doc = "Field `LP_DEBUG_SEL2` writer - need des"]
pub type LP_DEBUG_SEL2_W<'a, const O: u8> = crate::FieldWriter<'a, DEBUG_SEL0_SPEC, 7, O>;
#[doc = "Field `LP_DEBUG_SEL3` reader - need des"]
pub type LP_DEBUG_SEL3_R = crate::FieldReader;
#[doc = "Field `LP_DEBUG_SEL3` writer - need des"]
pub type LP_DEBUG_SEL3_W<'a, const O: u8> = crate::FieldWriter<'a, DEBUG_SEL0_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - need des"]
    #[inline(always)]
    pub fn lp_debug_sel0(&self) -> LP_DEBUG_SEL0_R {
        LP_DEBUG_SEL0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - need des"]
    #[inline(always)]
    pub fn lp_debug_sel1(&self) -> LP_DEBUG_SEL1_R {
        LP_DEBUG_SEL1_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - need des"]
    #[inline(always)]
    pub fn lp_debug_sel2(&self) -> LP_DEBUG_SEL2_R {
        LP_DEBUG_SEL2_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:27 - need des"]
    #[inline(always)]
    pub fn lp_debug_sel3(&self) -> LP_DEBUG_SEL3_R {
        LP_DEBUG_SEL3_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_SEL0")
            .field(
                "lp_debug_sel0",
                &format_args!("{}", self.lp_debug_sel0().bits()),
            )
            .field(
                "lp_debug_sel1",
                &format_args!("{}", self.lp_debug_sel1().bits()),
            )
            .field(
                "lp_debug_sel2",
                &format_args!("{}", self.lp_debug_sel2().bits()),
            )
            .field(
                "lp_debug_sel3",
                &format_args!("{}", self.lp_debug_sel3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEBUG_SEL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_debug_sel0(&mut self) -> LP_DEBUG_SEL0_W<0> {
        LP_DEBUG_SEL0_W::new(self)
    }
    #[doc = "Bits 7:13 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_debug_sel1(&mut self) -> LP_DEBUG_SEL1_W<7> {
        LP_DEBUG_SEL1_W::new(self)
    }
    #[doc = "Bits 14:20 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_debug_sel2(&mut self) -> LP_DEBUG_SEL2_W<14> {
        LP_DEBUG_SEL2_W::new(self)
    }
    #[doc = "Bits 21:27 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_debug_sel3(&mut self) -> LP_DEBUG_SEL3_W<21> {
        LP_DEBUG_SEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_sel0](index.html) module"]
pub struct DEBUG_SEL0_SPEC;
impl crate::RegisterSpec for DEBUG_SEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_sel0::R](R) reader structure"]
impl crate::Readable for DEBUG_SEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_sel0::W](W) writer structure"]
impl crate::Writable for DEBUG_SEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG_SEL0 to value 0"]
impl crate::Resettable for DEBUG_SEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
