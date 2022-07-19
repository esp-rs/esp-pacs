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
#[doc = "Field `WDT_CLK_IS_ACTIVE` reader - enable WDT's clock"]
pub type WDT_CLK_IS_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `WDT_CLK_IS_ACTIVE` writer - enable WDT's clock"]
pub type WDT_CLK_IS_ACTIVE_W<'a> = crate::BitWriter<'a, u32, REGCLK_SPEC, bool, 29>;
#[doc = "Field `TIMER_CLK_IS_ACTIVE` reader - enable Timer 30's clock"]
pub type TIMER_CLK_IS_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_CLK_IS_ACTIVE` writer - enable Timer 30's clock"]
pub type TIMER_CLK_IS_ACTIVE_W<'a> = crate::BitWriter<'a, u32, REGCLK_SPEC, bool, 30>;
#[doc = "Field `CLK_EN` reader - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software."]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software."]
pub type CLK_EN_W<'a> = crate::BitWriter<'a, u32, REGCLK_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 29 - enable WDT's clock"]
    #[inline(always)]
    pub fn wdt_clk_is_active(&self) -> WDT_CLK_IS_ACTIVE_R {
        WDT_CLK_IS_ACTIVE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable Timer 30's clock"]
    #[inline(always)]
    pub fn timer_clk_is_active(&self) -> TIMER_CLK_IS_ACTIVE_R {
        TIMER_CLK_IS_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - enable WDT's clock"]
    #[inline(always)]
    pub fn wdt_clk_is_active(&mut self) -> WDT_CLK_IS_ACTIVE_W {
        WDT_CLK_IS_ACTIVE_W::new(self)
    }
    #[doc = "Bit 30 - enable Timer 30's clock"]
    #[inline(always)]
    pub fn timer_clk_is_active(&mut self) -> TIMER_CLK_IS_ACTIVE_W {
        TIMER_CLK_IS_ACTIVE_W::new(self)
    }
    #[doc = "Bit 31 - Register clock gate signal. 1: Registers can be read and written to by software. 0: Registers can not be read or written to by software."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer group clock gate register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regclk](index.html) module"]
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
}
#[doc = "`reset()` method sets REGCLK to value 0x6000_0000"]
impl crate::Resettable for REGCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000_0000
    }
}
