#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_RX_WATERMARK` reader - FIFO reached Receive watermark level, not qualified with data transfer."]
pub type FIFO_RX_WATERMARK_R = crate::BitReader;
#[doc = "Field `FIFO_TX_WATERMARK` reader - FIFO reached Transmit watermark level, not qualified with data transfer."]
pub type FIFO_TX_WATERMARK_R = crate::BitReader;
#[doc = "Field `FIFO_EMPTY` reader - FIFO is empty status."]
pub type FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `FIFO_FULL` reader - FIFO is full status."]
pub type FIFO_FULL_R = crate::BitReader;
#[doc = "Field `COMMAND_FSM_STATES` reader - Command FSM states. 0: Idle; 1: Send init sequence; 2: Send cmd start bit; 3: Send cmd tx bit; 4: Send cmd index + arg; 5: Send cmd crc7; 6: Send cmd end bit; 7: Receive resp start bit; 8: Receive resp IRQ response; 9: Receive resp tx bit; 10: Receive resp cmd idx; 11: Receive resp data; 12: Receive resp crc7; 13: Receive resp end bit; 14: Cmd path wait NCC; 15: Wait, cmd-to-response turnaround."]
pub type COMMAND_FSM_STATES_R = crate::FieldReader;
#[doc = "Field `DATA_3_STATUS` reader - Raw selected sdhost_card_data\\[3\\], checks whether card is present. 0: card not present; 1: card present."]
pub type DATA_3_STATUS_R = crate::BitReader;
#[doc = "Field `DATA_BUSY` reader - Inverted version of raw selected sdhost_card_data\\[0\\]. 0: Card data not busy; 1: Card data busy."]
pub type DATA_BUSY_R = crate::BitReader;
#[doc = "Field `DATA_STATE_MC_BUSY` reader - Data transmit or receive state-machine is busy."]
pub type DATA_STATE_MC_BUSY_R = crate::BitReader;
#[doc = "Field `RESPONSE_INDEX` reader - Index of previous response, including any auto-stop sent by core."]
pub type RESPONSE_INDEX_R = crate::FieldReader;
#[doc = "Field `FIFO_COUNT` reader - FIFO count, number of filled locations in FIFO."]
pub type FIFO_COUNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - FIFO reached Receive watermark level, not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_rx_watermark(&self) -> FIFO_RX_WATERMARK_R {
        FIFO_RX_WATERMARK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level, not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_tx_watermark(&self) -> FIFO_TX_WATERMARK_R {
        FIFO_TX_WATERMARK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Command FSM states. 0: Idle; 1: Send init sequence; 2: Send cmd start bit; 3: Send cmd tx bit; 4: Send cmd index + arg; 5: Send cmd crc7; 6: Send cmd end bit; 7: Receive resp start bit; 8: Receive resp IRQ response; 9: Receive resp tx bit; 10: Receive resp cmd idx; 11: Receive resp data; 12: Receive resp crc7; 13: Receive resp end bit; 14: Cmd path wait NCC; 15: Wait, cmd-to-response turnaround."]
    #[inline(always)]
    pub fn command_fsm_states(&self) -> COMMAND_FSM_STATES_R {
        COMMAND_FSM_STATES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Raw selected sdhost_card_data\\[3\\], checks whether card is present. 0: card not present; 1: card present."]
    #[inline(always)]
    pub fn data_3_status(&self) -> DATA_3_STATUS_R {
        DATA_3_STATUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Inverted version of raw selected sdhost_card_data\\[0\\]. 0: Card data not busy; 1: Card data busy."]
    #[inline(always)]
    pub fn data_busy(&self) -> DATA_BUSY_R {
        DATA_BUSY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub fn data_state_mc_busy(&self) -> DATA_STATE_MC_BUSY_R {
        DATA_STATE_MC_BUSY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub fn response_index(&self) -> RESPONSE_INDEX_R {
        RESPONSE_INDEX_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:29 - FIFO count, number of filled locations in FIFO."]
    #[inline(always)]
    pub fn fifo_count(&self) -> FIFO_COUNT_R {
        FIFO_COUNT_R::new(((self.bits >> 17) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field(
                "fifo_rx_watermark",
                &format_args!("{}", self.fifo_rx_watermark().bit()),
            )
            .field(
                "fifo_tx_watermark",
                &format_args!("{}", self.fifo_tx_watermark().bit()),
            )
            .field("fifo_empty", &format_args!("{}", self.fifo_empty().bit()))
            .field("fifo_full", &format_args!("{}", self.fifo_full().bit()))
            .field(
                "command_fsm_states",
                &format_args!("{}", self.command_fsm_states().bits()),
            )
            .field(
                "data_3_status",
                &format_args!("{}", self.data_3_status().bit()),
            )
            .field("data_busy", &format_args!("{}", self.data_busy().bit()))
            .field(
                "data_state_mc_busy",
                &format_args!("{}", self.data_state_mc_busy().bit()),
            )
            .field(
                "response_index",
                &format_args!("{}", self.response_index().bits()),
            )
            .field("fifo_count", &format_args!("{}", self.fifo_count().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SD/MMC status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x0716"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0716;
}
