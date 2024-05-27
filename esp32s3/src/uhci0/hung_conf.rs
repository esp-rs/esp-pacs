///Register `HUNG_CONF` reader
pub type R = crate::R<HUNG_CONF_SPEC>;
///Register `HUNG_CONF` writer
pub type W = crate::W<HUNG_CONF_SPEC>;
///Field `TXFIFO_TIMEOUT` reader - This register stores the timeout value. It will produce the UHCI_TX_HUNG_INT interrupt when DMA takes more time to receive data.
pub type TXFIFO_TIMEOUT_R = crate::FieldReader;
///Field `TXFIFO_TIMEOUT` writer - This register stores the timeout value. It will produce the UHCI_TX_HUNG_INT interrupt when DMA takes more time to receive data.
pub type TXFIFO_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TXFIFO_TIMEOUT_SHIFT` reader - This register is used to configure the tick count maximum value.
pub type TXFIFO_TIMEOUT_SHIFT_R = crate::FieldReader;
///Field `TXFIFO_TIMEOUT_SHIFT` writer - This register is used to configure the tick count maximum value.
pub type TXFIFO_TIMEOUT_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TXFIFO_TIMEOUT_ENA` reader - This is the enable bit for Tx-FIFO receive-data timeout.
pub type TXFIFO_TIMEOUT_ENA_R = crate::BitReader;
///Field `TXFIFO_TIMEOUT_ENA` writer - This is the enable bit for Tx-FIFO receive-data timeout.
pub type TXFIFO_TIMEOUT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFO_TIMEOUT` reader - This register stores the timeout value. It will produce the UHCI_RX_HUNG_INT interrupt when DMA takes more time to read data from RAM.
pub type RXFIFO_TIMEOUT_R = crate::FieldReader;
///Field `RXFIFO_TIMEOUT` writer - This register stores the timeout value. It will produce the UHCI_RX_HUNG_INT interrupt when DMA takes more time to read data from RAM.
pub type RXFIFO_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RXFIFO_TIMEOUT_SHIFT` reader - This register is used to configure the tick count maximum value.
pub type RXFIFO_TIMEOUT_SHIFT_R = crate::FieldReader;
///Field `RXFIFO_TIMEOUT_SHIFT` writer - This register is used to configure the tick count maximum value.
pub type RXFIFO_TIMEOUT_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RXFIFO_TIMEOUT_ENA` reader - This is the enable bit for DMA send-data timeout.
pub type RXFIFO_TIMEOUT_ENA_R = crate::BitReader;
///Field `RXFIFO_TIMEOUT_ENA` writer - This is the enable bit for DMA send-data timeout.
pub type RXFIFO_TIMEOUT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - This register stores the timeout value. It will produce the UHCI_TX_HUNG_INT interrupt when DMA takes more time to receive data.
    #[inline(always)]
    pub fn txfifo_timeout(&self) -> TXFIFO_TIMEOUT_R {
        TXFIFO_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:10 - This register is used to configure the tick count maximum value.
    #[inline(always)]
    pub fn txfifo_timeout_shift(&self) -> TXFIFO_TIMEOUT_SHIFT_R {
        TXFIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - This is the enable bit for Tx-FIFO receive-data timeout.
    #[inline(always)]
    pub fn txfifo_timeout_ena(&self) -> TXFIFO_TIMEOUT_ENA_R {
        TXFIFO_TIMEOUT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:19 - This register stores the timeout value. It will produce the UHCI_RX_HUNG_INT interrupt when DMA takes more time to read data from RAM.
    #[inline(always)]
    pub fn rxfifo_timeout(&self) -> RXFIFO_TIMEOUT_R {
        RXFIFO_TIMEOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    ///Bits 20:22 - This register is used to configure the tick count maximum value.
    #[inline(always)]
    pub fn rxfifo_timeout_shift(&self) -> RXFIFO_TIMEOUT_SHIFT_R {
        RXFIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 23 - This is the enable bit for DMA send-data timeout.
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
    ///Bits 0:7 - This register stores the timeout value. It will produce the UHCI_TX_HUNG_INT interrupt when DMA takes more time to receive data.
    #[inline(always)]
    #[must_use]
    pub fn txfifo_timeout(&mut self) -> TXFIFO_TIMEOUT_W<HUNG_CONF_SPEC> {
        TXFIFO_TIMEOUT_W::new(self, 0)
    }
    ///Bits 8:10 - This register is used to configure the tick count maximum value.
    #[inline(always)]
    #[must_use]
    pub fn txfifo_timeout_shift(&mut self) -> TXFIFO_TIMEOUT_SHIFT_W<HUNG_CONF_SPEC> {
        TXFIFO_TIMEOUT_SHIFT_W::new(self, 8)
    }
    ///Bit 11 - This is the enable bit for Tx-FIFO receive-data timeout.
    #[inline(always)]
    #[must_use]
    pub fn txfifo_timeout_ena(&mut self) -> TXFIFO_TIMEOUT_ENA_W<HUNG_CONF_SPEC> {
        TXFIFO_TIMEOUT_ENA_W::new(self, 11)
    }
    ///Bits 12:19 - This register stores the timeout value. It will produce the UHCI_RX_HUNG_INT interrupt when DMA takes more time to read data from RAM.
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_timeout(&mut self) -> RXFIFO_TIMEOUT_W<HUNG_CONF_SPEC> {
        RXFIFO_TIMEOUT_W::new(self, 12)
    }
    ///Bits 20:22 - This register is used to configure the tick count maximum value.
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_timeout_shift(&mut self) -> RXFIFO_TIMEOUT_SHIFT_W<HUNG_CONF_SPEC> {
        RXFIFO_TIMEOUT_SHIFT_W::new(self, 20)
    }
    ///Bit 23 - This is the enable bit for DMA send-data timeout.
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_timeout_ena(&mut self) -> RXFIFO_TIMEOUT_ENA_W<HUNG_CONF_SPEC> {
        RXFIFO_TIMEOUT_ENA_W::new(self, 23)
    }
}
/**Timeout configuration

You can [`read`](crate::generic::Reg::read) this register and get [`hung_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hung_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HUNG_CONF_SPEC;
impl crate::RegisterSpec for HUNG_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hung_conf::R`](R) reader structure
impl crate::Readable for HUNG_CONF_SPEC {}
///`write(|w| ..)` method takes [`hung_conf::W`](W) writer structure
impl crate::Writable for HUNG_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HUNG_CONF to value 0x0081_0810
impl crate::Resettable for HUNG_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0081_0810;
}
