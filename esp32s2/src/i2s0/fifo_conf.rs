#[doc = "Register `FIFO_CONF` reader"]
pub struct R(crate::R<FIFO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_CONF` writer"]
pub struct W(crate::W<FIFO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CONF_SPEC>;
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
impl From<crate::W<FIFO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_DATA_NUM` reader - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]. (RX FIFO is almost full threshold.)"]
pub type RX_DATA_NUM_R = crate::FieldReader;
#[doc = "Field `RX_DATA_NUM` writer - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]. (RX FIFO is almost full threshold.)"]
pub type RX_DATA_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, FIFO_CONF_SPEC, 6, O>;
#[doc = "Field `TX_DATA_NUM` reader - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]. (TX FIFO is almost empty threshold.)"]
pub type TX_DATA_NUM_R = crate::FieldReader;
#[doc = "Field `TX_DATA_NUM` writer - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]. (TX FIFO is almost empty threshold.)"]
pub type TX_DATA_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, FIFO_CONF_SPEC, 6, O>;
#[doc = "Field `DSCR_EN` reader - Set this bit to enable I2S DMA mode."]
pub type DSCR_EN_R = crate::BitReader;
#[doc = "Field `DSCR_EN` writer - Set this bit to enable I2S DMA mode."]
pub type DSCR_EN_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `TX_FIFO_MOD` reader - Transmitter FIFO mode configuration bits"]
pub type TX_FIFO_MOD_R = crate::FieldReader;
#[doc = "Field `TX_FIFO_MOD` writer - Transmitter FIFO mode configuration bits"]
pub type TX_FIFO_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, FIFO_CONF_SPEC, 3, O>;
#[doc = "Field `RX_FIFO_MOD` reader - Receiver FIFO mode configuration bits"]
pub type RX_FIFO_MOD_R = crate::FieldReader;
#[doc = "Field `RX_FIFO_MOD` writer - Receiver FIFO mode configuration bits"]
pub type RX_FIFO_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, FIFO_CONF_SPEC, 3, O>;
#[doc = "Field `TX_FIFO_MOD_FORCE_EN` reader - The bit should always be set to 1"]
pub type TX_FIFO_MOD_FORCE_EN_R = crate::BitReader;
#[doc = "Field `TX_FIFO_MOD_FORCE_EN` writer - The bit should always be set to 1"]
pub type TX_FIFO_MOD_FORCE_EN_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `RX_FIFO_MOD_FORCE_EN` reader - The bit should always be set to 1"]
pub type RX_FIFO_MOD_FORCE_EN_R = crate::BitReader;
#[doc = "Field `RX_FIFO_MOD_FORCE_EN` writer - The bit should always be set to 1"]
pub type RX_FIFO_MOD_FORCE_EN_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `RX_FIFO_SYNC` reader - force write back rx data to memory"]
pub type RX_FIFO_SYNC_R = crate::BitReader;
#[doc = "Field `RX_FIFO_SYNC` writer - force write back rx data to memory"]
pub type RX_FIFO_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `RX_24MSB_EN` reader - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub type RX_24MSB_EN_R = crate::BitReader;
#[doc = "Field `RX_24MSB_EN` writer - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub type RX_24MSB_EN_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
#[doc = "Field `TX_24MSB_EN` reader - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub type TX_24MSB_EN_R = crate::BitReader;
#[doc = "Field `TX_24MSB_EN` writer - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
pub type TX_24MSB_EN_W<'a, const O: u8> = crate::BitWriter<'a, FIFO_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:5 - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]. (RX FIFO is almost full threshold.)"]
    #[inline(always)]
    pub fn rx_data_num(&self) -> RX_DATA_NUM_R {
        RX_DATA_NUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]. (TX FIFO is almost empty threshold.)"]
    #[inline(always)]
    pub fn tx_data_num(&self) -> TX_DATA_NUM_R {
        TX_DATA_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Set this bit to enable I2S DMA mode."]
    #[inline(always)]
    pub fn dscr_en(&self) -> DSCR_EN_R {
        DSCR_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Transmitter FIFO mode configuration bits"]
    #[inline(always)]
    pub fn tx_fifo_mod(&self) -> TX_FIFO_MOD_R {
        TX_FIFO_MOD_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Receiver FIFO mode configuration bits"]
    #[inline(always)]
    pub fn rx_fifo_mod(&self) -> RX_FIFO_MOD_R {
        RX_FIFO_MOD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - The bit should always be set to 1"]
    #[inline(always)]
    pub fn tx_fifo_mod_force_en(&self) -> TX_FIFO_MOD_FORCE_EN_R {
        TX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The bit should always be set to 1"]
    #[inline(always)]
    pub fn rx_fifo_mod_force_en(&self) -> RX_FIFO_MOD_FORCE_EN_R {
        RX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - force write back rx data to memory"]
    #[inline(always)]
    pub fn rx_fifo_sync(&self) -> RX_FIFO_SYNC_R {
        RX_FIFO_SYNC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    pub fn rx_24msb_en(&self) -> RX_24MSB_EN_R {
        RX_24MSB_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    pub fn tx_24msb_en(&self) -> TX_24MSB_EN_R {
        TX_24MSB_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CONF")
            .field(
                "rx_data_num",
                &format_args!("{}", self.rx_data_num().bits()),
            )
            .field(
                "tx_data_num",
                &format_args!("{}", self.tx_data_num().bits()),
            )
            .field("dscr_en", &format_args!("{}", self.dscr_en().bit()))
            .field(
                "tx_fifo_mod",
                &format_args!("{}", self.tx_fifo_mod().bits()),
            )
            .field(
                "rx_fifo_mod",
                &format_args!("{}", self.rx_fifo_mod().bits()),
            )
            .field(
                "tx_fifo_mod_force_en",
                &format_args!("{}", self.tx_fifo_mod_force_en().bit()),
            )
            .field(
                "rx_fifo_mod_force_en",
                &format_args!("{}", self.rx_fifo_mod_force_en().bit()),
            )
            .field(
                "rx_fifo_sync",
                &format_args!("{}", self.rx_fifo_sync().bit()),
            )
            .field("rx_24msb_en", &format_args!("{}", self.rx_24msb_en().bit()))
            .field("tx_24msb_en", &format_args!("{}", self.tx_24msb_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S_RX_TAKE_DATA_INT is triggered when the left and right channel data number in RX FIFO is larger than the value of I2S_RX_DATA_NUM\\[5:0\\]. (RX FIFO is almost full threshold.)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_num(&mut self) -> RX_DATA_NUM_W<0> {
        RX_DATA_NUM_W::new(self)
    }
    #[doc = "Bits 6:11 - I2S_TX_PUT_DATA_INT is triggered when the left and right channel data number in TX FIFO is smaller than the value of I2S_TX_DATA_NUM\\[5:0\\]. (TX FIFO is almost empty threshold.)"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_num(&mut self) -> TX_DATA_NUM_W<6> {
        TX_DATA_NUM_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to enable I2S DMA mode."]
    #[inline(always)]
    #[must_use]
    pub fn dscr_en(&mut self) -> DSCR_EN_W<12> {
        DSCR_EN_W::new(self)
    }
    #[doc = "Bits 13:15 - Transmitter FIFO mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_mod(&mut self) -> TX_FIFO_MOD_W<13> {
        TX_FIFO_MOD_W::new(self)
    }
    #[doc = "Bits 16:18 - Receiver FIFO mode configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_mod(&mut self) -> RX_FIFO_MOD_W<16> {
        RX_FIFO_MOD_W::new(self)
    }
    #[doc = "Bit 19 - The bit should always be set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_mod_force_en(&mut self) -> TX_FIFO_MOD_FORCE_EN_W<19> {
        TX_FIFO_MOD_FORCE_EN_W::new(self)
    }
    #[doc = "Bit 20 - The bit should always be set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_mod_force_en(&mut self) -> RX_FIFO_MOD_FORCE_EN_W<20> {
        RX_FIFO_MOD_FORCE_EN_W::new(self)
    }
    #[doc = "Bit 21 - force write back rx data to memory"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_sync(&mut self) -> RX_FIFO_SYNC_W<21> {
        RX_FIFO_SYNC_W::new(self)
    }
    #[doc = "Bit 22 - Only useful in rx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    #[must_use]
    pub fn rx_24msb_en(&mut self) -> RX_24MSB_EN_W<22> {
        RX_24MSB_EN_W::new(self)
    }
    #[doc = "Bit 23 - Only useful in tx 24bit mode. 1: the high 24 bits are effective in i2s fifo 0: the low 24 bits are effective in i2s fifo"]
    #[inline(always)]
    #[must_use]
    pub fn tx_24msb_en(&mut self) -> TX_24MSB_EN_W<23> {
        TX_24MSB_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S FIFO configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_conf](index.html) module"]
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_conf::R](R) reader structure"]
impl crate::Readable for FIFO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_conf::W](W) writer structure"]
impl crate::Writable for FIFO_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0x1820"]
impl crate::Resettable for FIFO_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x1820;
}
