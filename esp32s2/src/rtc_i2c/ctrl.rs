#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SDA_FORCE_OUT` reader - SDA output mode. 0: open drain. 1: push pull."]
pub type SDA_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SDA_FORCE_OUT` writer - SDA output mode. 0: open drain. 1: push pull."]
pub type SDA_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_FORCE_OUT` reader - SCL output mode. 0: open drain. 1: push pull."]
pub type SCL_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SCL_FORCE_OUT` writer - SCL output mode. 0: open drain. 1: push pull."]
pub type SCL_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS_MODE` reader - Set this bit to configure RTC I²C as a master."]
pub type MS_MODE_R = crate::BitReader;
#[doc = "Field `MS_MODE` writer - Set this bit to configure RTC I²C as a master."]
pub type MS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_START` reader - Set this bit to 1, RTC I2C starts sending data."]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `TRANS_START` writer - Set this bit to 1, RTC I2C starts sending data."]
pub type TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_LSB_FIRST` reader - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
pub type TX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `TX_LSB_FIRST` writer - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
pub type TX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_LSB_FIRST` reader - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
pub type RX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `RX_LSB_FIRST` writer - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
pub type RX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GATE_EN` reader - RTC I²C controller clock gate."]
pub type CLK_GATE_EN_R = crate::BitReader;
#[doc = "Field `CLK_GATE_EN` writer - RTC I²C controller clock gate."]
pub type CLK_GATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - RTC I²C software reset."]
pub type RESET_R = crate::BitReader;
#[doc = "Field `RESET` writer - RTC I²C software reset."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - rtc i2c reg clk gating"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - rtc i2c reg clk gating"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SDA output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to configure RTC I²C as a master."]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1, RTC I2C starts sending data."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC I²C controller clock gate."]
    #[inline(always)]
    pub fn clk_gate_en(&self) -> CLK_GATE_EN_R {
        CLK_GATE_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RTC I²C software reset."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - rtc i2c reg clk gating"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("sda_force_out", &self.sda_force_out())
            .field("scl_force_out", &self.scl_force_out())
            .field("ms_mode", &self.ms_mode())
            .field("trans_start", &self.trans_start())
            .field("tx_lsb_first", &self.tx_lsb_first())
            .field("rx_lsb_first", &self.rx_lsb_first())
            .field("clk_gate_en", &self.clk_gate_en())
            .field("reset", &self.reset())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SDA output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W<'_, CTRL_SPEC> {
        SDA_FORCE_OUT_W::new(self, 0)
    }
    #[doc = "Bit 1 - SCL output mode. 0: open drain. 1: push pull."]
    #[inline(always)]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W<'_, CTRL_SPEC> {
        SCL_FORCE_OUT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to configure RTC I²C as a master."]
    #[inline(always)]
    pub fn ms_mode(&mut self) -> MS_MODE_W<'_, CTRL_SPEC> {
        MS_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to 1, RTC I2C starts sending data."]
    #[inline(always)]
    pub fn trans_start(&mut self) -> TRANS_START_W<'_, CTRL_SPEC> {
        TRANS_START_W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is used to control the sending mode. 0: send data from the most significant bit. 1: send data from the least significant bit."]
    #[inline(always)]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W<'_, CTRL_SPEC> {
        TX_LSB_FIRST_W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is used to control the storage mode for received data. 0: receive data from the most significant bit. 1: receive data from the least significant bit."]
    #[inline(always)]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W<'_, CTRL_SPEC> {
        RX_LSB_FIRST_W::new(self, 5)
    }
    #[doc = "Bit 29 - RTC I²C controller clock gate."]
    #[inline(always)]
    pub fn clk_gate_en(&mut self) -> CLK_GATE_EN_W<'_, CTRL_SPEC> {
        CLK_GATE_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - RTC I²C software reset."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<'_, CTRL_SPEC> {
        RESET_W::new(self, 30)
    }
    #[doc = "Bit 31 - rtc i2c reg clk gating"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, CTRL_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "Transmission setting\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {}
