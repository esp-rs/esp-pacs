#[doc = "Register `I2C_CTR` reader"]
pub struct R(crate::R<I2C_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CTR` writer"]
pub struct W(crate::W<I2C_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CTR_SPEC>;
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
impl From<crate::W<I2C_CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SDA_FORCE_OUT` reader - 1: direct output, 0: open drain output."]
pub type I2C_SDA_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `I2C_SDA_FORCE_OUT` writer - 1: direct output, 0: open drain output."]
pub type I2C_SDA_FORCE_OUT_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
#[doc = "Field `I2C_SCL_FORCE_OUT` reader - 1: direct output, 0: open drain output."]
pub type I2C_SCL_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `I2C_SCL_FORCE_OUT` writer - 1: direct output, 0: open drain output."]
pub type I2C_SCL_FORCE_OUT_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
#[doc = "Field `I2C_SAMPLE_SCL_LEVEL` reader - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
pub type I2C_SAMPLE_SCL_LEVEL_R = crate::BitReader;
#[doc = "Field `I2C_SAMPLE_SCL_LEVEL` writer - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
pub type I2C_SAMPLE_SCL_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
#[doc = "Field `I2C_RX_FULL_ACK_LEVEL` reader - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
pub type I2C_RX_FULL_ACK_LEVEL_R = crate::BitReader;
#[doc = "Field `I2C_RX_FULL_ACK_LEVEL` writer - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
pub type I2C_RX_FULL_ACK_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
#[doc = "Field `I2C_TRANS_START` writer - Set this bit to start sending the data in txfifo."]
pub type I2C_TRANS_START_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
#[doc = "Field `I2C_TX_LSB_FIRST` reader - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
pub type I2C_TX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `I2C_TX_LSB_FIRST` writer - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
pub type I2C_TX_LSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
#[doc = "Field `I2C_RX_LSB_FIRST` reader - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit, 0: receive data from the most significant bit."]
pub type I2C_RX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `I2C_RX_LSB_FIRST` writer - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit, 0: receive data from the most significant bit."]
pub type I2C_RX_LSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
#[doc = "Field `I2C_CLK_EN` reader - Reserved"]
pub type I2C_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C_CLK_EN` writer - Reserved"]
pub type I2C_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
#[doc = "Field `I2C_ARBITRATION_EN` reader - This is the enable bit for arbitration_lost."]
pub type I2C_ARBITRATION_EN_R = crate::BitReader;
#[doc = "Field `I2C_ARBITRATION_EN` writer - This is the enable bit for arbitration_lost."]
pub type I2C_ARBITRATION_EN_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
#[doc = "Field `I2C_FSM_RST` writer - This register is used to reset the scl FMS."]
pub type I2C_FSM_RST_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
#[doc = "Field `I2C_CONF_UPGATE` writer - synchronization bit"]
pub type I2C_CONF_UPGATE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_CTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - 1: direct output, 0: open drain output."]
    #[inline(always)]
    pub fn i2c_sda_force_out(&self) -> I2C_SDA_FORCE_OUT_R {
        I2C_SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: direct output, 0: open drain output."]
    #[inline(always)]
    pub fn i2c_scl_force_out(&self) -> I2C_SCL_FORCE_OUT_R {
        I2C_SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
    #[inline(always)]
    pub fn i2c_sample_scl_level(&self) -> I2C_SAMPLE_SCL_LEVEL_R {
        I2C_SAMPLE_SCL_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
    #[inline(always)]
    pub fn i2c_rx_full_ack_level(&self) -> I2C_RX_FULL_ACK_LEVEL_R {
        I2C_RX_FULL_ACK_LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
    #[inline(always)]
    pub fn i2c_tx_lsb_first(&self) -> I2C_TX_LSB_FIRST_R {
        I2C_TX_LSB_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit, 0: receive data from the most significant bit."]
    #[inline(always)]
    pub fn i2c_rx_lsb_first(&self) -> I2C_RX_LSB_FIRST_R {
        I2C_RX_LSB_FIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the enable bit for arbitration_lost."]
    #[inline(always)]
    pub fn i2c_arbitration_en(&self) -> I2C_ARBITRATION_EN_R {
        I2C_ARBITRATION_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CTR")
            .field(
                "i2c_sda_force_out",
                &format_args!("{}", self.i2c_sda_force_out().bit()),
            )
            .field(
                "i2c_scl_force_out",
                &format_args!("{}", self.i2c_scl_force_out().bit()),
            )
            .field(
                "i2c_sample_scl_level",
                &format_args!("{}", self.i2c_sample_scl_level().bit()),
            )
            .field(
                "i2c_rx_full_ack_level",
                &format_args!("{}", self.i2c_rx_full_ack_level().bit()),
            )
            .field(
                "i2c_tx_lsb_first",
                &format_args!("{}", self.i2c_tx_lsb_first().bit()),
            )
            .field(
                "i2c_rx_lsb_first",
                &format_args!("{}", self.i2c_rx_lsb_first().bit()),
            )
            .field("i2c_clk_en", &format_args!("{}", self.i2c_clk_en().bit()))
            .field(
                "i2c_arbitration_en",
                &format_args!("{}", self.i2c_arbitration_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_CTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1: direct output, 0: open drain output."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sda_force_out(&mut self) -> I2C_SDA_FORCE_OUT_W<0> {
        I2C_SDA_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 1 - 1: direct output, 0: open drain output."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_force_out(&mut self) -> I2C_SCL_FORCE_OUT_W<1> {
        I2C_SCL_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 2 - This register is used to select the sample mode. 1: sample SDA data on the SCL low level. 0: sample SDA data on the SCL high level."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sample_scl_level(&mut self) -> I2C_SAMPLE_SCL_LEVEL_W<2> {
        I2C_SAMPLE_SCL_LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_rx_full_ack_level(&mut self) -> I2C_RX_FULL_ACK_LEVEL_W<3> {
        I2C_RX_FULL_ACK_LEVEL_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to start sending the data in txfifo."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_trans_start(&mut self) -> I2C_TRANS_START_W<5> {
        I2C_TRANS_START_W::new(self)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_tx_lsb_first(&mut self) -> I2C_TX_LSB_FIRST_W<6> {
        I2C_TX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received data. 1: receive data from the least significant bit, 0: receive data from the most significant bit."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_rx_lsb_first(&mut self) -> I2C_RX_LSB_FIRST_W<7> {
        I2C_RX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W<8> {
        I2C_CLK_EN_W::new(self)
    }
    #[doc = "Bit 9 - This is the enable bit for arbitration_lost."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_arbitration_en(&mut self) -> I2C_ARBITRATION_EN_W<9> {
        I2C_ARBITRATION_EN_W::new(self)
    }
    #[doc = "Bit 10 - This register is used to reset the scl FMS."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_fsm_rst(&mut self) -> I2C_FSM_RST_W<10> {
        I2C_FSM_RST_W::new(self)
    }
    #[doc = "Bit 11 - synchronization bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_conf_upgate(&mut self) -> I2C_CONF_UPGATE_W<11> {
        I2C_CONF_UPGATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmission setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctr](index.html) module"]
pub struct I2C_CTR_SPEC;
impl crate::RegisterSpec for I2C_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_ctr::R](R) reader structure"]
impl crate::Readable for I2C_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ctr::W](W) writer structure"]
impl crate::Writable for I2C_CTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_CTR to value 0x0208"]
impl crate::Resettable for I2C_CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0208;
}
