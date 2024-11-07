#[doc = "Register `HUNG_CONF` reader"]
pub type R = crate::R<HUNG_CONF_SPEC>;
#[doc = "Register `HUNG_CONF` writer"]
pub type W = crate::W<HUNG_CONF_SPEC>;
#[doc = "Field `TXFIFO_TIMEOUT` reader - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
pub type TXFIFO_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `TXFIFO_TIMEOUT` writer - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
pub type TXFIFO_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` reader - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
pub type TXFIFO_TIMEOUT_SHIFT_R = crate::FieldReader;
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` writer - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
pub type TXFIFO_TIMEOUT_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TXFIFO_TIMEOUT_ENA` reader - The enable bit for txfifo receive data timeout"]
pub type TXFIFO_TIMEOUT_ENA_R = crate::BitReader;
#[doc = "Field `TXFIFO_TIMEOUT_ENA` writer - The enable bit for txfifo receive data timeout"]
pub type TXFIFO_TIMEOUT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_TIMEOUT` reader - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
pub type RXFIFO_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `RXFIFO_TIMEOUT` writer - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
pub type RXFIFO_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` reader - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
pub type RXFIFO_TIMEOUT_SHIFT_R = crate::FieldReader;
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` writer - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
pub type RXFIFO_TIMEOUT_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXFIFO_TIMEOUT_ENA` reader - This is the enable bit for DMA send data timeout"]
pub type RXFIFO_TIMEOUT_ENA_R = crate::BitReader;
#[doc = "Field `RXFIFO_TIMEOUT_ENA` writer - This is the enable bit for DMA send data timeout"]
pub type RXFIFO_TIMEOUT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("txfifo_timeout", &self.txfifo_timeout())
            .field("txfifo_timeout_shift", &self.txfifo_timeout_shift())
            .field("txfifo_timeout_ena", &self.txfifo_timeout_ena())
            .field("rxfifo_timeout", &self.rxfifo_timeout())
            .field("rxfifo_timeout_shift", &self.rxfifo_timeout_shift())
            .field("rxfifo_timeout_ena", &self.rxfifo_timeout_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
    #[inline(always)]
    pub fn txfifo_timeout(&mut self) -> TXFIFO_TIMEOUT_W<HUNG_CONF_SPEC> {
        TXFIFO_TIMEOUT_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
    #[inline(always)]
    pub fn txfifo_timeout_shift(&mut self) -> TXFIFO_TIMEOUT_SHIFT_W<HUNG_CONF_SPEC> {
        TXFIFO_TIMEOUT_SHIFT_W::new(self, 8)
    }
    #[doc = "Bit 11 - The enable bit for txfifo receive data timeout"]
    #[inline(always)]
    pub fn txfifo_timeout_ena(&mut self) -> TXFIFO_TIMEOUT_ENA_W<HUNG_CONF_SPEC> {
        TXFIFO_TIMEOUT_ENA_W::new(self, 11)
    }
    #[doc = "Bits 12:19 - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_timeout(&mut self) -> RXFIFO_TIMEOUT_W<HUNG_CONF_SPEC> {
        RXFIFO_TIMEOUT_W::new(self, 12)
    }
    #[doc = "Bits 20:22 - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
    #[inline(always)]
    pub fn rxfifo_timeout_shift(&mut self) -> RXFIFO_TIMEOUT_SHIFT_W<HUNG_CONF_SPEC> {
        RXFIFO_TIMEOUT_SHIFT_W::new(self, 20)
    }
    #[doc = "Bit 23 - This is the enable bit for DMA send data timeout"]
    #[inline(always)]
    pub fn rxfifo_timeout_ena(&mut self) -> RXFIFO_TIMEOUT_ENA_W<HUNG_CONF_SPEC> {
        RXFIFO_TIMEOUT_ENA_W::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hung_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hung_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUNG_CONF_SPEC;
impl crate::RegisterSpec for HUNG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hung_conf::R`](R) reader structure"]
impl crate::Readable for HUNG_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hung_conf::W`](W) writer structure"]
impl crate::Writable for HUNG_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HUNG_CONF to value 0x0081_0810"]
impl crate::Resettable for HUNG_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0081_0810;
}
