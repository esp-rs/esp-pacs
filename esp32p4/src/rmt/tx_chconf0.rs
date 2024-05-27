///Register `TX_CH%sCONF0` reader
pub type R = crate::R<TX_CHCONF0_SPEC>;
///Register `TX_CH%sCONF0` writer
pub type W = crate::W<TX_CHCONF0_SPEC>;
///Field `TX_START_CH0` writer - Set this bit to start sending data on CHANNEL%s.
pub type TX_START_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MEM_RD_RST_CH0` writer - Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
pub type MEM_RD_RST_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB_MEM_RST_CH0` writer - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
pub type APB_MEM_RST_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_CONTI_MODE_CH0` reader - Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
pub type TX_CONTI_MODE_CH0_R = crate::BitReader;
///Field `TX_CONTI_MODE_CH0` writer - Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
pub type TX_CONTI_MODE_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MEM_TX_WRAP_EN_CH0` reader - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
pub type MEM_TX_WRAP_EN_CH0_R = crate::BitReader;
///Field `MEM_TX_WRAP_EN_CH0` writer - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
pub type MEM_TX_WRAP_EN_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLE_OUT_LV_CH0` reader - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
pub type IDLE_OUT_LV_CH0_R = crate::BitReader;
///Field `IDLE_OUT_LV_CH0` writer - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
pub type IDLE_OUT_LV_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLE_OUT_EN_CH0` reader - This is the output enable-control bit for CHANNEL%s in IDLE state.
pub type IDLE_OUT_EN_CH0_R = crate::BitReader;
///Field `IDLE_OUT_EN_CH0` writer - This is the output enable-control bit for CHANNEL%s in IDLE state.
pub type IDLE_OUT_EN_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_STOP_CH0` reader - Set this bit to stop the transmitter of CHANNEL%s sending data out.
pub type TX_STOP_CH0_R = crate::BitReader;
///Field `TX_STOP_CH0` writer - Set this bit to stop the transmitter of CHANNEL%s sending data out.
pub type TX_STOP_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIV_CNT_CH0` reader - This register is used to configure the divider for clock of CHANNEL%s.
pub type DIV_CNT_CH0_R = crate::FieldReader;
///Field `DIV_CNT_CH0` writer - This register is used to configure the divider for clock of CHANNEL%s.
pub type DIV_CNT_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEM_SIZE_CH0` reader - This register is used to configure the maximum size of memory allocated to CHANNEL%s.
pub type MEM_SIZE_CH0_R = crate::FieldReader;
///Field `MEM_SIZE_CH0` writer - This register is used to configure the maximum size of memory allocated to CHANNEL%s.
pub type MEM_SIZE_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CARRIER_EFF_EN_CH0` reader - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
pub type CARRIER_EFF_EN_CH0_R = crate::BitReader;
///Field `CARRIER_EFF_EN_CH0` writer - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
pub type CARRIER_EFF_EN_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARRIER_EN_CH0` reader - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
pub type CARRIER_EN_CH0_R = crate::BitReader;
///Field `CARRIER_EN_CH0` writer - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
pub type CARRIER_EN_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARRIER_OUT_LV_CH0` reader - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
pub type CARRIER_OUT_LV_CH0_R = crate::BitReader;
///Field `CARRIER_OUT_LV_CH0` writer - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
pub type CARRIER_OUT_LV_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFIFO_RST_CH0` writer - Reserved
pub type AFIFO_RST_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONF_UPDATE_CH0` writer - synchronization bit for CHANNEL%s
pub type CONF_UPDATE_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    pub fn tx_conti_mode_ch0(&self) -> TX_CONTI_MODE_CH0_R {
        TX_CONTI_MODE_CH0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch0(&self) -> MEM_TX_WRAP_EN_CH0_R {
        MEM_TX_WRAP_EN_CH0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    pub fn idle_out_lv_ch0(&self) -> IDLE_OUT_LV_CH0_R {
        IDLE_OUT_LV_CH0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    pub fn idle_out_en_ch0(&self) -> IDLE_OUT_EN_CH0_R {
        IDLE_OUT_EN_CH0_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    pub fn tx_stop_ch0(&self) -> TX_STOP_CH0_R {
        TX_STOP_CH0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    pub fn div_cnt_ch0(&self) -> DIV_CNT_CH0_R {
        DIV_CNT_CH0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    pub fn mem_size_ch0(&self) -> MEM_SIZE_CH0_R {
        MEM_SIZE_CH0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    pub fn carrier_eff_en_ch0(&self) -> CARRIER_EFF_EN_CH0_R {
        CARRIER_EFF_EN_CH0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    pub fn carrier_en_ch0(&self) -> CARRIER_EN_CH0_R {
        CARRIER_EN_CH0_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    pub fn carrier_out_lv_ch0(&self) -> CARRIER_OUT_LV_CH0_R {
        CARRIER_OUT_LV_CH0_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CHCONF0")
            .field("tx_conti_mode_ch0", &self.tx_conti_mode_ch0())
            .field("mem_tx_wrap_en_ch0", &self.mem_tx_wrap_en_ch0())
            .field("idle_out_lv_ch0", &self.idle_out_lv_ch0())
            .field("idle_out_en_ch0", &self.idle_out_en_ch0())
            .field("tx_stop_ch0", &self.tx_stop_ch0())
            .field("div_cnt_ch0", &self.div_cnt_ch0())
            .field("mem_size_ch0", &self.mem_size_ch0())
            .field("carrier_eff_en_ch0", &self.carrier_eff_en_ch0())
            .field("carrier_en_ch0", &self.carrier_en_ch0())
            .field("carrier_out_lv_ch0", &self.carrier_out_lv_ch0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to start sending data on CHANNEL%s.
    #[inline(always)]
    #[must_use]
    pub fn tx_start_ch0(&mut self) -> TX_START_CH0_W<TX_CHCONF0_SPEC> {
        TX_START_CH0_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to reset read ram address for CHANNEL%s by accessing transmitter.
    #[inline(always)]
    #[must_use]
    pub fn mem_rd_rst_ch0(&mut self) -> MEM_RD_RST_CH0_W<TX_CHCONF0_SPEC> {
        MEM_RD_RST_CH0_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo.
    #[inline(always)]
    #[must_use]
    pub fn apb_mem_rst_ch0(&mut self) -> APB_MEM_RST_CH0_W<TX_CHCONF0_SPEC> {
        APB_MEM_RST_CH0_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to restart transmission from the first data to the last data in CHANNEL%s.
    #[inline(always)]
    #[must_use]
    pub fn tx_conti_mode_ch0(&mut self) -> TX_CONTI_MODE_CH0_W<TX_CHCONF0_SPEC> {
        TX_CONTI_MODE_CH0_W::new(self, 3)
    }
    ///Bit 4 - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size.
    #[inline(always)]
    #[must_use]
    pub fn mem_tx_wrap_en_ch0(&mut self) -> MEM_TX_WRAP_EN_CH0_W<TX_CHCONF0_SPEC> {
        MEM_TX_WRAP_EN_CH0_W::new(self, 4)
    }
    ///Bit 5 - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state.
    #[inline(always)]
    #[must_use]
    pub fn idle_out_lv_ch0(&mut self) -> IDLE_OUT_LV_CH0_W<TX_CHCONF0_SPEC> {
        IDLE_OUT_LV_CH0_W::new(self, 5)
    }
    ///Bit 6 - This is the output enable-control bit for CHANNEL%s in IDLE state.
    #[inline(always)]
    #[must_use]
    pub fn idle_out_en_ch0(&mut self) -> IDLE_OUT_EN_CH0_W<TX_CHCONF0_SPEC> {
        IDLE_OUT_EN_CH0_W::new(self, 6)
    }
    ///Bit 7 - Set this bit to stop the transmitter of CHANNEL%s sending data out.
    #[inline(always)]
    #[must_use]
    pub fn tx_stop_ch0(&mut self) -> TX_STOP_CH0_W<TX_CHCONF0_SPEC> {
        TX_STOP_CH0_W::new(self, 7)
    }
    ///Bits 8:15 - This register is used to configure the divider for clock of CHANNEL%s.
    #[inline(always)]
    #[must_use]
    pub fn div_cnt_ch0(&mut self) -> DIV_CNT_CH0_W<TX_CHCONF0_SPEC> {
        DIV_CNT_CH0_W::new(self, 8)
    }
    ///Bits 16:19 - This register is used to configure the maximum size of memory allocated to CHANNEL%s.
    #[inline(always)]
    #[must_use]
    pub fn mem_size_ch0(&mut self) -> MEM_SIZE_CH0_W<TX_CHCONF0_SPEC> {
        MEM_SIZE_CH0_W::new(self, 16)
    }
    ///Bit 20 - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1.
    #[inline(always)]
    #[must_use]
    pub fn carrier_eff_en_ch0(&mut self) -> CARRIER_EFF_EN_CH0_W<TX_CHCONF0_SPEC> {
        CARRIER_EFF_EN_CH0_W::new(self, 20)
    }
    ///Bit 21 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out.
    #[inline(always)]
    #[must_use]
    pub fn carrier_en_ch0(&mut self) -> CARRIER_EN_CH0_W<TX_CHCONF0_SPEC> {
        CARRIER_EN_CH0_W::new(self, 21)
    }
    ///Bit 22 - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level.
    #[inline(always)]
    #[must_use]
    pub fn carrier_out_lv_ch0(&mut self) -> CARRIER_OUT_LV_CH0_W<TX_CHCONF0_SPEC> {
        CARRIER_OUT_LV_CH0_W::new(self, 22)
    }
    ///Bit 23 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn afifo_rst_ch0(&mut self) -> AFIFO_RST_CH0_W<TX_CHCONF0_SPEC> {
        AFIFO_RST_CH0_W::new(self, 23)
    }
    ///Bit 24 - synchronization bit for CHANNEL%s
    #[inline(always)]
    #[must_use]
    pub fn conf_update_ch0(&mut self) -> CONF_UPDATE_CH0_W<TX_CHCONF0_SPEC> {
        CONF_UPDATE_CH0_W::new(self, 24)
    }
}
/**Channel %s configure register 0

You can [`read`](crate::generic::Reg::read) this register and get [`tx_chconf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_chconf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_CHCONF0_SPEC;
impl crate::RegisterSpec for TX_CHCONF0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_chconf0::R`](R) reader structure
impl crate::Readable for TX_CHCONF0_SPEC {}
///`write(|w| ..)` method takes [`tx_chconf0::W`](W) writer structure
impl crate::Writable for TX_CHCONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_CH%sCONF0 to value 0x0071_0200
impl crate::Resettable for TX_CHCONF0_SPEC {
    const RESET_VALUE: u32 = 0x0071_0200;
}
