#[doc = "Register `RX_CONF1` reader"]
pub struct R(crate::R<RX_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CONF1` writer"]
pub struct W(crate::W<RX_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CONF1_SPEC>;
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
impl From<crate::W<RX_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_TDM_WS_WIDTH` reader - The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
pub type RX_TDM_WS_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_TDM_WS_WIDTH` writer - The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
pub type RX_TDM_WS_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_CONF1_SPEC, u8, u8, 7, O>;
#[doc = "Field `RX_BCK_DIV_NUM` reader - Bit clock configuration bits in receiver mode."]
pub type RX_BCK_DIV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_BCK_DIV_NUM` writer - Bit clock configuration bits in receiver mode."]
pub type RX_BCK_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_CONF1_SPEC, u8, u8, 6, O>;
#[doc = "Field `RX_BITS_MOD` reader - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
pub type RX_BITS_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_BITS_MOD` writer - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
pub type RX_BITS_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_CONF1_SPEC, u8, u8, 5, O>;
#[doc = "Field `RX_HALF_SAMPLE_BITS` reader - I2S Rx half sample bits -1."]
pub type RX_HALF_SAMPLE_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_HALF_SAMPLE_BITS` writer - I2S Rx half sample bits -1."]
pub type RX_HALF_SAMPLE_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_CONF1_SPEC, u8, u8, 6, O>;
#[doc = "Field `RX_TDM_CHAN_BITS` reader - The Rx bit number for each channel minus 1in TDM mode."]
pub type RX_TDM_CHAN_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_TDM_CHAN_BITS` writer - The Rx bit number for each channel minus 1in TDM mode."]
pub type RX_TDM_CHAN_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_CONF1_SPEC, u8, u8, 5, O>;
#[doc = "Field `RX_MSB_SHIFT` reader - Set this bit to enable receiver in Phillips standard mode"]
pub type RX_MSB_SHIFT_R = crate::BitReader<bool>;
#[doc = "Field `RX_MSB_SHIFT` writer - Set this bit to enable receiver in Phillips standard mode"]
pub type RX_MSB_SHIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_CONF1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
    #[inline(always)]
    pub fn rx_tdm_ws_width(&self) -> RX_TDM_WS_WIDTH_R {
        RX_TDM_WS_WIDTH_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:12 - Bit clock configuration bits in receiver mode."]
    #[inline(always)]
    pub fn rx_bck_div_num(&self) -> RX_BCK_DIV_NUM_R {
        RX_BCK_DIV_NUM_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:17 - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    #[inline(always)]
    pub fn rx_bits_mod(&self) -> RX_BITS_MOD_R {
        RX_BITS_MOD_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:23 - I2S Rx half sample bits -1."]
    #[inline(always)]
    pub fn rx_half_sample_bits(&self) -> RX_HALF_SAMPLE_BITS_R {
        RX_HALF_SAMPLE_BITS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - The Rx bit number for each channel minus 1in TDM mode."]
    #[inline(always)]
    pub fn rx_tdm_chan_bits(&self) -> RX_TDM_CHAN_BITS_R {
        RX_TDM_CHAN_BITS_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Set this bit to enable receiver in Phillips standard mode"]
    #[inline(always)]
    pub fn rx_msb_shift(&self) -> RX_MSB_SHIFT_R {
        RX_MSB_SHIFT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - The width of rx_ws_out in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[6:0\\] +1) * T_bck"]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_ws_width(&mut self) -> RX_TDM_WS_WIDTH_W<0> {
        RX_TDM_WS_WIDTH_W::new(self)
    }
    #[doc = "Bits 7:12 - Bit clock configuration bits in receiver mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bck_div_num(&mut self) -> RX_BCK_DIV_NUM_W<7> {
        RX_BCK_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 13:17 - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bits_mod(&mut self) -> RX_BITS_MOD_W<13> {
        RX_BITS_MOD_W::new(self)
    }
    #[doc = "Bits 18:23 - I2S Rx half sample bits -1."]
    #[inline(always)]
    #[must_use]
    pub fn rx_half_sample_bits(&mut self) -> RX_HALF_SAMPLE_BITS_W<18> {
        RX_HALF_SAMPLE_BITS_W::new(self)
    }
    #[doc = "Bits 24:28 - The Rx bit number for each channel minus 1in TDM mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_chan_bits(&mut self) -> RX_TDM_CHAN_BITS_W<24> {
        RX_TDM_CHAN_BITS_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to enable receiver in Phillips standard mode"]
    #[inline(always)]
    #[must_use]
    pub fn rx_msb_shift(&mut self) -> RX_MSB_SHIFT_W<29> {
        RX_MSB_SHIFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S RX configure register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_conf1](index.html) module"]
pub struct RX_CONF1_SPEC;
impl crate::RegisterSpec for RX_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_conf1::R](R) reader structure"]
impl crate::Readable for RX_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_conf1::W](W) writer structure"]
impl crate::Writable for RX_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CONF1 to value 0x2f3d_e300"]
impl crate::Resettable for RX_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x2f3d_e300;
}
