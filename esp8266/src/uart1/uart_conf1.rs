#[doc = "Register `UART_CONF1` reader"]
pub struct R(crate::R<UART_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_CONF1` writer"]
pub struct W(crate::W<UART_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_CONF1_SPEC>;
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
impl From<crate::W<UART_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxfifo_full_thrhd` reader - The config bits for rx fifo full threshold,0-127"]
pub type RXFIFO_FULL_THRHD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rxfifo_full_thrhd` writer - The config bits for rx fifo full threshold,0-127"]
pub type RXFIFO_FULL_THRHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CONF1_SPEC, u8, u8, 7, O>;
#[doc = "Field `txfifo_empty_thrhd` reader - The config bits for tx fifo empty threshold,0-127"]
pub type TXFIFO_EMPTY_THRHD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `txfifo_empty_thrhd` writer - The config bits for tx fifo empty threshold,0-127"]
pub type TXFIFO_EMPTY_THRHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CONF1_SPEC, u8, u8, 7, O>;
#[doc = "Field `rx_flow_thrhd` reader - The config bits for rx flow control threshold,0-127"]
pub type RX_FLOW_THRHD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_flow_thrhd` writer - The config bits for rx flow control threshold,0-127"]
pub type RX_FLOW_THRHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CONF1_SPEC, u8, u8, 7, O>;
#[doc = "Field `rx_flow_en` reader - Set this bit to enable rx hardware flow control"]
pub type RX_FLOW_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_flow_en` writer - Set this bit to enable rx hardware flow control"]
pub type RX_FLOW_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF1_SPEC, bool, O>;
#[doc = "Field `rx_tout_thrhd` reader - Config bits for rx time-out threshold,uint: byte,0-127"]
pub type RX_TOUT_THRHD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_tout_thrhd` writer - Config bits for rx time-out threshold,uint: byte,0-127"]
pub type RX_TOUT_THRHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_CONF1_SPEC, u8, u8, 7, O>;
#[doc = "Field `rx_tout_en` reader - Set this bit to enable rx time-out function"]
pub type RX_TOUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_tout_en` writer - Set this bit to enable rx time-out function"]
pub type RX_TOUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_CONF1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - The config bits for rx fifo full threshold,0-127"]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The config bits for tx fifo empty threshold,0-127"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The config bits for rx flow control threshold,0-127"]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Set this bit to enable rx hardware flow control"]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RX_FLOW_EN_R {
        RX_FLOW_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Config bits for rx time-out threshold,uint: byte,0-127"]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Set this bit to enable rx time-out function"]
    #[inline(always)]
    pub fn rx_tout_en(&self) -> RX_TOUT_EN_R {
        RX_TOUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The config bits for rx fifo full threshold,0-127"]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W<0> {
        RXFIFO_FULL_THRHD_W::new(self)
    }
    #[doc = "Bits 8:14 - The config bits for tx fifo empty threshold,0-127"]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W<8> {
        TXFIFO_EMPTY_THRHD_W::new(self)
    }
    #[doc = "Bits 16:22 - The config bits for rx flow control threshold,0-127"]
    #[inline(always)]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W<16> {
        RX_FLOW_THRHD_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to enable rx hardware flow control"]
    #[inline(always)]
    pub fn rx_flow_en(&mut self) -> RX_FLOW_EN_W<23> {
        RX_FLOW_EN_W::new(self)
    }
    #[doc = "Bits 24:30 - Config bits for rx time-out threshold,uint: byte,0-127"]
    #[inline(always)]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W<24> {
        RX_TOUT_THRHD_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to enable rx time-out function"]
    #[inline(always)]
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
#[doc = "Set this bit to enable rx time-out function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_conf1](index.html) module"]
pub struct UART_CONF1_SPEC;
impl crate::RegisterSpec for UART_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_conf1::R](R) reader structure"]
impl crate::Readable for UART_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_conf1::W](W) writer structure"]
impl crate::Writable for UART_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_CONF1 to value 0"]
impl crate::Resettable for UART_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
