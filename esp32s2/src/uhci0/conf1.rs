#[doc = "Register `CONF1` reader"]
pub struct R(crate::R<CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF1` writer"]
pub struct W(crate::W<CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF1_SPEC>;
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
impl From<crate::W<CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHECK_SUM_EN` reader - This is the enable bit to check header checksum when UHCI receives a data packet."]
pub type CHECK_SUM_EN_R = crate::BitReader;
#[doc = "Field `CHECK_SUM_EN` writer - This is the enable bit to check header checksum when UHCI receives a data packet."]
pub type CHECK_SUM_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
#[doc = "Field `CHECK_SEQ_EN` reader - This is the enable bit to check sequence number when UHCI receives a data packet."]
pub type CHECK_SEQ_EN_R = crate::BitReader;
#[doc = "Field `CHECK_SEQ_EN` writer - This is the enable bit to check sequence number when UHCI receives a data packet."]
pub type CHECK_SEQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
#[doc = "Field `CRC_DISABLE` reader - Set this bit to support CRC calculation. Data Integrity check present bit in UHCI packet frame should be 1."]
pub type CRC_DISABLE_R = crate::BitReader;
#[doc = "Field `CRC_DISABLE` writer - Set this bit to support CRC calculation. Data Integrity check present bit in UHCI packet frame should be 1."]
pub type CRC_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
#[doc = "Field `SAVE_HEAD` reader - Set this bit to save the packet header when UHCI receives a data packet."]
pub type SAVE_HEAD_R = crate::BitReader;
#[doc = "Field `SAVE_HEAD` writer - Set this bit to save the packet header when UHCI receives a data packet."]
pub type SAVE_HEAD_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
#[doc = "Field `TX_CHECK_SUM_RE` reader - Set this bit to encode the data packet with a checksum."]
pub type TX_CHECK_SUM_RE_R = crate::BitReader;
#[doc = "Field `TX_CHECK_SUM_RE` writer - Set this bit to encode the data packet with a checksum."]
pub type TX_CHECK_SUM_RE_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
#[doc = "Field `TX_ACK_NUM_RE` reader - Set this bit to encode the data packet with an acknowledgement when a reliable packet is to be transmit."]
pub type TX_ACK_NUM_RE_R = crate::BitReader;
#[doc = "Field `TX_ACK_NUM_RE` writer - Set this bit to encode the data packet with an acknowledgement when a reliable packet is to be transmit."]
pub type TX_ACK_NUM_RE_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
#[doc = "Field `CHECK_OWNER` reader - 1: Check the link list descriptor when link list owner is DMA controller; 0: Always check link list descriptor."]
pub type CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `CHECK_OWNER` writer - 1: Check the link list descriptor when link list owner is DMA controller; 0: Always check link list descriptor."]
pub type CHECK_OWNER_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
#[doc = "Field `WAIT_SW_START` reader - The UHCI encoder will jump to ST_SW_WAIT status if this register is set to 1."]
pub type WAIT_SW_START_R = crate::BitReader;
#[doc = "Field `WAIT_SW_START` writer - The UHCI encoder will jump to ST_SW_WAIT status if this register is set to 1."]
pub type WAIT_SW_START_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
#[doc = "Field `SW_START` reader - If current UHCI_ENCODE_STATE is ST_SW_WAIT, the UHCI will start to send data packet out when this bit is set to 1."]
pub type SW_START_R = crate::BitReader;
#[doc = "Field `SW_START` writer - If current UHCI_ENCODE_STATE is ST_SW_WAIT, the UHCI will start to send data packet out when this bit is set to 1."]
pub type SW_START_W<'a, const O: u8> = crate::BitWriter<'a, CONF1_SPEC, O>;
#[doc = "Field `DMA_INFIFO_FULL_THRS` reader - This field is used to generate the UHCI_DMA_INFIFO_FULL_WM_INT interrupt when the counter value of DMA RX FIFO exceeds the value of the register."]
pub type DMA_INFIFO_FULL_THRS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMA_INFIFO_FULL_THRS` writer - This field is used to generate the UHCI_DMA_INFIFO_FULL_WM_INT interrupt when the counter value of DMA RX FIFO exceeds the value of the register."]
pub type DMA_INFIFO_FULL_THRS_W<'a, const O: u8> =
    crate::FieldWriter<'a, CONF1_SPEC, 12, O, u16, u16>;
impl R {
    #[doc = "Bit 0 - This is the enable bit to check header checksum when UHCI receives a data packet."]
    #[inline(always)]
    pub fn check_sum_en(&self) -> CHECK_SUM_EN_R {
        CHECK_SUM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the enable bit to check sequence number when UHCI receives a data packet."]
    #[inline(always)]
    pub fn check_seq_en(&self) -> CHECK_SEQ_EN_R {
        CHECK_SEQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to support CRC calculation. Data Integrity check present bit in UHCI packet frame should be 1."]
    #[inline(always)]
    pub fn crc_disable(&self) -> CRC_DISABLE_R {
        CRC_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to save the packet header when UHCI receives a data packet."]
    #[inline(always)]
    pub fn save_head(&self) -> SAVE_HEAD_R {
        SAVE_HEAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to encode the data packet with a checksum."]
    #[inline(always)]
    pub fn tx_check_sum_re(&self) -> TX_CHECK_SUM_RE_R {
        TX_CHECK_SUM_RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to encode the data packet with an acknowledgement when a reliable packet is to be transmit."]
    #[inline(always)]
    pub fn tx_ack_num_re(&self) -> TX_ACK_NUM_RE_R {
        TX_ACK_NUM_RE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Check the link list descriptor when link list owner is DMA controller; 0: Always check link list descriptor."]
    #[inline(always)]
    pub fn check_owner(&self) -> CHECK_OWNER_R {
        CHECK_OWNER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The UHCI encoder will jump to ST_SW_WAIT status if this register is set to 1."]
    #[inline(always)]
    pub fn wait_sw_start(&self) -> WAIT_SW_START_R {
        WAIT_SW_START_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If current UHCI_ENCODE_STATE is ST_SW_WAIT, the UHCI will start to send data packet out when this bit is set to 1."]
    #[inline(always)]
    pub fn sw_start(&self) -> SW_START_R {
        SW_START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:20 - This field is used to generate the UHCI_DMA_INFIFO_FULL_WM_INT interrupt when the counter value of DMA RX FIFO exceeds the value of the register."]
    #[inline(always)]
    pub fn dma_infifo_full_thrs(&self) -> DMA_INFIFO_FULL_THRS_R {
        DMA_INFIFO_FULL_THRS_R::new(((self.bits >> 9) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field(
                "check_sum_en",
                &format_args!("{}", self.check_sum_en().bit()),
            )
            .field(
                "check_seq_en",
                &format_args!("{}", self.check_seq_en().bit()),
            )
            .field("crc_disable", &format_args!("{}", self.crc_disable().bit()))
            .field("save_head", &format_args!("{}", self.save_head().bit()))
            .field(
                "tx_check_sum_re",
                &format_args!("{}", self.tx_check_sum_re().bit()),
            )
            .field(
                "tx_ack_num_re",
                &format_args!("{}", self.tx_ack_num_re().bit()),
            )
            .field("check_owner", &format_args!("{}", self.check_owner().bit()))
            .field(
                "wait_sw_start",
                &format_args!("{}", self.wait_sw_start().bit()),
            )
            .field("sw_start", &format_args!("{}", self.sw_start().bit()))
            .field(
                "dma_infifo_full_thrs",
                &format_args!("{}", self.dma_infifo_full_thrs().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit to check header checksum when UHCI receives a data packet."]
    #[inline(always)]
    #[must_use]
    pub fn check_sum_en(&mut self) -> CHECK_SUM_EN_W<0> {
        CHECK_SUM_EN_W::new(self)
    }
    #[doc = "Bit 1 - This is the enable bit to check sequence number when UHCI receives a data packet."]
    #[inline(always)]
    #[must_use]
    pub fn check_seq_en(&mut self) -> CHECK_SEQ_EN_W<1> {
        CHECK_SEQ_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to support CRC calculation. Data Integrity check present bit in UHCI packet frame should be 1."]
    #[inline(always)]
    #[must_use]
    pub fn crc_disable(&mut self) -> CRC_DISABLE_W<2> {
        CRC_DISABLE_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to save the packet header when UHCI receives a data packet."]
    #[inline(always)]
    #[must_use]
    pub fn save_head(&mut self) -> SAVE_HEAD_W<3> {
        SAVE_HEAD_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to encode the data packet with a checksum."]
    #[inline(always)]
    #[must_use]
    pub fn tx_check_sum_re(&mut self) -> TX_CHECK_SUM_RE_W<4> {
        TX_CHECK_SUM_RE_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to encode the data packet with an acknowledgement when a reliable packet is to be transmit."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ack_num_re(&mut self) -> TX_ACK_NUM_RE_W<5> {
        TX_ACK_NUM_RE_W::new(self)
    }
    #[doc = "Bit 6 - 1: Check the link list descriptor when link list owner is DMA controller; 0: Always check link list descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn check_owner(&mut self) -> CHECK_OWNER_W<6> {
        CHECK_OWNER_W::new(self)
    }
    #[doc = "Bit 7 - The UHCI encoder will jump to ST_SW_WAIT status if this register is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn wait_sw_start(&mut self) -> WAIT_SW_START_W<7> {
        WAIT_SW_START_W::new(self)
    }
    #[doc = "Bit 8 - If current UHCI_ENCODE_STATE is ST_SW_WAIT, the UHCI will start to send data packet out when this bit is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn sw_start(&mut self) -> SW_START_W<8> {
        SW_START_W::new(self)
    }
    #[doc = "Bits 9:20 - This field is used to generate the UHCI_DMA_INFIFO_FULL_WM_INT interrupt when the counter value of DMA RX FIFO exceeds the value of the register."]
    #[inline(always)]
    #[must_use]
    pub fn dma_infifo_full_thrs(&mut self) -> DMA_INFIFO_FULL_THRS_W<9> {
        DMA_INFIFO_FULL_THRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHCI configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1](index.html) module"]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf1::R](R) reader structure"]
impl crate::Readable for CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf1::W](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0x33"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x33;
}
