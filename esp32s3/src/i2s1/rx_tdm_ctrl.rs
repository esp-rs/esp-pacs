#[doc = "Register `RX_TDM_CTRL` reader"]
pub struct R(crate::R<RX_TDM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_TDM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_TDM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_TDM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_TDM_CTRL` writer"]
pub struct W(crate::W<RX_TDM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_TDM_CTRL_SPEC>;
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
impl From<crate::W<RX_TDM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_TDM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_TDM_PDM_CHAN0_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN0_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_PDM_CHAN0_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_PDM_CHAN1_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN1_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_PDM_CHAN1_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_PDM_CHAN2_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 2. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN2_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_PDM_CHAN2_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 2. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN2_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_PDM_CHAN3_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 3. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN3_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_PDM_CHAN3_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 3. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN3_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_PDM_CHAN4_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 4. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN4_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_PDM_CHAN4_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 4. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN4_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_PDM_CHAN5_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 5. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN5_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_PDM_CHAN5_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 5. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN5_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_PDM_CHAN6_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 6. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN6_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_PDM_CHAN6_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 6. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN6_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_PDM_CHAN7_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 7. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN7_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_PDM_CHAN7_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 7. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_PDM_CHAN7_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_CHAN8_EN` reader - 1: Enable the valid data input of I2S RX TDM channel 8. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN8_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_CHAN8_EN` writer - 1: Enable the valid data input of I2S RX TDM channel 8. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN8_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_CHAN9_EN` reader - 1: Enable the valid data input of I2S RX TDM channel 9. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN9_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_CHAN9_EN` writer - 1: Enable the valid data input of I2S RX TDM channel 9. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN9_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_CHAN10_EN` reader - 1: Enable the valid data input of I2S RX TDM channel 10. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN10_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_CHAN10_EN` writer - 1: Enable the valid data input of I2S RX TDM channel 10. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN10_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_CHAN11_EN` reader - 1: Enable the valid data input of I2S RX TDM channel 11. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN11_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_CHAN11_EN` writer - 1: Enable the valid data input of I2S RX TDM channel 11. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN11_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_CHAN12_EN` reader - 1: Enable the valid data input of I2S RX TDM channel 12. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN12_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_CHAN12_EN` writer - 1: Enable the valid data input of I2S RX TDM channel 12. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN12_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_CHAN13_EN` reader - 1: Enable the valid data input of I2S RX TDM channel 13. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN13_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_CHAN13_EN` writer - 1: Enable the valid data input of I2S RX TDM channel 13. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN13_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_CHAN14_EN` reader - 1: Enable the valid data input of I2S RX TDM channel 14. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN14_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_CHAN14_EN` writer - 1: Enable the valid data input of I2S RX TDM channel 14. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN14_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_CHAN15_EN` reader - 1: Enable the valid data input of I2S RX TDM channel 15. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN15_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_TDM_CHAN15_EN` writer - 1: Enable the valid data input of I2S RX TDM channel 15. 0: Disable, just input 0 in this channel."]
pub type RX_TDM_CHAN15_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_TDM_CTRL_SPEC, bool, O>;
#[doc = "Field `RX_TDM_TOT_CHAN_NUM` reader - The total channel number of I2S TX TDM mode."]
pub type RX_TDM_TOT_CHAN_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_TDM_TOT_CHAN_NUM` writer - The total channel number of I2S TX TDM mode."]
pub type RX_TDM_TOT_CHAN_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_TDM_CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan0_en(&self) -> RX_TDM_PDM_CHAN0_EN_R {
        RX_TDM_PDM_CHAN0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan1_en(&self) -> RX_TDM_PDM_CHAN1_EN_R {
        RX_TDM_PDM_CHAN1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Enable the valid data input of I2S RX TDM or PDM channel 2. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan2_en(&self) -> RX_TDM_PDM_CHAN2_EN_R {
        RX_TDM_PDM_CHAN2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Enable the valid data input of I2S RX TDM or PDM channel 3. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan3_en(&self) -> RX_TDM_PDM_CHAN3_EN_R {
        RX_TDM_PDM_CHAN3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Enable the valid data input of I2S RX TDM or PDM channel 4. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan4_en(&self) -> RX_TDM_PDM_CHAN4_EN_R {
        RX_TDM_PDM_CHAN4_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Enable the valid data input of I2S RX TDM or PDM channel 5. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan5_en(&self) -> RX_TDM_PDM_CHAN5_EN_R {
        RX_TDM_PDM_CHAN5_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Enable the valid data input of I2S RX TDM or PDM channel 6. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan6_en(&self) -> RX_TDM_PDM_CHAN6_EN_R {
        RX_TDM_PDM_CHAN6_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Enable the valid data input of I2S RX TDM or PDM channel 7. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan7_en(&self) -> RX_TDM_PDM_CHAN7_EN_R {
        RX_TDM_PDM_CHAN7_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: Enable the valid data input of I2S RX TDM channel 8. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_chan8_en(&self) -> RX_TDM_CHAN8_EN_R {
        RX_TDM_CHAN8_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: Enable the valid data input of I2S RX TDM channel 9. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_chan9_en(&self) -> RX_TDM_CHAN9_EN_R {
        RX_TDM_CHAN9_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: Enable the valid data input of I2S RX TDM channel 10. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_chan10_en(&self) -> RX_TDM_CHAN10_EN_R {
        RX_TDM_CHAN10_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Enable the valid data input of I2S RX TDM channel 11. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_chan11_en(&self) -> RX_TDM_CHAN11_EN_R {
        RX_TDM_CHAN11_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: Enable the valid data input of I2S RX TDM channel 12. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_chan12_en(&self) -> RX_TDM_CHAN12_EN_R {
        RX_TDM_CHAN12_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Enable the valid data input of I2S RX TDM channel 13. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_chan13_en(&self) -> RX_TDM_CHAN13_EN_R {
        RX_TDM_CHAN13_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1: Enable the valid data input of I2S RX TDM channel 14. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_chan14_en(&self) -> RX_TDM_CHAN14_EN_R {
        RX_TDM_CHAN14_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Enable the valid data input of I2S RX TDM channel 15. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_chan15_en(&self) -> RX_TDM_CHAN15_EN_R {
        RX_TDM_CHAN15_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - The total channel number of I2S TX TDM mode."]
    #[inline(always)]
    pub fn rx_tdm_tot_chan_num(&self) -> RX_TDM_TOT_CHAN_NUM_R {
        RX_TDM_TOT_CHAN_NUM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_pdm_chan0_en(&mut self) -> RX_TDM_PDM_CHAN0_EN_W<0> {
        RX_TDM_PDM_CHAN0_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_pdm_chan1_en(&mut self) -> RX_TDM_PDM_CHAN1_EN_W<1> {
        RX_TDM_PDM_CHAN1_EN_W::new(self)
    }
    #[doc = "Bit 2 - 1: Enable the valid data input of I2S RX TDM or PDM channel 2. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_pdm_chan2_en(&mut self) -> RX_TDM_PDM_CHAN2_EN_W<2> {
        RX_TDM_PDM_CHAN2_EN_W::new(self)
    }
    #[doc = "Bit 3 - 1: Enable the valid data input of I2S RX TDM or PDM channel 3. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_pdm_chan3_en(&mut self) -> RX_TDM_PDM_CHAN3_EN_W<3> {
        RX_TDM_PDM_CHAN3_EN_W::new(self)
    }
    #[doc = "Bit 4 - 1: Enable the valid data input of I2S RX TDM or PDM channel 4. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_pdm_chan4_en(&mut self) -> RX_TDM_PDM_CHAN4_EN_W<4> {
        RX_TDM_PDM_CHAN4_EN_W::new(self)
    }
    #[doc = "Bit 5 - 1: Enable the valid data input of I2S RX TDM or PDM channel 5. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_pdm_chan5_en(&mut self) -> RX_TDM_PDM_CHAN5_EN_W<5> {
        RX_TDM_PDM_CHAN5_EN_W::new(self)
    }
    #[doc = "Bit 6 - 1: Enable the valid data input of I2S RX TDM or PDM channel 6. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_pdm_chan6_en(&mut self) -> RX_TDM_PDM_CHAN6_EN_W<6> {
        RX_TDM_PDM_CHAN6_EN_W::new(self)
    }
    #[doc = "Bit 7 - 1: Enable the valid data input of I2S RX TDM or PDM channel 7. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_pdm_chan7_en(&mut self) -> RX_TDM_PDM_CHAN7_EN_W<7> {
        RX_TDM_PDM_CHAN7_EN_W::new(self)
    }
    #[doc = "Bit 8 - 1: Enable the valid data input of I2S RX TDM channel 8. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_chan8_en(&mut self) -> RX_TDM_CHAN8_EN_W<8> {
        RX_TDM_CHAN8_EN_W::new(self)
    }
    #[doc = "Bit 9 - 1: Enable the valid data input of I2S RX TDM channel 9. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_chan9_en(&mut self) -> RX_TDM_CHAN9_EN_W<9> {
        RX_TDM_CHAN9_EN_W::new(self)
    }
    #[doc = "Bit 10 - 1: Enable the valid data input of I2S RX TDM channel 10. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_chan10_en(&mut self) -> RX_TDM_CHAN10_EN_W<10> {
        RX_TDM_CHAN10_EN_W::new(self)
    }
    #[doc = "Bit 11 - 1: Enable the valid data input of I2S RX TDM channel 11. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_chan11_en(&mut self) -> RX_TDM_CHAN11_EN_W<11> {
        RX_TDM_CHAN11_EN_W::new(self)
    }
    #[doc = "Bit 12 - 1: Enable the valid data input of I2S RX TDM channel 12. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_chan12_en(&mut self) -> RX_TDM_CHAN12_EN_W<12> {
        RX_TDM_CHAN12_EN_W::new(self)
    }
    #[doc = "Bit 13 - 1: Enable the valid data input of I2S RX TDM channel 13. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_chan13_en(&mut self) -> RX_TDM_CHAN13_EN_W<13> {
        RX_TDM_CHAN13_EN_W::new(self)
    }
    #[doc = "Bit 14 - 1: Enable the valid data input of I2S RX TDM channel 14. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_chan14_en(&mut self) -> RX_TDM_CHAN14_EN_W<14> {
        RX_TDM_CHAN14_EN_W::new(self)
    }
    #[doc = "Bit 15 - 1: Enable the valid data input of I2S RX TDM channel 15. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_chan15_en(&mut self) -> RX_TDM_CHAN15_EN_W<15> {
        RX_TDM_CHAN15_EN_W::new(self)
    }
    #[doc = "Bits 16:19 - The total channel number of I2S TX TDM mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_tdm_tot_chan_num(&mut self) -> RX_TDM_TOT_CHAN_NUM_W<16> {
        RX_TDM_TOT_CHAN_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX TDM mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_tdm_ctrl](index.html) module"]
pub struct RX_TDM_CTRL_SPEC;
impl crate::RegisterSpec for RX_TDM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_tdm_ctrl::R](R) reader structure"]
impl crate::Readable for RX_TDM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_tdm_ctrl::W](W) writer structure"]
impl crate::Writable for RX_TDM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_TDM_CTRL to value 0xffff"]
impl crate::Resettable for RX_TDM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
