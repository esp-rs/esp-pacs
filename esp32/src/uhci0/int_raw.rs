///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Field `RX_START` reader - when a separator char has been send it will produce uhci_rx_start_int interrupt.
pub type RX_START_R = crate::BitReader;
///Field `TX_START` reader - when DMA detects a separator char it will produce uhci_tx_start_int interrupt.
pub type TX_START_R = crate::BitReader;
///Field `RX_HUNG` reader - when DMA takes a lot of time to receive a data it will produce uhci_rx_hung_int interrupt.
pub type RX_HUNG_R = crate::BitReader;
///Field `TX_HUNG` reader - when DMA takes a lot of time to read a data from RAM it will produce uhci_tx_hung_int interrupt.
pub type TX_HUNG_R = crate::BitReader;
///Field `IN_DONE` reader - when a in link descriptor has been completed it will produce uhci_in_done_int interrupt.
pub type IN_DONE_R = crate::BitReader;
///Field `IN_SUC_EOF` reader - when a data packet has been received it will produce uhci_in_suc_eof_int interrupt.
pub type IN_SUC_EOF_R = crate::BitReader;
///Field `IN_ERR_EOF` reader - when there are some errors about eof in in link descriptor it will produce uhci_in_err_eof_int interrupt.
pub type IN_ERR_EOF_R = crate::BitReader;
///Field `OUT_DONE` reader - when a out link descriptor is completed it will produce uhci_out_done_int interrupt.
pub type OUT_DONE_R = crate::BitReader;
///Field `OUT_EOF` reader - when the current descriptor's eof bit is 1 it will produce uhci_out_eof_int interrupt.
pub type OUT_EOF_R = crate::BitReader;
///Field `IN_DSCR_ERR` reader - when there are some errors about the out link descriptor it will produce uhci_in_dscr_err_int interrupt.
pub type IN_DSCR_ERR_R = crate::BitReader;
///Field `OUT_DSCR_ERR` reader - when there are some errors about the in link descriptor it will produce uhci_out_dscr_err_int interrupt.
pub type OUT_DSCR_ERR_R = crate::BitReader;
///Field `IN_DSCR_EMPTY` reader - when there are not enough in links for DMA it will produce uhci_in_dscr_err_int interrupt.
pub type IN_DSCR_EMPTY_R = crate::BitReader;
///Field `OUTLINK_EOF_ERR` reader - when there are some errors about eof in outlink descriptor it will produce uhci_outlink_eof_err_int interrupt.
pub type OUTLINK_EOF_ERR_R = crate::BitReader;
///Field `OUT_TOTAL_EOF` reader - When all data have been send it will produce uhci_out_total_eof_int interrupt.
pub type OUT_TOTAL_EOF_R = crate::BitReader;
///Field `SEND_S_Q` reader - When use single send registers to send a short packets it will produce this interrupt when dma has send the short packet.
pub type SEND_S_Q_R = crate::BitReader;
///Field `SEND_A_Q` reader - When use always_send registers to send a series of short packets it will produce this interrupt when dma has send the short packet.
pub type SEND_A_Q_R = crate::BitReader;
///Field `DMA_INFIFO_FULL_WM` reader -
pub type DMA_INFIFO_FULL_WM_R = crate::BitReader;
impl R {
    ///Bit 0 - when a separator char has been send it will produce uhci_rx_start_int interrupt.
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - when DMA detects a separator char it will produce uhci_tx_start_int interrupt.
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - when DMA takes a lot of time to receive a data it will produce uhci_rx_hung_int interrupt.
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - when DMA takes a lot of time to read a data from RAM it will produce uhci_tx_hung_int interrupt.
    #[inline(always)]
    pub fn tx_hung(&self) -> TX_HUNG_R {
        TX_HUNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - when a in link descriptor has been completed it will produce uhci_in_done_int interrupt.
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - when a data packet has been received it will produce uhci_in_suc_eof_int interrupt.
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - when there are some errors about eof in in link descriptor it will produce uhci_in_err_eof_int interrupt.
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - when a out link descriptor is completed it will produce uhci_out_done_int interrupt.
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - when the current descriptor's eof bit is 1 it will produce uhci_out_eof_int interrupt.
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - when there are some errors about the out link descriptor it will produce uhci_in_dscr_err_int interrupt.
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - when there are some errors about the in link descriptor it will produce uhci_out_dscr_err_int interrupt.
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - when there are not enough in links for DMA it will produce uhci_in_dscr_err_int interrupt.
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - when there are some errors about eof in outlink descriptor it will produce uhci_outlink_eof_err_int interrupt.
    #[inline(always)]
    pub fn outlink_eof_err(&self) -> OUTLINK_EOF_ERR_R {
        OUTLINK_EOF_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - When all data have been send it will produce uhci_out_total_eof_int interrupt.
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - When use single send registers to send a short packets it will produce this interrupt when dma has send the short packet.
    #[inline(always)]
    pub fn send_s_q(&self) -> SEND_S_Q_R {
        SEND_S_Q_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - When use always_send registers to send a series of short packets it will produce this interrupt when dma has send the short packet.
    #[inline(always)]
    pub fn send_a_q(&self) -> SEND_A_Q_R {
        SEND_A_Q_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn dma_infifo_full_wm(&self) -> DMA_INFIFO_FULL_WM_R {
        DMA_INFIFO_FULL_WM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("rx_start", &self.rx_start())
            .field("tx_start", &self.tx_start())
            .field("rx_hung", &self.rx_hung())
            .field("tx_hung", &self.tx_hung())
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("in_err_eof", &self.in_err_eof())
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("out_dscr_err", &self.out_dscr_err())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("outlink_eof_err", &self.outlink_eof_err())
            .field("out_total_eof", &self.out_total_eof())
            .field("send_s_q", &self.send_s_q())
            .field("send_a_q", &self.send_a_q())
            .field("dma_infifo_full_wm", &self.dma_infifo_full_wm())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw::R`](R) reader structure
impl crate::Readable for INT_RAW_SPEC {}
///`reset()` method sets INT_RAW to value 0
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
