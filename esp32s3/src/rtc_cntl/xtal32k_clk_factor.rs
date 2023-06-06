#[doc = "Register `XTAL32K_CLK_FACTOR` reader"]
pub struct R(crate::R<XTAL32K_CLK_FACTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32K_CLK_FACTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32K_CLK_FACTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32K_CLK_FACTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32K_CLK_FACTOR` writer"]
pub struct W(crate::W<XTAL32K_CLK_FACTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32K_CLK_FACTOR_SPEC>;
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
impl From<crate::W<XTAL32K_CLK_FACTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32K_CLK_FACTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32K_CLK_FACTOR` reader - xtal 32k watch dog backup clock factor"]
pub type XTAL32K_CLK_FACTOR_R = crate::FieldReader<u32>;
#[doc = "Field `XTAL32K_CLK_FACTOR` writer - xtal 32k watch dog backup clock factor"]
pub type XTAL32K_CLK_FACTOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, XTAL32K_CLK_FACTOR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - xtal 32k watch dog backup clock factor"]
    #[inline(always)]
    pub fn xtal32k_clk_factor(&self) -> XTAL32K_CLK_FACTOR_R {
        XTAL32K_CLK_FACTOR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL32K_CLK_FACTOR")
            .field(
                "xtal32k_clk_factor",
                &format_args!("{}", self.xtal32k_clk_factor().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTAL32K_CLK_FACTOR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - xtal 32k watch dog backup clock factor"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_clk_factor(&mut self) -> XTAL32K_CLK_FACTOR_W<0> {
        XTAL32K_CLK_FACTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xtal 32k watch dog backup clock factor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k_clk_factor](index.html) module"]
pub struct XTAL32K_CLK_FACTOR_SPEC;
impl crate::RegisterSpec for XTAL32K_CLK_FACTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal32k_clk_factor::R](R) reader structure"]
impl crate::Readable for XTAL32K_CLK_FACTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32k_clk_factor::W](W) writer structure"]
impl crate::Writable for XTAL32K_CLK_FACTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTAL32K_CLK_FACTOR to value 0"]
impl crate::Resettable for XTAL32K_CLK_FACTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
