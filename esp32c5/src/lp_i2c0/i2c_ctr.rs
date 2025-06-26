#[doc = "Register `I2C_CTR` reader"]
pub type R = crate::R<I2C_CTR_SPEC>;
#[doc = "Register `I2C_CTR` writer"]
pub type W = crate::W<I2C_CTR_SPEC>;
#[doc = "Field `I2C_SDA_FORCE_OUT` reader - 1: direct output, 0: open drain output."]
pub type I2C_SDA_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `I2C_SDA_FORCE_OUT` writer - 1: direct output, 0: open drain output."]
pub type I2C_SDA_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_SCL_FORCE_OUT` reader - 1: direct output, 0: open drain output."]
pub type I2C_SCL_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `I2C_SCL_FORCE_OUT` writer - 1: direct output, 0: open drain output."]
pub type I2C_SCL_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_SAMPLE_SCL_LEVEL` reader - This register is used to select the sample mode.1: sample SDA data on the SCL low level.0: sample SDA data on the SCL high level."]
pub type I2C_SAMPLE_SCL_LEVEL_R = crate::BitReader;
#[doc = "Field `I2C_SAMPLE_SCL_LEVEL` writer - This register is used to select the sample mode.1: sample SDA data on the SCL low level.0: sample SDA data on the SCL high level."]
pub type I2C_SAMPLE_SCL_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_RX_FULL_ACK_LEVEL` reader - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
pub type I2C_RX_FULL_ACK_LEVEL_R = crate::BitReader;
#[doc = "Field `I2C_RX_FULL_ACK_LEVEL` writer - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
pub type I2C_RX_FULL_ACK_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TRANS_START` writer - Set this bit to start sending the data in txfifo."]
pub type I2C_TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TX_LSB_FIRST` reader - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit,0: send data from the most significant bit."]
pub type I2C_TX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `I2C_TX_LSB_FIRST` writer - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit,0: send data from the most significant bit."]
pub type I2C_TX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_RX_LSB_FIRST` reader - This bit is used to control the storage mode for received data.1: receive data from the least significant bit,0: receive data from the most significant bit."]
pub type I2C_RX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `I2C_RX_LSB_FIRST` writer - This bit is used to control the storage mode for received data.1: receive data from the least significant bit,0: receive data from the most significant bit."]
pub type I2C_RX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_CLK_EN` reader - Reserved"]
pub type I2C_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C_CLK_EN` writer - Reserved"]
pub type I2C_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ARBITRATION_EN` reader - This is the enable bit for arbitration_lost."]
pub type I2C_ARBITRATION_EN_R = crate::BitReader;
#[doc = "Field `I2C_ARBITRATION_EN` writer - This is the enable bit for arbitration_lost."]
pub type I2C_ARBITRATION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_FSM_RST` writer - This register is used to reset the scl FMS."]
pub type I2C_FSM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_CONF_UPGATE` writer - synchronization bit"]
pub type I2C_CONF_UPGATE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - This register is used to select the sample mode.1: sample SDA data on the SCL low level.0: sample SDA data on the SCL high level."]
    #[inline(always)]
    pub fn i2c_sample_scl_level(&self) -> I2C_SAMPLE_SCL_LEVEL_R {
        I2C_SAMPLE_SCL_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
    #[inline(always)]
    pub fn i2c_rx_full_ack_level(&self) -> I2C_RX_FULL_ACK_LEVEL_R {
        I2C_RX_FULL_ACK_LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit,0: send data from the most significant bit."]
    #[inline(always)]
    pub fn i2c_tx_lsb_first(&self) -> I2C_TX_LSB_FIRST_R {
        I2C_TX_LSB_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received data.1: receive data from the least significant bit,0: receive data from the most significant bit."]
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
            .field("i2c_sda_force_out", &self.i2c_sda_force_out())
            .field("i2c_scl_force_out", &self.i2c_scl_force_out())
            .field("i2c_sample_scl_level", &self.i2c_sample_scl_level())
            .field("i2c_rx_full_ack_level", &self.i2c_rx_full_ack_level())
            .field("i2c_tx_lsb_first", &self.i2c_tx_lsb_first())
            .field("i2c_rx_lsb_first", &self.i2c_rx_lsb_first())
            .field("i2c_clk_en", &self.i2c_clk_en())
            .field("i2c_arbitration_en", &self.i2c_arbitration_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: direct output, 0: open drain output."]
    #[inline(always)]
    pub fn i2c_sda_force_out(&mut self) -> I2C_SDA_FORCE_OUT_W<I2C_CTR_SPEC> {
        I2C_SDA_FORCE_OUT_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: direct output, 0: open drain output."]
    #[inline(always)]
    pub fn i2c_scl_force_out(&mut self) -> I2C_SCL_FORCE_OUT_W<I2C_CTR_SPEC> {
        I2C_SCL_FORCE_OUT_W::new(self, 1)
    }
    #[doc = "Bit 2 - This register is used to select the sample mode.1: sample SDA data on the SCL low level.0: sample SDA data on the SCL high level."]
    #[inline(always)]
    pub fn i2c_sample_scl_level(&mut self) -> I2C_SAMPLE_SCL_LEVEL_W<I2C_CTR_SPEC> {
        I2C_SAMPLE_SCL_LEVEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - This register is used to configure the ACK value that need to sent by master when the rx_fifo_cnt has reached the threshold."]
    #[inline(always)]
    pub fn i2c_rx_full_ack_level(&mut self) -> I2C_RX_FULL_ACK_LEVEL_W<I2C_CTR_SPEC> {
        I2C_RX_FULL_ACK_LEVEL_W::new(self, 3)
    }
    #[doc = "Bit 5 - Set this bit to start sending the data in txfifo."]
    #[inline(always)]
    pub fn i2c_trans_start(&mut self) -> I2C_TRANS_START_W<I2C_CTR_SPEC> {
        I2C_TRANS_START_W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data needing to be sent. 1: send data from the least significant bit,0: send data from the most significant bit."]
    #[inline(always)]
    pub fn i2c_tx_lsb_first(&mut self) -> I2C_TX_LSB_FIRST_W<I2C_CTR_SPEC> {
        I2C_TX_LSB_FIRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received data.1: receive data from the least significant bit,0: receive data from the most significant bit."]
    #[inline(always)]
    pub fn i2c_rx_lsb_first(&mut self) -> I2C_RX_LSB_FIRST_W<I2C_CTR_SPEC> {
        I2C_RX_LSB_FIRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W<I2C_CTR_SPEC> {
        I2C_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - This is the enable bit for arbitration_lost."]
    #[inline(always)]
    pub fn i2c_arbitration_en(&mut self) -> I2C_ARBITRATION_EN_W<I2C_CTR_SPEC> {
        I2C_ARBITRATION_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - This register is used to reset the scl FMS."]
    #[inline(always)]
    pub fn i2c_fsm_rst(&mut self) -> I2C_FSM_RST_W<I2C_CTR_SPEC> {
        I2C_FSM_RST_W::new(self, 10)
    }
    #[doc = "Bit 11 - synchronization bit"]
    #[inline(always)]
    pub fn i2c_conf_upgate(&mut self) -> I2C_CONF_UPGATE_W<I2C_CTR_SPEC> {
        I2C_CONF_UPGATE_W::new(self, 11)
    }
}
#[doc = "Transmission setting\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_ctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_ctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_CTR_SPEC;
impl crate::RegisterSpec for I2C_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_ctr::R`](R) reader structure"]
impl crate::Readable for I2C_CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_ctr::W`](W) writer structure"]
impl crate::Writable for I2C_CTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_CTR to value 0x0208"]
impl crate::Resettable for I2C_CTR_SPEC {
    const RESET_VALUE: u32 = 0x0208;
}
