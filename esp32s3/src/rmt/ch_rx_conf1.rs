#[doc = "Register `CH%s_RX_CONF1` reader"]
pub struct R(crate::R<CH_RX_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_RX_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_RX_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_RX_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_RX_CONF1` writer"]
pub struct W(crate::W<CH_RX_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_RX_CONF1_SPEC>;
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
impl From<crate::W<CH_RX_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_RX_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EN` reader - Set this bit to enable receiver to receive data on CHANNEL%s."]
pub type RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_EN` writer - Set this bit to enable receiver to receive data on CHANNEL%s."]
pub type RX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_RX_CONF1_SPEC, bool, O>;
#[doc = "Field `MEM_WR_RST` writer - Set this bit to reset write ram address for CHANNEL%s by accessing receiver."]
pub type MEM_WR_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_RX_CONF1_SPEC, bool, O>;
#[doc = "Field `APB_MEM_RST` writer - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
pub type APB_MEM_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_RX_CONF1_SPEC, bool, O>;
#[doc = "Field `MEM_OWNER` reader - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
pub type MEM_OWNER_R = crate::BitReader<bool>;
#[doc = "Field `MEM_OWNER` writer - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
pub type MEM_OWNER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_RX_CONF1_SPEC, bool, O>;
#[doc = "Field `RX_FILTER_EN` reader - This is the receive filter's enable bit for CHANNEL%s."]
pub type RX_FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_FILTER_EN` writer - This is the receive filter's enable bit for CHANNEL%s."]
pub type RX_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_RX_CONF1_SPEC, bool, O>;
#[doc = "Field `RX_FILTER_THRES` reader - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
pub type RX_FILTER_THRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_FILTER_THRES` writer - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
pub type RX_FILTER_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH_RX_CONF1_SPEC, u8, u8, 8, O>;
#[doc = "Field `MEM_RX_WRAP_EN` reader - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
pub type MEM_RX_WRAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `MEM_RX_WRAP_EN` writer - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
pub type MEM_RX_WRAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_RX_CONF1_SPEC, bool, O>;
#[doc = "Field `AFIFO_RST` writer - Reserved"]
pub type AFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_RX_CONF1_SPEC, bool, O>;
#[doc = "Field `CONF_UPDATE` writer - synchronization bit for CHANNEL%s"]
pub type CONF_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_RX_CONF1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable receiver to receive data on CHANNEL%s."]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
    #[inline(always)]
    pub fn mem_owner(&self) -> MEM_OWNER_R {
        MEM_OWNER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the receive filter's enable bit for CHANNEL%s."]
    #[inline(always)]
    pub fn rx_filter_en(&self) -> RX_FILTER_EN_R {
        RX_FILTER_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
    #[inline(always)]
    pub fn rx_filter_thres(&self) -> RX_FILTER_THRES_R {
        RX_FILTER_THRES_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
    #[inline(always)]
    pub fn mem_rx_wrap_en(&self) -> MEM_RX_WRAP_EN_R {
        MEM_RX_WRAP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable receiver to receive data on CHANNEL%s."]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RX_EN_W<0> {
        RX_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to reset write ram address for CHANNEL%s by accessing receiver."]
    #[inline(always)]
    pub fn mem_wr_rst(&mut self) -> MEM_WR_RST_W<1> {
        MEM_WR_RST_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
    #[inline(always)]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W<2> {
        APB_MEM_RST_W::new(self)
    }
    #[doc = "Bit 3 - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
    #[inline(always)]
    pub fn mem_owner(&mut self) -> MEM_OWNER_W<3> {
        MEM_OWNER_W::new(self)
    }
    #[doc = "Bit 4 - This is the receive filter's enable bit for CHANNEL%s."]
    #[inline(always)]
    pub fn rx_filter_en(&mut self) -> RX_FILTER_EN_W<4> {
        RX_FILTER_EN_W::new(self)
    }
    #[doc = "Bits 5:12 - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
    #[inline(always)]
    pub fn rx_filter_thres(&mut self) -> RX_FILTER_THRES_W<5> {
        RX_FILTER_THRES_W::new(self)
    }
    #[doc = "Bit 13 - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
    #[inline(always)]
    pub fn mem_rx_wrap_en(&mut self) -> MEM_RX_WRAP_EN_W<13> {
        MEM_RX_WRAP_EN_W::new(self)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn afifo_rst(&mut self) -> AFIFO_RST_W<14> {
        AFIFO_RST_W::new(self)
    }
    #[doc = "Bit 15 - synchronization bit for CHANNEL%s"]
    #[inline(always)]
    pub fn conf_update(&mut self) -> CONF_UPDATE_W<15> {
        CONF_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s configure register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_rx_conf1](index.html) module"]
pub struct CH_RX_CONF1_SPEC;
impl crate::RegisterSpec for CH_RX_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_rx_conf1::R](R) reader structure"]
impl crate::Readable for CH_RX_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_rx_conf1::W](W) writer structure"]
impl crate::Writable for CH_RX_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH%s_RX_CONF1 to value 0x01e8"]
impl crate::Resettable for CH_RX_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01e8
    }
}
