#[doc = "Register `PARL_CLK_RX_CONF` reader"]
pub type R = crate::R<PARL_CLK_RX_CONF_SPEC>;
#[doc = "Register `PARL_CLK_RX_CONF` writer"]
pub type W = crate::W<PARL_CLK_RX_CONF_SPEC>;
#[doc = "Field `PARL_CLK_RX_DIV_NUM` reader - The integral part of the frequency divider factor of the parl rx clock."]
pub type PARL_CLK_RX_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `PARL_CLK_RX_DIV_NUM` writer - The integral part of the frequency divider factor of the parl rx clock."]
pub type PARL_CLK_RX_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `PARL_CLK_RX_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad."]
pub type PARL_CLK_RX_SEL_R = crate::FieldReader;
#[doc = "Field `PARL_CLK_RX_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad."]
pub type PARL_CLK_RX_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PARL_CLK_RX_EN` reader - Set 1 to enable parl rx clock"]
pub type PARL_CLK_RX_EN_R = crate::BitReader;
#[doc = "Field `PARL_CLK_RX_EN` writer - Set 1 to enable parl rx clock"]
pub type PARL_CLK_RX_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PARL_RX_RST_EN` reader - Set 0 to reset parl rx module"]
pub type PARL_RX_RST_EN_R = crate::BitReader;
#[doc = "Field `PARL_RX_RST_EN` writer - Set 0 to reset parl rx module"]
pub type PARL_RX_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn parl_clk_rx_div_num(&mut self) -> PARL_CLK_RX_DIV_NUM_W<PARL_CLK_RX_CONF_SPEC, 0> {
        PARL_CLK_RX_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 16:17 - set this field to select clock-source. 0(default): XTAL, 1: 240MHz, 2: FOSC, 3: user clock from pad."]
    #[inline(always)]
    #[must_use]
    pub fn parl_clk_rx_sel(&mut self) -> PARL_CLK_RX_SEL_W<PARL_CLK_RX_CONF_SPEC, 16> {
        PARL_CLK_RX_SEL_W::new(self)
    }
    #[doc = "Bit 18 - Set 1 to enable parl rx clock"]
    #[inline(always)]
    #[must_use]
    pub fn parl_clk_rx_en(&mut self) -> PARL_CLK_RX_EN_W<PARL_CLK_RX_CONF_SPEC, 18> {
        PARL_CLK_RX_EN_W::new(self)
    }
    #[doc = "Bit 19 - Set 0 to reset parl rx module"]
    #[inline(always)]
    #[must_use]
    pub fn parl_rx_rst_en(&mut self) -> PARL_RX_RST_EN_W<PARL_CLK_RX_CONF_SPEC, 19> {
        PARL_RX_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PARL_CLK_RX configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`parl_clk_rx_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parl_clk_rx_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PARL_CLK_RX_CONF_SPEC;
impl crate::RegisterSpec for PARL_CLK_RX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`parl_clk_rx_conf::R`](R) reader structure"]
impl crate::Readable for PARL_CLK_RX_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`parl_clk_rx_conf::W`](W) writer structure"]
impl crate::Writable for PARL_CLK_RX_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PARL_CLK_RX_CONF to value 0x0004_0000"]
impl crate::Resettable for PARL_CLK_RX_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0000;
}
