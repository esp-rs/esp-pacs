#[doc = "Register `RX_PDM_CONF` reader"]
pub type R = crate::R<RX_PDM_CONF_SPEC>;
#[doc = "Register `RX_PDM_CONF` writer"]
pub type W = crate::W<RX_PDM_CONF_SPEC>;
#[doc = "Field `RX_PDM2PCM_EN` reader - 1: Enable PDM2PCM RX mode. 0: DIsable."]
pub type RX_PDM2PCM_EN_R = crate::BitReader;
#[doc = "Field `RX_PDM2PCM_EN` writer - 1: Enable PDM2PCM RX mode. 0: DIsable."]
pub type RX_PDM2PCM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` reader - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
pub type RX_PDM_SINC_DSR_16_EN_R = crate::BitReader;
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` writer - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
pub type RX_PDM_SINC_DSR_16_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PDM2PCM_AMPLIFY_NUM` reader - Configure PDM RX amplify number."]
pub type RX_PDM2PCM_AMPLIFY_NUM_R = crate::FieldReader;
#[doc = "Field `RX_PDM2PCM_AMPLIFY_NUM` writer - Configure PDM RX amplify number."]
pub type RX_PDM2PCM_AMPLIFY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_PDM_HP_BYPASS` reader - I2S PDM RX bypass hp filter or not."]
pub type RX_PDM_HP_BYPASS_R = crate::BitReader;
#[doc = "Field `RX_PDM_HP_BYPASS` writer - I2S PDM RX bypass hp filter or not."]
pub type RX_PDM_HP_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IIR_HP_MULT12_5` reader - The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5\\[2:0\\])"]
pub type RX_IIR_HP_MULT12_5_R = crate::FieldReader;
#[doc = "Field `RX_IIR_HP_MULT12_5` writer - The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5\\[2:0\\])"]
pub type RX_IIR_HP_MULT12_5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_IIR_HP_MULT12_0` reader - The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0\\[2:0\\])"]
pub type RX_IIR_HP_MULT12_0_R = crate::FieldReader;
#[doc = "Field `RX_IIR_HP_MULT12_0` writer - The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0\\[2:0\\])"]
pub type RX_IIR_HP_MULT12_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 19 - 1: Enable PDM2PCM RX mode. 0: DIsable."]
    #[inline(always)]
    pub fn rx_pdm2pcm_en(&self) -> RX_PDM2PCM_EN_R {
        RX_PDM2PCM_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
    #[inline(always)]
    pub fn rx_pdm_sinc_dsr_16_en(&self) -> RX_PDM_SINC_DSR_16_EN_R {
        RX_PDM_SINC_DSR_16_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:24 - Configure PDM RX amplify number."]
    #[inline(always)]
    pub fn rx_pdm2pcm_amplify_num(&self) -> RX_PDM2PCM_AMPLIFY_NUM_R {
        RX_PDM2PCM_AMPLIFY_NUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - I2S PDM RX bypass hp filter or not."]
    #[inline(always)]
    pub fn rx_pdm_hp_bypass(&self) -> RX_PDM_HP_BYPASS_R {
        RX_PDM_HP_BYPASS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:28 - The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5\\[2:0\\])"]
    #[inline(always)]
    pub fn rx_iir_hp_mult12_5(&self) -> RX_IIR_HP_MULT12_5_R {
        RX_IIR_HP_MULT12_5_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0\\[2:0\\])"]
    #[inline(always)]
    pub fn rx_iir_hp_mult12_0(&self) -> RX_IIR_HP_MULT12_0_R {
        RX_IIR_HP_MULT12_0_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_PDM_CONF")
            .field("rx_pdm2pcm_en", &self.rx_pdm2pcm_en())
            .field("rx_pdm_sinc_dsr_16_en", &self.rx_pdm_sinc_dsr_16_en())
            .field("rx_pdm2pcm_amplify_num", &self.rx_pdm2pcm_amplify_num())
            .field("rx_pdm_hp_bypass", &self.rx_pdm_hp_bypass())
            .field("rx_iir_hp_mult12_5", &self.rx_iir_hp_mult12_5())
            .field("rx_iir_hp_mult12_0", &self.rx_iir_hp_mult12_0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 19 - 1: Enable PDM2PCM RX mode. 0: DIsable."]
    #[inline(always)]
    pub fn rx_pdm2pcm_en(&mut self) -> RX_PDM2PCM_EN_W<'_, RX_PDM_CONF_SPEC> {
        RX_PDM2PCM_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
    #[inline(always)]
    pub fn rx_pdm_sinc_dsr_16_en(&mut self) -> RX_PDM_SINC_DSR_16_EN_W<'_, RX_PDM_CONF_SPEC> {
        RX_PDM_SINC_DSR_16_EN_W::new(self, 20)
    }
    #[doc = "Bits 21:24 - Configure PDM RX amplify number."]
    #[inline(always)]
    pub fn rx_pdm2pcm_amplify_num(&mut self) -> RX_PDM2PCM_AMPLIFY_NUM_W<'_, RX_PDM_CONF_SPEC> {
        RX_PDM2PCM_AMPLIFY_NUM_W::new(self, 21)
    }
    #[doc = "Bit 25 - I2S PDM RX bypass hp filter or not."]
    #[inline(always)]
    pub fn rx_pdm_hp_bypass(&mut self) -> RX_PDM_HP_BYPASS_W<'_, RX_PDM_CONF_SPEC> {
        RX_PDM_HP_BYPASS_W::new(self, 25)
    }
    #[doc = "Bits 26:28 - The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5\\[2:0\\])"]
    #[inline(always)]
    pub fn rx_iir_hp_mult12_5(&mut self) -> RX_IIR_HP_MULT12_5_W<'_, RX_PDM_CONF_SPEC> {
        RX_IIR_HP_MULT12_5_W::new(self, 26)
    }
    #[doc = "Bits 29:31 - The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0\\[2:0\\])"]
    #[inline(always)]
    pub fn rx_iir_hp_mult12_0(&mut self) -> RX_IIR_HP_MULT12_0_W<'_, RX_PDM_CONF_SPEC> {
        RX_IIR_HP_MULT12_0_W::new(self, 29)
    }
}
#[doc = "I2S RX configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_pdm_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_pdm_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_PDM_CONF_SPEC;
impl crate::RegisterSpec for RX_PDM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_pdm_conf::R`](R) reader structure"]
impl crate::Readable for RX_PDM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_pdm_conf::W`](W) writer structure"]
impl crate::Writable for RX_PDM_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_PDM_CONF to value 0xf820_0000"]
impl crate::Resettable for RX_PDM_CONF_SPEC {
    const RESET_VALUE: u32 = 0xf820_0000;
}
