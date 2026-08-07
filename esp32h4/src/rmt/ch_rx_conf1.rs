#[doc = "Register `CH%s_RX_CONF1` reader"]
pub type R = crate::R<CH_RX_CONF1_SPEC>;
#[doc = "Register `CH%s_RX_CONF1` writer"]
pub type W = crate::W<CH_RX_CONF1_SPEC>;
#[doc = "Field `RX_EN_CH` reader - Configures whether to enable the receiver to start receiving data in channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type RX_EN_CH_R = crate::BitReader;
#[doc = "Field `RX_EN_CH` writer - Configures whether to enable the receiver to start receiving data in channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type RX_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_WR_RST_CH` writer - Configures whether to reset RAM write address accessed by the receiver for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type MEM_WR_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_MEM_RST_CH` writer - Configures whether to reset RAM W/R address accessed by APB FIFO for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type APB_MEM_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_OWNER_CH` reader - Configures the ownership of channel %s's RAM block. \\\\ 0: APB bus is using the RAM\\\\ 1: Receiver is using the RAM\\\\"]
pub type MEM_OWNER_CH_R = crate::BitReader;
#[doc = "Field `MEM_OWNER_CH` writer - Configures the ownership of channel %s's RAM block. \\\\ 0: APB bus is using the RAM\\\\ 1: Receiver is using the RAM\\\\"]
pub type MEM_OWNER_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FILTER_EN_CH` reader - Configures whether to enable the receiver's filter for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type RX_FILTER_EN_CH_R = crate::BitReader;
#[doc = "Field `RX_FILTER_EN_CH` writer - Configures whether to enable the receiver's filter for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type RX_FILTER_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FILTER_THRES_CH` reader - Configures whether the receiver, when receiving data, ignores the input pulse when its width is shorter than this register value in units of rmt_sclk cycles. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type RX_FILTER_THRES_CH_R = crate::FieldReader;
#[doc = "Field `RX_FILTER_THRES_CH` writer - Configures whether the receiver, when receiving data, ignores the input pulse when its width is shorter than this register value in units of rmt_sclk cycles. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
pub type RX_FILTER_THRES_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_RX_WRAP_EN_CH` reader - Configures whether to enable wrap RX mode for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\ In this mode, if the RX data size is larger than channel %s's RAM block size, the receiver stores the RX data from the first address to the last address in loops.\\\\"]
pub type MEM_RX_WRAP_EN_CH_R = crate::BitReader;
#[doc = "Field `MEM_RX_WRAP_EN_CH` writer - Configures whether to enable wrap RX mode for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\ In this mode, if the RX data size is larger than channel %s's RAM block size, the receiver stores the RX data from the first address to the last address in loops.\\\\"]
pub type MEM_RX_WRAP_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFIFO_RST_CH` writer - Reserved"]
pub type AFIFO_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONF_UPDATE_CH` writer - Synchronization bit for channel %s."]
pub type CONF_UPDATE_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether to enable the receiver to start receiving data in channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn rx_en_ch(&self) -> RX_EN_CH_R {
        RX_EN_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the ownership of channel %s's RAM block. \\\\ 0: APB bus is using the RAM\\\\ 1: Receiver is using the RAM\\\\"]
    #[inline(always)]
    pub fn mem_owner_ch(&self) -> MEM_OWNER_CH_R {
        MEM_OWNER_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether to enable the receiver's filter for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn rx_filter_en_ch(&self) -> RX_FILTER_EN_CH_R {
        RX_FILTER_EN_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - Configures whether the receiver, when receiving data, ignores the input pulse when its width is shorter than this register value in units of rmt_sclk cycles. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn rx_filter_thres_ch(&self) -> RX_FILTER_THRES_CH_R {
        RX_FILTER_THRES_CH_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - Configures whether to enable wrap RX mode for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\ In this mode, if the RX data size is larger than channel %s's RAM block size, the receiver stores the RX data from the first address to the last address in loops.\\\\"]
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch(&self) -> MEM_RX_WRAP_EN_CH_R {
        MEM_RX_WRAP_EN_CH_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_RX_CONF1")
            .field("rx_en_ch", &self.rx_en_ch())
            .field("mem_owner_ch", &self.mem_owner_ch())
            .field("rx_filter_en_ch", &self.rx_filter_en_ch())
            .field("rx_filter_thres_ch", &self.rx_filter_thres_ch())
            .field("mem_rx_wrap_en_ch", &self.mem_rx_wrap_en_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to enable the receiver to start receiving data in channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn rx_en_ch(&mut self) -> RX_EN_CH_W<'_, CH_RX_CONF1_SPEC> {
        RX_EN_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to reset RAM write address accessed by the receiver for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn mem_wr_rst_ch(&mut self) -> MEM_WR_RST_CH_W<'_, CH_RX_CONF1_SPEC> {
        MEM_WR_RST_CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether to reset RAM W/R address accessed by APB FIFO for channel %s. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn apb_mem_rst_ch(&mut self) -> APB_MEM_RST_CH_W<'_, CH_RX_CONF1_SPEC> {
        APB_MEM_RST_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the ownership of channel %s's RAM block. \\\\ 0: APB bus is using the RAM\\\\ 1: Receiver is using the RAM\\\\"]
    #[inline(always)]
    pub fn mem_owner_ch(&mut self) -> MEM_OWNER_CH_W<'_, CH_RX_CONF1_SPEC> {
        MEM_OWNER_CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether to enable the receiver's filter for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn rx_filter_en_ch(&mut self) -> RX_FILTER_EN_CH_W<'_, CH_RX_CONF1_SPEC> {
        RX_FILTER_EN_CH_W::new(self, 4)
    }
    #[doc = "Bits 5:12 - Configures whether the receiver, when receiving data, ignores the input pulse when its width is shorter than this register value in units of rmt_sclk cycles. \\\\ 0: No effect\\\\ 1: Reset\\\\"]
    #[inline(always)]
    pub fn rx_filter_thres_ch(&mut self) -> RX_FILTER_THRES_CH_W<'_, CH_RX_CONF1_SPEC> {
        RX_FILTER_THRES_CH_W::new(self, 5)
    }
    #[doc = "Bit 13 - Configures whether to enable wrap RX mode for channel %s. \\\\ 0: Disable\\\\ 1: Enable\\\\ In this mode, if the RX data size is larger than channel %s's RAM block size, the receiver stores the RX data from the first address to the last address in loops.\\\\"]
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch(&mut self) -> MEM_RX_WRAP_EN_CH_W<'_, CH_RX_CONF1_SPEC> {
        MEM_RX_WRAP_EN_CH_W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn afifo_rst_ch(&mut self) -> AFIFO_RST_CH_W<'_, CH_RX_CONF1_SPEC> {
        AFIFO_RST_CH_W::new(self, 14)
    }
    #[doc = "Bit 15 - Synchronization bit for channel %s."]
    #[inline(always)]
    pub fn conf_update_ch(&mut self) -> CONF_UPDATE_CH_W<'_, CH_RX_CONF1_SPEC> {
        CONF_UPDATE_CH_W::new(self, 15)
    }
}
#[doc = "Configuration register 1 for channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_rx_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_rx_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_RX_CONF1_SPEC;
impl crate::RegisterSpec for CH_RX_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_rx_conf1::R`](R) reader structure"]
impl crate::Readable for CH_RX_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_rx_conf1::W`](W) writer structure"]
impl crate::Writable for CH_RX_CONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH%s_RX_CONF1 to value 0x01e8"]
impl crate::Resettable for CH_RX_CONF1_SPEC {
    const RESET_VALUE: u32 = 0x01e8;
}
