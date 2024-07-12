#[doc = "Register `CH%sCONF1` reader"]
pub type R = crate::R<CHCONF1_SPEC>;
#[doc = "Register `CH%sCONF1` writer"]
pub type W = crate::W<CHCONF1_SPEC>;
#[doc = "Field `TX_START` reader - Set this bit to start sending data on CHANNEL%s."]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - Set this bit to start sending data on CHANNEL%s."]
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EN` reader - Set this bit to enable receiver to receive data on CHANNEL%s."]
pub type RX_EN_R = crate::BitReader;
#[doc = "Field `RX_EN` writer - Set this bit to enable receiver to receive data on CHANNEL%s."]
pub type RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_WR_RST` writer - Set this bit to reset write ram address for CHANNEL%s by accessing receiver."]
pub type MEM_WR_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_RD_RST` writer - Set this bit to reset read ram address for CHANNEL%s by accessing transmitter."]
pub type MEM_RD_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_MEM_RST` writer - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
pub type APB_MEM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_OWNER` reader - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: Transmitter is using the ram."]
pub type MEM_OWNER_R = crate::BitReader;
#[doc = "Field `MEM_OWNER` writer - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: Transmitter is using the ram."]
pub type MEM_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CONTI_MODE` reader - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
pub type TX_CONTI_MODE_R = crate::BitReader;
#[doc = "Field `TX_CONTI_MODE` writer - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
pub type TX_CONTI_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FILTER_EN` reader - This is the receive filter's enable bit for CHANNEL%s."]
pub type RX_FILTER_EN_R = crate::BitReader;
#[doc = "Field `RX_FILTER_EN` writer - This is the receive filter's enable bit for CHANNEL%s."]
pub type RX_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FILTER_THRES` reader - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
pub type RX_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `RX_FILTER_THRES` writer - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
pub type RX_FILTER_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHK_RX_CARRIER_EN` reader - Set this bit to enable memory loop read mode when carrier modulation is enabled for channel %s."]
pub type CHK_RX_CARRIER_EN_R = crate::BitReader;
#[doc = "Field `CHK_RX_CARRIER_EN` writer - Set this bit to enable memory loop read mode when carrier modulation is enabled for channel %s."]
pub type CHK_RX_CARRIER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_ALWAYS_ON` reader - This bit is used to select the base clock for CHANNEL%s. 1'h1: clk_apb 1'h0:clk_ref"]
pub type REF_ALWAYS_ON_R = crate::BitReader;
#[doc = "Field `REF_ALWAYS_ON` writer - This bit is used to select the base clock for CHANNEL%s. 1'h1: clk_apb 1'h0:clk_ref"]
pub type REF_ALWAYS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_OUT_LV` reader - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
pub type IDLE_OUT_LV_R = crate::BitReader;
#[doc = "Field `IDLE_OUT_LV` writer - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
pub type IDLE_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_OUT_EN` reader - This is the output enable-control bit for CHANNEL%s in IDLE state."]
pub type IDLE_OUT_EN_R = crate::BitReader;
#[doc = "Field `IDLE_OUT_EN` writer - This is the output enable-control bit for CHANNEL%s in IDLE state."]
pub type IDLE_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STOP` reader - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
pub type TX_STOP_R = crate::BitReader;
#[doc = "Field `TX_STOP` writer - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
pub type TX_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to start sending data on CHANNEL%s."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable receiver to receive data on CHANNEL%s."]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: Transmitter is using the ram."]
    #[inline(always)]
    pub fn mem_owner(&self) -> MEM_OWNER_R {
        MEM_OWNER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
    #[inline(always)]
    pub fn tx_conti_mode(&self) -> TX_CONTI_MODE_R {
        TX_CONTI_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the receive filter's enable bit for CHANNEL%s."]
    #[inline(always)]
    pub fn rx_filter_en(&self) -> RX_FILTER_EN_R {
        RX_FILTER_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
    #[inline(always)]
    pub fn rx_filter_thres(&self) -> RX_FILTER_THRES_R {
        RX_FILTER_THRES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Set this bit to enable memory loop read mode when carrier modulation is enabled for channel %s."]
    #[inline(always)]
    pub fn chk_rx_carrier_en(&self) -> CHK_RX_CARRIER_EN_R {
        CHK_RX_CARRIER_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit is used to select the base clock for CHANNEL%s. 1'h1: clk_apb 1'h0:clk_ref"]
    #[inline(always)]
    pub fn ref_always_on(&self) -> REF_ALWAYS_ON_R {
        REF_ALWAYS_ON_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
    #[inline(always)]
    pub fn idle_out_lv(&self) -> IDLE_OUT_LV_R {
        IDLE_OUT_LV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This is the output enable-control bit for CHANNEL%s in IDLE state."]
    #[inline(always)]
    pub fn idle_out_en(&self) -> IDLE_OUT_EN_R {
        IDLE_OUT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
    #[inline(always)]
    pub fn tx_stop(&self) -> TX_STOP_R {
        TX_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHCONF1")
            .field("tx_start", &self.tx_start())
            .field("rx_en", &self.rx_en())
            .field("mem_owner", &self.mem_owner())
            .field("tx_conti_mode", &self.tx_conti_mode())
            .field("rx_filter_en", &self.rx_filter_en())
            .field("rx_filter_thres", &self.rx_filter_thres())
            .field("chk_rx_carrier_en", &self.chk_rx_carrier_en())
            .field("ref_always_on", &self.ref_always_on())
            .field("idle_out_lv", &self.idle_out_lv())
            .field("idle_out_en", &self.idle_out_en())
            .field("tx_stop", &self.tx_stop())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to start sending data on CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<CHCONF1_SPEC> {
        TX_START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to enable receiver to receive data on CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn rx_en(&mut self) -> RX_EN_W<CHCONF1_SPEC> {
        RX_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to reset write ram address for CHANNEL%s by accessing receiver."]
    #[inline(always)]
    #[must_use]
    pub fn mem_wr_rst(&mut self) -> MEM_WR_RST_W<CHCONF1_SPEC> {
        MEM_WR_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to reset read ram address for CHANNEL%s by accessing transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn mem_rd_rst(&mut self) -> MEM_RD_RST_W<CHCONF1_SPEC> {
        MEM_RD_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
    #[inline(always)]
    #[must_use]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W<CHCONF1_SPEC> {
        APB_MEM_RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: Transmitter is using the ram."]
    #[inline(always)]
    #[must_use]
    pub fn mem_owner(&mut self) -> MEM_OWNER_W<CHCONF1_SPEC> {
        MEM_OWNER_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn tx_conti_mode(&mut self) -> TX_CONTI_MODE_W<CHCONF1_SPEC> {
        TX_CONTI_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - This is the receive filter's enable bit for CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn rx_filter_en(&mut self) -> RX_FILTER_EN_W<CHCONF1_SPEC> {
        RX_FILTER_EN_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_filter_thres(&mut self) -> RX_FILTER_THRES_W<CHCONF1_SPEC> {
        RX_FILTER_THRES_W::new(self, 8)
    }
    #[doc = "Bit 16 - Set this bit to enable memory loop read mode when carrier modulation is enabled for channel %s."]
    #[inline(always)]
    #[must_use]
    pub fn chk_rx_carrier_en(&mut self) -> CHK_RX_CARRIER_EN_W<CHCONF1_SPEC> {
        CHK_RX_CARRIER_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - This bit is used to select the base clock for CHANNEL%s. 1'h1: clk_apb 1'h0:clk_ref"]
    #[inline(always)]
    #[must_use]
    pub fn ref_always_on(&mut self) -> REF_ALWAYS_ON_W<CHCONF1_SPEC> {
        REF_ALWAYS_ON_W::new(self, 17)
    }
    #[doc = "Bit 18 - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
    #[inline(always)]
    #[must_use]
    pub fn idle_out_lv(&mut self) -> IDLE_OUT_LV_W<CHCONF1_SPEC> {
        IDLE_OUT_LV_W::new(self, 18)
    }
    #[doc = "Bit 19 - This is the output enable-control bit for CHANNEL%s in IDLE state."]
    #[inline(always)]
    #[must_use]
    pub fn idle_out_en(&mut self) -> IDLE_OUT_EN_W<CHCONF1_SPEC> {
        IDLE_OUT_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
    #[inline(always)]
    #[must_use]
    pub fn tx_stop(&mut self) -> TX_STOP_W<CHCONF1_SPEC> {
        TX_STOP_W::new(self, 20)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%sCONF1 to value 0x0f20"]
impl crate::Resettable for CHCONF1_SPEC {
    const RESET_VALUE: u32 = 0x0f20;
}
