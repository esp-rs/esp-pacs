///Register `TX_PCM2PDM_CONF` reader
pub type R = crate::R<TX_PCM2PDM_CONF_SPEC>;
///Register `TX_PCM2PDM_CONF` writer
pub type W = crate::W<TX_PCM2PDM_CONF_SPEC>;
///Field `TX_PDM_HP_BYPASS` reader - I2S TX PDM bypass hp filter or not. The option has been removed.
pub type TX_PDM_HP_BYPASS_R = crate::BitReader;
///Field `TX_PDM_HP_BYPASS` writer - I2S TX PDM bypass hp filter or not. The option has been removed.
pub type TX_PDM_HP_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_PDM_SINC_OSR2` reader - I2S TX PDM OSR2 value
pub type TX_PDM_SINC_OSR2_R = crate::FieldReader;
///Field `TX_PDM_SINC_OSR2` writer - I2S TX PDM OSR2 value
pub type TX_PDM_SINC_OSR2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TX_PDM_PRESCALE` reader - I2S TX PDM prescale for sigmadelta
pub type TX_PDM_PRESCALE_R = crate::FieldReader;
///Field `TX_PDM_PRESCALE` writer - I2S TX PDM prescale for sigmadelta
pub type TX_PDM_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TX_PDM_HP_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
pub type TX_PDM_HP_IN_SHIFT_R = crate::FieldReader;
///Field `TX_PDM_HP_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
pub type TX_PDM_HP_IN_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TX_PDM_LP_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
pub type TX_PDM_LP_IN_SHIFT_R = crate::FieldReader;
///Field `TX_PDM_LP_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
pub type TX_PDM_LP_IN_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TX_PDM_SINC_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
pub type TX_PDM_SINC_IN_SHIFT_R = crate::FieldReader;
///Field `TX_PDM_SINC_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
pub type TX_PDM_SINC_IN_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TX_PDM_SIGMADELTA_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
pub type TX_PDM_SIGMADELTA_IN_SHIFT_R = crate::FieldReader;
///Field `TX_PDM_SIGMADELTA_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
pub type TX_PDM_SIGMADELTA_IN_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TX_PDM_SIGMADELTA_DITHER2` reader - I2S TX PDM sigmadelta dither2 value
pub type TX_PDM_SIGMADELTA_DITHER2_R = crate::BitReader;
///Field `TX_PDM_SIGMADELTA_DITHER2` writer - I2S TX PDM sigmadelta dither2 value
pub type TX_PDM_SIGMADELTA_DITHER2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_PDM_SIGMADELTA_DITHER` reader - I2S TX PDM sigmadelta dither value
pub type TX_PDM_SIGMADELTA_DITHER_R = crate::BitReader;
///Field `TX_PDM_SIGMADELTA_DITHER` writer - I2S TX PDM sigmadelta dither value
pub type TX_PDM_SIGMADELTA_DITHER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_PDM_DAC_2OUT_EN` reader - I2S TX PDM dac mode enable
pub type TX_PDM_DAC_2OUT_EN_R = crate::BitReader;
///Field `TX_PDM_DAC_2OUT_EN` writer - I2S TX PDM dac mode enable
pub type TX_PDM_DAC_2OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_PDM_DAC_MODE_EN` reader - I2S TX PDM dac 2channel enable
pub type TX_PDM_DAC_MODE_EN_R = crate::BitReader;
///Field `TX_PDM_DAC_MODE_EN` writer - I2S TX PDM dac 2channel enable
pub type TX_PDM_DAC_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCM2PDM_CONV_EN` reader - I2S TX PDM Converter enable
pub type PCM2PDM_CONV_EN_R = crate::BitReader;
///Field `PCM2PDM_CONV_EN` writer - I2S TX PDM Converter enable
pub type PCM2PDM_CONV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2S TX PDM bypass hp filter or not. The option has been removed.
    #[inline(always)]
    pub fn tx_pdm_hp_bypass(&self) -> TX_PDM_HP_BYPASS_R {
        TX_PDM_HP_BYPASS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - I2S TX PDM OSR2 value
    #[inline(always)]
    pub fn tx_pdm_sinc_osr2(&self) -> TX_PDM_SINC_OSR2_R {
        TX_PDM_SINC_OSR2_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bits 5:12 - I2S TX PDM prescale for sigmadelta
    #[inline(always)]
    pub fn tx_pdm_prescale(&self) -> TX_PDM_PRESCALE_R {
        TX_PDM_PRESCALE_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    ///Bits 13:14 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn tx_pdm_hp_in_shift(&self) -> TX_PDM_HP_IN_SHIFT_R {
        TX_PDM_HP_IN_SHIFT_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bits 15:16 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn tx_pdm_lp_in_shift(&self) -> TX_PDM_LP_IN_SHIFT_R {
        TX_PDM_LP_IN_SHIFT_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bits 17:18 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn tx_pdm_sinc_in_shift(&self) -> TX_PDM_SINC_IN_SHIFT_R {
        TX_PDM_SINC_IN_SHIFT_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:20 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_in_shift(&self) -> TX_PDM_SIGMADELTA_IN_SHIFT_R {
        TX_PDM_SIGMADELTA_IN_SHIFT_R::new(((self.bits >> 19) & 3) as u8)
    }
    ///Bit 21 - I2S TX PDM sigmadelta dither2 value
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither2(&self) -> TX_PDM_SIGMADELTA_DITHER2_R {
        TX_PDM_SIGMADELTA_DITHER2_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2S TX PDM sigmadelta dither value
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither(&self) -> TX_PDM_SIGMADELTA_DITHER_R {
        TX_PDM_SIGMADELTA_DITHER_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2S TX PDM dac mode enable
    #[inline(always)]
    pub fn tx_pdm_dac_2out_en(&self) -> TX_PDM_DAC_2OUT_EN_R {
        TX_PDM_DAC_2OUT_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - I2S TX PDM dac 2channel enable
    #[inline(always)]
    pub fn tx_pdm_dac_mode_en(&self) -> TX_PDM_DAC_MODE_EN_R {
        TX_PDM_DAC_MODE_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - I2S TX PDM Converter enable
    #[inline(always)]
    pub fn pcm2pdm_conv_en(&self) -> PCM2PDM_CONV_EN_R {
        PCM2PDM_CONV_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_PCM2PDM_CONF")
            .field("tx_pdm_hp_bypass", &self.tx_pdm_hp_bypass())
            .field("tx_pdm_sinc_osr2", &self.tx_pdm_sinc_osr2())
            .field("tx_pdm_prescale", &self.tx_pdm_prescale())
            .field("tx_pdm_hp_in_shift", &self.tx_pdm_hp_in_shift())
            .field("tx_pdm_lp_in_shift", &self.tx_pdm_lp_in_shift())
            .field("tx_pdm_sinc_in_shift", &self.tx_pdm_sinc_in_shift())
            .field(
                "tx_pdm_sigmadelta_in_shift",
                &self.tx_pdm_sigmadelta_in_shift(),
            )
            .field(
                "tx_pdm_sigmadelta_dither2",
                &self.tx_pdm_sigmadelta_dither2(),
            )
            .field("tx_pdm_sigmadelta_dither", &self.tx_pdm_sigmadelta_dither())
            .field("tx_pdm_dac_2out_en", &self.tx_pdm_dac_2out_en())
            .field("tx_pdm_dac_mode_en", &self.tx_pdm_dac_mode_en())
            .field("pcm2pdm_conv_en", &self.pcm2pdm_conv_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - I2S TX PDM bypass hp filter or not. The option has been removed.
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_hp_bypass(&mut self) -> TX_PDM_HP_BYPASS_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_HP_BYPASS_W::new(self, 0)
    }
    ///Bits 1:4 - I2S TX PDM OSR2 value
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sinc_osr2(&mut self) -> TX_PDM_SINC_OSR2_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_SINC_OSR2_W::new(self, 1)
    }
    ///Bits 5:12 - I2S TX PDM prescale for sigmadelta
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_prescale(&mut self) -> TX_PDM_PRESCALE_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_PRESCALE_W::new(self, 5)
    }
    ///Bits 13:14 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_hp_in_shift(&mut self) -> TX_PDM_HP_IN_SHIFT_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_HP_IN_SHIFT_W::new(self, 13)
    }
    ///Bits 15:16 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_lp_in_shift(&mut self) -> TX_PDM_LP_IN_SHIFT_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_LP_IN_SHIFT_W::new(self, 15)
    }
    ///Bits 17:18 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sinc_in_shift(&mut self) -> TX_PDM_SINC_IN_SHIFT_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_SINC_IN_SHIFT_W::new(self, 17)
    }
    ///Bits 19:20 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sigmadelta_in_shift(
        &mut self,
    ) -> TX_PDM_SIGMADELTA_IN_SHIFT_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_SIGMADELTA_IN_SHIFT_W::new(self, 19)
    }
    ///Bit 21 - I2S TX PDM sigmadelta dither2 value
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sigmadelta_dither2(
        &mut self,
    ) -> TX_PDM_SIGMADELTA_DITHER2_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_SIGMADELTA_DITHER2_W::new(self, 21)
    }
    ///Bit 22 - I2S TX PDM sigmadelta dither value
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sigmadelta_dither(&mut self) -> TX_PDM_SIGMADELTA_DITHER_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_SIGMADELTA_DITHER_W::new(self, 22)
    }
    ///Bit 23 - I2S TX PDM dac mode enable
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_dac_2out_en(&mut self) -> TX_PDM_DAC_2OUT_EN_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_DAC_2OUT_EN_W::new(self, 23)
    }
    ///Bit 24 - I2S TX PDM dac 2channel enable
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_dac_mode_en(&mut self) -> TX_PDM_DAC_MODE_EN_W<TX_PCM2PDM_CONF_SPEC> {
        TX_PDM_DAC_MODE_EN_W::new(self, 24)
    }
    ///Bit 25 - I2S TX PDM Converter enable
    #[inline(always)]
    #[must_use]
    pub fn pcm2pdm_conv_en(&mut self) -> PCM2PDM_CONV_EN_W<TX_PCM2PDM_CONF_SPEC> {
        PCM2PDM_CONV_EN_W::new(self, 25)
    }
}
/**I2S TX PCM2PDM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tx_pcm2pdm_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_pcm2pdm_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_PCM2PDM_CONF_SPEC;
impl crate::RegisterSpec for TX_PCM2PDM_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_pcm2pdm_conf::R`](R) reader structure
impl crate::Readable for TX_PCM2PDM_CONF_SPEC {}
///`write(|w| ..)` method takes [`tx_pcm2pdm_conf::W`](W) writer structure
impl crate::Writable for TX_PCM2PDM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_PCM2PDM_CONF to value 0x004a_a004
impl crate::Resettable for TX_PCM2PDM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x004a_a004;
}
