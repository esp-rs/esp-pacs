#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_START_INT_RAW` reader - when a separator char has been send it will produce uhci_rx_start_int interrupt."]
pub type RX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_START_INT_RAW` reader - when DMA detects a separator char it will produce uhci_tx_start_int interrupt."]
pub type TX_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_RAW` reader - when DMA takes a lot of time to receive a data it will produce uhci_rx_hung_int interrupt."]
pub type RX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_RAW` reader - when DMA takes a lot of time to read a data from RAM it will produce uhci_tx_hung_int interrupt."]
pub type TX_HUNG_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DONE_INT_RAW` reader - when a in link descriptor has been completed it will produce uhci_in_done_int interrupt."]
pub type IN_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_INT_RAW` reader - when a data packet has been received it will produce uhci_in_suc_eof_int interrupt."]
pub type IN_SUC_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_INT_RAW` reader - when there are some errors about eof in in link descriptor it will produce uhci_in_err_eof_int interrupt."]
pub type IN_ERR_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DONE_INT_RAW` reader - when a out link descriptor is completed it will produce uhci_out_done_int interrupt."]
pub type OUT_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_RAW` reader - when the current descriptor's eof bit is 1 it will produce uhci_out_eof_int interrupt."]
pub type OUT_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_INT_RAW` reader - when there are some errors about the out link descriptor it will produce uhci_in_dscr_err_int interrupt."]
pub type IN_DSCR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_INT_RAW` reader - when there are some errors about the in link descriptor it will produce uhci_out_dscr_err_int interrupt."]
pub type OUT_DSCR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_INT_RAW` reader - when there are not enough in links for DMA it will produce uhci_in_dscr_err_int interrupt."]
pub type IN_DSCR_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTLINK_EOF_ERR_INT_RAW` reader - when there are some errors about eof in outlink descriptor it will produce uhci_outlink_eof_err_int interrupt."]
pub type OUTLINK_EOF_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_INT_RAW` reader - When all data have been send it will produce uhci_out_total_eof_int interrupt."]
pub type OUT_TOTAL_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEND_S_Q_INT_RAW` reader - When use single send registers to send a short packets it will produce this interrupt when dma has send the short packet."]
pub type SEND_S_Q_INT_RAW_R = crate::BitReader;
#[doc = "Field `SEND_A_Q_INT_RAW` reader - When use always_send registers to send a series of short packets it will produce this interrupt when dma has send the short packet."]
pub type SEND_A_Q_INT_RAW_R = crate::BitReader;
#[doc = "Field `DMA_INFIFO_FULL_WM_INT_RAW` reader - "]
pub type DMA_INFIFO_FULL_WM_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - when a separator char has been send it will produce uhci_rx_start_int interrupt."]
    #[inline(always)]
    pub fn rx_start_int_raw(&self) -> RX_START_INT_RAW_R {
        RX_START_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when DMA detects a separator char it will produce uhci_tx_start_int interrupt."]
    #[inline(always)]
    pub fn tx_start_int_raw(&self) -> TX_START_INT_RAW_R {
        TX_START_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - when DMA takes a lot of time to receive a data it will produce uhci_rx_hung_int interrupt."]
    #[inline(always)]
    pub fn rx_hung_int_raw(&self) -> RX_HUNG_INT_RAW_R {
        RX_HUNG_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - when DMA takes a lot of time to read a data from RAM it will produce uhci_tx_hung_int interrupt."]
    #[inline(always)]
    pub fn tx_hung_int_raw(&self) -> TX_HUNG_INT_RAW_R {
        TX_HUNG_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - when a in link descriptor has been completed it will produce uhci_in_done_int interrupt."]
    #[inline(always)]
    pub fn in_done_int_raw(&self) -> IN_DONE_INT_RAW_R {
        IN_DONE_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - when a data packet has been received it will produce uhci_in_suc_eof_int interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_int_raw(&self) -> IN_SUC_EOF_INT_RAW_R {
        IN_SUC_EOF_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - when there are some errors about eof in in link descriptor it will produce uhci_in_err_eof_int interrupt."]
    #[inline(always)]
    pub fn in_err_eof_int_raw(&self) -> IN_ERR_EOF_INT_RAW_R {
        IN_ERR_EOF_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - when a out link descriptor is completed it will produce uhci_out_done_int interrupt."]
    #[inline(always)]
    pub fn out_done_int_raw(&self) -> OUT_DONE_INT_RAW_R {
        OUT_DONE_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - when the current descriptor's eof bit is 1 it will produce uhci_out_eof_int interrupt."]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - when there are some errors about the out link descriptor it will produce uhci_in_dscr_err_int interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_int_raw(&self) -> IN_DSCR_ERR_INT_RAW_R {
        IN_DSCR_ERR_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - when there are some errors about the in link descriptor it will produce uhci_out_dscr_err_int interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_int_raw(&self) -> OUT_DSCR_ERR_INT_RAW_R {
        OUT_DSCR_ERR_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - when there are not enough in links for DMA it will produce uhci_in_dscr_err_int interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_int_raw(&self) -> IN_DSCR_EMPTY_INT_RAW_R {
        IN_DSCR_EMPTY_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - when there are some errors about eof in outlink descriptor it will produce uhci_outlink_eof_err_int interrupt."]
    #[inline(always)]
    pub fn outlink_eof_err_int_raw(&self) -> OUTLINK_EOF_ERR_INT_RAW_R {
        OUTLINK_EOF_ERR_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When all data have been send it will produce uhci_out_total_eof_int interrupt."]
    #[inline(always)]
    pub fn out_total_eof_int_raw(&self) -> OUT_TOTAL_EOF_INT_RAW_R {
        OUT_TOTAL_EOF_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When use single send registers to send a short packets it will produce this interrupt when dma has send the short packet."]
    #[inline(always)]
    pub fn send_s_q_int_raw(&self) -> SEND_S_Q_INT_RAW_R {
        SEND_S_Q_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When use always_send registers to send a series of short packets it will produce this interrupt when dma has send the short packet."]
    #[inline(always)]
    pub fn send_a_q_int_raw(&self) -> SEND_A_Q_INT_RAW_R {
        SEND_A_Q_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dma_infifo_full_wm_int_raw(&self) -> DMA_INFIFO_FULL_WM_INT_RAW_R {
        DMA_INFIFO_FULL_WM_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "rx_start_int_raw",
                &format_args!("{}", self.rx_start_int_raw().bit()),
            )
            .field(
                "tx_start_int_raw",
                &format_args!("{}", self.tx_start_int_raw().bit()),
            )
            .field(
                "rx_hung_int_raw",
                &format_args!("{}", self.rx_hung_int_raw().bit()),
            )
            .field(
                "tx_hung_int_raw",
                &format_args!("{}", self.tx_hung_int_raw().bit()),
            )
            .field(
                "in_done_int_raw",
                &format_args!("{}", self.in_done_int_raw().bit()),
            )
            .field(
                "in_suc_eof_int_raw",
                &format_args!("{}", self.in_suc_eof_int_raw().bit()),
            )
            .field(
                "in_err_eof_int_raw",
                &format_args!("{}", self.in_err_eof_int_raw().bit()),
            )
            .field(
                "out_done_int_raw",
                &format_args!("{}", self.out_done_int_raw().bit()),
            )
            .field(
                "out_eof_int_raw",
                &format_args!("{}", self.out_eof_int_raw().bit()),
            )
            .field(
                "in_dscr_err_int_raw",
                &format_args!("{}", self.in_dscr_err_int_raw().bit()),
            )
            .field(
                "out_dscr_err_int_raw",
                &format_args!("{}", self.out_dscr_err_int_raw().bit()),
            )
            .field(
                "in_dscr_empty_int_raw",
                &format_args!("{}", self.in_dscr_empty_int_raw().bit()),
            )
            .field(
                "outlink_eof_err_int_raw",
                &format_args!("{}", self.outlink_eof_err_int_raw().bit()),
            )
            .field(
                "out_total_eof_int_raw",
                &format_args!("{}", self.out_total_eof_int_raw().bit()),
            )
            .field(
                "send_s_q_int_raw",
                &format_args!("{}", self.send_s_q_int_raw().bit()),
            )
            .field(
                "send_a_q_int_raw",
                &format_args!("{}", self.send_a_q_int_raw().bit()),
            )
            .field(
                "dma_infifo_full_wm_int_raw",
                &format_args!("{}", self.dma_infifo_full_wm_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
