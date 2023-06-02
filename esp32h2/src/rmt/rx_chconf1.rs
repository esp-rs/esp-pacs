#[doc = "Register `RX_CH%sCONF1` reader"]
pub struct R(crate::R<RX_CHCONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CHCONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CHCONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CHCONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CH%sCONF1` writer"]
pub struct W(crate::W<RX_CHCONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CHCONF1_SPEC>;
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
impl From<crate::W<RX_CHCONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CHCONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EN_CH2` reader - Set this bit to enable receiver to receive data on CHANNEL%s."]
pub type RX_EN_CH2_R = crate::BitReader;
#[doc = "Field `RX_EN_CH2` writer - Set this bit to enable receiver to receive data on CHANNEL%s."]
pub type RX_EN_CH2_W<'a, const O: u8> = crate::BitWriter<'a, RX_CHCONF1_SPEC, O>;
#[doc = "Field `MEM_WR_RST_CH2` writer - Set this bit to reset write ram address for CHANNEL%s by accessing receiver."]
pub type MEM_WR_RST_CH2_W<'a, const O: u8> = crate::BitWriter<'a, RX_CHCONF1_SPEC, O>;
#[doc = "Field `APB_MEM_RST_CH2` writer - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
pub type APB_MEM_RST_CH2_W<'a, const O: u8> = crate::BitWriter<'a, RX_CHCONF1_SPEC, O>;
#[doc = "Field `MEM_OWNER_CH2` reader - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
pub type MEM_OWNER_CH2_R = crate::BitReader;
#[doc = "Field `MEM_OWNER_CH2` writer - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
pub type MEM_OWNER_CH2_W<'a, const O: u8> = crate::BitWriter<'a, RX_CHCONF1_SPEC, O>;
#[doc = "Field `RX_FILTER_EN_CH2` reader - This is the receive filter's enable bit for CHANNEL%s."]
pub type RX_FILTER_EN_CH2_R = crate::BitReader;
#[doc = "Field `RX_FILTER_EN_CH2` writer - This is the receive filter's enable bit for CHANNEL%s."]
pub type RX_FILTER_EN_CH2_W<'a, const O: u8> = crate::BitWriter<'a, RX_CHCONF1_SPEC, O>;
#[doc = "Field `RX_FILTER_THRES_CH2` reader - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
pub type RX_FILTER_THRES_CH2_R = crate::FieldReader;
#[doc = "Field `RX_FILTER_THRES_CH2` writer - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
pub type RX_FILTER_THRES_CH2_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CHCONF1_SPEC, 8, O>;
#[doc = "Field `MEM_RX_WRAP_EN_CH2` reader - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
pub type MEM_RX_WRAP_EN_CH2_R = crate::BitReader;
#[doc = "Field `MEM_RX_WRAP_EN_CH2` writer - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
pub type MEM_RX_WRAP_EN_CH2_W<'a, const O: u8> = crate::BitWriter<'a, RX_CHCONF1_SPEC, O>;
#[doc = "Field `AFIFO_RST_CH2` writer - Reserved"]
pub type AFIFO_RST_CH2_W<'a, const O: u8> = crate::BitWriter<'a, RX_CHCONF1_SPEC, O>;
#[doc = "Field `CONF_UPDATE_CH2` writer - synchronization bit for CHANNEL%s"]
pub type CONF_UPDATE_CH2_W<'a, const O: u8> = crate::BitWriter<'a, RX_CHCONF1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable receiver to receive data on CHANNEL%s."]
    #[inline(always)]
    pub fn rx_en_ch2(&self) -> RX_EN_CH2_R {
        RX_EN_CH2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
    #[inline(always)]
    pub fn mem_owner_ch2(&self) -> MEM_OWNER_CH2_R {
        MEM_OWNER_CH2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the receive filter's enable bit for CHANNEL%s."]
    #[inline(always)]
    pub fn rx_filter_en_ch2(&self) -> RX_FILTER_EN_CH2_R {
        RX_FILTER_EN_CH2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
    #[inline(always)]
    pub fn rx_filter_thres_ch2(&self) -> RX_FILTER_THRES_CH2_R {
        RX_FILTER_THRES_CH2_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
    #[inline(always)]
    pub fn mem_rx_wrap_en_ch2(&self) -> MEM_RX_WRAP_EN_CH2_R {
        MEM_RX_WRAP_EN_CH2_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CHCONF1")
            .field("rx_en_ch2", &format_args!("{}", self.rx_en_ch2().bit()))
            .field(
                "mem_owner_ch2",
                &format_args!("{}", self.mem_owner_ch2().bit()),
            )
            .field(
                "rx_filter_en_ch2",
                &format_args!("{}", self.rx_filter_en_ch2().bit()),
            )
            .field(
                "rx_filter_thres_ch2",
                &format_args!("{}", self.rx_filter_thres_ch2().bits()),
            )
            .field(
                "mem_rx_wrap_en_ch2",
                &format_args!("{}", self.mem_rx_wrap_en_ch2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CHCONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable receiver to receive data on CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn rx_en_ch2(&mut self) -> RX_EN_CH2_W<0> {
        RX_EN_CH2_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to reset write ram address for CHANNEL%s by accessing receiver."]
    #[inline(always)]
    #[must_use]
    pub fn mem_wr_rst_ch2(&mut self) -> MEM_WR_RST_CH2_W<1> {
        MEM_WR_RST_CH2_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
    #[inline(always)]
    #[must_use]
    pub fn apb_mem_rst_ch2(&mut self) -> APB_MEM_RST_CH2_W<2> {
        APB_MEM_RST_CH2_W::new(self)
    }
    #[doc = "Bit 3 - This register marks the ownership of CHANNEL%s's ram block. 1'h1: Receiver is using the ram. 1'h0: APB bus is using the ram."]
    #[inline(always)]
    #[must_use]
    pub fn mem_owner_ch2(&mut self) -> MEM_OWNER_CH2_W<3> {
        MEM_OWNER_CH2_W::new(self)
    }
    #[doc = "Bit 4 - This is the receive filter's enable bit for CHANNEL%s."]
    #[inline(always)]
    #[must_use]
    pub fn rx_filter_en_ch2(&mut self) -> RX_FILTER_EN_CH2_W<4> {
        RX_FILTER_EN_CH2_W::new(self)
    }
    #[doc = "Bits 5:12 - Ignores the input pulse when its width is smaller than this register value in APB clock periods (in receive mode)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_filter_thres_ch2(&mut self) -> RX_FILTER_THRES_CH2_W<5> {
        RX_FILTER_THRES_CH2_W::new(self)
    }
    #[doc = "Bit 13 - This is the channel %s enable bit for wraparound mode: it will resume receiving at the start when the data to be received is more than its memory size."]
    #[inline(always)]
    #[must_use]
    pub fn mem_rx_wrap_en_ch2(&mut self) -> MEM_RX_WRAP_EN_CH2_W<13> {
        MEM_RX_WRAP_EN_CH2_W::new(self)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn afifo_rst_ch2(&mut self) -> AFIFO_RST_CH2_W<14> {
        AFIFO_RST_CH2_W::new(self)
    }
    #[doc = "Bit 15 - synchronization bit for CHANNEL%s"]
    #[inline(always)]
    #[must_use]
    pub fn conf_update_ch2(&mut self) -> CONF_UPDATE_CH2_W<15> {
        CONF_UPDATE_CH2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel %s configure register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_chconf1](index.html) module"]
pub struct RX_CHCONF1_SPEC;
impl crate::RegisterSpec for RX_CHCONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_chconf1::R](R) reader structure"]
impl crate::Readable for RX_CHCONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_chconf1::W](W) writer structure"]
impl crate::Writable for RX_CHCONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CH%sCONF1 to value 0x01e8"]
impl crate::Resettable for RX_CHCONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01e8;
}
