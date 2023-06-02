#[doc = "Register `CACHE_CONTROL` reader"]
pub struct R(crate::R<CACHE_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_CONTROL` writer"]
pub struct W(crate::W<CACHE_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_CONTROL_SPEC>;
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
impl From<crate::W<CACHE_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_CLK_ON` reader - Set this bit to enable clock of i-cache."]
pub type PRO_ICACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `PRO_ICACHE_CLK_ON` writer - Set this bit to enable clock of i-cache."]
pub type PRO_ICACHE_CLK_ON_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_CONTROL_SPEC, O>;
#[doc = "Field `PRO_DCACHE_CLK_ON` reader - Set this bit to enable clock of d-cache."]
pub type PRO_DCACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_CLK_ON` writer - Set this bit to enable clock of d-cache."]
pub type PRO_DCACHE_CLK_ON_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_CONTROL_SPEC, O>;
#[doc = "Field `PRO_CACHE_RESET` reader - Set this bit to reset cache."]
pub type PRO_CACHE_RESET_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_RESET` writer - Set this bit to reset cache."]
pub type PRO_CACHE_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_CONTROL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable clock of i-cache."]
    #[inline(always)]
    pub fn pro_icache_clk_on(&self) -> PRO_ICACHE_CLK_ON_R {
        PRO_ICACHE_CLK_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable clock of d-cache."]
    #[inline(always)]
    pub fn pro_dcache_clk_on(&self) -> PRO_DCACHE_CLK_ON_R {
        PRO_DCACHE_CLK_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset cache."]
    #[inline(always)]
    pub fn pro_cache_reset(&self) -> PRO_CACHE_RESET_R {
        PRO_CACHE_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CONTROL")
            .field(
                "pro_icache_clk_on",
                &format_args!("{}", self.pro_icache_clk_on().bit()),
            )
            .field(
                "pro_dcache_clk_on",
                &format_args!("{}", self.pro_dcache_clk_on().bit()),
            )
            .field(
                "pro_cache_reset",
                &format_args!("{}", self.pro_cache_reset().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_CONTROL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable clock of i-cache."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_clk_on(&mut self) -> PRO_ICACHE_CLK_ON_W<0> {
        PRO_ICACHE_CLK_ON_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to enable clock of d-cache."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_clk_on(&mut self) -> PRO_DCACHE_CLK_ON_W<1> {
        PRO_DCACHE_CLK_ON_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset cache."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_reset(&mut self) -> PRO_CACHE_RESET_W<2> {
        PRO_CACHE_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_control](index.html) module"]
pub struct CACHE_CONTROL_SPEC;
impl crate::RegisterSpec for CACHE_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_control::R](R) reader structure"]
impl crate::Readable for CACHE_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_control::W](W) writer structure"]
impl crate::Writable for CACHE_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_CONTROL to value 0x03"]
impl crate::Resettable for CACHE_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
