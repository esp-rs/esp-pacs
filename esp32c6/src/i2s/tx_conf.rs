#[doc = "Register `TX_CONF` reader"]
pub struct R(crate::R<TX_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CONF` writer"]
pub struct W(crate::W<TX_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CONF_SPEC>;
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
impl From<crate::W<TX_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_RESET` writer - Set this bit to reset transmitter"]
pub type TX_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_FIFO_RESET` writer - Set this bit to reset Tx AFIFO"]
pub type TX_FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_START` reader - Set this bit to start transmitting data"]
pub type TX_START_R = crate::BitReader<bool>;
#[doc = "Field `TX_START` writer - Set this bit to start transmitting data"]
pub type TX_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_SLAVE_MOD` reader - Set this bit to enable slave transmitter mode"]
pub type TX_SLAVE_MOD_R = crate::BitReader<bool>;
#[doc = "Field `TX_SLAVE_MOD` writer - Set this bit to enable slave transmitter mode"]
pub type TX_SLAVE_MOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_MONO` reader - Set this bit to enable transmitter in mono mode"]
pub type TX_MONO_R = crate::BitReader<bool>;
#[doc = "Field `TX_MONO` writer - Set this bit to enable transmitter in mono mode"]
pub type TX_MONO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_CHAN_EQUAL` reader - 1: The value of Left channel data is equal to the value of right channel data in I2S TX mono mode or TDM channel select mode. 0: The invalid channel data is reg_i2s_single_data in I2S TX mono mode or TDM channel select mode."]
pub type TX_CHAN_EQUAL_R = crate::BitReader<bool>;
#[doc = "Field `TX_CHAN_EQUAL` writer - 1: The value of Left channel data is equal to the value of right channel data in I2S TX mono mode or TDM channel select mode. 0: The invalid channel data is reg_i2s_single_data in I2S TX mono mode or TDM channel select mode."]
pub type TX_CHAN_EQUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_BIG_ENDIAN` reader - I2S Tx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
pub type TX_BIG_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `TX_BIG_ENDIAN` writer - I2S Tx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
pub type TX_BIG_ENDIAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_UPDATE` reader - Set 1 to update I2S TX registers from APB clock domain to I2S TX clock domain. This bit will be cleared by hardware after update register done."]
pub type TX_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `TX_UPDATE` writer - Set 1 to update I2S TX registers from APB clock domain to I2S TX clock domain. This bit will be cleared by hardware after update register done."]
pub type TX_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_MONO_FST_VLD` reader - 1: The first channel data value is valid in I2S TX mono mode. 0: The second channel data value is valid in I2S TX mono mode."]
pub type TX_MONO_FST_VLD_R = crate::BitReader<bool>;
#[doc = "Field `TX_MONO_FST_VLD` writer - 1: The first channel data value is valid in I2S TX mono mode. 0: The second channel data value is valid in I2S TX mono mode."]
pub type TX_MONO_FST_VLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_PCM_CONF` reader - I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
pub type TX_PCM_CONF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PCM_CONF` writer - I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
pub type TX_PCM_CONF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_PCM_BYPASS` reader - Set this bit to bypass Compress/Decompress module for transmitted data."]
pub type TX_PCM_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `TX_PCM_BYPASS` writer - Set this bit to bypass Compress/Decompress module for transmitted data."]
pub type TX_PCM_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_STOP_EN` reader - Set this bit to stop disable output BCK signal and WS signal when tx FIFO is emtpy"]
pub type TX_STOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_STOP_EN` writer - Set this bit to stop disable output BCK signal and WS signal when tx FIFO is emtpy"]
pub type TX_STOP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_LEFT_ALIGN` reader - 1: I2S TX left alignment mode. 0: I2S TX right alignment mode."]
pub type TX_LEFT_ALIGN_R = crate::BitReader<bool>;
#[doc = "Field `TX_LEFT_ALIGN` writer - 1: I2S TX left alignment mode. 0: I2S TX right alignment mode."]
pub type TX_LEFT_ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_24_FILL_EN` reader - 1: Sent 32 bits in 24 channel bits mode. 0: Sent 24 bits in 24 channel bits mode"]
pub type TX_24_FILL_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_24_FILL_EN` writer - 1: Sent 32 bits in 24 channel bits mode. 0: Sent 24 bits in 24 channel bits mode"]
pub type TX_24_FILL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_WS_IDLE_POL` reader - 0: WS should be 0 when sending left channel data, and WS is 1in right channel. 1: WS should be 1 when sending left channel data, and WS is 0in right channel."]
pub type TX_WS_IDLE_POL_R = crate::BitReader<bool>;
#[doc = "Field `TX_WS_IDLE_POL` writer - 0: WS should be 0 when sending left channel data, and WS is 1in right channel. 1: WS should be 1 when sending left channel data, and WS is 0in right channel."]
pub type TX_WS_IDLE_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_BIT_ORDER` reader - I2S Tx bit endian. 1:small endian, the LSB is sent first. 0:big endian, the MSB is sent first."]
pub type TX_BIT_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `TX_BIT_ORDER` writer - I2S Tx bit endian. 1:small endian, the LSB is sent first. 0:big endian, the MSB is sent first."]
pub type TX_BIT_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_TDM_EN` reader - 1: Enable I2S TDM Tx mode . 0: Disable."]
pub type TX_TDM_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_TDM_EN` writer - 1: Enable I2S TDM Tx mode . 0: Disable."]
pub type TX_TDM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_PDM_EN` reader - 1: Enable I2S PDM Tx mode . 0: Disable."]
pub type TX_PDM_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_PDM_EN` writer - 1: Enable I2S PDM Tx mode . 0: Disable."]
pub type TX_PDM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
#[doc = "Field `TX_CHAN_MOD` reader - I2S transmitter channel mode configuration bits."]
pub type TX_CHAN_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_CHAN_MOD` writer - I2S transmitter channel mode configuration bits."]
pub type TX_CHAN_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_CONF_SPEC, u8, u8, 3, O>;
#[doc = "Field `SIG_LOOPBACK` reader - Enable signal loop back mode with transmitter module and receiver module sharing the same WS and BCK signals."]
pub type SIG_LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `SIG_LOOPBACK` writer - Enable signal loop back mode with transmitter module and receiver module sharing the same WS and BCK signals."]
pub type SIG_LOOPBACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Set this bit to start transmitting data"]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable slave transmitter mode"]
    #[inline(always)]
    pub fn tx_slave_mod(&self) -> TX_SLAVE_MOD_R {
        TX_SLAVE_MOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable transmitter in mono mode"]
    #[inline(always)]
    pub fn tx_mono(&self) -> TX_MONO_R {
        TX_MONO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: The value of Left channel data is equal to the value of right channel data in I2S TX mono mode or TDM channel select mode. 0: The invalid channel data is reg_i2s_single_data in I2S TX mono mode or TDM channel select mode."]
    #[inline(always)]
    pub fn tx_chan_equal(&self) -> TX_CHAN_EQUAL_R {
        TX_CHAN_EQUAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2S Tx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
    #[inline(always)]
    pub fn tx_big_endian(&self) -> TX_BIG_ENDIAN_R {
        TX_BIG_ENDIAN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to update I2S TX registers from APB clock domain to I2S TX clock domain. This bit will be cleared by hardware after update register done."]
    #[inline(always)]
    pub fn tx_update(&self) -> TX_UPDATE_R {
        TX_UPDATE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: The first channel data value is valid in I2S TX mono mode. 0: The second channel data value is valid in I2S TX mono mode."]
    #[inline(always)]
    pub fn tx_mono_fst_vld(&self) -> TX_MONO_FST_VLD_R {
        TX_MONO_FST_VLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
    #[inline(always)]
    pub fn tx_pcm_conf(&self) -> TX_PCM_CONF_R {
        TX_PCM_CONF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Set this bit to bypass Compress/Decompress module for transmitted data."]
    #[inline(always)]
    pub fn tx_pcm_bypass(&self) -> TX_PCM_BYPASS_R {
        TX_PCM_BYPASS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to stop disable output BCK signal and WS signal when tx FIFO is emtpy"]
    #[inline(always)]
    pub fn tx_stop_en(&self) -> TX_STOP_EN_R {
        TX_STOP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: I2S TX left alignment mode. 0: I2S TX right alignment mode."]
    #[inline(always)]
    pub fn tx_left_align(&self) -> TX_LEFT_ALIGN_R {
        TX_LEFT_ALIGN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: Sent 32 bits in 24 channel bits mode. 0: Sent 24 bits in 24 channel bits mode"]
    #[inline(always)]
    pub fn tx_24_fill_en(&self) -> TX_24_FILL_EN_R {
        TX_24_FILL_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: WS should be 0 when sending left channel data, and WS is 1in right channel. 1: WS should be 1 when sending left channel data, and WS is 0in right channel."]
    #[inline(always)]
    pub fn tx_ws_idle_pol(&self) -> TX_WS_IDLE_POL_R {
        TX_WS_IDLE_POL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2S Tx bit endian. 1:small endian, the LSB is sent first. 0:big endian, the MSB is sent first."]
    #[inline(always)]
    pub fn tx_bit_order(&self) -> TX_BIT_ORDER_R {
        TX_BIT_ORDER_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Enable I2S TDM Tx mode . 0: Disable."]
    #[inline(always)]
    pub fn tx_tdm_en(&self) -> TX_TDM_EN_R {
        TX_TDM_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Enable I2S PDM Tx mode . 0: Disable."]
    #[inline(always)]
    pub fn tx_pdm_en(&self) -> TX_PDM_EN_R {
        TX_PDM_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - I2S transmitter channel mode configuration bits."]
    #[inline(always)]
    pub fn tx_chan_mod(&self) -> TX_CHAN_MOD_R {
        TX_CHAN_MOD_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Enable signal loop back mode with transmitter module and receiver module sharing the same WS and BCK signals."]
    #[inline(always)]
    pub fn sig_loopback(&self) -> SIG_LOOPBACK_R {
        SIG_LOOPBACK_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn tx_reset(&mut self) -> TX_RESET_W<0> {
        TX_RESET_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to reset Tx AFIFO"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_reset(&mut self) -> TX_FIFO_RESET_W<1> {
        TX_FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to start transmitting data"]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<2> {
        TX_START_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to enable slave transmitter mode"]
    #[inline(always)]
    #[must_use]
    pub fn tx_slave_mod(&mut self) -> TX_SLAVE_MOD_W<3> {
        TX_SLAVE_MOD_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to enable transmitter in mono mode"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mono(&mut self) -> TX_MONO_W<5> {
        TX_MONO_W::new(self)
    }
    #[doc = "Bit 6 - 1: The value of Left channel data is equal to the value of right channel data in I2S TX mono mode or TDM channel select mode. 0: The invalid channel data is reg_i2s_single_data in I2S TX mono mode or TDM channel select mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_chan_equal(&mut self) -> TX_CHAN_EQUAL_W<6> {
        TX_CHAN_EQUAL_W::new(self)
    }
    #[doc = "Bit 7 - I2S Tx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
    #[inline(always)]
    #[must_use]
    pub fn tx_big_endian(&mut self) -> TX_BIG_ENDIAN_W<7> {
        TX_BIG_ENDIAN_W::new(self)
    }
    #[doc = "Bit 8 - Set 1 to update I2S TX registers from APB clock domain to I2S TX clock domain. This bit will be cleared by hardware after update register done."]
    #[inline(always)]
    #[must_use]
    pub fn tx_update(&mut self) -> TX_UPDATE_W<8> {
        TX_UPDATE_W::new(self)
    }
    #[doc = "Bit 9 - 1: The first channel data value is valid in I2S TX mono mode. 0: The second channel data value is valid in I2S TX mono mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_mono_fst_vld(&mut self) -> TX_MONO_FST_VLD_W<9> {
        TX_MONO_FST_VLD_W::new(self)
    }
    #[doc = "Bits 10:11 - I2S TX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pcm_conf(&mut self) -> TX_PCM_CONF_W<10> {
        TX_PCM_CONF_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to bypass Compress/Decompress module for transmitted data."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pcm_bypass(&mut self) -> TX_PCM_BYPASS_W<12> {
        TX_PCM_BYPASS_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to stop disable output BCK signal and WS signal when tx FIFO is emtpy"]
    #[inline(always)]
    #[must_use]
    pub fn tx_stop_en(&mut self) -> TX_STOP_EN_W<13> {
        TX_STOP_EN_W::new(self)
    }
    #[doc = "Bit 15 - 1: I2S TX left alignment mode. 0: I2S TX right alignment mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_left_align(&mut self) -> TX_LEFT_ALIGN_W<15> {
        TX_LEFT_ALIGN_W::new(self)
    }
    #[doc = "Bit 16 - 1: Sent 32 bits in 24 channel bits mode. 0: Sent 24 bits in 24 channel bits mode"]
    #[inline(always)]
    #[must_use]
    pub fn tx_24_fill_en(&mut self) -> TX_24_FILL_EN_W<16> {
        TX_24_FILL_EN_W::new(self)
    }
    #[doc = "Bit 17 - 0: WS should be 0 when sending left channel data, and WS is 1in right channel. 1: WS should be 1 when sending left channel data, and WS is 0in right channel."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ws_idle_pol(&mut self) -> TX_WS_IDLE_POL_W<17> {
        TX_WS_IDLE_POL_W::new(self)
    }
    #[doc = "Bit 18 - I2S Tx bit endian. 1:small endian, the LSB is sent first. 0:big endian, the MSB is sent first."]
    #[inline(always)]
    #[must_use]
    pub fn tx_bit_order(&mut self) -> TX_BIT_ORDER_W<18> {
        TX_BIT_ORDER_W::new(self)
    }
    #[doc = "Bit 19 - 1: Enable I2S TDM Tx mode . 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_tdm_en(&mut self) -> TX_TDM_EN_W<19> {
        TX_TDM_EN_W::new(self)
    }
    #[doc = "Bit 20 - 1: Enable I2S PDM Tx mode . 0: Disable."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_en(&mut self) -> TX_PDM_EN_W<20> {
        TX_PDM_EN_W::new(self)
    }
    #[doc = "Bits 24:26 - I2S transmitter channel mode configuration bits."]
    #[inline(always)]
    #[must_use]
    pub fn tx_chan_mod(&mut self) -> TX_CHAN_MOD_W<24> {
        TX_CHAN_MOD_W::new(self)
    }
    #[doc = "Bit 27 - Enable signal loop back mode with transmitter module and receiver module sharing the same WS and BCK signals."]
    #[inline(always)]
    #[must_use]
    pub fn sig_loopback(&mut self) -> SIG_LOOPBACK_W<27> {
        SIG_LOOPBACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_conf](index.html) module"]
pub struct TX_CONF_SPEC;
impl crate::RegisterSpec for TX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_conf::R](R) reader structure"]
impl crate::Readable for TX_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_conf::W](W) writer structure"]
impl crate::Writable for TX_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CONF to value 0xb200"]
impl crate::Resettable for TX_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0xb200;
}
