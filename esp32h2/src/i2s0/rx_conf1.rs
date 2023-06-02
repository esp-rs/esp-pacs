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
#[doc = "Field `RX_TDM_WS_WIDTH` reader - The width of rx_ws_out at idle level in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[8:0\\] +1) * T_bck"]
pub type RX_TDM_WS_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_TDM_WS_WIDTH` writer - The width of rx_ws_out at idle level in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[8:0\\] +1) * T_bck"]
pub type RX_TDM_WS_WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CONF1_SPEC, 9, O, u16, u16>;
#[doc = "Field `RX_BITS_MOD` reader - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
pub type RX_BITS_MOD_R = crate::FieldReader;
#[doc = "Field `RX_BITS_MOD` writer - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
pub type RX_BITS_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CONF1_SPEC, 5, O>;
#[doc = "Field `RX_HALF_SAMPLE_BITS` reader - I2S Rx half sample bits -1."]
pub type RX_HALF_SAMPLE_BITS_R = crate::FieldReader;
#[doc = "Field `RX_HALF_SAMPLE_BITS` writer - I2S Rx half sample bits -1."]
pub type RX_HALF_SAMPLE_BITS_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CONF1_SPEC, 8, O>;
#[doc = "Field `RX_TDM_CHAN_BITS` reader - The Rx bit number for each channel minus 1in TDM mode."]
pub type RX_TDM_CHAN_BITS_R = crate::FieldReader;
#[doc = "Field `RX_TDM_CHAN_BITS` writer - The Rx bit number for each channel minus 1in TDM mode."]
pub type RX_TDM_CHAN_BITS_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CONF1_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:8 - The width of rx_ws_out at idle level in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[8:0\\] +1) * T_bck"]
    #[inline(always)]
    pub fn rx_tdm_ws_width(&self) -> RX_TDM_WS_WIDTH_R {
        RX_TDM_WS_WIDTH_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 14:18 - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    #[inline(always)]
    pub fn rx_bits_mod(&self) -> RX_BITS_MOD_R {
        RX_BITS_MOD_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:26 - I2S Rx half sample bits -1."]
    #[inline(always)]
    pub fn rx_half_sample_bits(&self) -> RX_HALF_SAMPLE_BITS_R {
        RX_HALF_SAMPLE_BITS_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bits 27:31 - The Rx bit number for each channel minus 1in TDM mode."]
    #[inline(always)]
    pub fn rx_tdm_chan_bits(&self) -> RX_TDM_CHAN_BITS_R {
        RX_TDM_CHAN_BITS_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CONF1")
            .field(
                "rx_tdm_ws_width",
                &format_args!("{}", self.rx_tdm_ws_width().bits()),
            )
            .field(
                "rx_bits_mod",
                &format_args!("{}", self.rx_bits_mod().bits()),
            )
            .field(
                "rx_half_sample_bits",
                &format_args!("{}", self.rx_half_sample_bits().bits()),
            )
            .field(
                "rx_tdm_chan_bits",
                &format_args!("{}", self.rx_tdm_chan_bits().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - The width of rx_ws_out at idle level in TDM mode is (I2S_RX_TDM_WS_WIDTH\\[8:0\\] +1) * T_bck"]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_ws_width(&mut self) -> RX_TDM_WS_WIDTH_W<0> {
        RX_TDM_WS_WIDTH_W::new(self)
    }
    #[doc = "Bits 14:18 - Set the bits to configure the valid data bit length of I2S receiver channel. 7: all the valid channel data is in 8-bit-mode. 15: all the valid channel data is in 16-bit-mode. 23: all the valid channel data is in 24-bit-mode. 31:all the valid channel data is in 32-bit-mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bits_mod(&mut self) -> RX_BITS_MOD_W<14> {
        RX_BITS_MOD_W::new(self)
    }
    #[doc = "Bits 19:26 - I2S Rx half sample bits -1."]
    #[inline(always)]
    #[must_use]
    pub fn rx_half_sample_bits(&mut self) -> RX_HALF_SAMPLE_BITS_W<19> {
        RX_HALF_SAMPLE_BITS_W::new(self)
    }
    #[doc = "Bits 27:31 - The Rx bit number for each channel minus 1in TDM mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_chan_bits(&mut self) -> RX_TDM_CHAN_BITS_W<27> {
        RX_TDM_CHAN_BITS_W::new(self)
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
#[doc = "`reset()` method sets RX_CONF1 to value 0x787b_c000"]
impl crate::Resettable for RX_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x787b_c000;
}
