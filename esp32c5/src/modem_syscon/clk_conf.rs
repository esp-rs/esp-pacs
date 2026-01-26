#[doc = "Register `CLK_CONF` reader"]
pub type R = crate::R<CLK_CONF_SPEC>;
#[doc = "Register `CLK_CONF` writer"]
pub type W = crate::W<CLK_CONF_SPEC>;
#[doc = "Field `PWDET_SAR_CLOCK_ENA` reader - "]
pub type PWDET_SAR_CLOCK_ENA_R = crate::BitReader;
#[doc = "Field `PWDET_SAR_CLOCK_ENA` writer - "]
pub type PWDET_SAR_CLOCK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWDET_CLK_DIV_NUM` reader - "]
pub type PWDET_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PWDET_CLK_DIV_NUM` writer - "]
pub type PWDET_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLK_TX_DAC_INV_ENA` reader - "]
pub type CLK_TX_DAC_INV_ENA_R = crate::BitReader;
#[doc = "Field `CLK_TX_DAC_INV_ENA` writer - "]
pub type CLK_TX_DAC_INV_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_RX_ADC_INV_ENA` reader - "]
pub type CLK_RX_ADC_INV_ENA_R = crate::BitReader;
#[doc = "Field `CLK_RX_ADC_INV_ENA` writer - "]
pub type CLK_RX_ADC_INV_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PWDET_ADC_INV_ENA` reader - "]
pub type CLK_PWDET_ADC_INV_ENA_R = crate::BitReader;
#[doc = "Field `CLK_PWDET_ADC_INV_ENA` writer - "]
pub type CLK_PWDET_ADC_INV_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C_MST_SEL_160M` reader - "]
pub type CLK_I2C_MST_SEL_160M_R = crate::BitReader;
#[doc = "Field `CLK_I2C_MST_SEL_160M` writer - "]
pub type CLK_I2C_MST_SEL_160M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DATA_DUMP_MUX` reader - "]
pub type CLK_DATA_DUMP_MUX_R = crate::BitReader;
#[doc = "Field `CLK_DATA_DUMP_MUX` writer - "]
pub type CLK_DATA_DUMP_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ETM_EN` reader - "]
pub type CLK_ETM_EN_R = crate::BitReader;
#[doc = "Field `CLK_ETM_EN` writer - "]
pub type CLK_ETM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ZB_APB_EN` reader - "]
pub type CLK_ZB_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_ZB_APB_EN` writer - "]
pub type CLK_ZB_APB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ZBMAC_EN` reader - "]
pub type CLK_ZBMAC_EN_R = crate::BitReader;
#[doc = "Field `CLK_ZBMAC_EN` writer - "]
pub type CLK_ZBMAC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_ECB_EN` reader - "]
pub type CLK_MODEM_SEC_ECB_EN_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_ECB_EN` writer - "]
pub type CLK_MODEM_SEC_ECB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_CCM_EN` reader - "]
pub type CLK_MODEM_SEC_CCM_EN_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_CCM_EN` writer - "]
pub type CLK_MODEM_SEC_CCM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_BAH_EN` reader - "]
pub type CLK_MODEM_SEC_BAH_EN_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_BAH_EN` writer - "]
pub type CLK_MODEM_SEC_BAH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_APB_EN` reader - "]
pub type CLK_MODEM_SEC_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_APB_EN` writer - "]
pub type CLK_MODEM_SEC_APB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_EN` reader - "]
pub type CLK_MODEM_SEC_EN_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_EN` writer - "]
pub type CLK_MODEM_SEC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BLE_TIMER_EN` reader - "]
pub type CLK_BLE_TIMER_EN_R = crate::BitReader;
#[doc = "Field `CLK_BLE_TIMER_EN` writer - "]
pub type CLK_BLE_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DATA_DUMP_EN` reader - "]
pub type CLK_DATA_DUMP_EN_R = crate::BitReader;
#[doc = "Field `CLK_DATA_DUMP_EN` writer - "]
pub type CLK_DATA_DUMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwdet_sar_clock_ena(&self) -> PWDET_SAR_CLOCK_ENA_R {
        PWDET_SAR_CLOCK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8"]
    #[inline(always)]
    pub fn pwdet_clk_div_num(&self) -> PWDET_CLK_DIV_NUM_R {
        PWDET_CLK_DIV_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_tx_dac_inv_ena(&self) -> CLK_TX_DAC_INV_ENA_R {
        CLK_TX_DAC_INV_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_rx_adc_inv_ena(&self) -> CLK_RX_ADC_INV_ENA_R {
        CLK_RX_ADC_INV_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_pwdet_adc_inv_ena(&self) -> CLK_PWDET_ADC_INV_ENA_R {
        CLK_PWDET_ADC_INV_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_i2c_mst_sel_160m(&self) -> CLK_I2C_MST_SEL_160M_R {
        CLK_I2C_MST_SEL_160M_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_data_dump_mux(&self) -> CLK_DATA_DUMP_MUX_R {
        CLK_DATA_DUMP_MUX_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_etm_en(&self) -> CLK_ETM_EN_R {
        CLK_ETM_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_zb_apb_en(&self) -> CLK_ZB_APB_EN_R {
        CLK_ZB_APB_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_zbmac_en(&self) -> CLK_ZBMAC_EN_R {
        CLK_ZBMAC_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_modem_sec_ecb_en(&self) -> CLK_MODEM_SEC_ECB_EN_R {
        CLK_MODEM_SEC_ECB_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn clk_modem_sec_ccm_en(&self) -> CLK_MODEM_SEC_CCM_EN_R {
        CLK_MODEM_SEC_CCM_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn clk_modem_sec_bah_en(&self) -> CLK_MODEM_SEC_BAH_EN_R {
        CLK_MODEM_SEC_BAH_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_modem_sec_apb_en(&self) -> CLK_MODEM_SEC_APB_EN_R {
        CLK_MODEM_SEC_APB_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_modem_sec_en(&self) -> CLK_MODEM_SEC_EN_R {
        CLK_MODEM_SEC_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_ble_timer_en(&self) -> CLK_BLE_TIMER_EN_R {
        CLK_BLE_TIMER_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_data_dump_en(&self) -> CLK_DATA_DUMP_EN_R {
        CLK_DATA_DUMP_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field("pwdet_sar_clock_ena", &self.pwdet_sar_clock_ena())
            .field("pwdet_clk_div_num", &self.pwdet_clk_div_num())
            .field("clk_tx_dac_inv_ena", &self.clk_tx_dac_inv_ena())
            .field("clk_rx_adc_inv_ena", &self.clk_rx_adc_inv_ena())
            .field("clk_pwdet_adc_inv_ena", &self.clk_pwdet_adc_inv_ena())
            .field("clk_i2c_mst_sel_160m", &self.clk_i2c_mst_sel_160m())
            .field("clk_data_dump_mux", &self.clk_data_dump_mux())
            .field("clk_etm_en", &self.clk_etm_en())
            .field("clk_zb_apb_en", &self.clk_zb_apb_en())
            .field("clk_zbmac_en", &self.clk_zbmac_en())
            .field("clk_modem_sec_ecb_en", &self.clk_modem_sec_ecb_en())
            .field("clk_modem_sec_ccm_en", &self.clk_modem_sec_ccm_en())
            .field("clk_modem_sec_bah_en", &self.clk_modem_sec_bah_en())
            .field("clk_modem_sec_apb_en", &self.clk_modem_sec_apb_en())
            .field("clk_modem_sec_en", &self.clk_modem_sec_en())
            .field("clk_ble_timer_en", &self.clk_ble_timer_en())
            .field("clk_data_dump_en", &self.clk_data_dump_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwdet_sar_clock_ena(&mut self) -> PWDET_SAR_CLOCK_ENA_W<'_, CLK_CONF_SPEC> {
        PWDET_SAR_CLOCK_ENA_W::new(self, 0)
    }
    #[doc = "Bits 1:8"]
    #[inline(always)]
    pub fn pwdet_clk_div_num(&mut self) -> PWDET_CLK_DIV_NUM_W<'_, CLK_CONF_SPEC> {
        PWDET_CLK_DIV_NUM_W::new(self, 1)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_tx_dac_inv_ena(&mut self) -> CLK_TX_DAC_INV_ENA_W<'_, CLK_CONF_SPEC> {
        CLK_TX_DAC_INV_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_rx_adc_inv_ena(&mut self) -> CLK_RX_ADC_INV_ENA_W<'_, CLK_CONF_SPEC> {
        CLK_RX_ADC_INV_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_pwdet_adc_inv_ena(&mut self) -> CLK_PWDET_ADC_INV_ENA_W<'_, CLK_CONF_SPEC> {
        CLK_PWDET_ADC_INV_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_i2c_mst_sel_160m(&mut self) -> CLK_I2C_MST_SEL_160M_W<'_, CLK_CONF_SPEC> {
        CLK_I2C_MST_SEL_160M_W::new(self, 12)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_data_dump_mux(&mut self) -> CLK_DATA_DUMP_MUX_W<'_, CLK_CONF_SPEC> {
        CLK_DATA_DUMP_MUX_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_etm_en(&mut self) -> CLK_ETM_EN_W<'_, CLK_CONF_SPEC> {
        CLK_ETM_EN_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_zb_apb_en(&mut self) -> CLK_ZB_APB_EN_W<'_, CLK_CONF_SPEC> {
        CLK_ZB_APB_EN_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_zbmac_en(&mut self) -> CLK_ZBMAC_EN_W<'_, CLK_CONF_SPEC> {
        CLK_ZBMAC_EN_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_modem_sec_ecb_en(&mut self) -> CLK_MODEM_SEC_ECB_EN_W<'_, CLK_CONF_SPEC> {
        CLK_MODEM_SEC_ECB_EN_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn clk_modem_sec_ccm_en(&mut self) -> CLK_MODEM_SEC_CCM_EN_W<'_, CLK_CONF_SPEC> {
        CLK_MODEM_SEC_CCM_EN_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn clk_modem_sec_bah_en(&mut self) -> CLK_MODEM_SEC_BAH_EN_W<'_, CLK_CONF_SPEC> {
        CLK_MODEM_SEC_BAH_EN_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_modem_sec_apb_en(&mut self) -> CLK_MODEM_SEC_APB_EN_W<'_, CLK_CONF_SPEC> {
        CLK_MODEM_SEC_APB_EN_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_modem_sec_en(&mut self) -> CLK_MODEM_SEC_EN_W<'_, CLK_CONF_SPEC> {
        CLK_MODEM_SEC_EN_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_ble_timer_en(&mut self) -> CLK_BLE_TIMER_EN_W<'_, CLK_CONF_SPEC> {
        CLK_BLE_TIMER_EN_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_data_dump_en(&mut self) -> CLK_DATA_DUMP_EN_W<'_, CLK_CONF_SPEC> {
        CLK_DATA_DUMP_EN_W::new(self, 31)
    }
}
#[doc = "CLK_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x0020_0002"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0020_0002;
}
