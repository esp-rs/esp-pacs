#[doc = "Register `CTR` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CTR_SPEC>;
#[doc = "Field `SDA_FORCE_OUT` reader - Configures the SDA output mode 1: Direct output, 0: Open drain output."]
pub type SDA_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SDA_FORCE_OUT` writer - Configures the SDA output mode 1: Direct output, 0: Open drain output."]
pub type SDA_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_FORCE_OUT` reader - Configures the SCL output mode 1: Direct output, 0: Open drain output."]
pub type SCL_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SCL_FORCE_OUT` writer - Configures the SCL output mode 1: Direct output, 0: Open drain output."]
pub type SCL_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_SCL_LEVEL` reader - Configures the sample mode for SDA. 1: Sample SDA data on the SCL low level. 0: Sample SDA data on the SCL high level."]
pub type SAMPLE_SCL_LEVEL_R = crate::BitReader;
#[doc = "Field `SAMPLE_SCL_LEVEL` writer - Configures the sample mode for SDA. 1: Sample SDA data on the SCL low level. 0: Sample SDA data on the SCL high level."]
pub type SAMPLE_SCL_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FULL_ACK_LEVEL` reader - Configures the ACK value that needs to be sent by master when the rx_fifo_cnt has reached the threshold."]
pub type RX_FULL_ACK_LEVEL_R = crate::BitReader;
#[doc = "Field `RX_FULL_ACK_LEVEL` writer - Configures the ACK value that needs to be sent by master when the rx_fifo_cnt has reached the threshold."]
pub type RX_FULL_ACK_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS_MODE` reader - Configures the module as an I2C Master or Slave. 0: Slave 1: Master"]
pub type MS_MODE_R = crate::BitReader;
#[doc = "Field `MS_MODE` writer - Configures the module as an I2C Master or Slave. 0: Slave 1: Master"]
pub type MS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_START` writer - Configures to start sending the data in txfifo for slave. 0: No effect 1: Start"]
pub type TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_LSB_FIRST` reader - Configures to control the sending order for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
pub type TX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `TX_LSB_FIRST` writer - Configures to control the sending order for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
pub type TX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_LSB_FIRST` reader - Configures to control the storage order for received data. 1: receive data from the least significant bit 0: receive data from the most significant bit."]
pub type RX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `RX_LSB_FIRST` writer - Configures to control the storage order for received data. 1: receive data from the least significant bit 0: receive data from the most significant bit."]
pub type RX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Configures whether to gate clock signal for registers. 0: Force clock on for registers 1: Support clock only when registers are read or written to by software."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures whether to gate clock signal for registers. 0: Force clock on for registers 1: Support clock only when registers are read or written to by software."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_EN` reader - Configures to enable I2C bus arbitration detection. 0: No effect 1: Enable"]
pub type ARBITRATION_EN_R = crate::BitReader;
#[doc = "Field `ARBITRATION_EN` writer - Configures to enable I2C bus arbitration detection. 0: No effect 1: Enable"]
pub type ARBITRATION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSM_RST` writer - Configures to reset the SCL_FSM. 0: No effect 1: Reset"]
pub type FSM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONF_UPGATE` writer - Configures this bit for synchronization 0: No effect 1: Synchronize"]
pub type CONF_UPGATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_TX_AUTO_START_EN` reader - Configures to enable slave to send data automatically 0: Disable 1: Enable"]
pub type SLV_TX_AUTO_START_EN_R = crate::BitReader;
#[doc = "Field `SLV_TX_AUTO_START_EN` writer - Configures to enable slave to send data automatically 0: Disable 1: Enable"]
pub type SLV_TX_AUTO_START_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_10BIT_RW_CHECK_EN` reader - Configures to check if the r/w bit of 10bit addressing consists with I2C protocol. 0: Not check 1: Check"]
pub type ADDR_10BIT_RW_CHECK_EN_R = crate::BitReader;
#[doc = "Field `ADDR_10BIT_RW_CHECK_EN` writer - Configures to check if the r/w bit of 10bit addressing consists with I2C protocol. 0: Not check 1: Check"]
pub type ADDR_10BIT_RW_CHECK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_BROADCASTING_EN` reader - Configures to support the 7bit general call function. 0: Not support 1: Support"]
pub type ADDR_BROADCASTING_EN_R = crate::BitReader;
#[doc = "Field `ADDR_BROADCASTING_EN` writer - Configures to support the 7bit general call function. 0: Not support 1: Support"]
pub type ADDR_BROADCASTING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the SDA output mode 1: Direct output, 0: Open drain output."]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures the SCL output mode 1: Direct output, 0: Open drain output."]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures the sample mode for SDA. 1: Sample SDA data on the SCL low level. 0: Sample SDA data on the SCL high level."]
    #[inline(always)]
    pub fn sample_scl_level(&self) -> SAMPLE_SCL_LEVEL_R {
        SAMPLE_SCL_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the ACK value that needs to be sent by master when the rx_fifo_cnt has reached the threshold."]
    #[inline(always)]
    pub fn rx_full_ack_level(&self) -> RX_FULL_ACK_LEVEL_R {
        RX_FULL_ACK_LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures the module as an I2C Master or Slave. 0: Slave 1: Master"]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures to control the sending order for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures to control the storage order for received data. 1: receive data from the least significant bit 0: receive data from the most significant bit."]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether to gate clock signal for registers. 0: Force clock on for registers 1: Support clock only when registers are read or written to by software."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures to enable I2C bus arbitration detection. 0: No effect 1: Enable"]
    #[inline(always)]
    pub fn arbitration_en(&self) -> ARBITRATION_EN_R {
        ARBITRATION_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures to enable slave to send data automatically 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn slv_tx_auto_start_en(&self) -> SLV_TX_AUTO_START_EN_R {
        SLV_TX_AUTO_START_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures to check if the r/w bit of 10bit addressing consists with I2C protocol. 0: Not check 1: Check"]
    #[inline(always)]
    pub fn addr_10bit_rw_check_en(&self) -> ADDR_10BIT_RW_CHECK_EN_R {
        ADDR_10BIT_RW_CHECK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures to support the 7bit general call function. 0: Not support 1: Support"]
    #[inline(always)]
    pub fn addr_broadcasting_en(&self) -> ADDR_BROADCASTING_EN_R {
        ADDR_BROADCASTING_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTR")
            .field("sda_force_out", &self.sda_force_out().bit())
            .field("scl_force_out", &self.scl_force_out().bit())
            .field("sample_scl_level", &self.sample_scl_level().bit())
            .field("rx_full_ack_level", &self.rx_full_ack_level().bit())
            .field("ms_mode", &self.ms_mode().bit())
            .field("tx_lsb_first", &self.tx_lsb_first().bit())
            .field("rx_lsb_first", &self.rx_lsb_first().bit())
            .field("clk_en", &self.clk_en().bit())
            .field("arbitration_en", &self.arbitration_en().bit())
            .field("slv_tx_auto_start_en", &self.slv_tx_auto_start_en().bit())
            .field(
                "addr_10bit_rw_check_en",
                &self.addr_10bit_rw_check_en().bit(),
            )
            .field("addr_broadcasting_en", &self.addr_broadcasting_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the SDA output mode 1: Direct output, 0: Open drain output."]
    #[inline(always)]
    #[must_use]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W<CTR_SPEC> {
        SDA_FORCE_OUT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures the SCL output mode 1: Direct output, 0: Open drain output."]
    #[inline(always)]
    #[must_use]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W<CTR_SPEC> {
        SCL_FORCE_OUT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures the sample mode for SDA. 1: Sample SDA data on the SCL low level. 0: Sample SDA data on the SCL high level."]
    #[inline(always)]
    #[must_use]
    pub fn sample_scl_level(&mut self) -> SAMPLE_SCL_LEVEL_W<CTR_SPEC> {
        SAMPLE_SCL_LEVEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the ACK value that needs to be sent by master when the rx_fifo_cnt has reached the threshold."]
    #[inline(always)]
    #[must_use]
    pub fn rx_full_ack_level(&mut self) -> RX_FULL_ACK_LEVEL_W<CTR_SPEC> {
        RX_FULL_ACK_LEVEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures the module as an I2C Master or Slave. 0: Slave 1: Master"]
    #[inline(always)]
    #[must_use]
    pub fn ms_mode(&mut self) -> MS_MODE_W<CTR_SPEC> {
        MS_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures to start sending the data in txfifo for slave. 0: No effect 1: Start"]
    #[inline(always)]
    #[must_use]
    pub fn trans_start(&mut self) -> TRANS_START_W<CTR_SPEC> {
        TRANS_START_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures to control the sending order for data needing to be sent. 1: send data from the least significant bit, 0: send data from the most significant bit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W<CTR_SPEC> {
        TX_LSB_FIRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures to control the storage order for received data. 1: receive data from the least significant bit 0: receive data from the most significant bit."]
    #[inline(always)]
    #[must_use]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W<CTR_SPEC> {
        RX_LSB_FIRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether to gate clock signal for registers. 0: Force clock on for registers 1: Support clock only when registers are read or written to by software."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CTR_SPEC> {
        CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures to enable I2C bus arbitration detection. 0: No effect 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_en(&mut self) -> ARBITRATION_EN_W<CTR_SPEC> {
        ARBITRATION_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures to reset the SCL_FSM. 0: No effect 1: Reset"]
    #[inline(always)]
    #[must_use]
    pub fn fsm_rst(&mut self) -> FSM_RST_W<CTR_SPEC> {
        FSM_RST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures this bit for synchronization 0: No effect 1: Synchronize"]
    #[inline(always)]
    #[must_use]
    pub fn conf_upgate(&mut self) -> CONF_UPGATE_W<CTR_SPEC> {
        CONF_UPGATE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures to enable slave to send data automatically 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn slv_tx_auto_start_en(&mut self) -> SLV_TX_AUTO_START_EN_W<CTR_SPEC> {
        SLV_TX_AUTO_START_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures to check if the r/w bit of 10bit addressing consists with I2C protocol. 0: Not check 1: Check"]
    #[inline(always)]
    #[must_use]
    pub fn addr_10bit_rw_check_en(&mut self) -> ADDR_10BIT_RW_CHECK_EN_W<CTR_SPEC> {
        ADDR_10BIT_RW_CHECK_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures to support the 7bit general call function. 0: Not support 1: Support"]
    #[inline(always)]
    #[must_use]
    pub fn addr_broadcasting_en(&mut self) -> ADDR_BROADCASTING_EN_W<CTR_SPEC> {
        ADDR_BROADCASTING_EN_W::new(self, 14)
    }
}
#[doc = "Transmission setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctr::R`](R) reader structure"]
impl crate::Readable for CTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctr::W`](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTR to value 0x0208"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: u32 = 0x0208;
}
