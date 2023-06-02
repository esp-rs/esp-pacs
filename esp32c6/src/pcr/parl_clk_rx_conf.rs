#[doc = "Register `PARL_CLK_RX_CONF` reader"]
pub struct R(crate::R<PARL_CLK_RX_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARL_CLK_RX_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARL_CLK_RX_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARL_CLK_RX_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PARL_CLK_RX_CONF` writer"]
pub struct W(crate::W<PARL_CLK_RX_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PARL_CLK_RX_CONF_SPEC>;
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
impl From<crate::W<PARL_CLK_RX_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PARL_CLK_RX_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARL_CLK_RX_DIV_NUM` reader - The integral part of the frequency divider factor of the parl rx clock."]
pub type PARL_CLK_RX_DIV_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PARL_CLK_RX_DIV_NUM` writer - The integral part of the frequency divider factor of the parl rx clock."]
pub type PARL_CLK_RX_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, PARL_CLK_RX_CONF_SPEC, 16, O, u16, u16>;
#[doc = "Field `PARL_CLK_RX_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad."]
pub type PARL_CLK_RX_SEL_R = crate::FieldReader;
#[doc = "Field `PARL_CLK_RX_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad."]
pub type PARL_CLK_RX_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, PARL_CLK_RX_CONF_SPEC, 2, O>;
#[doc = "Field `PARL_CLK_RX_EN` reader - Set 1 to enable parl rx clock"]
pub type PARL_CLK_RX_EN_R = crate::BitReader;
#[doc = "Field `PARL_CLK_RX_EN` writer - Set 1 to enable parl rx clock"]
pub type PARL_CLK_RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, PARL_CLK_RX_CONF_SPEC, O>;
#[doc = "Field `PARL_RX_RST_EN` reader - Set 0 to reset parl rx module"]
pub type PARL_RX_RST_EN_R = crate::BitReader;
#[doc = "Field `PARL_RX_RST_EN` writer - Set 0 to reset parl rx module"]
pub type PARL_RX_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, PARL_CLK_RX_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - The integral part of the frequency divider factor of the parl rx clock."]
    #[inline(always)]
    pub fn parl_clk_rx_div_num(&self) -> PARL_CLK_RX_DIV_NUM_R {
        PARL_CLK_RX_DIV_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad."]
    #[inline(always)]
    pub fn parl_clk_rx_sel(&self) -> PARL_CLK_RX_SEL_R {
        PARL_CLK_RX_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Set 1 to enable parl rx clock"]
    #[inline(always)]
    pub fn parl_clk_rx_en(&self) -> PARL_CLK_RX_EN_R {
        PARL_CLK_RX_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set 0 to reset parl rx module"]
    #[inline(always)]
    pub fn parl_rx_rst_en(&self) -> PARL_RX_RST_EN_R {
        PARL_RX_RST_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARL_CLK_RX_CONF")
            .field(
                "parl_clk_rx_div_num",
                &format_args!("{}", self.parl_clk_rx_div_num().bits()),
            )
            .field(
                "parl_clk_rx_sel",
                &format_args!("{}", self.parl_clk_rx_sel().bits()),
            )
            .field(
                "parl_clk_rx_en",
                &format_args!("{}", self.parl_clk_rx_en().bit()),
            )
            .field(
                "parl_rx_rst_en",
                &format_args!("{}", self.parl_rx_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PARL_CLK_RX_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The integral part of the frequency divider factor of the parl rx clock."]
    #[inline(always)]
    #[must_use]
    pub fn parl_clk_rx_div_num(&mut self) -> PARL_CLK_RX_DIV_NUM_W<0> {
        PARL_CLK_RX_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 16:17 - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad."]
    #[inline(always)]
    #[must_use]
    pub fn parl_clk_rx_sel(&mut self) -> PARL_CLK_RX_SEL_W<16> {
        PARL_CLK_RX_SEL_W::new(self)
    }
    #[doc = "Bit 18 - Set 1 to enable parl rx clock"]
    #[inline(always)]
    #[must_use]
    pub fn parl_clk_rx_en(&mut self) -> PARL_CLK_RX_EN_W<18> {
        PARL_CLK_RX_EN_W::new(self)
    }
    #[doc = "Bit 19 - Set 0 to reset parl rx module"]
    #[inline(always)]
    #[must_use]
    pub fn parl_rx_rst_en(&mut self) -> PARL_RX_RST_EN_W<19> {
        PARL_RX_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PARL_CLK_RX configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parl_clk_rx_conf](index.html) module"]
pub struct PARL_CLK_RX_CONF_SPEC;
impl crate::RegisterSpec for PARL_CLK_RX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [parl_clk_rx_conf::R](R) reader structure"]
impl crate::Readable for PARL_CLK_RX_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [parl_clk_rx_conf::W](W) writer structure"]
impl crate::Writable for PARL_CLK_RX_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PARL_CLK_RX_CONF to value 0x0004_0000"]
impl crate::Resettable for PARL_CLK_RX_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0000;
}
