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
#[doc = "Field `RS485_EN` reader - Set this bit to choose RS485 mode."]
pub type RS485_EN_R = crate::BitReader;
#[doc = "Field `RS485_EN` writer - Set this bit to choose RS485 mode."]
pub type RS485_EN_W<'a, const O: u8> = crate::BitWriter<'a, RS485_CONF_SPEC, O>;
#[doc = "Field `DL0_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type DL0_EN_R = crate::BitReader;
#[doc = "Field `DL0_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type DL0_EN_W<'a, const O: u8> = crate::BitWriter<'a, RS485_CONF_SPEC, O>;
#[doc = "Field `DL1_EN` reader - Set this bit to delay the stop bit by 1 bit."]
pub type DL1_EN_R = crate::BitReader;
#[doc = "Field `DL1_EN` writer - Set this bit to delay the stop bit by 1 bit."]
pub type DL1_EN_W<'a, const O: u8> = crate::BitWriter<'a, RS485_CONF_SPEC, O>;
#[doc = "Field `RS485TX_RX_EN` reader - Set this bit to enable the receiver could receive data when the transmitter is transmitting data in RS485 mode."]
pub type RS485TX_RX_EN_R = crate::BitReader;
#[doc = "Field `RS485TX_RX_EN` writer - Set this bit to enable the receiver could receive data when the transmitter is transmitting data in RS485 mode."]
pub type RS485TX_RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, RS485_CONF_SPEC, O>;
#[doc = "Field `RS485RXBY_TX_EN` reader - 1: enable RS485 transmitter to send data when RS485 receiver line is busy. 0: RS485 transmitter should not send data when its receiver is busy."]
pub type RS485RXBY_TX_EN_R = crate::BitReader;
#[doc = "Field `RS485RXBY_TX_EN` writer - 1: enable RS485 transmitter to send data when RS485 receiver line is busy. 0: RS485 transmitter should not send data when its receiver is busy."]
pub type RS485RXBY_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, RS485_CONF_SPEC, O>;
#[doc = "Field `RS485_RX_DLY_NUM` reader - This register is used to delay the receiver's internal data signal."]
pub type RS485_RX_DLY_NUM_R = crate::BitReader;
#[doc = "Field `RS485_RX_DLY_NUM` writer - This register is used to delay the receiver's internal data signal."]
pub type RS485_RX_DLY_NUM_W<'a, const O: u8> = crate::BitWriter<'a, RS485_CONF_SPEC, O>;
#[doc = "Field `RS485_TX_DLY_NUM` reader - This register is used to delay the transmitter's internal data signal."]
pub type RS485_TX_DLY_NUM_R = crate::FieldReader;
#[doc = "Field `RS485_TX_DLY_NUM` writer - This register is used to delay the transmitter's internal data signal."]
pub type RS485_TX_DLY_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, RS485_CONF_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to choose RS485 mode."]
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
    #[doc = "Bit 3 - Set this bit to enable the receiver could receive data when the transmitter is transmitting data in RS485 mode."]
    #[inline(always)]
    pub fn rs485tx_rx_en(&self) -> RS485TX_RX_EN_R {
        RS485TX_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: enable RS485 transmitter to send data when RS485 receiver line is busy. 0: RS485 transmitter should not send data when its receiver is busy."]
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RS485_CONF")
            .field("rs485_en", &format_args!("{}", self.rs485_en().bit()))
            .field("dl0_en", &format_args!("{}", self.dl0_en().bit()))
            .field("dl1_en", &format_args!("{}", self.dl1_en().bit()))
            .field(
                "rs485tx_rx_en",
                &format_args!("{}", self.rs485tx_rx_en().bit()),
            )
            .field(
                "rs485rxby_tx_en",
                &format_args!("{}", self.rs485rxby_tx_en().bit()),
            )
            .field(
                "rs485_rx_dly_num",
                &format_args!("{}", self.rs485_rx_dly_num().bit()),
            )
            .field(
                "rs485_tx_dly_num",
                &format_args!("{}", self.rs485_tx_dly_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RS485_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to choose RS485 mode."]
    #[inline(always)]
    #[must_use]
    pub fn rs485_en(&mut self) -> RS485_EN_W<0> {
        RS485_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    #[must_use]
    pub fn dl0_en(&mut self) -> DL0_EN_W<1> {
        DL0_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to delay the stop bit by 1 bit."]
    #[inline(always)]
    #[must_use]
    pub fn dl1_en(&mut self) -> DL1_EN_W<2> {
        DL1_EN_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to enable the receiver could receive data when the transmitter is transmitting data in RS485 mode."]
    #[inline(always)]
    #[must_use]
    pub fn rs485tx_rx_en(&mut self) -> RS485TX_RX_EN_W<3> {
        RS485TX_RX_EN_W::new(self)
    }
    #[doc = "Bit 4 - 1: enable RS485 transmitter to send data when RS485 receiver line is busy. 0: RS485 transmitter should not send data when its receiver is busy."]
    #[inline(always)]
    #[must_use]
    pub fn rs485rxby_tx_en(&mut self) -> RS485RXBY_TX_EN_W<4> {
        RS485RXBY_TX_EN_W::new(self)
    }
    #[doc = "Bit 5 - This register is used to delay the receiver's internal data signal."]
    #[inline(always)]
    #[must_use]
    pub fn rs485_rx_dly_num(&mut self) -> RS485_RX_DLY_NUM_W<5> {
        RS485_RX_DLY_NUM_W::new(self)
    }
    #[doc = "Bits 6:9 - This register is used to delay the transmitter's internal data signal."]
    #[inline(always)]
    #[must_use]
    pub fn rs485_tx_dly_num(&mut self) -> RS485_TX_DLY_NUM_W<6> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RS485_CONF to value 0"]
impl crate::Resettable for RS485_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
