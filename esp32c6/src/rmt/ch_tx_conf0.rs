#[doc = "Register `CH%s_TX_CONF0` reader"]
pub struct R(crate::R<CH_TX_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_TX_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_TX_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_TX_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_TX_CONF0` writer"]
pub struct W(crate::W<CH_TX_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_TX_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CH_TX_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_TX_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_START` writer - Set this bit to start sending data on CHANNEL%s."]
pub type TX_START_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `MEM_RD_RST` writer - Set this bit to reset read ram address for CHANNEL%s by accessing transmitter."]
pub type MEM_RD_RST_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `APB_MEM_RST` writer - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
pub type APB_MEM_RST_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `TX_CONTI_MODE` reader - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
pub type TX_CONTI_MODE_R = crate::BitReader;
#[doc = "Field `TX_CONTI_MODE` writer - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
pub type TX_CONTI_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `MEM_TX_WRAP_EN` reader - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
pub type MEM_TX_WRAP_EN_R = crate::BitReader;
#[doc = "Field `MEM_TX_WRAP_EN` writer - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
pub type MEM_TX_WRAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `IDLE_OUT_LV` reader - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
pub type IDLE_OUT_LV_R = crate::BitReader;
#[doc = "Field `IDLE_OUT_LV` writer - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
pub type IDLE_OUT_LV_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `IDLE_OUT_EN` reader - This is the output enable-control bit for CHANNEL%s in IDLE state."]
pub type IDLE_OUT_EN_R = crate::BitReader;
#[doc = "Field `IDLE_OUT_EN` writer - This is the output enable-control bit for CHANNEL%s in IDLE state."]
pub type IDLE_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `TX_STOP` reader - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
pub type TX_STOP_R = crate::BitReader;
#[doc = "Field `TX_STOP` writer - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
pub type TX_STOP_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `DIV_CNT` reader - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DIV_CNT_R = crate::FieldReader;
#[doc = "Field `DIV_CNT` writer - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DIV_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, CH_TX_CONF0_SPEC, 8, O>;
#[doc = "Field `MEM_SIZE` reader - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MEM_SIZE_R = crate::FieldReader;
#[doc = "Field `MEM_SIZE` writer - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MEM_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, CH_TX_CONF0_SPEC, 3, O>;
#[doc = "Field `CARRIER_EFF_EN` reader - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
pub type CARRIER_EFF_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EFF_EN` writer - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
pub type CARRIER_EFF_EN_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `CARRIER_EN` reader - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CARRIER_EN_R = crate::BitReader;
#[doc = "Field `CARRIER_EN` writer - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CARRIER_EN_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `CARRIER_OUT_LV` reader - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
pub type CARRIER_OUT_LV_R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV` writer - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
pub type CARRIER_OUT_LV_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `AFIFO_RST` writer - Reserved"]
pub type AFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
#[doc = "Field `CONF_UPDATE` writer - synchronization bit for CHANNEL%s"]
pub type CONF_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_CONF0_SPEC, O>;
impl R {
    #[doc = "Bit 3 - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
    #[inline(always)]
    pub fn tx_conti_mode(&self) -> TX_CONTI_MODE_R {
        TX_CONTI_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&self) -> MEM_TX_WRAP_EN_R {
        MEM_TX_WRAP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
    #[inline(always)]
    pub fn idle_out_lv(&self) -> IDLE_OUT_LV_R {
        IDLE_OUT_LV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the output enable-control bit for CHANNEL%s in IDLE state."]
    #[inline(always)]
    pub fn idle_out_en(&self) -> IDLE_OUT_EN_R {
        IDLE_OUT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
    #[inline(always)]
    pub fn tx_stop(&self) -> TX_STOP_R {
        TX_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
    #[inline(always)]
    pub fn carrier_eff_en(&self) -> CARRIER_EFF_EN_R {
        CARRIER_EFF_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_CONF0")
            .field(
                "tx_conti_mode",
                &format_args!("{}", self.tx_conti_mode().bit()),
            )
            .field(
                "mem_tx_wrap_en",
                &format_args!("{}", self.mem_tx_wrap_en().bit()),
            )
            .field("idle_out_lv", &format_args!("{}", self.idle_out_lv().bit()))
            .field("idle_out_en", &format_args!("{}", self.idle_out_en().bit()))
            .field("tx_stop", &format_args!("{}", self.tx_stop().bit()))
            .field("div_cnt", &format_args!("{}", self.div_cnt().bits()))
            .field("mem_size", &format_args!("{}", self.mem_size().bits()))
            .field(
                "carrier_eff_en",
                &format_args!("{}", self.carrier_eff_en().bit()),
            )
            .field("carrier_en", &format_args!("{}", self.carrier_en().bit()))
            .field(
                "carrier_out_lv",
                &format_args!("{}", self.carrier_out_lv().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_TX_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to start sending data on CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<0> {
        TX_START_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to reset read ram address for CHANNEL%s by accessing transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn mem_rd_rst(&mut self) -> MEM_RD_RST_W<1> {
        MEM_RD_RST_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
    #[inline(always)]
    #[must_use]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W<2> {
        APB_MEM_RST_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn tx_conti_mode(&mut self) -> TX_CONTI_MODE_W<3> {
        TX_CONTI_MODE_W::new(self)
    }
    #[doc = "Bit 4 - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
    #[inline(always)]
    #[must_use]
    pub fn mem_tx_wrap_en(&mut self) -> MEM_TX_WRAP_EN_W<4> {
        MEM_TX_WRAP_EN_W::new(self)
    }
    #[doc = "Bit 5 - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
    #[inline(always)]
    #[must_use]
    pub fn idle_out_lv(&mut self) -> IDLE_OUT_LV_W<5> {
        IDLE_OUT_LV_W::new(self)
    }
    #[doc = "Bit 6 - This is the output enable-control bit for CHANNEL%s in IDLE state."]
    #[inline(always)]
    #[must_use]
    pub fn idle_out_en(&mut self) -> IDLE_OUT_EN_W<6> {
        IDLE_OUT_EN_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
    #[inline(always)]
    #[must_use]
    pub fn tx_stop(&mut self) -> TX_STOP_W<7> {
        TX_STOP_W::new(self)
    }
    #[doc = "Bits 8:15 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<8> {
        DIV_CNT_W::new(self)
    }
    #[doc = "Bits 16:18 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<16> {
        MEM_SIZE_W::new(self)
    }
    #[doc = "Bit 20 - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_eff_en(&mut self) -> CARRIER_EFF_EN_W<20> {
        CARRIER_EFF_EN_W::new(self)
    }
    #[doc = "Bit 21 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<21> {
        CARRIER_EN_W::new(self)
    }
    #[doc = "Bit 22 - This bit is used to configure the position of carrier wave for CHANNEL%s. 1'h0: add carrier wave on low level. 1'h1: add carrier wave on high level."]
    #[inline(always)]
    #[must_use]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<22> {
        CARRIER_OUT_LV_W::new(self)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn afifo_rst(&mut self) -> AFIFO_RST_W<23> {
        AFIFO_RST_W::new(self)
    }
    #[doc = "Bit 24 - synchronization bit for CHANNEL%s"]
    #[inline(always)]
    #[must_use]
    pub fn conf_update(&mut self) -> CONF_UPDATE_W<24> {
        CONF_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s configure register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_tx_conf0](index.html) module"]
pub struct CH_TX_CONF0_SPEC;
impl crate::RegisterSpec for CH_TX_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_tx_conf0::R](R) reader structure"]
impl crate::Readable for CH_TX_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_tx_conf0::W](W) writer structure"]
impl crate::Writable for CH_TX_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_TX_CONF0 to value 0x0071_0200"]
impl crate::Resettable for CH_TX_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0071_0200;
}
