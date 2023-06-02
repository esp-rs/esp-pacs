#[doc = "Register `CONF1` reader"]
pub struct R(crate::R<CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF1` writer"]
pub struct W(crate::W<CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF1_SPEC>;
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
impl From<crate::W<CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_FULL_THRHD` reader - When receiver receives more data than its threshold value.receiver will produce rxfifo_full_int_raw interrupt.the threshold value is (rx_flow_thrhd_h3 rxfifo_full_thrhd)."]
pub type RXFIFO_FULL_THRHD_R = crate::FieldReader;
#[doc = "Field `RXFIFO_FULL_THRHD` writer - When receiver receives more data than its threshold value.receiver will produce rxfifo_full_int_raw interrupt.the threshold value is (rx_flow_thrhd_h3 rxfifo_full_thrhd)."]
pub type RXFIFO_FULL_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, CONF1_SPEC, 7, O>;
#[doc = "Field `TXFIFO_EMPTY_THRHD` reader - when the data amount in transmitter fifo is less than its threshold value. it will produce txfifo_empty_int_raw interrupt. the threshold value is (tx_mem_empty_thrhd txfifo_empty_thrhd)"]
pub type TXFIFO_EMPTY_THRHD_R = crate::FieldReader;
#[doc = "Field `TXFIFO_EMPTY_THRHD` writer - when the data amount in transmitter fifo is less than its threshold value. it will produce txfifo_empty_int_raw interrupt. the threshold value is (tx_mem_empty_thrhd txfifo_empty_thrhd)"]
pub type TXFIFO_EMPTY_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, CONF1_SPEC, 7, O>;
#[doc = "Field `RX_FLOW_THRHD` reader - when receiver receives more data than its threshold value. receiver produce signal to tell the transmitter stop transferring data. the threshold value is (rx_flow_thrhd_h3 rx_flow_thrhd)."]
pub type RX_FLOW_THRHD_R = crate::FieldReader;
#[doc = "Field `RX_FLOW_THRHD` writer - when receiver receives more data than its threshold value. receiver produce signal to tell the transmitter stop transferring data. the threshold value is (rx_flow_thrhd_h3 rx_flow_thrhd)."]
pub type RX_FLOW_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, CONF1_SPEC, 7, O>;
#[doc = "Field `RX_FLOW_EN` reader - This is the flow enable bit for uart receiver. 1:choose software flow control with configuring sw_rts signal"]
pub type RX_FLOW_EN_R = crate::BitReader;
#[doc = "Field `RX_FLOW_EN` writer - This is the flow enable bit for uart receiver. 1:choose software flow control with configuring sw_rts signal"]
pub type RX_FLOW_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
#[doc = "Field `RX_TOUT_THRHD` reader - This register is used to configure the timeout value for uart receiver receiving a byte."]
pub type RX_TOUT_THRHD_R = crate::FieldReader;
#[doc = "Field `RX_TOUT_THRHD` writer - This register is used to configure the timeout value for uart receiver receiving a byte."]
pub type RX_TOUT_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, CONF1_SPEC, 7, O>;
#[doc = "Field `RX_TOUT_EN` reader - This is the enble bit for uart receiver's timeout function."]
pub type RX_TOUT_EN_R = crate::BitReader;
#[doc = "Field `RX_TOUT_EN` writer - This is the enble bit for uart receiver's timeout function."]
pub type RX_TOUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
impl R {
    #[doc = "Bits 0:6 - When receiver receives more data than its threshold value.receiver will produce rxfifo_full_int_raw interrupt.the threshold value is (rx_flow_thrhd_h3 rxfifo_full_thrhd)."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - when the data amount in transmitter fifo is less than its threshold value. it will produce txfifo_empty_int_raw interrupt. the threshold value is (tx_mem_empty_thrhd txfifo_empty_thrhd)"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - when receiver receives more data than its threshold value. receiver produce signal to tell the transmitter stop transferring data. the threshold value is (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - This is the flow enable bit for uart receiver. 1:choose software flow control with configuring sw_rts signal"]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RX_FLOW_EN_R {
        RX_FLOW_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - This register is used to configure the timeout value for uart receiver receiving a byte."]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&self) -> RX_TOUT_EN_R {
        RX_TOUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field(
                "rxfifo_full_thrhd",
                &format_args!("{}", self.rxfifo_full_thrhd().bits()),
            )
            .field(
                "txfifo_empty_thrhd",
                &format_args!("{}", self.txfifo_empty_thrhd().bits()),
            )
            .field(
                "rx_flow_thrhd",
                &format_args!("{}", self.rx_flow_thrhd().bits()),
            )
            .field("rx_flow_en", &format_args!("{}", self.rx_flow_en().bit()))
            .field(
                "rx_tout_thrhd",
                &format_args!("{}", self.rx_tout_thrhd().bits()),
            )
            .field("rx_tout_en", &format_args!("{}", self.rx_tout_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - When receiver receives more data than its threshold value.receiver will produce rxfifo_full_int_raw interrupt.the threshold value is (rx_flow_thrhd_h3 rxfifo_full_thrhd)."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W<0> {
        RXFIFO_FULL_THRHD_W::new(self)
    }
    #[doc = "Bits 8:14 - when the data amount in transmitter fifo is less than its threshold value. it will produce txfifo_empty_int_raw interrupt. the threshold value is (tx_mem_empty_thrhd txfifo_empty_thrhd)"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W<8> {
        TXFIFO_EMPTY_THRHD_W::new(self)
    }
    #[doc = "Bits 16:22 - when receiver receives more data than its threshold value. receiver produce signal to tell the transmitter stop transferring data. the threshold value is (rx_flow_thrhd_h3 rx_flow_thrhd)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W<16> {
        RX_FLOW_THRHD_W::new(self)
    }
    #[doc = "Bit 23 - This is the flow enable bit for uart receiver. 1:choose software flow control with configuring sw_rts signal"]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_en(&mut self) -> RX_FLOW_EN_W<23> {
        RX_FLOW_EN_W::new(self)
    }
    #[doc = "Bits 24:30 - This register is used to configure the timeout value for uart receiver receiving a byte."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W<24> {
        RX_TOUT_THRHD_W::new(self)
    }
    #[doc = "Bit 31 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tout_en(&mut self) -> RX_TOUT_EN_W<31> {
        RX_TOUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1](index.html) module"]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf1::R](R) reader structure"]
impl crate::Readable for CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf1::W](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0x6060"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x6060;
}
