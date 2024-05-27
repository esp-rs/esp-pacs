///Register `CONF1` reader
pub type R = crate::R<CONF1_SPEC>;
///Register `CONF1` writer
pub type W = crate::W<CONF1_SPEC>;
///Field `CHECK_SUM_EN` reader - Set this bit to enable decoder to check check_sum in packet header.
pub type CHECK_SUM_EN_R = crate::BitReader;
///Field `CHECK_SUM_EN` writer - Set this bit to enable decoder to check check_sum in packet header.
pub type CHECK_SUM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHECK_SEQ_EN` reader - Set this bit to enable decoder to check seq num in packet header.
pub type CHECK_SEQ_EN_R = crate::BitReader;
///Field `CHECK_SEQ_EN` writer - Set this bit to enable decoder to check seq num in packet header.
pub type CHECK_SEQ_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC_DISABLE` reader - Set this bit to disable crc calculation.
pub type CRC_DISABLE_R = crate::BitReader;
///Field `CRC_DISABLE` writer - Set this bit to disable crc calculation.
pub type CRC_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAVE_HEAD` reader - Set this bit to save packet header .
pub type SAVE_HEAD_R = crate::BitReader;
///Field `SAVE_HEAD` writer - Set this bit to save packet header .
pub type SAVE_HEAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_CHECK_SUM_RE` reader - Set this bit to enable hardware replace check_sum in packet header automatically.
pub type TX_CHECK_SUM_RE_R = crate::BitReader;
///Field `TX_CHECK_SUM_RE` writer - Set this bit to enable hardware replace check_sum in packet header automatically.
pub type TX_CHECK_SUM_RE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_ACK_NUM_RE` reader - Set this bit to enable hardware replace ack num in packet header automatically.
pub type TX_ACK_NUM_RE_R = crate::BitReader;
///Field `TX_ACK_NUM_RE` writer - Set this bit to enable hardware replace ack num in packet header automatically.
pub type TX_ACK_NUM_RE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHECK_OWNER` reader - Set this bit to check the owner bit in link descriptor.
pub type CHECK_OWNER_R = crate::BitReader;
///Field `CHECK_OWNER` writer - Set this bit to check the owner bit in link descriptor.
pub type CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAIT_SW_START` reader - Set this bit to enable software way to add packet header.
pub type WAIT_SW_START_R = crate::BitReader;
///Field `WAIT_SW_START` writer - Set this bit to enable software way to add packet header.
pub type WAIT_SW_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW_START` reader - Set this bit to start inserting the packet header.
pub type SW_START_R = crate::BitReader;
///Field `SW_START` writer - Set this bit to start inserting the packet header.
pub type SW_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_INFIFO_FULL_THRS` reader - when data amount in link descriptor's fifo is more than this register value it will produce uhci_dma_infifo_full_wm_int interrupt.
pub type DMA_INFIFO_FULL_THRS_R = crate::FieldReader<u16>;
///Field `DMA_INFIFO_FULL_THRS` writer - when data amount in link descriptor's fifo is more than this register value it will produce uhci_dma_infifo_full_wm_int interrupt.
pub type DMA_INFIFO_FULL_THRS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bit 0 - Set this bit to enable decoder to check check_sum in packet header.
    #[inline(always)]
    pub fn check_sum_en(&self) -> CHECK_SUM_EN_R {
        CHECK_SUM_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set this bit to enable decoder to check seq num in packet header.
    #[inline(always)]
    pub fn check_seq_en(&self) -> CHECK_SEQ_EN_R {
        CHECK_SEQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set this bit to disable crc calculation.
    #[inline(always)]
    pub fn crc_disable(&self) -> CRC_DISABLE_R {
        CRC_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set this bit to save packet header .
    #[inline(always)]
    pub fn save_head(&self) -> SAVE_HEAD_R {
        SAVE_HEAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Set this bit to enable hardware replace check_sum in packet header automatically.
    #[inline(always)]
    pub fn tx_check_sum_re(&self) -> TX_CHECK_SUM_RE_R {
        TX_CHECK_SUM_RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Set this bit to enable hardware replace ack num in packet header automatically.
    #[inline(always)]
    pub fn tx_ack_num_re(&self) -> TX_ACK_NUM_RE_R {
        TX_ACK_NUM_RE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Set this bit to check the owner bit in link descriptor.
    #[inline(always)]
    pub fn check_owner(&self) -> CHECK_OWNER_R {
        CHECK_OWNER_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Set this bit to enable software way to add packet header.
    #[inline(always)]
    pub fn wait_sw_start(&self) -> WAIT_SW_START_R {
        WAIT_SW_START_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Set this bit to start inserting the packet header.
    #[inline(always)]
    pub fn sw_start(&self) -> SW_START_R {
        SW_START_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:20 - when data amount in link descriptor's fifo is more than this register value it will produce uhci_dma_infifo_full_wm_int interrupt.
    #[inline(always)]
    pub fn dma_infifo_full_thrs(&self) -> DMA_INFIFO_FULL_THRS_R {
        DMA_INFIFO_FULL_THRS_R::new(((self.bits >> 9) & 0x0fff) as u16)
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
            .field("check_owner", &self.check_owner())
            .field("wait_sw_start", &self.wait_sw_start())
            .field("sw_start", &self.sw_start())
            .field("dma_infifo_full_thrs", &self.dma_infifo_full_thrs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to enable decoder to check check_sum in packet header.
    #[inline(always)]
    #[must_use]
    pub fn check_sum_en(&mut self) -> CHECK_SUM_EN_W<CONF1_SPEC> {
        CHECK_SUM_EN_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to enable decoder to check seq num in packet header.
    #[inline(always)]
    #[must_use]
    pub fn check_seq_en(&mut self) -> CHECK_SEQ_EN_W<CONF1_SPEC> {
        CHECK_SEQ_EN_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to disable crc calculation.
    #[inline(always)]
    #[must_use]
    pub fn crc_disable(&mut self) -> CRC_DISABLE_W<CONF1_SPEC> {
        CRC_DISABLE_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to save packet header .
    #[inline(always)]
    #[must_use]
    pub fn save_head(&mut self) -> SAVE_HEAD_W<CONF1_SPEC> {
        SAVE_HEAD_W::new(self, 3)
    }
    ///Bit 4 - Set this bit to enable hardware replace check_sum in packet header automatically.
    #[inline(always)]
    #[must_use]
    pub fn tx_check_sum_re(&mut self) -> TX_CHECK_SUM_RE_W<CONF1_SPEC> {
        TX_CHECK_SUM_RE_W::new(self, 4)
    }
    ///Bit 5 - Set this bit to enable hardware replace ack num in packet header automatically.
    #[inline(always)]
    #[must_use]
    pub fn tx_ack_num_re(&mut self) -> TX_ACK_NUM_RE_W<CONF1_SPEC> {
        TX_ACK_NUM_RE_W::new(self, 5)
    }
    ///Bit 6 - Set this bit to check the owner bit in link descriptor.
    #[inline(always)]
    #[must_use]
    pub fn check_owner(&mut self) -> CHECK_OWNER_W<CONF1_SPEC> {
        CHECK_OWNER_W::new(self, 6)
    }
    ///Bit 7 - Set this bit to enable software way to add packet header.
    #[inline(always)]
    #[must_use]
    pub fn wait_sw_start(&mut self) -> WAIT_SW_START_W<CONF1_SPEC> {
        WAIT_SW_START_W::new(self, 7)
    }
    ///Bit 8 - Set this bit to start inserting the packet header.
    #[inline(always)]
    #[must_use]
    pub fn sw_start(&mut self) -> SW_START_W<CONF1_SPEC> {
        SW_START_W::new(self, 8)
    }
    ///Bits 9:20 - when data amount in link descriptor's fifo is more than this register value it will produce uhci_dma_infifo_full_wm_int interrupt.
    #[inline(always)]
    #[must_use]
    pub fn dma_infifo_full_thrs(&mut self) -> DMA_INFIFO_FULL_THRS_W<CONF1_SPEC> {
        DMA_INFIFO_FULL_THRS_W::new(self, 9)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf1::R`](R) reader structure
impl crate::Readable for CONF1_SPEC {}
///`write(|w| ..)` method takes [`conf1::W`](W) writer structure
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF1 to value 0x33
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0x33;
}
