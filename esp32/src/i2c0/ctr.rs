#[doc = "Register `CTR` reader"]
pub type R = crate::R<CTR_SPEC>;
#[doc = "Register `CTR` writer"]
pub type W = crate::W<CTR_SPEC>;
#[doc = "Field `SDA_FORCE_OUT` reader - 1: normally ouput sda data 0: exchange the function of sda_o and sda_oe (sda_o is the original internal output sda signal sda_oe is the enable bit for the internal output sda signal)"]
pub type SDA_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SDA_FORCE_OUT` writer - 1: normally ouput sda data 0: exchange the function of sda_o and sda_oe (sda_o is the original internal output sda signal sda_oe is the enable bit for the internal output sda signal)"]
pub type SDA_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_FORCE_OUT` reader - 1: normally ouput scl clock 0: exchange the function of scl_o and scl_oe (scl_o is the original internal output scl signal scl_oe is the enable bit for the internal output scl signal)"]
pub type SCL_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SCL_FORCE_OUT` writer - 1: normally ouput scl clock 0: exchange the function of scl_o and scl_oe (scl_o is the original internal output scl signal scl_oe is the enable bit for the internal output scl signal)"]
pub type SCL_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_SCL_LEVEL` reader - Set this bit to sample data in SCL low level. clear this bit to sample data in SCL high level."]
pub type SAMPLE_SCL_LEVEL_R = crate::BitReader;
#[doc = "Field `SAMPLE_SCL_LEVEL` writer - Set this bit to sample data in SCL low level. clear this bit to sample data in SCL high level."]
pub type SAMPLE_SCL_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS_MODE` reader - Set this bit to configure the module as i2c master clear this bit to configure the module as i2c slave."]
pub type MS_MODE_R = crate::BitReader;
#[doc = "Field `MS_MODE` writer - Set this bit to configure the module as i2c master clear this bit to configure the module as i2c slave."]
pub type MS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_START` reader - Set this bit to start sending data in txfifo."]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `TRANS_START` writer - Set this bit to start sending data in txfifo."]
pub type TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_LSB_FIRST` reader - This bit is used to control the sending mode for data need to be send. 1: receive data from most significant bit 0: receive data from least significant bit"]
pub type TX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `TX_LSB_FIRST` writer - This bit is used to control the sending mode for data need to be send. 1: receive data from most significant bit 0: receive data from least significant bit"]
pub type TX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_LSB_FIRST` reader - This bit is used to control the storage mode for received datas. 1: receive data from most significant bit 0: receive data from least significant bit"]
pub type RX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `RX_LSB_FIRST` writer - This bit is used to control the storage mode for received datas. 1: receive data from most significant bit 0: receive data from least significant bit"]
pub type RX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - This is the clock gating control bit for reading or writing registers."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - This is the clock gating control bit for reading or writing registers."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: normally ouput sda data 0: exchange the function of sda_o and sda_oe (sda_o is the original internal output sda signal sda_oe is the enable bit for the internal output sda signal)"]
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: normally ouput scl clock 0: exchange the function of scl_o and scl_oe (scl_o is the original internal output scl signal scl_oe is the enable bit for the internal output scl signal)"]
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to sample data in SCL low level. clear this bit to sample data in SCL high level."]
    #[inline(always)]
    pub fn sample_scl_level(&self) -> SAMPLE_SCL_LEVEL_R {
        SAMPLE_SCL_LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to configure the module as i2c master clear this bit to configure the module as i2c slave."]
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to start sending data in txfifo."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data need to be send. 1: receive data from most significant bit 0: receive data from least significant bit"]
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received datas. 1: receive data from most significant bit 0: receive data from least significant bit"]
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the clock gating control bit for reading or writing registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTR")
            .field("sda_force_out", &self.sda_force_out())
            .field("scl_force_out", &self.scl_force_out())
            .field("sample_scl_level", &self.sample_scl_level())
            .field("ms_mode", &self.ms_mode())
            .field("trans_start", &self.trans_start())
            .field("tx_lsb_first", &self.tx_lsb_first())
            .field("rx_lsb_first", &self.rx_lsb_first())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: normally ouput sda data 0: exchange the function of sda_o and sda_oe (sda_o is the original internal output sda signal sda_oe is the enable bit for the internal output sda signal)"]
    #[inline(always)]
    #[must_use]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W<CTR_SPEC> {
        SDA_FORCE_OUT_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: normally ouput scl clock 0: exchange the function of scl_o and scl_oe (scl_o is the original internal output scl signal scl_oe is the enable bit for the internal output scl signal)"]
    #[inline(always)]
    #[must_use]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W<CTR_SPEC> {
        SCL_FORCE_OUT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to sample data in SCL low level. clear this bit to sample data in SCL high level."]
    #[inline(always)]
    #[must_use]
    pub fn sample_scl_level(&mut self) -> SAMPLE_SCL_LEVEL_W<CTR_SPEC> {
        SAMPLE_SCL_LEVEL_W::new(self, 2)
    }
    #[doc = "Bit 4 - Set this bit to configure the module as i2c master clear this bit to configure the module as i2c slave."]
    #[inline(always)]
    #[must_use]
    pub fn ms_mode(&mut self) -> MS_MODE_W<CTR_SPEC> {
        MS_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to start sending data in txfifo."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start(&mut self) -> TRANS_START_W<CTR_SPEC> {
        TRANS_START_W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is used to control the sending mode for data need to be send. 1: receive data from most significant bit 0: receive data from least significant bit"]
    #[inline(always)]
    #[must_use]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W<CTR_SPEC> {
        TX_LSB_FIRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit is used to control the storage mode for received datas. 1: receive data from most significant bit 0: receive data from least significant bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W<CTR_SPEC> {
        RX_LSB_FIRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - This is the clock gating control bit for reading or writing registers."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CTR_SPEC> {
        CLK_EN_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTR to value 0x03"]
impl crate::Resettable for CTR_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
