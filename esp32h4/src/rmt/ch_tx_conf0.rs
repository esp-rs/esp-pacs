#[doc = "Register `CH%s_TX_CONF0` reader"]
pub type R = crate::R<CH_TX_CONF0_SPEC>;
#[doc = "Register `CH%s_TX_CONF0` writer"]
pub type W = crate::W<CH_TX_CONF0_SPEC>;
#[doc = "Field `TX_START` writer - Configures whether to enable sending data in channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_RD_RST` writer - Configures whether to reset RAM read address accessed by the transmitter for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type MEM_RD_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_MEM_RST` writer - Configures whether to reset RAM W/R address accessed by APB FIFO for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type APB_MEM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CONTI_MODE` reader - Configures whether to enable continuous TX mode for channel %s. \\\\ 0: No Effect\\\\ 1: Enable\\\\ In this mode, the transmitter starts transmission from the first data. If an end-marker is encountered, the transmitter starts transmitting data from the first data again. if no end-marker is encountered, the transmitter starts transmitting the first data again when the last data is transmitted.\\\\"]
pub type TX_CONTI_MODE_R = crate::BitReader;
#[doc = "Field `TX_CONTI_MODE` writer - Configures whether to enable continuous TX mode for channel %s. \\\\ 0: No Effect\\\\ 1: Enable\\\\ In this mode, the transmitter starts transmission from the first data. If an end-marker is encountered, the transmitter starts transmitting data from the first data again. if no end-marker is encountered, the transmitter starts transmitting the first data again when the last data is transmitted.\\\\"]
pub type TX_CONTI_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TX_WRAP_EN` reader - Configures whether to enable wrap TX mode for channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\ In this mode, if the TX data size is larger than the channel's RAM block size, the transmitter continues transmitting the first data to the last data in loops.\\\\"]
pub type MEM_TX_WRAP_EN_R = crate::BitReader;
#[doc = "Field `MEM_TX_WRAP_EN` writer - Configures whether to enable wrap TX mode for channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\ In this mode, if the TX data size is larger than the channel's RAM block size, the transmitter continues transmitting the first data to the last data in loops.\\\\"]
pub type MEM_TX_WRAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_OUT_LV` reader - Configures the level of output signal for channel %s when the transmitter is in idle state."]
pub type IDLE_OUT_LV_R = crate::BitReader;
#[doc = "Field `IDLE_OUT_LV` writer - Configures the level of output signal for channel %s when the transmitter is in idle state."]
pub type IDLE_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_OUT_EN` reader - Configures whether to enable the output for channel %s in idle state. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
pub type IDLE_OUT_EN_R = crate::BitReader;
#[doc = "Field `IDLE_OUT_EN` writer - Configures whether to enable the output for channel %s in idle state. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
pub type IDLE_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STOP` reader - Configures whether to stop the transmitter of channel %s sending data out. \\\\ 0: No effect\\\\ 1: Stop\\\\"]
pub type TX_STOP_R = crate::BitReader;
#[doc = "Field `TX_STOP` writer - Configures whether to stop the transmitter of channel %s sending data out. \\\\ 0: No effect\\\\ 1: Stop\\\\"]
pub type TX_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_CNT` reader - Configures the divider for clock of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
pub type DIV_CNT_R = crate::FieldReader;
#[doc = "Field `DIV_CNT` writer - Configures the divider for clock of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
pub type DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_SIZE` reader - Configures the maximum number of memory blocks allocated to channel %s."]
pub type MEM_SIZE_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE` writer - Configures the maximum number of memory blocks allocated to channel %s."]
pub type MEM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CARRIER_EFF_EN` reader - Configures whether to add carrier modulation on the output signal only at data-sending state for channel %s. \\\\ 0: Add carrier modulation on the output signal at data-sending state and idle state for channel %s\\\\ 1: Add carrier modulation on the output signal only at data-sending state for channel %s\\\\ Only valid when RMT_CARRIER_EN_CH%s is 1.\\\\"]
pub type CARRIER_EFF_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EFF_EN` writer - Configures whether to add carrier modulation on the output signal only at data-sending state for channel %s. \\\\ 0: Add carrier modulation on the output signal at data-sending state and idle state for channel %s\\\\ 1: Add carrier modulation on the output signal only at data-sending state for channel %s\\\\ Only valid when RMT_CARRIER_EN_CH%s is 1.\\\\"]
pub type CARRIER_EFF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_EN` reader - Configures whether to enable the carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CARRIER_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EN` writer - Configures whether to enable the carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CARRIER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_OUT_LV` reader - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
pub type CARRIER_OUT_LV_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV` writer - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
pub type CARRIER_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFIFO_RST` writer - Reserved"]
pub type AFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONF_UPDATE` writer - Synchronization bit for channel %s."]
pub type CONF_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Configures whether to enable continuous TX mode for channel %s. \\\\ 0: No Effect\\\\ 1: Enable\\\\ In this mode, the transmitter starts transmission from the first data. If an end-marker is encountered, the transmitter starts transmitting data from the first data again. if no end-marker is encountered, the transmitter starts transmitting the first data again when the last data is transmitted.\\\\"]
    #[inline(always)]
    pub fn tx_conti_mode(&self) -> TX_CONTI_MODE_R {
        TX_CONTI_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether to enable wrap TX mode for channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\ In this mode, if the TX data size is larger than the channel's RAM block size, the transmitter continues transmitting the first data to the last data in loops.\\\\"]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&self) -> MEM_TX_WRAP_EN_R {
        MEM_TX_WRAP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures the level of output signal for channel %s when the transmitter is in idle state."]
    #[inline(always)]
    pub fn idle_out_lv(&self) -> IDLE_OUT_LV_R {
        IDLE_OUT_LV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether to enable the output for channel %s in idle state. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn idle_out_en(&self) -> IDLE_OUT_EN_R {
        IDLE_OUT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether to stop the transmitter of channel %s sending data out. \\\\ 0: No effect\\\\ 1: Stop\\\\"]
    #[inline(always)]
    pub fn tx_stop(&self) -> TX_STOP_R {
        TX_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Configures the divider for clock of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Configures the maximum number of memory blocks allocated to channel %s."]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Configures whether to add carrier modulation on the output signal only at data-sending state for channel %s. \\\\ 0: Add carrier modulation on the output signal at data-sending state and idle state for channel %s\\\\ 1: Add carrier modulation on the output signal only at data-sending state for channel %s\\\\ Only valid when RMT_CARRIER_EN_CH%s is 1.\\\\"]
    #[inline(always)]
    pub fn carrier_eff_en(&self) -> CARRIER_EFF_EN_R {
        CARRIER_EFF_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures whether to enable the carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_CONF0")
            .field("tx_conti_mode", &self.tx_conti_mode())
            .field("mem_tx_wrap_en", &self.mem_tx_wrap_en())
            .field("idle_out_lv", &self.idle_out_lv())
            .field("idle_out_en", &self.idle_out_en())
            .field("tx_stop", &self.tx_stop())
            .field("div_cnt", &self.div_cnt())
            .field("mem_size", &self.mem_size())
            .field("carrier_eff_en", &self.carrier_eff_en())
            .field("carrier_en", &self.carrier_en())
            .field("carrier_out_lv", &self.carrier_out_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to enable sending data in channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W<'_, CH_TX_CONF0_SPEC> {
        TX_START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to reset RAM read address accessed by the transmitter for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn mem_rd_rst(&mut self) -> MEM_RD_RST_W<'_, CH_TX_CONF0_SPEC> {
        MEM_RD_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether to reset RAM W/R address accessed by APB FIFO for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W<'_, CH_TX_CONF0_SPEC> {
        APB_MEM_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether to enable continuous TX mode for channel %s. \\\\ 0: No Effect\\\\ 1: Enable\\\\ In this mode, the transmitter starts transmission from the first data. If an end-marker is encountered, the transmitter starts transmitting data from the first data again. if no end-marker is encountered, the transmitter starts transmitting the first data again when the last data is transmitted.\\\\"]
    #[inline(always)]
    pub fn tx_conti_mode(&mut self) -> TX_CONTI_MODE_W<'_, CH_TX_CONF0_SPEC> {
        TX_CONTI_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether to enable wrap TX mode for channel %s. \\\\ 0: No effect\\\\ 1: Enable\\\\ In this mode, if the TX data size is larger than the channel's RAM block size, the transmitter continues transmitting the first data to the last data in loops.\\\\"]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&mut self) -> MEM_TX_WRAP_EN_W<'_, CH_TX_CONF0_SPEC> {
        MEM_TX_WRAP_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures the level of output signal for channel %s when the transmitter is in idle state."]
    #[inline(always)]
    pub fn idle_out_lv(&mut self) -> IDLE_OUT_LV_W<'_, CH_TX_CONF0_SPEC> {
        IDLE_OUT_LV_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether to enable the output for channel %s in idle state. \\\\ 0: No effect\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn idle_out_en(&mut self) -> IDLE_OUT_EN_W<'_, CH_TX_CONF0_SPEC> {
        IDLE_OUT_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether to stop the transmitter of channel %s sending data out. \\\\ 0: No effect\\\\ 1: Stop\\\\"]
    #[inline(always)]
    pub fn tx_stop(&mut self) -> TX_STOP_W<'_, CH_TX_CONF0_SPEC> {
        TX_STOP_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Configures the divider for clock of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
    #[inline(always)]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<'_, CH_TX_CONF0_SPEC> {
        DIV_CNT_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Configures the maximum number of memory blocks allocated to channel %s."]
    #[inline(always)]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<'_, CH_TX_CONF0_SPEC> {
        MEM_SIZE_W::new(self, 16)
    }
    #[doc = "Bit 20 - Configures whether to add carrier modulation on the output signal only at data-sending state for channel %s. \\\\ 0: Add carrier modulation on the output signal at data-sending state and idle state for channel %s\\\\ 1: Add carrier modulation on the output signal only at data-sending state for channel %s\\\\ Only valid when RMT_CARRIER_EN_CH%s is 1.\\\\"]
    #[inline(always)]
    pub fn carrier_eff_en(&mut self) -> CARRIER_EFF_EN_W<'_, CH_TX_CONF0_SPEC> {
        CARRIER_EFF_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether to enable the carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<'_, CH_TX_CONF0_SPEC> {
        CARRIER_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
    #[inline(always)]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<'_, CH_TX_CONF0_SPEC> {
        CARRIER_OUT_LV_W::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn afifo_rst(&mut self) -> AFIFO_RST_W<'_, CH_TX_CONF0_SPEC> {
        AFIFO_RST_W::new(self, 23)
    }
    #[doc = "Bit 24 - Synchronization bit for channel %s."]
    #[inline(always)]
    pub fn conf_update(&mut self) -> CONF_UPDATE_W<'_, CH_TX_CONF0_SPEC> {
        CONF_UPDATE_W::new(self, 24)
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
