#[doc = "Register `TX_PCM2PDM_CONF` reader"]
pub struct R(crate::R<TX_PCM2PDM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PCM2PDM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PCM2PDM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PCM2PDM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_PCM2PDM_CONF` writer"]
pub struct W(crate::W<TX_PCM2PDM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_PCM2PDM_CONF_SPEC>;
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
impl From<crate::W<TX_PCM2PDM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_PCM2PDM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PDM_HP_BYPASS` reader - I2S TX PDM bypass hp filter or not. The option has been removed."]
pub type TX_PDM_HP_BYPASS_R = crate::BitReader;
#[doc = "Field `TX_PDM_HP_BYPASS` writer - I2S TX PDM bypass hp filter or not. The option has been removed."]
pub type TX_PDM_HP_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, TX_PCM2PDM_CONF_SPEC, O>;
#[doc = "Field `TX_PDM_SINC_OSR2` reader - I2S TX PDM OSR2 value"]
pub type TX_PDM_SINC_OSR2_R = crate::FieldReader;
#[doc = "Field `TX_PDM_SINC_OSR2` writer - I2S TX PDM OSR2 value"]
pub type TX_PDM_SINC_OSR2_W<'a, const O: u8> = crate::FieldWriter<'a, TX_PCM2PDM_CONF_SPEC, 4, O>;
#[doc = "Field `TX_PDM_PRESCALE` reader - I2S TX PDM prescale for sigmadelta"]
pub type TX_PDM_PRESCALE_R = crate::FieldReader;
#[doc = "Field `TX_PDM_PRESCALE` writer - I2S TX PDM prescale for sigmadelta"]
pub type TX_PDM_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, TX_PCM2PDM_CONF_SPEC, 8, O>;
#[doc = "Field `TX_PDM_HP_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TX_PDM_HP_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_HP_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TX_PDM_HP_IN_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, TX_PCM2PDM_CONF_SPEC, 2, O>;
#[doc = "Field `TX_PDM_LP_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TX_PDM_LP_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_LP_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TX_PDM_LP_IN_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, TX_PCM2PDM_CONF_SPEC, 2, O>;
#[doc = "Field `TX_PDM_SINC_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TX_PDM_SINC_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_SINC_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TX_PDM_SINC_IN_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, TX_PCM2PDM_CONF_SPEC, 2, O>;
#[doc = "Field `TX_PDM_SIGMADELTA_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TX_PDM_SIGMADELTA_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_SIGMADELTA_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TX_PDM_SIGMADELTA_IN_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, TX_PCM2PDM_CONF_SPEC, 2, O>;
#[doc = "Field `TX_PDM_SIGMADELTA_DITHER2` reader - I2S TX PDM sigmadelta dither2 value"]
pub type TX_PDM_SIGMADELTA_DITHER2_R = crate::BitReader;
#[doc = "Field `TX_PDM_SIGMADELTA_DITHER2` writer - I2S TX PDM sigmadelta dither2 value"]
pub type TX_PDM_SIGMADELTA_DITHER2_W<'a, const O: u8> =
    crate::BitWriter<'a, TX_PCM2PDM_CONF_SPEC, O>;
#[doc = "Field `TX_PDM_SIGMADELTA_DITHER` reader - I2S TX PDM sigmadelta dither value"]
pub type TX_PDM_SIGMADELTA_DITHER_R = crate::BitReader;
#[doc = "Field `TX_PDM_SIGMADELTA_DITHER` writer - I2S TX PDM sigmadelta dither value"]
pub type TX_PDM_SIGMADELTA_DITHER_W<'a, const O: u8> =
    crate::BitWriter<'a, TX_PCM2PDM_CONF_SPEC, O>;
#[doc = "Field `TX_PDM_DAC_2OUT_EN` reader - I2S TX PDM dac mode enable"]
pub type TX_PDM_DAC_2OUT_EN_R = crate::BitReader;
#[doc = "Field `TX_PDM_DAC_2OUT_EN` writer - I2S TX PDM dac mode enable"]
pub type TX_PDM_DAC_2OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, TX_PCM2PDM_CONF_SPEC, O>;
#[doc = "Field `TX_PDM_DAC_MODE_EN` reader - I2S TX PDM dac 2channel enable"]
pub type TX_PDM_DAC_MODE_EN_R = crate::BitReader;
#[doc = "Field `TX_PDM_DAC_MODE_EN` writer - I2S TX PDM dac 2channel enable"]
pub type TX_PDM_DAC_MODE_EN_W<'a, const O: u8> = crate::BitWriter<'a, TX_PCM2PDM_CONF_SPEC, O>;
#[doc = "Field `PCM2PDM_CONV_EN` reader - I2S TX PDM Converter enable"]
pub type PCM2PDM_CONV_EN_R = crate::BitReader;
#[doc = "Field `PCM2PDM_CONV_EN` writer - I2S TX PDM Converter enable"]
pub type PCM2PDM_CONV_EN_W<'a, const O: u8> = crate::BitWriter<'a, TX_PCM2PDM_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - I2S TX PDM bypass hp filter or not. The option has been removed."]
    #[inline(always)]
    pub fn tx_pdm_hp_bypass(&self) -> TX_PDM_HP_BYPASS_R {
        TX_PDM_HP_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - I2S TX PDM OSR2 value"]
    #[inline(always)]
    pub fn tx_pdm_sinc_osr2(&self) -> TX_PDM_SINC_OSR2_R {
        TX_PDM_SINC_OSR2_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:12 - I2S TX PDM prescale for sigmadelta"]
    #[inline(always)]
    pub fn tx_pdm_prescale(&self) -> TX_PDM_PRESCALE_R {
        TX_PDM_PRESCALE_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bits 13:14 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_hp_in_shift(&self) -> TX_PDM_HP_IN_SHIFT_R {
        TX_PDM_HP_IN_SHIFT_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_lp_in_shift(&self) -> TX_PDM_LP_IN_SHIFT_R {
        TX_PDM_LP_IN_SHIFT_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_sinc_in_shift(&self) -> TX_PDM_SINC_IN_SHIFT_R {
        TX_PDM_SINC_IN_SHIFT_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_in_shift(&self) -> TX_PDM_SIGMADELTA_IN_SHIFT_R {
        TX_PDM_SIGMADELTA_IN_SHIFT_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - I2S TX PDM sigmadelta dither2 value"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither2(&self) -> TX_PDM_SIGMADELTA_DITHER2_R {
        TX_PDM_SIGMADELTA_DITHER2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2S TX PDM sigmadelta dither value"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither(&self) -> TX_PDM_SIGMADELTA_DITHER_R {
        TX_PDM_SIGMADELTA_DITHER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2S TX PDM dac mode enable"]
    #[inline(always)]
    pub fn tx_pdm_dac_2out_en(&self) -> TX_PDM_DAC_2OUT_EN_R {
        TX_PDM_DAC_2OUT_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2S TX PDM dac 2channel enable"]
    #[inline(always)]
    pub fn tx_pdm_dac_mode_en(&self) -> TX_PDM_DAC_MODE_EN_R {
        TX_PDM_DAC_MODE_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2S TX PDM Converter enable"]
    #[inline(always)]
    pub fn pcm2pdm_conv_en(&self) -> PCM2PDM_CONV_EN_R {
        PCM2PDM_CONV_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_PCM2PDM_CONF")
            .field(
                "tx_pdm_hp_bypass",
                &format_args!("{}", self.tx_pdm_hp_bypass().bit()),
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
                "tx_pdm_sigmadelta_dither2",
                &format_args!("{}", self.tx_pdm_sigmadelta_dither2().bit()),
            )
            .field(
                "tx_pdm_sigmadelta_dither",
                &format_args!("{}", self.tx_pdm_sigmadelta_dither().bit()),
            )
            .field(
                "tx_pdm_dac_2out_en",
                &format_args!("{}", self.tx_pdm_dac_2out_en().bit()),
            )
            .field(
                "tx_pdm_dac_mode_en",
                &format_args!("{}", self.tx_pdm_dac_mode_en().bit()),
            )
            .field(
                "pcm2pdm_conv_en",
                &format_args!("{}", self.pcm2pdm_conv_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_PCM2PDM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - I2S TX PDM bypass hp filter or not. The option has been removed."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_hp_bypass(&mut self) -> TX_PDM_HP_BYPASS_W<0> {
        TX_PDM_HP_BYPASS_W::new(self)
    }
    #[doc = "Bits 1:4 - I2S TX PDM OSR2 value"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sinc_osr2(&mut self) -> TX_PDM_SINC_OSR2_W<1> {
        TX_PDM_SINC_OSR2_W::new(self)
    }
    #[doc = "Bits 5:12 - I2S TX PDM prescale for sigmadelta"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_prescale(&mut self) -> TX_PDM_PRESCALE_W<5> {
        TX_PDM_PRESCALE_W::new(self)
    }
    #[doc = "Bits 13:14 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_hp_in_shift(&mut self) -> TX_PDM_HP_IN_SHIFT_W<13> {
        TX_PDM_HP_IN_SHIFT_W::new(self)
    }
    #[doc = "Bits 15:16 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_lp_in_shift(&mut self) -> TX_PDM_LP_IN_SHIFT_W<15> {
        TX_PDM_LP_IN_SHIFT_W::new(self)
    }
    #[doc = "Bits 17:18 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sinc_in_shift(&mut self) -> TX_PDM_SINC_IN_SHIFT_W<17> {
        TX_PDM_SINC_IN_SHIFT_W::new(self)
    }
    #[doc = "Bits 19:20 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sigmadelta_in_shift(&mut self) -> TX_PDM_SIGMADELTA_IN_SHIFT_W<19> {
        TX_PDM_SIGMADELTA_IN_SHIFT_W::new(self)
    }
    #[doc = "Bit 21 - I2S TX PDM sigmadelta dither2 value"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sigmadelta_dither2(&mut self) -> TX_PDM_SIGMADELTA_DITHER2_W<21> {
        TX_PDM_SIGMADELTA_DITHER2_W::new(self)
    }
    #[doc = "Bit 22 - I2S TX PDM sigmadelta dither value"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sigmadelta_dither(&mut self) -> TX_PDM_SIGMADELTA_DITHER_W<22> {
        TX_PDM_SIGMADELTA_DITHER_W::new(self)
    }
    #[doc = "Bit 23 - I2S TX PDM dac mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_dac_2out_en(&mut self) -> TX_PDM_DAC_2OUT_EN_W<23> {
        TX_PDM_DAC_2OUT_EN_W::new(self)
    }
    #[doc = "Bit 24 - I2S TX PDM dac 2channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_dac_mode_en(&mut self) -> TX_PDM_DAC_MODE_EN_W<24> {
        TX_PDM_DAC_MODE_EN_W::new(self)
    }
    #[doc = "Bit 25 - I2S TX PDM Converter enable"]
    #[inline(always)]
    #[must_use]
    pub fn pcm2pdm_conv_en(&mut self) -> PCM2PDM_CONV_EN_W<25> {
        PCM2PDM_CONV_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX PCM2PDM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_pcm2pdm_conf](index.html) module"]
pub struct TX_PCM2PDM_CONF_SPEC;
impl crate::RegisterSpec for TX_PCM2PDM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_pcm2pdm_conf::R](R) reader structure"]
impl crate::Readable for TX_PCM2PDM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_pcm2pdm_conf::W](W) writer structure"]
impl crate::Writable for TX_PCM2PDM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_PCM2PDM_CONF to value 0x004a_a004"]
impl crate::Resettable for TX_PCM2PDM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x004a_a004;
}
