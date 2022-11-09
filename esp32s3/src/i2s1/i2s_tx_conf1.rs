#[doc = "Register `I2S_TX_CONF1` reader"]
pub struct R(crate::R<I2S_TX_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_TX_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_TX_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_TX_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_TX_CONF1` writer"]
pub struct W(crate::W<I2S_TX_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_TX_CONF1_SPEC>;
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
impl From<crate::W<I2S_TX_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_TX_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_TX_TDM_WS_WIDTH` reader - The width of tx_ws_out in TDM mode is (I2S_TX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
pub type I2S_TX_TDM_WS_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_TDM_WS_WIDTH` writer - The width of tx_ws_out in TDM mode is (I2S_TX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
pub type I2S_TX_TDM_WS_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_CONF1_SPEC, u8, u8, 7, O>;
#[doc = "Field `I2S_TX_BCK_DIV_NUM` reader - Bit clock configuration bits in transmitter mode."]
pub type I2S_TX_BCK_DIV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_BCK_DIV_NUM` writer - Bit clock configuration bits in transmitter mode."]
pub type I2S_TX_BCK_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_CONF1_SPEC, u8, u8, 6, O>;
#[doc = "Field `I2S_TX_BITS_MOD` reader - Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
pub type I2S_TX_BITS_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_BITS_MOD` writer - Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
pub type I2S_TX_BITS_MOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_CONF1_SPEC, u8, u8, 5, O>;
#[doc = "Field `I2S_TX_HALF_SAMPLE_BITS` reader - I2S Tx half sample bits -1."]
pub type I2S_TX_HALF_SAMPLE_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_HALF_SAMPLE_BITS` writer - I2S Tx half sample bits -1."]
pub type I2S_TX_HALF_SAMPLE_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_CONF1_SPEC, u8, u8, 6, O>;
#[doc = "Field `I2S_TX_TDM_CHAN_BITS` reader - The Tx bit number for each channel minus 1in TDM mode."]
pub type I2S_TX_TDM_CHAN_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_TDM_CHAN_BITS` writer - The Tx bit number for each channel minus 1in TDM mode."]
pub type I2S_TX_TDM_CHAN_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_CONF1_SPEC, u8, u8, 5, O>;
#[doc = "Field `I2S_TX_MSB_SHIFT` reader - Set this bit to enable transmitter in Phillips standard mode"]
pub type I2S_TX_MSB_SHIFT_R = crate::BitReader<bool>;
#[doc = "Field `I2S_TX_MSB_SHIFT` writer - Set this bit to enable transmitter in Phillips standard mode"]
pub type I2S_TX_MSB_SHIFT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_TX_CONF1_SPEC, bool, O>;
#[doc = "Field `I2S_TX_BCK_NO_DLY` reader - 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode."]
pub type I2S_TX_BCK_NO_DLY_R = crate::BitReader<bool>;
#[doc = "Field `I2S_TX_BCK_NO_DLY` writer - 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode."]
pub type I2S_TX_BCK_NO_DLY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_TX_CONF1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - The width of tx_ws_out in TDM mode is (I2S_TX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
    #[inline(always)]
    pub fn i2s_tx_tdm_ws_width(&self) -> I2S_TX_TDM_WS_WIDTH_R {
        I2S_TX_TDM_WS_WIDTH_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:12 - Bit clock configuration bits in transmitter mode."]
    #[inline(always)]
    pub fn i2s_tx_bck_div_num(&self) -> I2S_TX_BCK_DIV_NUM_R {
        I2S_TX_BCK_DIV_NUM_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:17 - Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    #[inline(always)]
    pub fn i2s_tx_bits_mod(&self) -> I2S_TX_BITS_MOD_R {
        I2S_TX_BITS_MOD_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:23 - I2S Tx half sample bits -1."]
    #[inline(always)]
    pub fn i2s_tx_half_sample_bits(&self) -> I2S_TX_HALF_SAMPLE_BITS_R {
        I2S_TX_HALF_SAMPLE_BITS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - The Tx bit number for each channel minus 1in TDM mode."]
    #[inline(always)]
    pub fn i2s_tx_tdm_chan_bits(&self) -> I2S_TX_TDM_CHAN_BITS_R {
        I2S_TX_TDM_CHAN_BITS_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Set this bit to enable transmitter in Phillips standard mode"]
    #[inline(always)]
    pub fn i2s_tx_msb_shift(&self) -> I2S_TX_MSB_SHIFT_R {
        I2S_TX_MSB_SHIFT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode."]
    #[inline(always)]
    pub fn i2s_tx_bck_no_dly(&self) -> I2S_TX_BCK_NO_DLY_R {
        I2S_TX_BCK_NO_DLY_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The width of tx_ws_out in TDM mode is (I2S_TX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_tdm_ws_width(&mut self) -> I2S_TX_TDM_WS_WIDTH_W<0> {
        I2S_TX_TDM_WS_WIDTH_W::new(self)
    }
    #[doc = "Bits 7:12 - Bit clock configuration bits in transmitter mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_bck_div_num(&mut self) -> I2S_TX_BCK_DIV_NUM_W<7> {
        I2S_TX_BCK_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 13:17 - Set the bits to configure the valid data bit length of I2S transmitter channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_bits_mod(&mut self) -> I2S_TX_BITS_MOD_W<13> {
        I2S_TX_BITS_MOD_W::new(self)
    }
    #[doc = "Bits 18:23 - I2S Tx half sample bits -1."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_half_sample_bits(&mut self) -> I2S_TX_HALF_SAMPLE_BITS_W<18> {
        I2S_TX_HALF_SAMPLE_BITS_W::new(self)
    }
    #[doc = "Bits 24:28 - The Tx bit number for each channel minus 1in TDM mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_tdm_chan_bits(&mut self) -> I2S_TX_TDM_CHAN_BITS_W<24> {
        I2S_TX_TDM_CHAN_BITS_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to enable transmitter in Phillips standard mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_msb_shift(&mut self) -> I2S_TX_MSB_SHIFT_W<29> {
        I2S_TX_MSB_SHIFT_W::new(self)
    }
    #[doc = "Bit 30 - 1: BCK is not delayed to generate pos/neg edge in master mode. 0: BCK is delayed to generate pos/neg edge in master mode."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_bck_no_dly(&mut self) -> I2S_TX_BCK_NO_DLY_W<30> {
        I2S_TX_BCK_NO_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX configure register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_conf1](index.html) module"]
pub struct I2S_TX_CONF1_SPEC;
impl crate::RegisterSpec for I2S_TX_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_tx_conf1::R](R) reader structure"]
impl crate::Readable for I2S_TX_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_tx_conf1::W](W) writer structure"]
impl crate::Writable for I2S_TX_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_TX_CONF1 to value 0x6f3d_e300"]
impl crate::Resettable for I2S_TX_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x6f3d_e300;
}
