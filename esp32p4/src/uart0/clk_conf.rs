#[doc = "Register `CLK_CONF` reader"]
pub type R = crate::R<CLK_CONF_SPEC>;
#[doc = "Register `CLK_CONF` writer"]
pub type W = crate::W<CLK_CONF_SPEC>;
#[doc = "Field `TX_SCLK_EN` reader - Set this bit to enable UART Tx clock."]
pub type TX_SCLK_EN_R = crate::BitReader;
#[doc = "Field `TX_SCLK_EN` writer - Set this bit to enable UART Tx clock."]
pub type TX_SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SCLK_EN` reader - Set this bit to enable UART Rx clock."]
pub type RX_SCLK_EN_R = crate::BitReader;
#[doc = "Field `RX_SCLK_EN` writer - Set this bit to enable UART Rx clock."]
pub type RX_SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_RST_CORE` reader - Write 1 then write 0 to this bit to reset UART Tx."]
pub type TX_RST_CORE_R = crate::BitReader;
#[doc = "Field `TX_RST_CORE` writer - Write 1 then write 0 to this bit to reset UART Tx."]
pub type TX_RST_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RST_CORE` reader - Write 1 then write 0 to this bit to reset UART Rx."]
pub type RX_RST_CORE_R = crate::BitReader;
#[doc = "Field `RX_RST_CORE` writer - Write 1 then write 0 to this bit to reset UART Rx."]
pub type RX_RST_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - Set this bit to enable UART Tx clock."]
    #[inline(always)]
    pub fn tx_sclk_en(&self) -> TX_SCLK_EN_R {
        TX_SCLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to enable UART Rx clock."]
    #[inline(always)]
    pub fn rx_sclk_en(&self) -> RX_SCLK_EN_R {
        RX_SCLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write 1 then write 0 to this bit to reset UART Tx."]
    #[inline(always)]
    pub fn tx_rst_core(&self) -> TX_RST_CORE_R {
        TX_RST_CORE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write 1 then write 0 to this bit to reset UART Rx."]
    #[inline(always)]
    pub fn rx_rst_core(&self) -> RX_RST_CORE_R {
        RX_RST_CORE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field("tx_sclk_en", &format_args!("{}", self.tx_sclk_en().bit()))
            .field("rx_sclk_en", &format_args!("{}", self.rx_sclk_en().bit()))
            .field("tx_rst_core", &format_args!("{}", self.tx_rst_core().bit()))
            .field("rx_rst_core", &format_args!("{}", self.rx_rst_core().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 24 - Set this bit to enable UART Tx clock."]
    #[inline(always)]
    #[must_use]
    pub fn tx_sclk_en(&mut self) -> TX_SCLK_EN_W<CLK_CONF_SPEC> {
        TX_SCLK_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Set this bit to enable UART Rx clock."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sclk_en(&mut self) -> RX_SCLK_EN_W<CLK_CONF_SPEC> {
        RX_SCLK_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Write 1 then write 0 to this bit to reset UART Tx."]
    #[inline(always)]
    #[must_use]
    pub fn tx_rst_core(&mut self) -> TX_RST_CORE_W<CLK_CONF_SPEC> {
        TX_RST_CORE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Write 1 then write 0 to this bit to reset UART Rx."]
    #[inline(always)]
    #[must_use]
    pub fn rx_rst_core(&mut self) -> RX_RST_CORE_W<CLK_CONF_SPEC> {
        RX_RST_CORE_W::new(self, 27)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART core clock configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x0300_0000"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0000;
}
