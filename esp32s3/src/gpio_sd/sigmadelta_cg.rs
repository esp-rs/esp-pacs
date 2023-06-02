#[doc = "Register `SIGMADELTA_CG` reader"]
pub struct R(crate::R<SIGMADELTA_CG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA_CG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA_CG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA_CG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA_CG` writer"]
pub struct W(crate::W<SIGMADELTA_CG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA_CG_SPEC>;
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
impl From<crate::W<SIGMADELTA_CG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA_CG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - Clock enable bit of configuration registers for sigma delta modulation."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Clock enable bit of configuration registers for sigma delta modulation."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, SIGMADELTA_CG_SPEC, O>;
impl R {
    #[doc = "Bit 31 - Clock enable bit of configuration registers for sigma delta modulation."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA_CG")
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA_CG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - Clock enable bit of configuration registers for sigma delta modulation."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Gating Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta_cg](index.html) module"]
pub struct SIGMADELTA_CG_SPEC;
impl crate::RegisterSpec for SIGMADELTA_CG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta_cg::R](R) reader structure"]
impl crate::Readable for SIGMADELTA_CG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta_cg::W](W) writer structure"]
impl crate::Writable for SIGMADELTA_CG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA_CG to value 0"]
impl crate::Resettable for SIGMADELTA_CG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
