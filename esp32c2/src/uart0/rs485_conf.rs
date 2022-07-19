#[doc = "Register `RS485_CONF` reader"]
pub struct R(crate::R<RS485_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RS485_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RS485_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RS485_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RS485_CONF` writer"]
pub struct W(crate::W<RS485_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RS485_CONF_SPEC>;
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
impl From<crate::W<RS485_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RS485_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS485_EN` reader - Set this bit to choose the rs485 mode."]
pub type RS485_EN_R = crate::BitReader<bool>;
#[doc = "Field `RS485_EN` writer - Set this bit to choose the rs485 mode."]
pub type RS485_EN_W<'a> = crate::BitWriter<'a, u32, RS485_CONF_SPEC, bool, 0>;
#[doc = "Field `DL0_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type DL0_EN_R = crate::BitReader<bool>;
#[doc = "Field `DL0_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type DL0_EN_W<'a> = crate::BitWriter<'a, u32, RS485_CONF_SPEC, bool, 1>;
#[doc = "Field `DL1_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type DL1_EN_R = crate::BitReader<bool>;
#[doc = "Field `DL1_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type DL1_EN_W<'a> = crate::BitWriter<'a, u32, RS485_CONF_SPEC, bool, 2>;
#[doc = "Field `RS485TX_RX_EN` reader - Set this bit to enable receiver could receive data when the transmitter is transmitting data in rs485 mode."]
pub type RS485TX_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RS485TX_RX_EN` writer - Set this bit to enable receiver could receive data when the transmitter is transmitting data in rs485 mode."]
pub type RS485TX_RX_EN_W<'a> = crate::BitWriter<'a, u32, RS485_CONF_SPEC, bool, 3>;
#[doc = "Field `RS485RXBY_TX_EN` reader - 1'h1: enable rs485 transmitter to send data when rs485 receiver line is busy."]
pub type RS485RXBY_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RS485RXBY_TX_EN` writer - 1'h1: enable rs485 transmitter to send data when rs485 receiver line is busy."]
pub type RS485RXBY_TX_EN_W<'a> = crate::BitWriter<'a, u32, RS485_CONF_SPEC, bool, 4>;
#[doc = "Field `RS485_RX_DLY_NUM` reader - This register is used to delay the receiver's internal data signal."]
pub type RS485_RX_DLY_NUM_R = crate::BitReader<bool>;
#[doc = "Field `RS485_RX_DLY_NUM` writer - This register is used to delay the receiver's internal data signal."]
pub type RS485_RX_DLY_NUM_W<'a> = crate::BitWriter<'a, u32, RS485_CONF_SPEC, bool, 5>;
#[doc = "Field `RS485_TX_DLY_NUM` reader - This register is used to delay the transmitter's internal data signal."]
pub type RS485_TX_DLY_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS485_TX_DLY_NUM` writer - This register is used to delay the transmitter's internal data signal."]
pub type RS485_TX_DLY_NUM_W<'a> = crate::FieldWriter<'a, u32, RS485_CONF_SPEC, u8, u8, 4, 6>;
impl R {
    #[doc = "Bit 0 - Set this bit to choose the rs485 mode."]
    #[inline(always)]
    pub fn rs485_en(&self) -> RS485_EN_R {
        RS485_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&self) -> DL0_EN_R {
        DL0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&self) -> DL1_EN_R {
        DL1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable receiver could receive data when the transmitter is transmitting data in rs485 mode."]
    #[inline(always)]
    pub fn rs485tx_rx_en(&self) -> RS485TX_RX_EN_R {
        RS485TX_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1'h1: enable rs485 transmitter to send data when rs485 receiver line is busy."]
    #[inline(always)]
    pub fn rs485rxby_tx_en(&self) -> RS485RXBY_TX_EN_R {
        RS485RXBY_TX_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    pub fn rs485_rx_dly_num(&self) -> RS485_RX_DLY_NUM_R {
        RS485_RX_DLY_NUM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    pub fn rs485_tx_dly_num(&self) -> RS485_TX_DLY_NUM_R {
        RS485_TX_DLY_NUM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to choose the rs485 mode."]
    #[inline(always)]
    pub fn rs485_en(&mut self) -> RS485_EN_W {
        RS485_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl0_en(&mut self) -> DL0_EN_W {
        DL0_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    pub fn dl1_en(&mut self) -> DL1_EN_W {
        DL1_EN_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to enable receiver could receive data when the transmitter is transmitting data in rs485 mode."]
    #[inline(always)]
    pub fn rs485tx_rx_en(&mut self) -> RS485TX_RX_EN_W {
        RS485TX_RX_EN_W::new(self)
    }
    #[doc = "Bit 4 - 1'h1: enable rs485 transmitter to send data when rs485 receiver line is busy."]
    #[inline(always)]
    pub fn rs485rxby_tx_en(&mut self) -> RS485RXBY_TX_EN_W {
        RS485RXBY_TX_EN_W::new(self)
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    pub fn rs485_rx_dly_num(&mut self) -> RS485_RX_DLY_NUM_W {
        RS485_RX_DLY_NUM_W::new(self)
    }
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    pub fn rs485_tx_dly_num(&mut self) -> RS485_TX_DLY_NUM_W {
        RS485_TX_DLY_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RS485 mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485_conf](index.html) module"]
pub struct RS485_CONF_SPEC;
impl crate::RegisterSpec for RS485_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rs485_conf::R](R) reader structure"]
impl crate::Readable for RS485_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rs485_conf::W](W) writer structure"]
impl crate::Writable for RS485_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RS485_CONF to value 0"]
impl crate::Resettable for RS485_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
