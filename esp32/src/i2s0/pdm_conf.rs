#[doc = "Register `PDM_CONF` reader"]
pub struct R(crate::R<PDM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDM_CONF` writer"]
pub struct W(crate::W<PDM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_CONF_SPEC>;
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
impl From<crate::W<PDM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PDM_EN` reader - "]
pub type TX_PDM_EN_R = crate::BitReader;
#[doc = "Field `TX_PDM_EN` writer - "]
pub type TX_PDM_EN_W<'a, const O: u8> = crate::BitWriter<'a, PDM_CONF_SPEC, O>;
#[doc = "Field `RX_PDM_EN` reader - "]
pub type RX_PDM_EN_R = crate::BitReader;
#[doc = "Field `RX_PDM_EN` writer - "]
pub type RX_PDM_EN_W<'a, const O: u8> = crate::BitWriter<'a, PDM_CONF_SPEC, O>;
#[doc = "Field `PCM2PDM_CONV_EN` reader - "]
pub type PCM2PDM_CONV_EN_R = crate::BitReader;
#[doc = "Field `PCM2PDM_CONV_EN` writer - "]
pub type PCM2PDM_CONV_EN_W<'a, const O: u8> = crate::BitWriter<'a, PDM_CONF_SPEC, O>;
#[doc = "Field `PDM2PCM_CONV_EN` reader - "]
pub type PDM2PCM_CONV_EN_R = crate::BitReader;
#[doc = "Field `PDM2PCM_CONV_EN` writer - "]
pub type PDM2PCM_CONV_EN_W<'a, const O: u8> = crate::BitWriter<'a, PDM_CONF_SPEC, O>;
#[doc = "Field `TX_PDM_SINC_OSR2` reader - "]
pub type TX_PDM_SINC_OSR2_R = crate::FieldReader;
#[doc = "Field `TX_PDM_SINC_OSR2` writer - "]
pub type TX_PDM_SINC_OSR2_W<'a, const O: u8> = crate::FieldWriter<'a, PDM_CONF_SPEC, 4, O>;
#[doc = "Field `TX_PDM_PRESCALE` reader - "]
pub type TX_PDM_PRESCALE_R = crate::FieldReader;
#[doc = "Field `TX_PDM_PRESCALE` writer - "]
pub type TX_PDM_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, PDM_CONF_SPEC, 8, O>;
#[doc = "Field `TX_PDM_HP_IN_SHIFT` reader - "]
pub type TX_PDM_HP_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_HP_IN_SHIFT` writer - "]
pub type TX_PDM_HP_IN_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, PDM_CONF_SPEC, 2, O>;
#[doc = "Field `TX_PDM_LP_IN_SHIFT` reader - "]
pub type TX_PDM_LP_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_LP_IN_SHIFT` writer - "]
pub type TX_PDM_LP_IN_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, PDM_CONF_SPEC, 2, O>;
#[doc = "Field `TX_PDM_SINC_IN_SHIFT` reader - "]
pub type TX_PDM_SINC_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_SINC_IN_SHIFT` writer - "]
pub type TX_PDM_SINC_IN_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, PDM_CONF_SPEC, 2, O>;
#[doc = "Field `TX_PDM_SIGMADELTA_IN_SHIFT` reader - "]
pub type TX_PDM_SIGMADELTA_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_SIGMADELTA_IN_SHIFT` writer - "]
pub type TX_PDM_SIGMADELTA_IN_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, PDM_CONF_SPEC, 2, O>;
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` reader - "]
pub type RX_PDM_SINC_DSR_16_EN_R = crate::BitReader;
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` writer - "]
pub type RX_PDM_SINC_DSR_16_EN_W<'a, const O: u8> = crate::BitWriter<'a, PDM_CONF_SPEC, O>;
#[doc = "Field `TX_PDM_HP_BYPASS` reader - "]
pub type TX_PDM_HP_BYPASS_R = crate::BitReader;
#[doc = "Field `TX_PDM_HP_BYPASS` writer - "]
pub type TX_PDM_HP_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, PDM_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_pdm_en(&self) -> TX_PDM_EN_R {
        TX_PDM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_pdm_en(&self) -> RX_PDM_EN_R {
        RX_PDM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pcm2pdm_conv_en(&self) -> PCM2PDM_CONV_EN_R {
        PCM2PDM_CONV_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdm2pcm_conv_en(&self) -> PDM2PCM_CONV_EN_R {
        PDM2PCM_CONV_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn tx_pdm_sinc_osr2(&self) -> TX_PDM_SINC_OSR2_R {
        TX_PDM_SINC_OSR2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tx_pdm_prescale(&self) -> TX_PDM_PRESCALE_R {
        TX_PDM_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tx_pdm_hp_in_shift(&self) -> TX_PDM_HP_IN_SHIFT_R {
        TX_PDM_HP_IN_SHIFT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn tx_pdm_lp_in_shift(&self) -> TX_PDM_LP_IN_SHIFT_R {
        TX_PDM_LP_IN_SHIFT_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tx_pdm_sinc_in_shift(&self) -> TX_PDM_SINC_IN_SHIFT_R {
        TX_PDM_SINC_IN_SHIFT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_in_shift(&self) -> TX_PDM_SIGMADELTA_IN_SHIFT_R {
        TX_PDM_SIGMADELTA_IN_SHIFT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rx_pdm_sinc_dsr_16_en(&self) -> RX_PDM_SINC_DSR_16_EN_R {
        RX_PDM_SINC_DSR_16_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tx_pdm_hp_bypass(&self) -> TX_PDM_HP_BYPASS_R {
        TX_PDM_HP_BYPASS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDM_CONF")
            .field("tx_pdm_en", &format_args!("{}", self.tx_pdm_en().bit()))
            .field("rx_pdm_en", &format_args!("{}", self.rx_pdm_en().bit()))
            .field(
                "pcm2pdm_conv_en",
                &format_args!("{}", self.pcm2pdm_conv_en().bit()),
            )
            .field(
                "pdm2pcm_conv_en",
                &format_args!("{}", self.pdm2pcm_conv_en().bit()),
            )
            .field(
                "tx_pdm_sinc_osr2",
                &format_args!("{}", self.tx_pdm_sinc_osr2().bits()),
            )
            .field(
                "tx_pdm_prescale",
                &format_args!("{}", self.tx_pdm_prescale().bits()),
            )
            .field(
                "tx_pdm_hp_in_shift",
                &format_args!("{}", self.tx_pdm_hp_in_shift().bits()),
            )
            .field(
                "tx_pdm_lp_in_shift",
                &format_args!("{}", self.tx_pdm_lp_in_shift().bits()),
            )
            .field(
                "tx_pdm_sinc_in_shift",
                &format_args!("{}", self.tx_pdm_sinc_in_shift().bits()),
            )
            .field(
                "tx_pdm_sigmadelta_in_shift",
                &format_args!("{}", self.tx_pdm_sigmadelta_in_shift().bits()),
            )
            .field(
                "rx_pdm_sinc_dsr_16_en",
                &format_args!("{}", self.rx_pdm_sinc_dsr_16_en().bit()),
            )
            .field(
                "tx_pdm_hp_bypass",
                &format_args!("{}", self.tx_pdm_hp_bypass().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PDM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_en(&mut self) -> TX_PDM_EN_W<0> {
        TX_PDM_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pdm_en(&mut self) -> RX_PDM_EN_W<1> {
        RX_PDM_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcm2pdm_conv_en(&mut self) -> PCM2PDM_CONV_EN_W<2> {
        PCM2PDM_CONV_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pdm2pcm_conv_en(&mut self) -> PDM2PCM_CONV_EN_W<3> {
        PDM2PCM_CONV_EN_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sinc_osr2(&mut self) -> TX_PDM_SINC_OSR2_W<4> {
        TX_PDM_SINC_OSR2_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_prescale(&mut self) -> TX_PDM_PRESCALE_W<8> {
        TX_PDM_PRESCALE_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_hp_in_shift(&mut self) -> TX_PDM_HP_IN_SHIFT_W<16> {
        TX_PDM_HP_IN_SHIFT_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_lp_in_shift(&mut self) -> TX_PDM_LP_IN_SHIFT_W<18> {
        TX_PDM_LP_IN_SHIFT_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sinc_in_shift(&mut self) -> TX_PDM_SINC_IN_SHIFT_W<20> {
        TX_PDM_SINC_IN_SHIFT_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sigmadelta_in_shift(&mut self) -> TX_PDM_SIGMADELTA_IN_SHIFT_W<22> {
        TX_PDM_SIGMADELTA_IN_SHIFT_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pdm_sinc_dsr_16_en(&mut self) -> RX_PDM_SINC_DSR_16_EN_W<24> {
        RX_PDM_SINC_DSR_16_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_hp_bypass(&mut self) -> TX_PDM_HP_BYPASS_W<25> {
        TX_PDM_HP_BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_conf](index.html) module"]
pub struct PDM_CONF_SPEC;
impl crate::RegisterSpec for PDM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_conf::R](R) reader structure"]
impl crate::Readable for PDM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_conf::W](W) writer structure"]
impl crate::Writable for PDM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDM_CONF to value 0x0155_0020"]
impl crate::Resettable for PDM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0155_0020;
}
