#[doc = "Register `CH%s_RX_CONF0` reader"]
pub type R = crate::R<CH_RX_CONF0_SPEC>;
#[doc = "Register `CH%s_RX_CONF0` writer"]
pub type W = crate::W<CH_RX_CONF0_SPEC>;
#[doc = "Field `DIV_CNT_CH` reader - Configures the clock divider of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
pub type DIV_CNT_CH_R = crate::FieldReader;
#[doc = "Field `DIV_CNT_CH` writer - Configures the clock divider of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
pub type DIV_CNT_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDLE_THRES_CH` reader - Configures RX threshold. \\\\ When no edge is detected on the input signal for continuous clock cycles longer than this field value, the receiver stops receiving data.\\\\ Measurement unit: clk_div\\\\"]
pub type IDLE_THRES_CH_R = crate::FieldReader<u16>;
#[doc = "Field `IDLE_THRES_CH` writer - Configures RX threshold. \\\\ When no edge is detected on the input signal for continuous clock cycles longer than this field value, the receiver stops receiving data.\\\\ Measurement unit: clk_div\\\\"]
pub type IDLE_THRES_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MEM_SIZE_CH` reader - Configures the maximum number of memory blocks allocated to channel %s."]
pub type MEM_SIZE_CH_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE_CH` writer - Configures the maximum number of memory blocks allocated to channel %s."]
pub type MEM_SIZE_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CARRIER_EN_CH` reader - Configures whether to enable carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CARRIER_EN_CH_R = crate::BitReader;
#[doc = "Field `CARRIER_EN_CH` writer - Configures whether to enable carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CARRIER_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_OUT_LV_CH` reader - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
pub type CARRIER_OUT_LV_CH_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV_CH` writer - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
pub type CARRIER_OUT_LV_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Configures the clock divider of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
    #[inline(always)]
    pub fn div_cnt_ch(&self) -> DIV_CNT_CH_R {
        DIV_CNT_CH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:22 - Configures RX threshold. \\\\ When no edge is detected on the input signal for continuous clock cycles longer than this field value, the receiver stops receiving data.\\\\ Measurement unit: clk_div\\\\"]
    #[inline(always)]
    pub fn idle_thres_ch(&self) -> IDLE_THRES_CH_R {
        IDLE_THRES_CH_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bits 23:25 - Configures the maximum number of memory blocks allocated to channel %s."]
    #[inline(always)]
    pub fn mem_size_ch(&self) -> MEM_SIZE_CH_R {
        MEM_SIZE_CH_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 28 - Configures whether to enable carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn carrier_en_ch(&self) -> CARRIER_EN_CH_R {
        CARRIER_EN_CH_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
    #[inline(always)]
    pub fn carrier_out_lv_ch(&self) -> CARRIER_OUT_LV_CH_R {
        CARRIER_OUT_LV_CH_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_CONF0")
            .field("div_cnt_ch", &self.div_cnt_ch())
            .field("idle_thres_ch", &self.idle_thres_ch())
            .field("mem_size_ch", &self.mem_size_ch())
            .field("carrier_en_ch", &self.carrier_en_ch())
            .field("carrier_out_lv_ch", &self.carrier_out_lv_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the clock divider of channel %s. \\\\ Measurement unit: rmt_sclk\\\\"]
    #[inline(always)]
    pub fn div_cnt_ch(&mut self) -> DIV_CNT_CH_W<'_, CH_RX_CONF0_SPEC> {
        DIV_CNT_CH_W::new(self, 0)
    }
    #[doc = "Bits 8:22 - Configures RX threshold. \\\\ When no edge is detected on the input signal for continuous clock cycles longer than this field value, the receiver stops receiving data.\\\\ Measurement unit: clk_div\\\\"]
    #[inline(always)]
    pub fn idle_thres_ch(&mut self) -> IDLE_THRES_CH_W<'_, CH_RX_CONF0_SPEC> {
        IDLE_THRES_CH_W::new(self, 8)
    }
    #[doc = "Bits 23:25 - Configures the maximum number of memory blocks allocated to channel %s."]
    #[inline(always)]
    pub fn mem_size_ch(&mut self) -> MEM_SIZE_CH_W<'_, CH_RX_CONF0_SPEC> {
        MEM_SIZE_CH_W::new(self, 23)
    }
    #[doc = "Bit 28 - Configures whether to enable carrier modulation on output signal for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn carrier_en_ch(&mut self) -> CARRIER_EN_CH_W<'_, CH_RX_CONF0_SPEC> {
        CARRIER_EN_CH_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures the position of carrier wave for channel %s. \\\\ 0: Add carrier wave on low level\\\\ 1: Add carrier wave on high level\\\\"]
    #[inline(always)]
    pub fn carrier_out_lv_ch(&mut self) -> CARRIER_OUT_LV_CH_W<'_, CH_RX_CONF0_SPEC> {
        CARRIER_OUT_LV_CH_W::new(self, 29)
    }
}
#[doc = "Configuration register 0 for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_rx_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_rx_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_CONF0_SPEC;
impl crate::RegisterSpec for CH_RX_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_conf0::R`](R) reader structure"]
impl crate::Readable for CH_RX_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_rx_conf0::W`](W) writer structure"]
impl crate::Writable for CH_RX_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH%s_RX_CONF0 to value 0x30ff_ff02"]
impl crate::Resettable for CH_RX_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x30ff_ff02;
}
