#[doc = "Register `HUNG_CONF` reader"]
pub type R = crate::R<HUNG_CONF_SPEC>;
#[doc = "Register `HUNG_CONF` writer"]
pub type W = crate::W<HUNG_CONF_SPEC>;
#[doc = "Field `TXFIFO_TIMEOUT` reader - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
pub type TXFIFO_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `TXFIFO_TIMEOUT` writer - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
pub type TXFIFO_TIMEOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` reader - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
pub type TXFIFO_TIMEOUT_SHIFT_R = crate::FieldReader;
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` writer - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
pub type TXFIFO_TIMEOUT_SHIFT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TXFIFO_TIMEOUT_ENA` reader - The enable bit for txfifo receive data timeout"]
pub type TXFIFO_TIMEOUT_ENA_R = crate::BitReader;
#[doc = "Field `TXFIFO_TIMEOUT_ENA` writer - The enable bit for txfifo receive data timeout"]
pub type TXFIFO_TIMEOUT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFIFO_TIMEOUT` reader - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
pub type RXFIFO_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `RXFIFO_TIMEOUT` writer - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
pub type RXFIFO_TIMEOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` reader - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
pub type RXFIFO_TIMEOUT_SHIFT_R = crate::FieldReader;
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` writer - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
pub type RXFIFO_TIMEOUT_SHIFT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RXFIFO_TIMEOUT_ENA` reader - This is the enable bit for DMA send data timeout"]
pub type RXFIFO_TIMEOUT_ENA_R = crate::BitReader;
#[doc = "Field `RXFIFO_TIMEOUT_ENA` writer - This is the enable bit for DMA send data timeout"]
pub type RXFIFO_TIMEOUT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
    #[inline(always)]
    pub fn txfifo_timeout(&self) -> TXFIFO_TIMEOUT_R {
        TXFIFO_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
    #[inline(always)]
    pub fn txfifo_timeout_shift(&self) -> TXFIFO_TIMEOUT_SHIFT_R {
        TXFIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - The enable bit for txfifo receive data timeout"]
    #[inline(always)]
    pub fn txfifo_timeout_ena(&self) -> TXFIFO_TIMEOUT_ENA_R {
        TXFIFO_TIMEOUT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:19 - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_timeout(&self) -> RXFIFO_TIMEOUT_R {
        RXFIFO_TIMEOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:22 - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
    #[inline(always)]
    pub fn rxfifo_timeout_shift(&self) -> RXFIFO_TIMEOUT_SHIFT_R {
        RXFIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - This is the enable bit for DMA send data timeout"]
    #[inline(always)]
    pub fn rxfifo_timeout_ena(&self) -> RXFIFO_TIMEOUT_ENA_R {
        RXFIFO_TIMEOUT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUNG_CONF")
            .field(
                "txfifo_timeout",
                &format_args!("{}", self.txfifo_timeout().bits()),
            )
            .field(
                "txfifo_timeout_shift",
                &format_args!("{}", self.txfifo_timeout_shift().bits()),
            )
            .field(
                "txfifo_timeout_ena",
                &format_args!("{}", self.txfifo_timeout_ena().bit()),
            )
            .field(
                "rxfifo_timeout",
                &format_args!("{}", self.rxfifo_timeout().bits()),
            )
            .field(
                "rxfifo_timeout_shift",
                &format_args!("{}", self.rxfifo_timeout_shift().bits()),
            )
            .field(
                "rxfifo_timeout_ena",
                &format_args!("{}", self.rxfifo_timeout_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HUNG_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_timeout(&mut self) -> TXFIFO_TIMEOUT_W<HUNG_CONF_SPEC, 0> {
        TXFIFO_TIMEOUT_W::new(self)
    }
    #[doc = "Bits 8:10 - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_timeout_shift(&mut self) -> TXFIFO_TIMEOUT_SHIFT_W<HUNG_CONF_SPEC, 8> {
        TXFIFO_TIMEOUT_SHIFT_W::new(self)
    }
    #[doc = "Bit 11 - The enable bit for txfifo receive data timeout"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_timeout_ena(&mut self) -> TXFIFO_TIMEOUT_ENA_W<HUNG_CONF_SPEC, 11> {
        TXFIFO_TIMEOUT_ENA_W::new(self)
    }
    #[doc = "Bits 12:19 - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_timeout(&mut self) -> RXFIFO_TIMEOUT_W<HUNG_CONF_SPEC, 12> {
        RXFIFO_TIMEOUT_W::new(self)
    }
    #[doc = "Bits 20:22 - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_timeout_shift(&mut self) -> RXFIFO_TIMEOUT_SHIFT_W<HUNG_CONF_SPEC, 20> {
        RXFIFO_TIMEOUT_SHIFT_W::new(self)
    }
    #[doc = "Bit 23 - This is the enable bit for DMA send data timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_timeout_ena(&mut self) -> RXFIFO_TIMEOUT_ENA_W<HUNG_CONF_SPEC, 23> {
        RXFIFO_TIMEOUT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hung_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hung_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUNG_CONF_SPEC;
impl crate::RegisterSpec for HUNG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hung_conf::R`](R) reader structure"]
impl crate::Readable for HUNG_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hung_conf::W`](W) writer structure"]
impl crate::Writable for HUNG_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HUNG_CONF to value 0x0081_0810"]
impl crate::Resettable for HUNG_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0081_0810;
}
