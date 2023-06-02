#[doc = "Register `GEN0_CFG0` reader"]
pub struct R(crate::R<GEN0_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN0_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN0_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN0_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN0_CFG0` writer"]
pub struct W(crate::W<GEN0_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN0_CFG0_SPEC>;
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
impl From<crate::W<GEN0_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN0_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN0_CFG_UPMETHOD` reader - "]
pub type GEN0_CFG_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `GEN0_CFG_UPMETHOD` writer - "]
pub type GEN0_CFG_UPMETHOD_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_CFG0_SPEC, 4, O>;
#[doc = "Field `GEN0_T0_SEL` reader - "]
pub type GEN0_T0_SEL_R = crate::FieldReader;
#[doc = "Field `GEN0_T0_SEL` writer - "]
pub type GEN0_T0_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_CFG0_SPEC, 3, O>;
#[doc = "Field `GEN0_T1_SEL` reader - "]
pub type GEN0_T1_SEL_R = crate::FieldReader;
#[doc = "Field `GEN0_T1_SEL` writer - "]
pub type GEN0_T1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_CFG0_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn gen0_cfg_upmethod(&self) -> GEN0_CFG_UPMETHOD_R {
        GEN0_CFG_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn gen0_t0_sel(&self) -> GEN0_T0_SEL_R {
        GEN0_T0_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn gen0_t1_sel(&self) -> GEN0_T1_SEL_R {
        GEN0_T1_SEL_R::new(((self.bits >> 7) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN0_CFG0")
            .field(
                "gen0_cfg_upmethod",
                &format_args!("{}", self.gen0_cfg_upmethod().bits()),
            )
            .field(
                "gen0_t0_sel",
                &format_args!("{}", self.gen0_t0_sel().bits()),
            )
            .field(
                "gen0_t1_sel",
                &format_args!("{}", self.gen0_t1_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN0_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_cfg_upmethod(&mut self) -> GEN0_CFG_UPMETHOD_W<0> {
        GEN0_CFG_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_t0_sel(&mut self) -> GEN0_T0_SEL_W<4> {
        GEN0_T0_SEL_W::new(self)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_t1_sel(&mut self) -> GEN0_T1_SEL_W<7> {
        GEN0_T1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen0_cfg0](index.html) module"]
pub struct GEN0_CFG0_SPEC;
impl crate::RegisterSpec for GEN0_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen0_cfg0::R](R) reader structure"]
impl crate::Readable for GEN0_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen0_cfg0::W](W) writer structure"]
impl crate::Writable for GEN0_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GEN0_CFG0 to value 0"]
impl crate::Resettable for GEN0_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
