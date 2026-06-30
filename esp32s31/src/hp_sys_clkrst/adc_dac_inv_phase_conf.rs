#[doc = "Register `ADC_DAC_INV_PHASE_CONF` reader"]
pub type R = crate::R<ADC_DAC_INV_PHASE_CONF_SPEC>;
#[doc = "Register `ADC_DAC_INV_PHASE_CONF` writer"]
pub type W = crate::W<ADC_DAC_INV_PHASE_CONF_SPEC>;
#[doc = "Field `CLK_RX_ADC_INV_PHASE_ENA` reader - xxxx"]
pub type CLK_RX_ADC_INV_PHASE_ENA_R = crate::BitReader;
#[doc = "Field `CLK_RX_ADC_INV_PHASE_ENA` writer - xxxx"]
pub type CLK_RX_ADC_INV_PHASE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TX_DAC_INV_PHASE_ENA` reader - xxxx"]
pub type CLK_TX_DAC_INV_PHASE_ENA_R = crate::BitReader;
#[doc = "Field `CLK_TX_DAC_INV_PHASE_ENA` writer - xxxx"]
pub type CLK_TX_DAC_INV_PHASE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PWDET_ADC_INV_PHASE_ENA` reader - xxxx"]
pub type CLK_PWDET_ADC_INV_PHASE_ENA_R = crate::BitReader;
#[doc = "Field `CLK_PWDET_ADC_INV_PHASE_ENA` writer - xxxx"]
pub type CLK_PWDET_ADC_INV_PHASE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn clk_rx_adc_inv_phase_ena(&self) -> CLK_RX_ADC_INV_PHASE_ENA_R {
        CLK_RX_ADC_INV_PHASE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - xxxx"]
    #[inline(always)]
    pub fn clk_tx_dac_inv_phase_ena(&self) -> CLK_TX_DAC_INV_PHASE_ENA_R {
        CLK_TX_DAC_INV_PHASE_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - xxxx"]
    #[inline(always)]
    pub fn clk_pwdet_adc_inv_phase_ena(&self) -> CLK_PWDET_ADC_INV_PHASE_ENA_R {
        CLK_PWDET_ADC_INV_PHASE_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_DAC_INV_PHASE_CONF")
            .field("clk_rx_adc_inv_phase_ena", &self.clk_rx_adc_inv_phase_ena())
            .field("clk_tx_dac_inv_phase_ena", &self.clk_tx_dac_inv_phase_ena())
            .field(
                "clk_pwdet_adc_inv_phase_ena",
                &self.clk_pwdet_adc_inv_phase_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn clk_rx_adc_inv_phase_ena(
        &mut self,
    ) -> CLK_RX_ADC_INV_PHASE_ENA_W<'_, ADC_DAC_INV_PHASE_CONF_SPEC> {
        CLK_RX_ADC_INV_PHASE_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - xxxx"]
    #[inline(always)]
    pub fn clk_tx_dac_inv_phase_ena(
        &mut self,
    ) -> CLK_TX_DAC_INV_PHASE_ENA_W<'_, ADC_DAC_INV_PHASE_CONF_SPEC> {
        CLK_TX_DAC_INV_PHASE_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - xxxx"]
    #[inline(always)]
    pub fn clk_pwdet_adc_inv_phase_ena(
        &mut self,
    ) -> CLK_PWDET_ADC_INV_PHASE_ENA_W<'_, ADC_DAC_INV_PHASE_CONF_SPEC> {
        CLK_PWDET_ADC_INV_PHASE_ENA_W::new(self, 2)
    }
}
#[doc = "xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_dac_inv_phase_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_dac_inv_phase_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_DAC_INV_PHASE_CONF_SPEC;
impl crate::RegisterSpec for ADC_DAC_INV_PHASE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_dac_inv_phase_conf::R`](R) reader structure"]
impl crate::Readable for ADC_DAC_INV_PHASE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_dac_inv_phase_conf::W`](W) writer structure"]
impl crate::Writable for ADC_DAC_INV_PHASE_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC_DAC_INV_PHASE_CONF to value 0"]
impl crate::Resettable for ADC_DAC_INV_PHASE_CONF_SPEC {}
