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
#[doc = "Field `ICACHE_CLK_ON` reader - Set 1 to enable icache clock"]
pub type ICACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `ICACHE_CLK_ON` writer - Set 1 to enable icache clock"]
pub type ICACHE_CLK_ON_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_CONTROL_SPEC, O>;
#[doc = "Field `ICACHE_RESET` reader - Set 1 to let icache reset"]
pub type ICACHE_RESET_R = crate::BitReader;
#[doc = "Field `ICACHE_RESET` writer - Set 1 to let icache reset"]
pub type ICACHE_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_CONTROL_SPEC, O>;
#[doc = "Field `DCACHE_CLK_ON` reader - Set 1 to enable dcache clock"]
pub type DCACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `DCACHE_CLK_ON` writer - Set 1 to enable dcache clock"]
pub type DCACHE_CLK_ON_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_CONTROL_SPEC, O>;
#[doc = "Field `DCACHE_RESET` reader - Set 1 to let dcache reset"]
pub type DCACHE_RESET_R = crate::BitReader;
#[doc = "Field `DCACHE_RESET` writer - Set 1 to let dcache reset"]
pub type DCACHE_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_CONTROL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable icache clock"]
    #[inline(always)]
    pub fn icache_clk_on(&self) -> ICACHE_CLK_ON_R {
        ICACHE_CLK_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to let icache reset"]
    #[inline(always)]
    pub fn icache_reset(&self) -> ICACHE_RESET_R {
        ICACHE_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable dcache clock"]
    #[inline(always)]
    pub fn dcache_clk_on(&self) -> DCACHE_CLK_ON_R {
        DCACHE_CLK_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to let dcache reset"]
    #[inline(always)]
    pub fn dcache_reset(&self) -> DCACHE_RESET_R {
        DCACHE_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CONTROL")
            .field(
                "icache_clk_on",
                &format_args!("{}", self.icache_clk_on().bit()),
            )
            .field(
                "icache_reset",
                &format_args!("{}", self.icache_reset().bit()),
            )
            .field(
                "dcache_clk_on",
                &format_args!("{}", self.dcache_clk_on().bit()),
            )
            .field(
                "dcache_reset",
                &format_args!("{}", self.dcache_reset().bit()),
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
    #[doc = "Bit 0 - Set 1 to enable icache clock"]
    #[inline(always)]
    #[must_use]
    pub fn icache_clk_on(&mut self) -> ICACHE_CLK_ON_W<0> {
        ICACHE_CLK_ON_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 to let icache reset"]
    #[inline(always)]
    #[must_use]
    pub fn icache_reset(&mut self) -> ICACHE_RESET_W<1> {
        ICACHE_RESET_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to enable dcache clock"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_clk_on(&mut self) -> DCACHE_CLK_ON_W<2> {
        DCACHE_CLK_ON_W::new(self)
    }
    #[doc = "Bit 3 - Set 1 to let dcache reset"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_reset(&mut self) -> DCACHE_RESET_W<3> {
        DCACHE_RESET_W::new(self)
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
#[doc = "`reset()` method sets CACHE_CONTROL to value 0x05"]
impl crate::Resettable for CACHE_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
