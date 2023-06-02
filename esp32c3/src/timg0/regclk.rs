#[doc = "Register `REGCLK` reader"]
pub struct R(crate::R<REGCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGCLK` writer"]
pub struct W(crate::W<REGCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGCLK_SPEC>;
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
impl From<crate::W<REGCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_CLK_IS_ACTIVE` reader - reg_wdt_clk_is_active."]
pub type WDT_CLK_IS_ACTIVE_R = crate::BitReader;
#[doc = "Field `WDT_CLK_IS_ACTIVE` writer - reg_wdt_clk_is_active."]
pub type WDT_CLK_IS_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, REGCLK_SPEC, O>;
#[doc = "Field `TIMER_CLK_IS_ACTIVE` reader - reg_timer_clk_is_active."]
pub type TIMER_CLK_IS_ACTIVE_R = crate::BitReader;
#[doc = "Field `TIMER_CLK_IS_ACTIVE` writer - reg_timer_clk_is_active."]
pub type TIMER_CLK_IS_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, REGCLK_SPEC, O>;
#[doc = "Field `CLK_EN` reader - reg_clk_en."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - reg_clk_en."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, REGCLK_SPEC, O>;
impl R {
    #[doc = "Bit 29 - reg_wdt_clk_is_active."]
    #[inline(always)]
    pub fn wdt_clk_is_active(&self) -> WDT_CLK_IS_ACTIVE_R {
        WDT_CLK_IS_ACTIVE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reg_timer_clk_is_active."]
    #[inline(always)]
    pub fn timer_clk_is_active(&self) -> TIMER_CLK_IS_ACTIVE_R {
        TIMER_CLK_IS_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_clk_en."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGCLK")
            .field(
                "wdt_clk_is_active",
                &format_args!("{}", self.wdt_clk_is_active().bit()),
            )
            .field(
                "timer_clk_is_active",
                &format_args!("{}", self.timer_clk_is_active().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGCLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 29 - reg_wdt_clk_is_active."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_clk_is_active(&mut self) -> WDT_CLK_IS_ACTIVE_W<29> {
        WDT_CLK_IS_ACTIVE_W::new(self)
    }
    #[doc = "Bit 30 - reg_timer_clk_is_active."]
    #[inline(always)]
    #[must_use]
    pub fn timer_clk_is_active(&mut self) -> TIMER_CLK_IS_ACTIVE_W<30> {
        TIMER_CLK_IS_ACTIVE_W::new(self)
    }
    #[doc = "Bit 31 - reg_clk_en."]
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
#[doc = "TIMG_REGCLK_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regclk](index.html) module"]
pub struct REGCLK_SPEC;
impl crate::RegisterSpec for REGCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regclk::R](R) reader structure"]
impl crate::Readable for REGCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regclk::W](W) writer structure"]
impl crate::Writable for REGCLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGCLK to value 0x6000_0000"]
impl crate::Resettable for REGCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_0000;
}
