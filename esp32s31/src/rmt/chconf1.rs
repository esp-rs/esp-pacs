#[doc = "Register `CH%sCONF1` reader"]
pub type R = crate::R<CHCONF1_SPEC>;
#[doc = "Register `CH%sCONF1` writer"]
pub type W = crate::W<CHCONF1_SPEC>;
#[doc = "Field `RX_EN_CH` reader - Set this bit to enable receiver to receive data on CHANNEL%s."]
pub type RX_EN_CH_R = crate::BitReader;
#[doc = "Field `RX_EN_CH` writer - Set this bit to enable receiver to receive data on CHANNEL%s."]
pub type RX_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_WR_RST_CH` writer - Set this bit to reset write ram address for CHANNEL%s by accessing receiver."]
pub type MEM_WR_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_MEM_RST_CH` writer - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
pub type APB_MEM_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_OWNER_CH` reader - This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
pub type MEM_OWNER_CH_R = crate::BitReader;
#[doc = "Field `MEM_OWNER_CH` writer - This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
pub type MEM_OWNER_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FILTER_EN_CH` reader - This is the receive filter's enable bit for CHANNEL%s."]
pub type RX_FILTER_EN_CH_R = crate::BitReader;
#[doc = "Field `RX_FILTER_EN_CH` writer - This is the receive filter's enable bit for CHANNEL%s."]
pub type RX_FILTER_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FILTER_THRES_CH` reader - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
pub type RX_FILTER_THRES_CH_R = crate::FieldReader;
#[doc = "Field `RX_FILTER_THRES_CH` writer - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
pub type RX_FILTER_THRES_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_RX_WRAP_EN_CH` reader - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
pub type MEM_RX_WRAP_EN_CH_R = crate::BitReader;
#[doc = "Field `MEM_RX_WRAP_EN_CH` writer - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
pub type MEM_RX_WRAP_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFIFO_RST_CH` writer - Reserved"]
pub type AFIFO_RST_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONF_UPDATE_CH` writer - synchronization bit for CHANNEL%s"]
pub type CONF_UPDATE_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable receiver to receive data on CHANNEL%s."]
    #[inline(always)]
    pub fn rx_en_ch(&self) -> RX_EN_CH_R {
        RX_EN_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
    #[inline(always)]
    pub fn mem_owner_ch(&self) -> MEM_OWNER_CH_R {
        MEM_OWNER_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the receive filter's enable bit for CHANNEL%s."]
    #[inline(always)]
    pub fn rx_filter_en_ch(&self) -> RX_FILTER_EN_CH_R {
        RX_FILTER_EN_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
    #[inline(always)]
    pub fn rx_filter_thres_ch(&self) -> RX_FILTER_THRES_CH_R {
        RX_FILTER_THRES_CH_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch(&self) -> MEM_RX_WRAP_EN_CH_R {
        MEM_RX_WRAP_EN_CH_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHCONF1")
            .field("rx_en_ch", &self.rx_en_ch())
            .field("mem_owner_ch", &self.mem_owner_ch())
            .field("rx_filter_en_ch", &self.rx_filter_en_ch())
            .field("rx_filter_thres_ch", &self.rx_filter_thres_ch())
            .field("mem_rx_wrap_en_ch", &self.mem_rx_wrap_en_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable receiver to receive data on CHANNEL%s."]
    #[inline(always)]
    pub fn rx_en_ch(&mut self) -> RX_EN_CH_W<'_, CHCONF1_SPEC> {
        RX_EN_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to reset write ram address for CHANNEL%s by accessing receiver."]
    #[inline(always)]
    pub fn mem_wr_rst_ch(&mut self) -> MEM_WR_RST_CH_W<'_, CHCONF1_SPEC> {
        MEM_WR_RST_CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
    #[inline(always)]
    pub fn apb_mem_rst_ch(&mut self) -> APB_MEM_RST_CH_W<'_, CHCONF1_SPEC> {
        APB_MEM_RST_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - This register marks the ownership of CHANNEL%s's ram block.1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
    #[inline(always)]
    pub fn mem_owner_ch(&mut self) -> MEM_OWNER_CH_W<'_, CHCONF1_SPEC> {
        MEM_OWNER_CH_W::new(self, 3)
    }
    #[doc = "Bit 4 - This is the receive filter's enable bit for CHANNEL%s."]
    #[inline(always)]
    pub fn rx_filter_en_ch(&mut self) -> RX_FILTER_EN_CH_W<'_, CHCONF1_SPEC> {
        RX_FILTER_EN_CH_W::new(self, 4)
    }
    #[doc = "Bits 5:12 - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
    #[inline(always)]
    pub fn rx_filter_thres_ch(&mut self) -> RX_FILTER_THRES_CH_W<'_, CHCONF1_SPEC> {
        RX_FILTER_THRES_CH_W::new(self, 5)
    }
    #[doc = "Bit 13 - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch(&mut self) -> MEM_RX_WRAP_EN_CH_W<'_, CHCONF1_SPEC> {
        MEM_RX_WRAP_EN_CH_W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn afifo_rst_ch(&mut self) -> AFIFO_RST_CH_W<'_, CHCONF1_SPEC> {
        AFIFO_RST_CH_W::new(self, 14)
    }
    #[doc = "Bit 15 - synchronization bit for CHANNEL%s"]
    #[inline(always)]
    pub fn conf_update_ch(&mut self) -> CONF_UPDATE_CH_W<'_, CHCONF1_SPEC> {
        CONF_UPDATE_CH_W::new(self, 15)
    }
}
#[doc = "Channel %s configure register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`chconf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chconf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHCONF1_SPEC;
impl crate::RegisterSpec for CHCONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chconf1::R`](R) reader structure"]
impl crate::Readable for CHCONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chconf1::W`](W) writer structure"]
impl crate::Writable for CHCONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH%sCONF1 to value 0x01e8"]
impl crate::Resettable for CHCONF1_SPEC {
    const RESET_VALUE: u32 = 0x01e8;
}
