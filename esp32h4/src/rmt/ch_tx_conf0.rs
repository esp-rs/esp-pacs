#[doc = "Register `CH%s_TX_CONF0` reader"]
pub type R = crate::R<CH_TX_CONF0_SPEC>;
#[doc = "Register `CH%s_TX_CONF0` writer"]
pub type W = crate::W<CH_TX_CONF0_SPEC>;
#[doc = "Field `TX_START_CH` writer - Configures whether to enable sending data in channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
pub type TX_START_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_RD_RST_CH` writer - Configures whether to reset RAM read address accessed by the transmitter for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type MEM_RD_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_MEM_RST_CH` writer - Configures whether to reset RAM W/R address accessed by APB FIFO for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type APB_MEM_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CONTI_MODE_CH` reader - Configures whether to enable continuous TX mode for channel %s. \\\\ 0: No Effect\\\\ 1: Enable\\\\ In this mode, the transmitter starts transmission from the first data. If an end-marker is encountered, the transmitter starts transmitting data from the first data again. if no end-marker is encountered, the transmitter starts transmitting the first data again when the last data is transmitted.\\\\"]
pub type TX_CONTI_MODE_CH_R = crate::BitReader;
#[doc = "Field `TX_CONTI_MODE_CH` writer - Configures whether to enable continuous TX mode for channel %s. \\\\ 0: No Effect\\\\ 1: Enable\\\\ In this mode, the transmitter starts transmission from the first data. If an end-marker is encountered, the transmitter starts transmitting data from the first data again. if no end-marker is encountered, the transmitter starts transmitting the first data again when the last data is transmitted.\\\\"]
pub type TX_CONTI_MODE_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TX_WRAP_EN_CH` reader - Configures whether to enable wrap TX mode for channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\ In this mode, if the TX data size is larger than the channel's RAM block size, the transmitter continues transmitting the first data to the last data in loops.\\\\"]
pub type MEM_TX_WRAP_EN_CH_R = crate::BitReader;
#[doc = "Field `MEM_TX_WRAP_EN_CH` writer - Configures whether to enable wrap TX mode for channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\ In this mode, if the TX data size is larger than the channel's RAM block size, the transmitter continues transmitting the first data to the last data in loops.\\\\"]
pub type MEM_TX_WRAP_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_OUT_LV_CH` reader - Configures the level of output signal for channel %s when the transmitter is in idle state."]
pub type IDLE_OUT_LV_CH_R = crate::BitReader;
#[doc = "Field `IDLE_OUT_LV_CH` writer - Configures the level of output signal for channel %s when the transmitter is in idle state."]
pub type IDLE_OUT_LV_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_OUT_EN_CH` reader - Configures whether to enable the output for channel %s in idle state. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
pub type IDLE_OUT_EN_CH_R = crate::BitReader;
#[doc = "Field `IDLE_OUT_EN_CH` writer - Configures whether to enable the output for channel %s in idle state. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
pub type IDLE_OUT_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STOP_CH` reader - Configures whether to stop the transmitter of channel %s sending data out. \\\\ 0: No effect\\\\ 1: Stop\\\\"]
pub type TX_STOP_CH_R = crate::BitReader;
#[doc = "Field `TX_STOP_CH` writer - Configures whether to stop the transmitter of channel %s sending data out. \\\\ 0: No effect\\\\ 1: Stop\\\\"]
pub type TX_STOP_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_CNT_CH` reader - Configures the divider for clock of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
pub type DIV_CNT_CH_R = crate::FieldReader;
#[doc = "Field `DIV_CNT_CH` writer - Configures the divider for clock of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
pub type DIV_CNT_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_SIZE_CH` reader - Configures the maximum number of memory blocks allocated to channel %s."]
pub type MEM_SIZE_CH_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE_CH` writer - Configures the maximum number of memory blocks allocated to channel %s."]
pub type MEM_SIZE_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CARRIER_EFF_EN_CH` reader - Configures whether to add carrier modulation on the output signal only at data-sending state for channel %s. \\\\ 0: Add carrier modulation on the output signal at data-sending state and idle state for channel %s\\\\ 1: Add carrier modulation on the output signal only at data-sending state for channel %s\\\\ Only valid when RMT_CARRIER_EN_CH%s is 1.\\\\"]
pub type CARRIER_EFF_EN_CH_R = crate::BitReader;
#[doc = "Field `CARRIER_EFF_EN_CH` writer - Configures whether to add carrier modulation on the output signal only at data-sending state for channel %s. \\\\ 0: Add carrier modulation on the output signal at data-sending state and idle state for channel %s\\\\ 1: Add carrier modulation on the output signal only at data-sending state for channel %s\\\\ Only valid when RMT_CARRIER_EN_CH%s is 1.\\\\"]
pub type CARRIER_EFF_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_EN_CH` reader - Configures whether to enable the carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CARRIER_EN_CH_R = crate::BitReader;
#[doc = "Field `CARRIER_EN_CH` writer - Configures whether to enable the carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CARRIER_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_OUT_LV_CH` reader - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
pub type CARRIER_OUT_LV_CH_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV_CH` writer - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
pub type CARRIER_OUT_LV_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFIFO_RST_CH` writer - Reserved"]
pub type AFIFO_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONF_UPDATE_CH` writer - Synchronization bit for channel %s."]
pub type CONF_UPDATE_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Configures whether to enable continuous TX mode for channel %s. \\\\ 0: No Effect\\\\ 1: Enable\\\\ In this mode, the transmitter starts transmission from the first data. If an end-marker is encountered, the transmitter starts transmitting data from the first data again. if no end-marker is encountered, the transmitter starts transmitting the first data again when the last data is transmitted.\\\\"]
    #[inline(always)]
    pub fn tx_conti_mode_ch(&self) -> TX_CONTI_MODE_CH_R {
        TX_CONTI_MODE_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether to enable wrap TX mode for channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\ In this mode, if the TX data size is larger than the channel's RAM block size, the transmitter continues transmitting the first data to the last data in loops.\\\\"]
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch(&self) -> MEM_TX_WRAP_EN_CH_R {
        MEM_TX_WRAP_EN_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures the level of output signal for channel %s when the transmitter is in idle state."]
    #[inline(always)]
    pub fn idle_out_lv_ch(&self) -> IDLE_OUT_LV_CH_R {
        IDLE_OUT_LV_CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether to enable the output for channel %s in idle state. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn idle_out_en_ch(&self) -> IDLE_OUT_EN_CH_R {
        IDLE_OUT_EN_CH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether to stop the transmitter of channel %s sending data out. \\\\ 0: No effect\\\\ 1: Stop\\\\"]
    #[inline(always)]
    pub fn tx_stop_ch(&self) -> TX_STOP_CH_R {
        TX_STOP_CH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Configures the divider for clock of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
    #[inline(always)]
    pub fn div_cnt_ch(&self) -> DIV_CNT_CH_R {
        DIV_CNT_CH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Configures the maximum number of memory blocks allocated to channel %s."]
    #[inline(always)]
    pub fn mem_size_ch(&self) -> MEM_SIZE_CH_R {
        MEM_SIZE_CH_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Configures whether to add carrier modulation on the output signal only at data-sending state for channel %s. \\\\ 0: Add carrier modulation on the output signal at data-sending state and idle state for channel %s\\\\ 1: Add carrier modulation on the output signal only at data-sending state for channel %s\\\\ Only valid when RMT_CARRIER_EN_CH%s is 1.\\\\"]
    #[inline(always)]
    pub fn carrier_eff_en_ch(&self) -> CARRIER_EFF_EN_CH_R {
        CARRIER_EFF_EN_CH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures whether to enable the carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn carrier_en_ch(&self) -> CARRIER_EN_CH_R {
        CARRIER_EN_CH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
    #[inline(always)]
    pub fn carrier_out_lv_ch(&self) -> CARRIER_OUT_LV_CH_R {
        CARRIER_OUT_LV_CH_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_CONF0")
            .field("tx_conti_mode_ch", &self.tx_conti_mode_ch())
            .field("mem_tx_wrap_en_ch", &self.mem_tx_wrap_en_ch())
            .field("idle_out_lv_ch", &self.idle_out_lv_ch())
            .field("idle_out_en_ch", &self.idle_out_en_ch())
            .field("tx_stop_ch", &self.tx_stop_ch())
            .field("div_cnt_ch", &self.div_cnt_ch())
            .field("mem_size_ch", &self.mem_size_ch())
            .field("carrier_eff_en_ch", &self.carrier_eff_en_ch())
            .field("carrier_en_ch", &self.carrier_en_ch())
            .field("carrier_out_lv_ch", &self.carrier_out_lv_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to enable sending data in channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn tx_start_ch(&mut self) -> TX_START_CH_W<'_, CH_TX_CONF0_SPEC> {
        TX_START_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to reset RAM read address accessed by the transmitter for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn mem_rd_rst_ch(&mut self) -> MEM_RD_RST_CH_W<'_, CH_TX_CONF0_SPEC> {
        MEM_RD_RST_CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether to reset RAM W/R address accessed by APB FIFO for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn apb_mem_rst_ch(&mut self) -> APB_MEM_RST_CH_W<'_, CH_TX_CONF0_SPEC> {
        APB_MEM_RST_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether to enable continuous TX mode for channel %s. \\\\ 0: No Effect\\\\ 1: Enable\\\\ In this mode, the transmitter starts transmission from the first data. If an end-marker is encountered, the transmitter starts transmitting data from the first data again. if no end-marker is encountered, the transmitter starts transmitting the first data again when the last data is transmitted.\\\\"]
    #[inline(always)]
    pub fn tx_conti_mode_ch(&mut self) -> TX_CONTI_MODE_CH_W<'_, CH_TX_CONF0_SPEC> {
        TX_CONTI_MODE_CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether to enable wrap TX mode for channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\ In this mode, if the TX data size is larger than the channel's RAM block size, the transmitter continues transmitting the first data to the last data in loops.\\\\"]
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch(&mut self) -> MEM_TX_WRAP_EN_CH_W<'_, CH_TX_CONF0_SPEC> {
        MEM_TX_WRAP_EN_CH_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures the level of output signal for channel %s when the transmitter is in idle state."]
    #[inline(always)]
    pub fn idle_out_lv_ch(&mut self) -> IDLE_OUT_LV_CH_W<'_, CH_TX_CONF0_SPEC> {
        IDLE_OUT_LV_CH_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether to enable the output for channel %s in idle state. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn idle_out_en_ch(&mut self) -> IDLE_OUT_EN_CH_W<'_, CH_TX_CONF0_SPEC> {
        IDLE_OUT_EN_CH_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether to stop the transmitter of channel %s sending data out. \\\\ 0: No effect\\\\ 1: Stop\\\\"]
    #[inline(always)]
    pub fn tx_stop_ch(&mut self) -> TX_STOP_CH_W<'_, CH_TX_CONF0_SPEC> {
        TX_STOP_CH_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Configures the divider for clock of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
    #[inline(always)]
    pub fn div_cnt_ch(&mut self) -> DIV_CNT_CH_W<'_, CH_TX_CONF0_SPEC> {
        DIV_CNT_CH_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Configures the maximum number of memory blocks allocated to channel %s."]
    #[inline(always)]
    pub fn mem_size_ch(&mut self) -> MEM_SIZE_CH_W<'_, CH_TX_CONF0_SPEC> {
        MEM_SIZE_CH_W::new(self, 16)
    }
    #[doc = "Bit 20 - Configures whether to add carrier modulation on the output signal only at data-sending state for channel %s. \\\\ 0: Add carrier modulation on the output signal at data-sending state and idle state for channel %s\\\\ 1: Add carrier modulation on the output signal only at data-sending state for channel %s\\\\ Only valid when RMT_CARRIER_EN_CH%s is 1.\\\\"]
    #[inline(always)]
    pub fn carrier_eff_en_ch(&mut self) -> CARRIER_EFF_EN_CH_W<'_, CH_TX_CONF0_SPEC> {
        CARRIER_EFF_EN_CH_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether to enable the carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn carrier_en_ch(&mut self) -> CARRIER_EN_CH_W<'_, CH_TX_CONF0_SPEC> {
        CARRIER_EN_CH_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
    #[inline(always)]
    pub fn carrier_out_lv_ch(&mut self) -> CARRIER_OUT_LV_CH_W<'_, CH_TX_CONF0_SPEC> {
        CARRIER_OUT_LV_CH_W::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn afifo_rst_ch(&mut self) -> AFIFO_RST_CH_W<'_, CH_TX_CONF0_SPEC> {
        AFIFO_RST_CH_W::new(self, 23)
    }
    #[doc = "Bit 24 - Synchronization bit for channel %s."]
    #[inline(always)]
    pub fn conf_update_ch(&mut self) -> CONF_UPDATE_CH_W<'_, CH_TX_CONF0_SPEC> {
        CONF_UPDATE_CH_W::new(self, 24)
    }
}
#[doc = "Configuration register 0 for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_tx_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_tx_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_TX_CONF0_SPEC;
impl crate::RegisterSpec for CH_TX_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_tx_conf0::R`](R) reader structure"]
impl crate::Readable for CH_TX_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_tx_conf0::W`](W) writer structure"]
impl crate::Writable for CH_TX_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH%s_TX_CONF0 to value 0x0071_0200"]
impl crate::Resettable for CH_TX_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x0071_0200;
}
