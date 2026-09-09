#[doc = "Register `CLK_CONF_FORCE_ON` reader"]
pub type R = crate::R<CLK_CONF_FORCE_ON_SPEC>;
#[doc = "Register `CLK_CONF_FORCE_ON` writer"]
pub type W = crate::W<CLK_CONF_FORCE_ON_SPEC>;
#[doc = "Field `CLK_AON_FO` reader - "]
pub type CLK_AON_FO_R = crate::BitReader;
#[doc = "Field `CLK_AON_FO` writer - "]
pub type CLK_AON_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PLL_FO` reader - "]
pub type CLK_PLL_FO_R = crate::BitReader;
#[doc = "Field `CLK_PLL_FO` writer - "]
pub type CLK_PLL_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_FO` reader - "]
pub type CLK_MODEM_SEC_FO_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_FO` writer - "]
pub type CLK_MODEM_SEC_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_ECB_FO` reader - "]
pub type CLK_MODEM_SEC_ECB_FO_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_ECB_FO` writer - "]
pub type CLK_MODEM_SEC_ECB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_CCM_FO` reader - "]
pub type CLK_MODEM_SEC_CCM_FO_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_CCM_FO` writer - "]
pub type CLK_MODEM_SEC_CCM_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_BAH_FO` reader - "]
pub type CLK_MODEM_SEC_BAH_FO_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_BAH_FO` writer - "]
pub type CLK_MODEM_SEC_BAH_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_APB_FO` reader - "]
pub type CLK_MODEM_SEC_APB_FO_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_APB_FO` writer - "]
pub type CLK_MODEM_SEC_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MODEM_SEC_AHB_FO` reader - "]
pub type CLK_MODEM_SEC_AHB_FO_R = crate::BitReader;
#[doc = "Field `CLK_MODEM_SEC_AHB_FO` writer - "]
pub type CLK_MODEM_SEC_AHB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_PWDET_ADC_INF_FO` reader - "]
pub type CLK_FE_PWDET_ADC_INF_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_PWDET_ADC_INF_FO` writer - "]
pub type CLK_FE_PWDET_ADC_INF_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_DAC_INF_FO` reader - "]
pub type CLK_FE_DAC_INF_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_DAC_INF_FO` writer - "]
pub type CLK_FE_DAC_INF_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_APB_FO` reader - "]
pub type CLK_FE_APB_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_APB_FO` writer - "]
pub type CLK_FE_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_ADC_INF_FO` reader - "]
pub type CLK_FE_ADC_INF_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_ADC_INF_FO` writer - "]
pub type CLK_FE_ADC_INF_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_80M_FO` reader - "]
pub type CLK_FE_80M_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_80M_FO` writer - "]
pub type CLK_FE_80M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_40M_FO` reader - "]
pub type CLK_FE_40M_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_40M_FO` writer - "]
pub type CLK_FE_40M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_20M_FO` reader - "]
pub type CLK_FE_20M_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_20M_FO` writer - "]
pub type CLK_FE_20M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_160M_FO` reader - "]
pub type CLK_FE_160M_FO_R = crate::BitReader;
#[doc = "Field `CLK_FE_160M_FO` writer - "]
pub type CLK_FE_160M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ETM_FO` reader - "]
pub type CLK_ETM_FO_R = crate::BitReader;
#[doc = "Field `CLK_ETM_FO` writer - "]
pub type CLK_ETM_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ETM_APB_FO` reader - "]
pub type CLK_ETM_APB_FO_R = crate::BitReader;
#[doc = "Field `CLK_ETM_APB_FO` writer - "]
pub type CLK_ETM_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DATA_DUMP_MEM_FO` reader - "]
pub type CLK_DATA_DUMP_MEM_FO_R = crate::BitReader;
#[doc = "Field `CLK_DATA_DUMP_MEM_FO` writer - "]
pub type CLK_DATA_DUMP_MEM_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DATA_DUMP_FO` reader - "]
pub type CLK_DATA_DUMP_FO_R = crate::BitReader;
#[doc = "Field `CLK_DATA_DUMP_FO` writer - "]
pub type CLK_DATA_DUMP_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BTMAC_FO` reader - "]
pub type CLK_BTMAC_FO_R = crate::BitReader;
#[doc = "Field `CLK_BTMAC_FO` writer - "]
pub type CLK_BTMAC_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BTMAC_AHB_FO` reader - "]
pub type CLK_BTMAC_AHB_FO_R = crate::BitReader;
#[doc = "Field `CLK_BTMAC_AHB_FO` writer - "]
pub type CLK_BTMAC_AHB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_APB_FO` reader - "]
pub type CLK_BT_APB_FO_R = crate::BitReader;
#[doc = "Field `CLK_BT_APB_FO` writer - "]
pub type CLK_BT_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_APB_32M_FO` reader - "]
pub type CLK_BT_APB_32M_FO_R = crate::BitReader;
#[doc = "Field `CLK_BT_APB_32M_FO` writer - "]
pub type CLK_BT_APB_32M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_8M_FO` reader - "]
pub type CLK_BT_8M_FO_R = crate::BitReader;
#[doc = "Field `CLK_BT_8M_FO` writer - "]
pub type CLK_BT_8M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_80M_FO` reader - "]
pub type CLK_BT_80M_FO_R = crate::BitReader;
#[doc = "Field `CLK_BT_80M_FO` writer - "]
pub type CLK_BT_80M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_40M_FO` reader - "]
pub type CLK_BT_40M_FO_R = crate::BitReader;
#[doc = "Field `CLK_BT_40M_FO` writer - "]
pub type CLK_BT_40M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_32M_FO` reader - "]
pub type CLK_BT_32M_FO_R = crate::BitReader;
#[doc = "Field `CLK_BT_32M_FO` writer - "]
pub type CLK_BT_32M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_16M_FO` reader - "]
pub type CLK_BT_16M_FO_R = crate::BitReader;
#[doc = "Field `CLK_BT_16M_FO` writer - "]
pub type CLK_BT_16M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_160M_FO` reader - "]
pub type CLK_BT_160M_FO_R = crate::BitReader;
#[doc = "Field `CLK_BT_160M_FO` writer - "]
pub type CLK_BT_160M_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BLE_TIMER_FO` reader - "]
pub type CLK_BLE_TIMER_FO_R = crate::BitReader;
#[doc = "Field `CLK_BLE_TIMER_FO` writer - "]
pub type CLK_BLE_TIMER_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BLE_TIMER_APB_FO` reader - "]
pub type CLK_BLE_TIMER_APB_FO_R = crate::BitReader;
#[doc = "Field `CLK_BLE_TIMER_APB_FO` writer - "]
pub type CLK_BLE_TIMER_APB_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_aon_fo(&self) -> CLK_AON_FO_R {
        CLK_AON_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_pll_fo(&self) -> CLK_PLL_FO_R {
        CLK_PLL_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_modem_sec_fo(&self) -> CLK_MODEM_SEC_FO_R {
        CLK_MODEM_SEC_FO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_modem_sec_ecb_fo(&self) -> CLK_MODEM_SEC_ECB_FO_R {
        CLK_MODEM_SEC_ECB_FO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_modem_sec_ccm_fo(&self) -> CLK_MODEM_SEC_CCM_FO_R {
        CLK_MODEM_SEC_CCM_FO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_modem_sec_bah_fo(&self) -> CLK_MODEM_SEC_BAH_FO_R {
        CLK_MODEM_SEC_BAH_FO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_modem_sec_apb_fo(&self) -> CLK_MODEM_SEC_APB_FO_R {
        CLK_MODEM_SEC_APB_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_modem_sec_ahb_fo(&self) -> CLK_MODEM_SEC_AHB_FO_R {
        CLK_MODEM_SEC_AHB_FO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_fe_pwdet_adc_inf_fo(&self) -> CLK_FE_PWDET_ADC_INF_FO_R {
        CLK_FE_PWDET_ADC_INF_FO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_fe_dac_inf_fo(&self) -> CLK_FE_DAC_INF_FO_R {
        CLK_FE_DAC_INF_FO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_fe_apb_fo(&self) -> CLK_FE_APB_FO_R {
        CLK_FE_APB_FO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_fe_adc_inf_fo(&self) -> CLK_FE_ADC_INF_FO_R {
        CLK_FE_ADC_INF_FO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_fe_80m_fo(&self) -> CLK_FE_80M_FO_R {
        CLK_FE_80M_FO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_fe_40m_fo(&self) -> CLK_FE_40M_FO_R {
        CLK_FE_40M_FO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_fe_20m_fo(&self) -> CLK_FE_20M_FO_R {
        CLK_FE_20M_FO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk_fe_160m_fo(&self) -> CLK_FE_160M_FO_R {
        CLK_FE_160M_FO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_etm_fo(&self) -> CLK_ETM_FO_R {
        CLK_ETM_FO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clk_etm_apb_fo(&self) -> CLK_ETM_APB_FO_R {
        CLK_ETM_APB_FO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_data_dump_mem_fo(&self) -> CLK_DATA_DUMP_MEM_FO_R {
        CLK_DATA_DUMP_MEM_FO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clk_data_dump_fo(&self) -> CLK_DATA_DUMP_FO_R {
        CLK_DATA_DUMP_FO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_btmac_fo(&self) -> CLK_BTMAC_FO_R {
        CLK_BTMAC_FO_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_btmac_ahb_fo(&self) -> CLK_BTMAC_AHB_FO_R {
        CLK_BTMAC_AHB_FO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_bt_apb_fo(&self) -> CLK_BT_APB_FO_R {
        CLK_BT_APB_FO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_bt_apb_32m_fo(&self) -> CLK_BT_APB_32M_FO_R {
        CLK_BT_APB_32M_FO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_bt_8m_fo(&self) -> CLK_BT_8M_FO_R {
        CLK_BT_8M_FO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_bt_80m_fo(&self) -> CLK_BT_80M_FO_R {
        CLK_BT_80M_FO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn clk_bt_40m_fo(&self) -> CLK_BT_40M_FO_R {
        CLK_BT_40M_FO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn clk_bt_32m_fo(&self) -> CLK_BT_32M_FO_R {
        CLK_BT_32M_FO_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_bt_16m_fo(&self) -> CLK_BT_16M_FO_R {
        CLK_BT_16M_FO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_bt_160m_fo(&self) -> CLK_BT_160M_FO_R {
        CLK_BT_160M_FO_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_ble_timer_fo(&self) -> CLK_BLE_TIMER_FO_R {
        CLK_BLE_TIMER_FO_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_ble_timer_apb_fo(&self) -> CLK_BLE_TIMER_APB_FO_R {
        CLK_BLE_TIMER_APB_FO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF_FORCE_ON")
            .field("clk_aon_fo", &self.clk_aon_fo())
            .field("clk_pll_fo", &self.clk_pll_fo())
            .field("clk_modem_sec_fo", &self.clk_modem_sec_fo())
            .field("clk_modem_sec_ecb_fo", &self.clk_modem_sec_ecb_fo())
            .field("clk_modem_sec_ccm_fo", &self.clk_modem_sec_ccm_fo())
            .field("clk_modem_sec_bah_fo", &self.clk_modem_sec_bah_fo())
            .field("clk_modem_sec_apb_fo", &self.clk_modem_sec_apb_fo())
            .field("clk_modem_sec_ahb_fo", &self.clk_modem_sec_ahb_fo())
            .field("clk_fe_pwdet_adc_inf_fo", &self.clk_fe_pwdet_adc_inf_fo())
            .field("clk_fe_dac_inf_fo", &self.clk_fe_dac_inf_fo())
            .field("clk_fe_apb_fo", &self.clk_fe_apb_fo())
            .field("clk_fe_adc_inf_fo", &self.clk_fe_adc_inf_fo())
            .field("clk_fe_80m_fo", &self.clk_fe_80m_fo())
            .field("clk_fe_40m_fo", &self.clk_fe_40m_fo())
            .field("clk_fe_20m_fo", &self.clk_fe_20m_fo())
            .field("clk_fe_160m_fo", &self.clk_fe_160m_fo())
            .field("clk_etm_fo", &self.clk_etm_fo())
            .field("clk_etm_apb_fo", &self.clk_etm_apb_fo())
            .field("clk_data_dump_mem_fo", &self.clk_data_dump_mem_fo())
            .field("clk_data_dump_fo", &self.clk_data_dump_fo())
            .field("clk_btmac_fo", &self.clk_btmac_fo())
            .field("clk_btmac_ahb_fo", &self.clk_btmac_ahb_fo())
            .field("clk_bt_apb_fo", &self.clk_bt_apb_fo())
            .field("clk_bt_apb_32m_fo", &self.clk_bt_apb_32m_fo())
            .field("clk_bt_8m_fo", &self.clk_bt_8m_fo())
            .field("clk_bt_80m_fo", &self.clk_bt_80m_fo())
            .field("clk_bt_40m_fo", &self.clk_bt_40m_fo())
            .field("clk_bt_32m_fo", &self.clk_bt_32m_fo())
            .field("clk_bt_16m_fo", &self.clk_bt_16m_fo())
            .field("clk_bt_160m_fo", &self.clk_bt_160m_fo())
            .field("clk_ble_timer_fo", &self.clk_ble_timer_fo())
            .field("clk_ble_timer_apb_fo", &self.clk_ble_timer_apb_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_aon_fo(&mut self) -> CLK_AON_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_AON_FO_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_pll_fo(&mut self) -> CLK_PLL_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_PLL_FO_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_modem_sec_fo(&mut self) -> CLK_MODEM_SEC_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_FO_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_modem_sec_ecb_fo(&mut self) -> CLK_MODEM_SEC_ECB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_ECB_FO_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clk_modem_sec_ccm_fo(&mut self) -> CLK_MODEM_SEC_CCM_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_CCM_FO_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_modem_sec_bah_fo(&mut self) -> CLK_MODEM_SEC_BAH_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_BAH_FO_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clk_modem_sec_apb_fo(&mut self) -> CLK_MODEM_SEC_APB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_APB_FO_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clk_modem_sec_ahb_fo(&mut self) -> CLK_MODEM_SEC_AHB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_MODEM_SEC_AHB_FO_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_fe_pwdet_adc_inf_fo(
        &mut self,
    ) -> CLK_FE_PWDET_ADC_INF_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_PWDET_ADC_INF_FO_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_fe_dac_inf_fo(&mut self) -> CLK_FE_DAC_INF_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_DAC_INF_FO_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_fe_apb_fo(&mut self) -> CLK_FE_APB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_APB_FO_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_fe_adc_inf_fo(&mut self) -> CLK_FE_ADC_INF_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_ADC_INF_FO_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_fe_80m_fo(&mut self) -> CLK_FE_80M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_80M_FO_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_fe_40m_fo(&mut self) -> CLK_FE_40M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_40M_FO_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_fe_20m_fo(&mut self) -> CLK_FE_20M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_20M_FO_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk_fe_160m_fo(&mut self) -> CLK_FE_160M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_FE_160M_FO_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_etm_fo(&mut self) -> CLK_ETM_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_ETM_FO_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clk_etm_apb_fo(&mut self) -> CLK_ETM_APB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_ETM_APB_FO_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_data_dump_mem_fo(&mut self) -> CLK_DATA_DUMP_MEM_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_DATA_DUMP_MEM_FO_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clk_data_dump_fo(&mut self) -> CLK_DATA_DUMP_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_DATA_DUMP_FO_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_btmac_fo(&mut self) -> CLK_BTMAC_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BTMAC_FO_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn clk_btmac_ahb_fo(&mut self) -> CLK_BTMAC_AHB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BTMAC_AHB_FO_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_bt_apb_fo(&mut self) -> CLK_BT_APB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BT_APB_FO_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn clk_bt_apb_32m_fo(&mut self) -> CLK_BT_APB_32M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BT_APB_32M_FO_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn clk_bt_8m_fo(&mut self) -> CLK_BT_8M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BT_8M_FO_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_bt_80m_fo(&mut self) -> CLK_BT_80M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BT_80M_FO_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn clk_bt_40m_fo(&mut self) -> CLK_BT_40M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BT_40M_FO_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn clk_bt_32m_fo(&mut self) -> CLK_BT_32M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BT_32M_FO_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clk_bt_16m_fo(&mut self) -> CLK_BT_16M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BT_16M_FO_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_bt_160m_fo(&mut self) -> CLK_BT_160M_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BT_160M_FO_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn clk_ble_timer_fo(&mut self) -> CLK_BLE_TIMER_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BLE_TIMER_FO_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_ble_timer_apb_fo(&mut self) -> CLK_BLE_TIMER_APB_FO_W<'_, CLK_CONF_FORCE_ON_SPEC> {
        CLK_BLE_TIMER_APB_FO_W::new(self, 31)
    }
}
#[doc = "CLK_CONF_FORCE_ON\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf_force_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf_force_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLK_CONF_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf_force_on::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_FORCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf_force_on::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_CONF_FORCE_ON to value 0"]
impl crate::Resettable for CLK_CONF_FORCE_ON_SPEC {}
