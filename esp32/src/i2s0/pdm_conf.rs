#[doc = "Register `PDM_CONF` reader"]
pub type R = crate::R<PDM_CONF_SPEC>;
#[doc = "Register `PDM_CONF` writer"]
pub type W = crate::W<PDM_CONF_SPEC>;
#[doc = "Field `TX_PDM_EN` reader - "]
pub type TX_PDM_EN_R = crate::BitReader;
#[doc = "Field `TX_PDM_EN` writer - "]
pub type TX_PDM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PDM_EN` reader - "]
pub type RX_PDM_EN_R = crate::BitReader;
#[doc = "Field `RX_PDM_EN` writer - "]
pub type RX_PDM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCM2PDM_CONV_EN` reader - "]
pub type PCM2PDM_CONV_EN_R = crate::BitReader;
#[doc = "Field `PCM2PDM_CONV_EN` writer - "]
pub type PCM2PDM_CONV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDM2PCM_CONV_EN` reader - "]
pub type PDM2PCM_CONV_EN_R = crate::BitReader;
#[doc = "Field `PDM2PCM_CONV_EN` writer - "]
pub type PDM2PCM_CONV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PDM_SINC_OSR2` reader - "]
pub type TX_PDM_SINC_OSR2_R = crate::FieldReader;
#[doc = "Field `TX_PDM_SINC_OSR2` writer - "]
pub type TX_PDM_SINC_OSR2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_PDM_PRESCALE` reader - "]
pub type TX_PDM_PRESCALE_R = crate::FieldReader;
#[doc = "Field `TX_PDM_PRESCALE` writer - "]
pub type TX_PDM_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TX_PDM_HP_IN_SHIFT` reader - "]
pub type TX_PDM_HP_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_HP_IN_SHIFT` writer - "]
pub type TX_PDM_HP_IN_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_PDM_LP_IN_SHIFT` reader - "]
pub type TX_PDM_LP_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_LP_IN_SHIFT` writer - "]
pub type TX_PDM_LP_IN_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_PDM_SINC_IN_SHIFT` reader - "]
pub type TX_PDM_SINC_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_SINC_IN_SHIFT` writer - "]
pub type TX_PDM_SINC_IN_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_PDM_SIGMADELTA_IN_SHIFT` reader - "]
pub type TX_PDM_SIGMADELTA_IN_SHIFT_R = crate::FieldReader;
#[doc = "Field `TX_PDM_SIGMADELTA_IN_SHIFT` writer - "]
pub type TX_PDM_SIGMADELTA_IN_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` reader - "]
pub type RX_PDM_SINC_DSR_16_EN_R = crate::BitReader;
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` writer - "]
pub type RX_PDM_SINC_DSR_16_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PDM_HP_BYPASS` reader - "]
pub type TX_PDM_HP_BYPASS_R = crate::BitReader;
#[doc = "Field `TX_PDM_HP_BYPASS` writer - "]
pub type TX_PDM_HP_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("tx_pdm_en", &self.tx_pdm_en())
            .field("rx_pdm_en", &self.rx_pdm_en())
            .field("pcm2pdm_conv_en", &self.pcm2pdm_conv_en())
            .field("pdm2pcm_conv_en", &self.pdm2pcm_conv_en())
            .field("tx_pdm_sinc_osr2", &self.tx_pdm_sinc_osr2())
            .field("tx_pdm_prescale", &self.tx_pdm_prescale())
            .field("tx_pdm_hp_in_shift", &self.tx_pdm_hp_in_shift())
            .field("tx_pdm_lp_in_shift", &self.tx_pdm_lp_in_shift())
            .field("tx_pdm_sinc_in_shift", &self.tx_pdm_sinc_in_shift())
            .field(
                "tx_pdm_sigmadelta_in_shift",
                &self.tx_pdm_sigmadelta_in_shift(),
            )
            .field("rx_pdm_sinc_dsr_16_en", &self.rx_pdm_sinc_dsr_16_en())
            .field("tx_pdm_hp_bypass", &self.tx_pdm_hp_bypass())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_en(&mut self) -> TX_PDM_EN_W<PDM_CONF_SPEC> {
        TX_PDM_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pdm_en(&mut self) -> RX_PDM_EN_W<PDM_CONF_SPEC> {
        RX_PDM_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcm2pdm_conv_en(&mut self) -> PCM2PDM_CONV_EN_W<PDM_CONF_SPEC> {
        PCM2PDM_CONV_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pdm2pcm_conv_en(&mut self) -> PDM2PCM_CONV_EN_W<PDM_CONF_SPEC> {
        PDM2PCM_CONV_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sinc_osr2(&mut self) -> TX_PDM_SINC_OSR2_W<PDM_CONF_SPEC> {
        TX_PDM_SINC_OSR2_W::new(self, 4)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_prescale(&mut self) -> TX_PDM_PRESCALE_W<PDM_CONF_SPEC> {
        TX_PDM_PRESCALE_W::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_hp_in_shift(&mut self) -> TX_PDM_HP_IN_SHIFT_W<PDM_CONF_SPEC> {
        TX_PDM_HP_IN_SHIFT_W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_lp_in_shift(&mut self) -> TX_PDM_LP_IN_SHIFT_W<PDM_CONF_SPEC> {
        TX_PDM_LP_IN_SHIFT_W::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sinc_in_shift(&mut self) -> TX_PDM_SINC_IN_SHIFT_W<PDM_CONF_SPEC> {
        TX_PDM_SINC_IN_SHIFT_W::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_sigmadelta_in_shift(&mut self) -> TX_PDM_SIGMADELTA_IN_SHIFT_W<PDM_CONF_SPEC> {
        TX_PDM_SIGMADELTA_IN_SHIFT_W::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pdm_sinc_dsr_16_en(&mut self) -> RX_PDM_SINC_DSR_16_EN_W<PDM_CONF_SPEC> {
        RX_PDM_SINC_DSR_16_EN_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_hp_bypass(&mut self) -> TX_PDM_HP_BYPASS_W<PDM_CONF_SPEC> {
        TX_PDM_HP_BYPASS_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdm_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdm_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDM_CONF_SPEC;
impl crate::RegisterSpec for PDM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdm_conf::R`](R) reader structure"]
impl crate::Readable for PDM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdm_conf::W`](W) writer structure"]
impl crate::Writable for PDM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDM_CONF to value 0x0155_0020"]
impl crate::Resettable for PDM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0155_0020;
}
