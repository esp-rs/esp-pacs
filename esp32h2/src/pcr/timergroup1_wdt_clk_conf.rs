#[doc = "Register `TIMERGROUP1_WDT_CLK_CONF` reader"]
pub struct R(crate::R<TIMERGROUP1_WDT_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMERGROUP1_WDT_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMERGROUP1_WDT_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMERGROUP1_WDT_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMERGROUP1_WDT_CLK_CONF` writer"]
pub struct W(crate::W<TIMERGROUP1_WDT_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMERGROUP1_WDT_CLK_CONF_SPEC>;
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
impl From<crate::W<TIMERGROUP1_WDT_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMERGROUP1_WDT_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TG1_WDT_CLK_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
pub type TG1_WDT_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `TG1_WDT_CLK_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
pub type TG1_WDT_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, TIMERGROUP1_WDT_CLK_CONF_SPEC, 2, O>;
#[doc = "Field `TG1_WDT_CLK_EN` reader - Set 1 to enable timer_group0 wdt clock"]
pub type TG1_WDT_CLK_EN_R = crate::BitReader;
#[doc = "Field `TG1_WDT_CLK_EN` writer - Set 1 to enable timer_group0 wdt clock"]
pub type TG1_WDT_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, TIMERGROUP1_WDT_CLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
    #[inline(always)]
    pub fn tg1_wdt_clk_sel(&self) -> TG1_WDT_CLK_SEL_R {
        TG1_WDT_CLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable timer_group0 wdt clock"]
    #[inline(always)]
    pub fn tg1_wdt_clk_en(&self) -> TG1_WDT_CLK_EN_R {
        TG1_WDT_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGROUP1_WDT_CLK_CONF")
            .field(
                "tg1_wdt_clk_sel",
                &format_args!("{}", self.tg1_wdt_clk_sel().bits()),
            )
            .field(
                "tg1_wdt_clk_en",
                &format_args!("{}", self.tg1_wdt_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMERGROUP1_WDT_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): XTAL, 1: 80MHz, 2: FOSC, 3: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tg1_wdt_clk_sel(&mut self) -> TG1_WDT_CLK_SEL_W<20> {
        TG1_WDT_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable timer_group0 wdt clock"]
    #[inline(always)]
    #[must_use]
    pub fn tg1_wdt_clk_en(&mut self) -> TG1_WDT_CLK_EN_W<22> {
        TG1_WDT_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMERGROUP1_WDT_CLK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timergroup1_wdt_clk_conf](index.html) module"]
pub struct TIMERGROUP1_WDT_CLK_CONF_SPEC;
impl crate::RegisterSpec for TIMERGROUP1_WDT_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timergroup1_wdt_clk_conf::R](R) reader structure"]
impl crate::Readable for TIMERGROUP1_WDT_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timergroup1_wdt_clk_conf::W](W) writer structure"]
impl crate::Writable for TIMERGROUP1_WDT_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMERGROUP1_WDT_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for TIMERGROUP1_WDT_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
