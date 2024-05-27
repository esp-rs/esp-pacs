#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SDA_FORCE_OUT` reader - 1=push pull,0=open drain"]
pub type SDA_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SDA_FORCE_OUT` writer - 1=push pull,0=open drain"]
pub type SDA_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_FORCE_OUT` reader - 1=push pull,0=open drain"]
pub type SCL_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SCL_FORCE_OUT` writer - 1=push pull,0=open drain"]
pub type SCL_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS_MODE` reader - 1=master,0=slave"]
pub type MS_MODE_R = crate::BitReader;
#[doc = "Field `MS_MODE` writer - 1=master,0=slave"]
pub type MS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_START` reader - force start"]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `TRANS_START` writer - force start"]
pub type TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_LSB_FIRST` reader - transit lsb first"]
pub type TX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `TX_LSB_FIRST` writer - transit lsb first"]
pub type TX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_LSB_FIRST` reader - receive lsb first"]
pub type RX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `RX_LSB_FIRST` writer - receive lsb first"]
pub type RX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_CTRL_CLK_GATE_EN` reader - configure i2c ctrl clk enable"]
pub type I2C_CTRL_CLK_GATE_EN_R = crate::BitReader;
#[doc = "Field `I2C_CTRL_CLK_GATE_EN` writer - configure i2c ctrl clk enable"]
pub type I2C_CTRL_CLK_GATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_RESET` reader - rtc i2c sw reset"]
pub type I2C_RESET_R = crate::BitReader;
#[doc = "Field `I2C_RESET` writer - rtc i2c sw reset"]
pub type I2C_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CCLK_EN` reader - rtc i2c reg clk gating"]
pub type I2CCLK_EN_R = crate::BitReader;
#[doc = "Field `I2CCLK_EN` writer - rtc i2c reg clk gating"]
pub type I2CCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1=push pull,0=open drain"]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1=push pull,0=open drain"]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1=master,0=slave"]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - force start"]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - transit lsb first"]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - receive lsb first"]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 29 - configure i2c ctrl clk enable"]
    #[inline(always)]
    pub fn i2c_ctrl_clk_gate_en(&self) -> I2C_CTRL_CLK_GATE_EN_R {
        I2C_CTRL_CLK_GATE_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - rtc i2c sw reset"]
    #[inline(always)]
    pub fn i2c_reset(&self) -> I2C_RESET_R {
        I2C_RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - rtc i2c reg clk gating"]
    #[inline(always)]
    pub fn i2cclk_en(&self) -> I2CCLK_EN_R {
        I2CCLK_EN_R::new(((self.bits >> 31) & 1) != 0)
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
            .field("i2c_ctrl_clk_gate_en", &self.i2c_ctrl_clk_gate_en())
            .field("i2c_reset", &self.i2c_reset())
            .field("i2cclk_en", &self.i2cclk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1=push pull,0=open drain"]
    #[inline(always)]
    #[must_use]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W<CTRL_SPEC> {
        SDA_FORCE_OUT_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1=push pull,0=open drain"]
    #[inline(always)]
    #[must_use]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W<CTRL_SPEC> {
        SCL_FORCE_OUT_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1=master,0=slave"]
    #[inline(always)]
    #[must_use]
    pub fn ms_mode(&mut self) -> MS_MODE_W<CTRL_SPEC> {
        MS_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - force start"]
    #[inline(always)]
    #[must_use]
    pub fn trans_start(&mut self) -> TRANS_START_W<CTRL_SPEC> {
        TRANS_START_W::new(self, 3)
    }
    #[doc = "Bit 4 - transit lsb first"]
    #[inline(always)]
    #[must_use]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W<CTRL_SPEC> {
        TX_LSB_FIRST_W::new(self, 4)
    }
    #[doc = "Bit 5 - receive lsb first"]
    #[inline(always)]
    #[must_use]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W<CTRL_SPEC> {
        RX_LSB_FIRST_W::new(self, 5)
    }
    #[doc = "Bit 29 - configure i2c ctrl clk enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ctrl_clk_gate_en(&mut self) -> I2C_CTRL_CLK_GATE_EN_W<CTRL_SPEC> {
        I2C_CTRL_CLK_GATE_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - rtc i2c sw reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_reset(&mut self) -> I2C_RESET_W<CTRL_SPEC> {
        I2C_RESET_W::new(self, 30)
    }
    #[doc = "Bit 31 - rtc i2c reg clk gating"]
    #[inline(always)]
    #[must_use]
    pub fn i2cclk_en(&mut self) -> I2CCLK_EN_W<CTRL_SPEC> {
        I2CCLK_EN_W::new(self, 31)
    }
}
#[doc = "configure i2c ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
