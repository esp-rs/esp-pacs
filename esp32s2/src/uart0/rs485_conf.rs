///Register `RS485_CONF` reader
pub type R = crate::R<RS485_CONF_SPEC>;
///Register `RS485_CONF` writer
pub type W = crate::W<RS485_CONF_SPEC>;
///Field `RS485_EN` reader - Set this bit to choose RS485 mode.
pub type RS485_EN_R = crate::BitReader;
///Field `RS485_EN` writer - Set this bit to choose RS485 mode.
pub type RS485_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DL0_EN` reader - Set this bit to delay the stop bit by 1 bit.
pub type DL0_EN_R = crate::BitReader;
///Field `DL0_EN` writer - Set this bit to delay the stop bit by 1 bit.
pub type DL0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DL1_EN` reader - Set this bit to delay the stop bit by 1 bit.
pub type DL1_EN_R = crate::BitReader;
///Field `DL1_EN` writer - Set this bit to delay the stop bit by 1 bit.
pub type DL1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RS485TX_RX_EN` reader - Set this bit to enable the receiver could receive data when the transmitter is transmitting data in RS485 mode.
pub type RS485TX_RX_EN_R = crate::BitReader;
///Field `RS485TX_RX_EN` writer - Set this bit to enable the receiver could receive data when the transmitter is transmitting data in RS485 mode.
pub type RS485TX_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RS485RXBY_TX_EN` reader - 1: enable RS485 transmitter to send data when RS485 receiver line is busy. 0: RS485 transmitter should not send data when its receiver is busy.
pub type RS485RXBY_TX_EN_R = crate::BitReader;
///Field `RS485RXBY_TX_EN` writer - 1: enable RS485 transmitter to send data when RS485 receiver line is busy. 0: RS485 transmitter should not send data when its receiver is busy.
pub type RS485RXBY_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RS485_RX_DLY_NUM` reader - This register is used to delay the receiver's internal data signal.
pub type RS485_RX_DLY_NUM_R = crate::BitReader;
///Field `RS485_RX_DLY_NUM` writer - This register is used to delay the receiver's internal data signal.
pub type RS485_RX_DLY_NUM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RS485_TX_DLY_NUM` reader - This register is used to delay the transmitter's internal data signal.
pub type RS485_TX_DLY_NUM_R = crate::FieldReader;
///Field `RS485_TX_DLY_NUM` writer - This register is used to delay the transmitter's internal data signal.
pub type RS485_TX_DLY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Set this bit to choose RS485 mode.
    #[inline(always)]
    pub fn rs485_en(&self) -> RS485_EN_R {
        RS485_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    pub fn dl0_en(&self) -> DL0_EN_R {
        DL0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    pub fn dl1_en(&self) -> DL1_EN_R {
        DL1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set this bit to enable the receiver could receive data when the transmitter is transmitting data in RS485 mode.
    #[inline(always)]
    pub fn rs485tx_rx_en(&self) -> RS485TX_RX_EN_R {
        RS485TX_RX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 1: enable RS485 transmitter to send data when RS485 receiver line is busy. 0: RS485 transmitter should not send data when its receiver is busy.
    #[inline(always)]
    pub fn rs485rxby_tx_en(&self) -> RS485RXBY_TX_EN_R {
        RS485RXBY_TX_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - This register is used to delay the receiver's internal data signal.
    #[inline(always)]
    pub fn rs485_rx_dly_num(&self) -> RS485_RX_DLY_NUM_R {
        RS485_RX_DLY_NUM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:9 - This register is used to delay the transmitter's internal data signal.
    #[inline(always)]
    pub fn rs485_tx_dly_num(&self) -> RS485_TX_DLY_NUM_R {
        RS485_TX_DLY_NUM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RS485_CONF")
            .field("rs485_en", &self.rs485_en())
            .field("dl0_en", &self.dl0_en())
            .field("dl1_en", &self.dl1_en())
            .field("rs485tx_rx_en", &self.rs485tx_rx_en())
            .field("rs485rxby_tx_en", &self.rs485rxby_tx_en())
            .field("rs485_rx_dly_num", &self.rs485_rx_dly_num())
            .field("rs485_tx_dly_num", &self.rs485_tx_dly_num())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to choose RS485 mode.
    #[inline(always)]
    #[must_use]
    pub fn rs485_en(&mut self) -> RS485_EN_W<RS485_CONF_SPEC> {
        RS485_EN_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    #[must_use]
    pub fn dl0_en(&mut self) -> DL0_EN_W<RS485_CONF_SPEC> {
        DL0_EN_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to delay the stop bit by 1 bit.
    #[inline(always)]
    #[must_use]
    pub fn dl1_en(&mut self) -> DL1_EN_W<RS485_CONF_SPEC> {
        DL1_EN_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to enable the receiver could receive data when the transmitter is transmitting data in RS485 mode.
    #[inline(always)]
    #[must_use]
    pub fn rs485tx_rx_en(&mut self) -> RS485TX_RX_EN_W<RS485_CONF_SPEC> {
        RS485TX_RX_EN_W::new(self, 3)
    }
    ///Bit 4 - 1: enable RS485 transmitter to send data when RS485 receiver line is busy. 0: RS485 transmitter should not send data when its receiver is busy.
    #[inline(always)]
    #[must_use]
    pub fn rs485rxby_tx_en(&mut self) -> RS485RXBY_TX_EN_W<RS485_CONF_SPEC> {
        RS485RXBY_TX_EN_W::new(self, 4)
    }
    ///Bit 5 - This register is used to delay the receiver's internal data signal.
    #[inline(always)]
    #[must_use]
    pub fn rs485_rx_dly_num(&mut self) -> RS485_RX_DLY_NUM_W<RS485_CONF_SPEC> {
        RS485_RX_DLY_NUM_W::new(self, 5)
    }
    ///Bits 6:9 - This register is used to delay the transmitter's internal data signal.
    #[inline(always)]
    #[must_use]
    pub fn rs485_tx_dly_num(&mut self) -> RS485_TX_DLY_NUM_W<RS485_CONF_SPEC> {
        RS485_TX_DLY_NUM_W::new(self, 6)
    }
}
/**RS485 mode configuration

You can [`read`](crate::generic::Reg::read) this register and get [`rs485_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RS485_CONF_SPEC;
impl crate::RegisterSpec for RS485_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rs485_conf::R`](R) reader structure
impl crate::Readable for RS485_CONF_SPEC {}
///`write(|w| ..)` method takes [`rs485_conf::W`](W) writer structure
impl crate::Writable for RS485_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RS485_CONF to value 0
impl crate::Resettable for RS485_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
