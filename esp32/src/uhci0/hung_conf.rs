#[doc = "Register `HUNG_CONF` reader"]
pub struct R(crate::R<HUNG_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUNG_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUNG_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUNG_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HUNG_CONF` writer"]
pub struct W(crate::W<HUNG_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HUNG_CONF_SPEC>;
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
impl From<crate::W<HUNG_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HUNG_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFIFO_TIMEOUT` reader - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
pub type TXFIFO_TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFO_TIMEOUT` writer - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
pub type TXFIFO_TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HUNG_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` reader - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
pub type TXFIFO_TIMEOUT_SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` writer - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
pub type TXFIFO_TIMEOUT_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HUNG_CONF_SPEC, u8, u8, 3, O>;
#[doc = "Field `TXFIFO_TIMEOUT_ENA` reader - The enable bit for txfifo receive data timeout"]
pub type TXFIFO_TIMEOUT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFO_TIMEOUT_ENA` writer - The enable bit for txfifo receive data timeout"]
pub type TXFIFO_TIMEOUT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HUNG_CONF_SPEC, bool, O>;
#[doc = "Field `RXFIFO_TIMEOUT` reader - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
pub type RXFIFO_TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFIFO_TIMEOUT` writer - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
pub type RXFIFO_TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HUNG_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` reader - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
pub type RXFIFO_TIMEOUT_SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` writer - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
pub type RXFIFO_TIMEOUT_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HUNG_CONF_SPEC, u8, u8, 3, O>;
#[doc = "Field `RXFIFO_TIMEOUT_ENA` reader - This is the enable bit for DMA send data timeout"]
pub type RXFIFO_TIMEOUT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO_TIMEOUT_ENA` writer - This is the enable bit for DMA send data timeout"]
pub type RXFIFO_TIMEOUT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HUNG_CONF_SPEC, bool, O>;
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
impl W {
    #[doc = "Bits 0:7 - This register stores the timeout value.when DMA takes more time than this register value to receive a data it will produce uhci_tx_hung_int interrupt."]
    #[inline(always)]
    pub fn txfifo_timeout(&mut self) -> TXFIFO_TIMEOUT_W<0> {
        TXFIFO_TIMEOUT_W::new(self)
    }
    #[doc = "Bits 8:10 - The tick count is cleared when its value >=(17'd8000>>reg_txfifo_timeout_shift)"]
    #[inline(always)]
    pub fn txfifo_timeout_shift(&mut self) -> TXFIFO_TIMEOUT_SHIFT_W<8> {
        TXFIFO_TIMEOUT_SHIFT_W::new(self)
    }
    #[doc = "Bit 11 - The enable bit for txfifo receive data timeout"]
    #[inline(always)]
    pub fn txfifo_timeout_ena(&mut self) -> TXFIFO_TIMEOUT_ENA_W<11> {
        TXFIFO_TIMEOUT_ENA_W::new(self)
    }
    #[doc = "Bits 12:19 - This register stores the timeout value.when DMA takes more time than this register value to read a data from RAM it will produce uhci_rx_hung_int interrupt."]
    #[inline(always)]
    pub fn rxfifo_timeout(&mut self) -> RXFIFO_TIMEOUT_W<12> {
        RXFIFO_TIMEOUT_W::new(self)
    }
    #[doc = "Bits 20:22 - The tick count is cleared when its value >=(17'd8000>>reg_rxfifo_timeout_shift)"]
    #[inline(always)]
    pub fn rxfifo_timeout_shift(&mut self) -> RXFIFO_TIMEOUT_SHIFT_W<20> {
        RXFIFO_TIMEOUT_SHIFT_W::new(self)
    }
    #[doc = "Bit 23 - This is the enable bit for DMA send data timeout"]
    #[inline(always)]
    pub fn rxfifo_timeout_ena(&mut self) -> RXFIFO_TIMEOUT_ENA_W<23> {
        RXFIFO_TIMEOUT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hung_conf](index.html) module"]
pub struct HUNG_CONF_SPEC;
impl crate::RegisterSpec for HUNG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hung_conf::R](R) reader structure"]
impl crate::Readable for HUNG_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hung_conf::W](W) writer structure"]
impl crate::Writable for HUNG_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HUNG_CONF to value 0x0081_0810"]
impl crate::Resettable for HUNG_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0081_0810
    }
}
