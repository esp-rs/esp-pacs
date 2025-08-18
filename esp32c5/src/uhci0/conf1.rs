#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `CHECK_SUM_EN` reader - Configures whether or not to enable header checksum validation when UHCI receives a data packet.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CHECK_SUM_EN_R = crate::BitReader;
#[doc = "Field `CHECK_SUM_EN` writer - Configures whether or not to enable header checksum validation when UHCI receives a data packet.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CHECK_SUM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK_SEQ_EN` reader - Configures whether or not to enable the sequence number check when UHCI receives a data packet.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CHECK_SEQ_EN_R = crate::BitReader;
#[doc = "Field `CHECK_SEQ_EN` writer - Configures whether or not to enable the sequence number check when UHCI receives a data packet.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CHECK_SEQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_DISABLE` reader - Configures whether or not to enable CRC calculation.\\\\ 0: Disable\\\\ 1: Enable\\\\ Valid only when the Data Integrity Check Present bit in UHCI packet is 1.\\\\"]
pub type CRC_DISABLE_R = crate::BitReader;
#[doc = "Field `CRC_DISABLE` writer - Configures whether or not to enable CRC calculation.\\\\ 0: Disable\\\\ 1: Enable\\\\ Valid only when the Data Integrity Check Present bit in UHCI packet is 1.\\\\"]
pub type CRC_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAVE_HEAD` reader - Configures whether or not to save the packet header when UHCI receives a data packet.\\\\ 0: Not save\\\\ 1: Save\\\\"]
pub type SAVE_HEAD_R = crate::BitReader;
#[doc = "Field `SAVE_HEAD` writer - Configures whether or not to save the packet header when UHCI receives a data packet.\\\\ 0: Not save\\\\ 1: Save\\\\"]
pub type SAVE_HEAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CHECK_SUM_RE` reader - Configures whether or not to encode the data packet with a checksum.\\\\ 0: Not use checksum\\\\ 1: Use checksum\\\\"]
pub type TX_CHECK_SUM_RE_R = crate::BitReader;
#[doc = "Field `TX_CHECK_SUM_RE` writer - Configures whether or not to encode the data packet with a checksum.\\\\ 0: Not use checksum\\\\ 1: Use checksum\\\\"]
pub type TX_CHECK_SUM_RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ACK_NUM_RE` reader - Configures whether or not to encode the data packet with an acknowledgment when a reliable packet is to be transmitted.\\\\ 0: Not use acknowledgement\\\\ 1: Use acknowledgement\\\\"]
pub type TX_ACK_NUM_RE_R = crate::BitReader;
#[doc = "Field `TX_ACK_NUM_RE` writer - Configures whether or not to encode the data packet with an acknowledgment when a reliable packet is to be transmitted.\\\\ 0: Not use acknowledgement\\\\ 1: Use acknowledgement\\\\"]
pub type TX_ACK_NUM_RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT_SW_START` reader - Configures whether or not to put the UHCI encoder state machine to ST_SW_WAIT state.\\\\ 0: No\\\\ 1: Yes\\\\"]
pub type WAIT_SW_START_R = crate::BitReader;
#[doc = "Field `WAIT_SW_START` writer - Configures whether or not to put the UHCI encoder state machine to ST_SW_WAIT state.\\\\ 0: No\\\\ 1: Yes\\\\"]
pub type WAIT_SW_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_START` writer - Write 1 to send data packets when the encoder state machine is in ST_SW_WAIT state."]
pub type SW_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable header checksum validation when UHCI receives a data packet.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn check_sum_en(&self) -> CHECK_SUM_EN_R {
        CHECK_SUM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable the sequence number check when UHCI receives a data packet.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn check_seq_en(&self) -> CHECK_SEQ_EN_R {
        CHECK_SEQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable CRC calculation.\\\\ 0: Disable\\\\ 1: Enable\\\\ Valid only when the Data Integrity Check Present bit in UHCI packet is 1.\\\\"]
    #[inline(always)]
    pub fn crc_disable(&self) -> CRC_DISABLE_R {
        CRC_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to save the packet header when UHCI receives a data packet.\\\\ 0: Not save\\\\ 1: Save\\\\"]
    #[inline(always)]
    pub fn save_head(&self) -> SAVE_HEAD_R {
        SAVE_HEAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to encode the data packet with a checksum.\\\\ 0: Not use checksum\\\\ 1: Use checksum\\\\"]
    #[inline(always)]
    pub fn tx_check_sum_re(&self) -> TX_CHECK_SUM_RE_R {
        TX_CHECK_SUM_RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to encode the data packet with an acknowledgment when a reliable packet is to be transmitted.\\\\ 0: Not use acknowledgement\\\\ 1: Use acknowledgement\\\\"]
    #[inline(always)]
    pub fn tx_ack_num_re(&self) -> TX_ACK_NUM_RE_R {
        TX_ACK_NUM_RE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to put the UHCI encoder state machine to ST_SW_WAIT state.\\\\ 0: No\\\\ 1: Yes\\\\"]
    #[inline(always)]
    pub fn wait_sw_start(&self) -> WAIT_SW_START_R {
        WAIT_SW_START_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("check_sum_en", &self.check_sum_en())
            .field("check_seq_en", &self.check_seq_en())
            .field("crc_disable", &self.crc_disable())
            .field("save_head", &self.save_head())
            .field("tx_check_sum_re", &self.tx_check_sum_re())
            .field("tx_ack_num_re", &self.tx_ack_num_re())
            .field("wait_sw_start", &self.wait_sw_start())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable header checksum validation when UHCI receives a data packet.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn check_sum_en(&mut self) -> CHECK_SUM_EN_W<'_, CONF1_SPEC> {
        CHECK_SUM_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable the sequence number check when UHCI receives a data packet.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn check_seq_en(&mut self) -> CHECK_SEQ_EN_W<'_, CONF1_SPEC> {
        CHECK_SEQ_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable CRC calculation.\\\\ 0: Disable\\\\ 1: Enable\\\\ Valid only when the Data Integrity Check Present bit in UHCI packet is 1.\\\\"]
    #[inline(always)]
    pub fn crc_disable(&mut self) -> CRC_DISABLE_W<'_, CONF1_SPEC> {
        CRC_DISABLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to save the packet header when UHCI receives a data packet.\\\\ 0: Not save\\\\ 1: Save\\\\"]
    #[inline(always)]
    pub fn save_head(&mut self) -> SAVE_HEAD_W<'_, CONF1_SPEC> {
        SAVE_HEAD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to encode the data packet with a checksum.\\\\ 0: Not use checksum\\\\ 1: Use checksum\\\\"]
    #[inline(always)]
    pub fn tx_check_sum_re(&mut self) -> TX_CHECK_SUM_RE_W<'_, CONF1_SPEC> {
        TX_CHECK_SUM_RE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to encode the data packet with an acknowledgment when a reliable packet is to be transmitted.\\\\ 0: Not use acknowledgement\\\\ 1: Use acknowledgement\\\\"]
    #[inline(always)]
    pub fn tx_ack_num_re(&mut self) -> TX_ACK_NUM_RE_W<'_, CONF1_SPEC> {
        TX_ACK_NUM_RE_W::new(self, 5)
    }
    #[doc = "Bit 7 - Configures whether or not to put the UHCI encoder state machine to ST_SW_WAIT state.\\\\ 0: No\\\\ 1: Yes\\\\"]
    #[inline(always)]
    pub fn wait_sw_start(&mut self) -> WAIT_SW_START_W<'_, CONF1_SPEC> {
        WAIT_SW_START_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to send data packets when the encoder state machine is in ST_SW_WAIT state."]
    #[inline(always)]
    pub fn sw_start(&mut self) -> SW_START_W<'_, CONF1_SPEC> {
        SW_START_W::new(self, 8)
    }
}
#[doc = "UHCI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF1 to value 0x33"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0x33;
}
