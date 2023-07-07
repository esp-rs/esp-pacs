#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_FORCE_OUT` reader - 1=push pull,0=open drain"]
pub type SDA_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SDA_FORCE_OUT` writer - 1=push pull,0=open drain"]
pub type SDA_FORCE_OUT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `SCL_FORCE_OUT` reader - 1=push pull,0=open drain"]
pub type SCL_FORCE_OUT_R = crate::BitReader;
#[doc = "Field `SCL_FORCE_OUT` writer - 1=push pull,0=open drain"]
pub type SCL_FORCE_OUT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `MS_MODE` reader - 1=master,0=slave"]
pub type MS_MODE_R = crate::BitReader;
#[doc = "Field `MS_MODE` writer - 1=master,0=slave"]
pub type MS_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `TRANS_START` reader - force start"]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `TRANS_START` writer - force start"]
pub type TRANS_START_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `TX_LSB_FIRST` reader - transit lsb first"]
pub type TX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `TX_LSB_FIRST` writer - transit lsb first"]
pub type TX_LSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `RX_LSB_FIRST` reader - receive lsb first"]
pub type RX_LSB_FIRST_R = crate::BitReader;
#[doc = "Field `RX_LSB_FIRST` writer - receive lsb first"]
pub type RX_LSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `I2C_CTRL_CLK_GATE_EN` reader - configure i2c ctrl clk enable"]
pub type I2C_CTRL_CLK_GATE_EN_R = crate::BitReader;
#[doc = "Field `I2C_CTRL_CLK_GATE_EN` writer - configure i2c ctrl clk enable"]
pub type I2C_CTRL_CLK_GATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `I2C_RESET` reader - rtc i2c sw reset"]
pub type I2C_RESET_R = crate::BitReader;
#[doc = "Field `I2C_RESET` writer - rtc i2c sw reset"]
pub type I2C_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `I2CCLK_EN` reader - rtc i2c reg clk gating"]
pub type I2CCLK_EN_R = crate::BitReader;
#[doc = "Field `I2CCLK_EN` writer - rtc i2c reg clk gating"]
pub type I2CCLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
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
            .field(
                "sda_force_out",
                &format_args!("{}", self.sda_force_out().bit()),
            )
            .field(
                "scl_force_out",
                &format_args!("{}", self.scl_force_out().bit()),
            )
            .field("ms_mode", &format_args!("{}", self.ms_mode().bit()))
            .field("trans_start", &format_args!("{}", self.trans_start().bit()))
            .field(
                "tx_lsb_first",
                &format_args!("{}", self.tx_lsb_first().bit()),
            )
            .field(
                "rx_lsb_first",
                &format_args!("{}", self.rx_lsb_first().bit()),
            )
            .field(
                "i2c_ctrl_clk_gate_en",
                &format_args!("{}", self.i2c_ctrl_clk_gate_en().bit()),
            )
            .field("i2c_reset", &format_args!("{}", self.i2c_reset().bit()))
            .field("i2cclk_en", &format_args!("{}", self.i2cclk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1=push pull,0=open drain"]
    #[inline(always)]
    #[must_use]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W<0> {
        SDA_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 1 - 1=push pull,0=open drain"]
    #[inline(always)]
    #[must_use]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W<1> {
        SCL_FORCE_OUT_W::new(self)
    }
    #[doc = "Bit 2 - 1=master,0=slave"]
    #[inline(always)]
    #[must_use]
    pub fn ms_mode(&mut self) -> MS_MODE_W<2> {
        MS_MODE_W::new(self)
    }
    #[doc = "Bit 3 - force start"]
    #[inline(always)]
    #[must_use]
    pub fn trans_start(&mut self) -> TRANS_START_W<3> {
        TRANS_START_W::new(self)
    }
    #[doc = "Bit 4 - transit lsb first"]
    #[inline(always)]
    #[must_use]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W<4> {
        TX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 5 - receive lsb first"]
    #[inline(always)]
    #[must_use]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W<5> {
        RX_LSB_FIRST_W::new(self)
    }
    #[doc = "Bit 29 - configure i2c ctrl clk enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ctrl_clk_gate_en(&mut self) -> I2C_CTRL_CLK_GATE_EN_W<29> {
        I2C_CTRL_CLK_GATE_EN_W::new(self)
    }
    #[doc = "Bit 30 - rtc i2c sw reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_reset(&mut self) -> I2C_RESET_W<30> {
        I2C_RESET_W::new(self)
    }
    #[doc = "Bit 31 - rtc i2c reg clk gating"]
    #[inline(always)]
    #[must_use]
    pub fn i2cclk_en(&mut self) -> I2CCLK_EN_W<31> {
        I2CCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure i2c ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
