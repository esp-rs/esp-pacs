#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_RESET` writer - Set this bit to reset transmitter."]
pub type TX_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_RESET` writer - Set this bit to reset receiver."]
pub type RX_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_FIFO_RESET` writer - Set this bit to reset TX FIFO."]
pub type TX_FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_FIFO_RESET` writer - Set this bit to reset RX FIFO."]
pub type RX_FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_START` reader - Set this bit to start transmitting data."]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - Set this bit to start transmitting data."]
pub type TX_START_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_START` reader - Set this bit to start receiving data."]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `RX_START` writer - Set this bit to start receiving data."]
pub type RX_START_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_SLAVE_MOD` reader - Set this bit to enable slave transmitter mode."]
pub type TX_SLAVE_MOD_R = crate::BitReader;
#[doc = "Field `TX_SLAVE_MOD` writer - Set this bit to enable slave transmitter mode."]
pub type TX_SLAVE_MOD_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_SLAVE_MOD` reader - Set this bit to enable slave receiver mode."]
pub type RX_SLAVE_MOD_R = crate::BitReader;
#[doc = "Field `RX_SLAVE_MOD` writer - Set this bit to enable slave receiver mode."]
pub type RX_SLAVE_MOD_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_RIGHT_FIRST` reader - Set this bit to transmit right channel data first."]
pub type TX_RIGHT_FIRST_R = crate::BitReader;
#[doc = "Field `TX_RIGHT_FIRST` writer - Set this bit to transmit right channel data first."]
pub type TX_RIGHT_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_RIGHT_FIRST` reader - Set this bit to receive right channel data first."]
pub type RX_RIGHT_FIRST_R = crate::BitReader;
#[doc = "Field `RX_RIGHT_FIRST` writer - Set this bit to receive right channel data first."]
pub type RX_RIGHT_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_MSB_SHIFT` reader - Set this bit to enable transmitter in Phillips standard mode."]
pub type TX_MSB_SHIFT_R = crate::BitReader;
#[doc = "Field `TX_MSB_SHIFT` writer - Set this bit to enable transmitter in Phillips standard mode."]
pub type TX_MSB_SHIFT_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_MSB_SHIFT` reader - Set this bit to enable receiver in Phillips standard mode."]
pub type RX_MSB_SHIFT_R = crate::BitReader;
#[doc = "Field `RX_MSB_SHIFT` writer - Set this bit to enable receiver in Phillips standard mode."]
pub type RX_MSB_SHIFT_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_SHORT_SYNC` reader - Set this bit to enable transmitter in PCM standard mode."]
pub type TX_SHORT_SYNC_R = crate::BitReader;
#[doc = "Field `TX_SHORT_SYNC` writer - Set this bit to enable transmitter in PCM standard mode."]
pub type TX_SHORT_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_SHORT_SYNC` reader - Set this bit to enable receiver in PCM standard mode."]
pub type RX_SHORT_SYNC_R = crate::BitReader;
#[doc = "Field `RX_SHORT_SYNC` writer - Set this bit to enable receiver in PCM standard mode."]
pub type RX_SHORT_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_MONO` reader - Set this bit to enable transmitter in mono mode."]
pub type TX_MONO_R = crate::BitReader;
#[doc = "Field `TX_MONO` writer - Set this bit to enable transmitter in mono mode."]
pub type TX_MONO_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_MONO` reader - Set this bit to enable receiver in mono mode."]
pub type RX_MONO_R = crate::BitReader;
#[doc = "Field `RX_MONO` writer - Set this bit to enable receiver in mono mode."]
pub type RX_MONO_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_MSB_RIGHT` reader - Set this bit to place right channel data at the MSB in TX FIFO."]
pub type TX_MSB_RIGHT_R = crate::BitReader;
#[doc = "Field `TX_MSB_RIGHT` writer - Set this bit to place right channel data at the MSB in TX FIFO."]
pub type TX_MSB_RIGHT_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_MSB_RIGHT` reader - Set this bit to place right channel data at the MSB in RX FIFO."]
pub type RX_MSB_RIGHT_R = crate::BitReader;
#[doc = "Field `RX_MSB_RIGHT` writer - Set this bit to place right channel data at the MSB in RX FIFO."]
pub type RX_MSB_RIGHT_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_LSB_FIRST_DMA` reader - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
pub type TX_LSB_FIRST_DMA_R = crate::BitReader;
#[doc = "Field `TX_LSB_FIRST_DMA` writer - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
pub type TX_LSB_FIRST_DMA_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_LSB_FIRST_DMA` reader - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
pub type RX_LSB_FIRST_DMA_R = crate::BitReader;
#[doc = "Field `RX_LSB_FIRST_DMA` writer - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
pub type RX_LSB_FIRST_DMA_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `SIG_LOOPBACK` reader - Enable signal loopback mode with transmitter module and receiver module sharing the same WS and BCK signals."]
pub type SIG_LOOPBACK_R = crate::BitReader;
#[doc = "Field `SIG_LOOPBACK` writer - Enable signal loopback mode with transmitter module and receiver module sharing the same WS and BCK signals."]
pub type SIG_LOOPBACK_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_FIFO_RESET_ST` reader - I2S TX FIFO reset status. 1: I2S_TX_FIFO_RESET is not completed. 0: I2S_TX_FIFO_RESET is completed."]
pub type TX_FIFO_RESET_ST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_RESET_ST` reader - I2S RX FIFO reset status. 1: I2S_RX_FIFO_RESET is not completed. 0: I2S_RX_FIFO_RESET is completed."]
pub type RX_FIFO_RESET_ST_R = crate::BitReader;
#[doc = "Field `TX_RESET_ST` reader - I2S TX reset status. 1: I2S_TX_RESET is not completed. 0: I2S_TX_RESET is completed."]
pub type TX_RESET_ST_R = crate::BitReader;
#[doc = "Field `TX_DMA_EQUAL` reader - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
pub type TX_DMA_EQUAL_R = crate::BitReader;
#[doc = "Field `TX_DMA_EQUAL` writer - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
pub type TX_DMA_EQUAL_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_DMA_EQUAL` reader - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
pub type RX_DMA_EQUAL_R = crate::BitReader;
#[doc = "Field `RX_DMA_EQUAL` writer - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
pub type RX_DMA_EQUAL_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `PRE_REQ_EN` reader - Set this bit to enable I2S to prepare data earlier."]
pub type PRE_REQ_EN_R = crate::BitReader;
#[doc = "Field `PRE_REQ_EN` writer - Set this bit to enable I2S to prepare data earlier."]
pub type PRE_REQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `TX_BIG_ENDIAN` reader - I2S TX byte endianness."]
pub type TX_BIG_ENDIAN_R = crate::BitReader;
#[doc = "Field `TX_BIG_ENDIAN` writer - I2S TX byte endianness."]
pub type TX_BIG_ENDIAN_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_BIG_ENDIAN` reader - I2S RX byte endianness."]
pub type RX_BIG_ENDIAN_R = crate::BitReader;
#[doc = "Field `RX_BIG_ENDIAN` writer - I2S RX byte endianness."]
pub type RX_BIG_ENDIAN_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `RX_RESET_ST` reader - I2S RX reset status. 1: I2S_RX_RESET is not completed. 0: I2S_RX_RESET is completed."]
pub type RX_RESET_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Set this bit to start transmitting data."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to start receiving data."]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable slave transmitter mode."]
    #[inline(always)]
    pub fn tx_slave_mod(&self) -> TX_SLAVE_MOD_R {
        TX_SLAVE_MOD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable slave receiver mode."]
    #[inline(always)]
    pub fn rx_slave_mod(&self) -> RX_SLAVE_MOD_R {
        RX_SLAVE_MOD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to transmit right channel data first."]
    #[inline(always)]
    pub fn tx_right_first(&self) -> TX_RIGHT_FIRST_R {
        TX_RIGHT_FIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to receive right channel data first."]
    #[inline(always)]
    pub fn rx_right_first(&self) -> RX_RIGHT_FIRST_R {
        RX_RIGHT_FIRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable transmitter in Phillips standard mode."]
    #[inline(always)]
    pub fn tx_msb_shift(&self) -> TX_MSB_SHIFT_R {
        TX_MSB_SHIFT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable receiver in Phillips standard mode."]
    #[inline(always)]
    pub fn rx_msb_shift(&self) -> RX_MSB_SHIFT_R {
        RX_MSB_SHIFT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable transmitter in PCM standard mode."]
    #[inline(always)]
    pub fn tx_short_sync(&self) -> TX_SHORT_SYNC_R {
        TX_SHORT_SYNC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable receiver in PCM standard mode."]
    #[inline(always)]
    pub fn rx_short_sync(&self) -> RX_SHORT_SYNC_R {
        RX_SHORT_SYNC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable transmitter in mono mode."]
    #[inline(always)]
    pub fn tx_mono(&self) -> TX_MONO_R {
        TX_MONO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable receiver in mono mode."]
    #[inline(always)]
    pub fn rx_mono(&self) -> RX_MONO_R {
        RX_MONO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to place right channel data at the MSB in TX FIFO."]
    #[inline(always)]
    pub fn tx_msb_right(&self) -> TX_MSB_RIGHT_R {
        TX_MSB_RIGHT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to place right channel data at the MSB in RX FIFO."]
    #[inline(always)]
    pub fn rx_msb_right(&self) -> RX_MSB_RIGHT_R {
        RX_MSB_RIGHT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
    #[inline(always)]
    pub fn tx_lsb_first_dma(&self) -> TX_LSB_FIRST_DMA_R {
        TX_LSB_FIRST_DMA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
    #[inline(always)]
    pub fn rx_lsb_first_dma(&self) -> RX_LSB_FIRST_DMA_R {
        RX_LSB_FIRST_DMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable signal loopback mode with transmitter module and receiver module sharing the same WS and BCK signals."]
    #[inline(always)]
    pub fn sig_loopback(&self) -> SIG_LOOPBACK_R {
        SIG_LOOPBACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2S TX FIFO reset status. 1: I2S_TX_FIFO_RESET is not completed. 0: I2S_TX_FIFO_RESET is completed."]
    #[inline(always)]
    pub fn tx_fifo_reset_st(&self) -> TX_FIFO_RESET_ST_R {
        TX_FIFO_RESET_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2S RX FIFO reset status. 1: I2S_RX_FIFO_RESET is not completed. 0: I2S_RX_FIFO_RESET is completed."]
    #[inline(always)]
    pub fn rx_fifo_reset_st(&self) -> RX_FIFO_RESET_ST_R {
        RX_FIFO_RESET_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2S TX reset status. 1: I2S_TX_RESET is not completed. 0: I2S_TX_RESET is completed."]
    #[inline(always)]
    pub fn tx_reset_st(&self) -> TX_RESET_ST_R {
        TX_RESET_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
    #[inline(always)]
    pub fn tx_dma_equal(&self) -> TX_DMA_EQUAL_R {
        TX_DMA_EQUAL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
    #[inline(always)]
    pub fn rx_dma_equal(&self) -> RX_DMA_EQUAL_R {
        RX_DMA_EQUAL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable I2S to prepare data earlier."]
    #[inline(always)]
    pub fn pre_req_en(&self) -> PRE_REQ_EN_R {
        PRE_REQ_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - I2S TX byte endianness."]
    #[inline(always)]
    pub fn tx_big_endian(&self) -> TX_BIG_ENDIAN_R {
        TX_BIG_ENDIAN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - I2S RX byte endianness."]
    #[inline(always)]
    pub fn rx_big_endian(&self) -> RX_BIG_ENDIAN_R {
        RX_BIG_ENDIAN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - I2S RX reset status. 1: I2S_RX_RESET is not completed. 0: I2S_RX_RESET is completed."]
    #[inline(always)]
    pub fn rx_reset_st(&self) -> RX_RESET_ST_R {
        RX_RESET_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("tx_start", &format_args!("{}", self.tx_start().bit()))
            .field("rx_start", &format_args!("{}", self.rx_start().bit()))
            .field(
                "tx_slave_mod",
                &format_args!("{}", self.tx_slave_mod().bit()),
            )
            .field(
                "rx_slave_mod",
                &format_args!("{}", self.rx_slave_mod().bit()),
            )
            .field(
                "tx_right_first",
                &format_args!("{}", self.tx_right_first().bit()),
            )
            .field(
                "rx_right_first",
                &format_args!("{}", self.rx_right_first().bit()),
            )
            .field(
                "tx_msb_shift",
                &format_args!("{}", self.tx_msb_shift().bit()),
            )
            .field(
                "rx_msb_shift",
                &format_args!("{}", self.rx_msb_shift().bit()),
            )
            .field(
                "tx_short_sync",
                &format_args!("{}", self.tx_short_sync().bit()),
            )
            .field(
                "rx_short_sync",
                &format_args!("{}", self.rx_short_sync().bit()),
            )
            .field("tx_mono", &format_args!("{}", self.tx_mono().bit()))
            .field("rx_mono", &format_args!("{}", self.rx_mono().bit()))
            .field(
                "tx_msb_right",
                &format_args!("{}", self.tx_msb_right().bit()),
            )
            .field(
                "rx_msb_right",
                &format_args!("{}", self.rx_msb_right().bit()),
            )
            .field(
                "tx_lsb_first_dma",
                &format_args!("{}", self.tx_lsb_first_dma().bit()),
            )
            .field(
                "rx_lsb_first_dma",
                &format_args!("{}", self.rx_lsb_first_dma().bit()),
            )
            .field(
                "sig_loopback",
                &format_args!("{}", self.sig_loopback().bit()),
            )
            .field(
                "tx_fifo_reset_st",
                &format_args!("{}", self.tx_fifo_reset_st().bit()),
            )
            .field(
                "rx_fifo_reset_st",
                &format_args!("{}", self.rx_fifo_reset_st().bit()),
            )
            .field("tx_reset_st", &format_args!("{}", self.tx_reset_st().bit()))
            .field(
                "tx_dma_equal",
                &format_args!("{}", self.tx_dma_equal().bit()),
            )
            .field(
                "rx_dma_equal",
                &format_args!("{}", self.rx_dma_equal().bit()),
            )
            .field("pre_req_en", &format_args!("{}", self.pre_req_en().bit()))
            .field(
                "tx_big_endian",
                &format_args!("{}", self.tx_big_endian().bit()),
            )
            .field(
                "rx_big_endian",
                &format_args!("{}", self.rx_big_endian().bit()),
            )
            .field("rx_reset_st", &format_args!("{}", self.rx_reset_st().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn tx_reset(&mut self) -> TX_RESET_W<0> {
        TX_RESET_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to reset receiver."]
    #[inline(always)]
    #[must_use]
    pub fn rx_reset(&mut self) -> RX_RESET_W<1> {
        RX_RESET_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_reset(&mut self) -> TX_FIFO_RESET_W<2> {
        TX_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to reset RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_reset(&mut self) -> RX_FIFO_RESET_W<3> {
        RX_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to start transmitting data."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<4> {
        TX_START_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to start receiving data."]
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<5> {
        RX_START_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to enable slave transmitter mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_slave_mod(&mut self) -> TX_SLAVE_MOD_W<6> {
        TX_SLAVE_MOD_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to enable slave receiver mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_slave_mod(&mut self) -> RX_SLAVE_MOD_W<7> {
        RX_SLAVE_MOD_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to transmit right channel data first."]
    #[inline(always)]
    #[must_use]
    pub fn tx_right_first(&mut self) -> TX_RIGHT_FIRST_W<8> {
        TX_RIGHT_FIRST_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to receive right channel data first."]
    #[inline(always)]
    #[must_use]
    pub fn rx_right_first(&mut self) -> RX_RIGHT_FIRST_W<9> {
        RX_RIGHT_FIRST_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to enable transmitter in Phillips standard mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_msb_shift(&mut self) -> TX_MSB_SHIFT_W<10> {
        TX_MSB_SHIFT_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to enable receiver in Phillips standard mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_msb_shift(&mut self) -> RX_MSB_SHIFT_W<11> {
        RX_MSB_SHIFT_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to enable transmitter in PCM standard mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_short_sync(&mut self) -> TX_SHORT_SYNC_W<12> {
        TX_SHORT_SYNC_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to enable receiver in PCM standard mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_short_sync(&mut self) -> RX_SHORT_SYNC_W<13> {
        RX_SHORT_SYNC_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to enable transmitter in mono mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_mono(&mut self) -> TX_MONO_W<14> {
        TX_MONO_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to enable receiver in mono mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_mono(&mut self) -> RX_MONO_W<15> {
        RX_MONO_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to place right channel data at the MSB in TX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn tx_msb_right(&mut self) -> TX_MSB_RIGHT_W<16> {
        TX_MSB_RIGHT_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to place right channel data at the MSB in RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_msb_right(&mut self) -> RX_MSB_RIGHT_W<17> {
        RX_MSB_RIGHT_W::new(self)
    }
    #[doc = "Bit 18 - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lsb_first_dma(&mut self) -> TX_LSB_FIRST_DMA_W<18> {
        TX_LSB_FIRST_DMA_W::new(self)
    }
    #[doc = "Bit 19 - 1:the data in DMA/APB transform from low bits. 0:the data from DMA/APB transform from high bits."]
    #[inline(always)]
    #[must_use]
    pub fn rx_lsb_first_dma(&mut self) -> RX_LSB_FIRST_DMA_W<19> {
        RX_LSB_FIRST_DMA_W::new(self)
    }
    #[doc = "Bit 20 - Enable signal loopback mode with transmitter module and receiver module sharing the same WS and BCK signals."]
    #[inline(always)]
    #[must_use]
    pub fn sig_loopback(&mut self) -> SIG_LOOPBACK_W<20> {
        SIG_LOOPBACK_W::new(self)
    }
    #[doc = "Bit 24 - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
    #[inline(always)]
    #[must_use]
    pub fn tx_dma_equal(&mut self) -> TX_DMA_EQUAL_W<24> {
        TX_DMA_EQUAL_W::new(self)
    }
    #[doc = "Bit 25 - 1: Data in left channel is equal to data in right channel. 0: Data in left channel is not equal to data in right channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma_equal(&mut self) -> RX_DMA_EQUAL_W<25> {
        RX_DMA_EQUAL_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to enable I2S to prepare data earlier."]
    #[inline(always)]
    #[must_use]
    pub fn pre_req_en(&mut self) -> PRE_REQ_EN_W<26> {
        PRE_REQ_EN_W::new(self)
    }
    #[doc = "Bit 27 - I2S TX byte endianness."]
    #[inline(always)]
    #[must_use]
    pub fn tx_big_endian(&mut self) -> TX_BIG_ENDIAN_W<27> {
        TX_BIG_ENDIAN_W::new(self)
    }
    #[doc = "Bit 28 - I2S RX byte endianness."]
    #[inline(always)]
    #[must_use]
    pub fn rx_big_endian(&mut self) -> RX_BIG_ENDIAN_W<28> {
        RX_BIG_ENDIAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF to value 0x000c_0300"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x000c_0300;
}
