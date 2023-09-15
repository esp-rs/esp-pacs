#[doc = "Register `DMASTATUS` reader"]
pub type R = crate::R<DMASTATUS_SPEC>;
#[doc = "Register `DMASTATUS` writer"]
pub type W = crate::W<DMASTATUS_SPEC>;
#[doc = "Field `TRANS_INT` reader - This bit indicates that the frame transmission is complete. When transmission is complete Bit\\[31\\] (OWN) of TDES0 is reset and the specific frame status information is updated in the Descriptor."]
pub type TRANS_INT_R = crate::BitReader;
#[doc = "Field `TRANS_INT` writer - This bit indicates that the frame transmission is complete. When transmission is complete Bit\\[31\\] (OWN) of TDES0 is reset and the specific frame status information is updated in the Descriptor."]
pub type TRANS_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANS_PROC_STOP` reader - This bit is set when the transmission is stopped."]
pub type TRANS_PROC_STOP_R = crate::BitReader;
#[doc = "Field `TRANS_PROC_STOP` writer - This bit is set when the transmission is stopped."]
pub type TRANS_PROC_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANS_BUF_UNAVAIL` reader - This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it. Transmission is suspended. Bits\\[22:20\\] explain the Transmit Process state transitions. To resume processing Transmit descriptors the host should change the ownership of the descriptor by setting TDES0\\[31\\] and then issue a Transmit Poll Demand Command."]
pub type TRANS_BUF_UNAVAIL_R = crate::BitReader;
#[doc = "Field `TRANS_BUF_UNAVAIL` writer - This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it. Transmission is suspended. Bits\\[22:20\\] explain the Transmit Process state transitions. To resume processing Transmit descriptors the host should change the ownership of the descriptor by setting TDES0\\[31\\] and then issue a Transmit Poll Demand Command."]
pub type TRANS_BUF_UNAVAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANS_JABBER_TO` reader - This bit indicates that the Transmit Jabber Timer expired which happens when the frame size exceeds 2 048 (10 240 bytes when the Jumbo frame is enabled). When the Jabber Timeout occurs the transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\] flag to assert."]
pub type TRANS_JABBER_TO_R = crate::BitReader;
#[doc = "Field `TRANS_JABBER_TO` writer - This bit indicates that the Transmit Jabber Timer expired which happens when the frame size exceeds 2 048 (10 240 bytes when the Jumbo frame is enabled). When the Jabber Timeout occurs the transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\] flag to assert."]
pub type TRANS_JABBER_TO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RECV_OVFLOW` reader - This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to the application the overflow status is set in RDES0\\[11\\]."]
pub type RECV_OVFLOW_R = crate::BitReader;
#[doc = "Field `RECV_OVFLOW` writer - This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to the application the overflow status is set in RDES0\\[11\\]."]
pub type RECV_OVFLOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANS_UNDFLOW` reader - This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\] is set."]
pub type TRANS_UNDFLOW_R = crate::BitReader;
#[doc = "Field `TRANS_UNDFLOW` writer - This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\] is set."]
pub type TRANS_UNDFLOW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RECV_INT` reader - This bit indicates that the frame reception is complete. When reception is complete the Bit\\[31\\] of RDES1 (Disable Interrupt on Completion) is reset in the last Descriptor and the specific frame status information is updated in the descriptor. The reception remains in the Running state."]
pub type RECV_INT_R = crate::BitReader;
#[doc = "Field `RECV_INT` writer - This bit indicates that the frame reception is complete. When reception is complete the Bit\\[31\\] of RDES1 (Disable Interrupt on Completion) is reset in the last Descriptor and the specific frame status information is updated in the descriptor. The reception remains in the Running state."]
pub type RECV_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RECV_BUF_UNAVAIL` reader - This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it. The Receive Process is suspended. To resume processing Receive descriptors the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued the Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor is owned by the DMA."]
pub type RECV_BUF_UNAVAIL_R = crate::BitReader;
#[doc = "Field `RECV_BUF_UNAVAIL` writer - This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it. The Receive Process is suspended. To resume processing Receive descriptors the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued the Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor is owned by the DMA."]
pub type RECV_BUF_UNAVAIL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RECV_PROC_STOP` reader - This bit is asserted when the Receive Process enters the Stopped state."]
pub type RECV_PROC_STOP_R = crate::BitReader;
#[doc = "Field `RECV_PROC_STOP` writer - This bit is asserted when the Receive Process enters the Stopped state."]
pub type RECV_PROC_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RECV_WDT_TO` reader - When set this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout."]
pub type RECV_WDT_TO_R = crate::BitReader;
#[doc = "Field `RECV_WDT_TO` writer - When set this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout."]
pub type RECV_WDT_TO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EARLY_TRANS_INT` reader - This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO."]
pub type EARLY_TRANS_INT_R = crate::BitReader;
#[doc = "Field `EARLY_TRANS_INT` writer - This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO."]
pub type EARLY_TRANS_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FATAL_BUS_ERR_INT` reader - This bit indicates that a bus error occurred as described in Bits \\[25:23\\]. When this bit is set the corresponding DMA engine disables all of its bus accesses."]
pub type FATAL_BUS_ERR_INT_R = crate::BitReader;
#[doc = "Field `FATAL_BUS_ERR_INT` writer - This bit indicates that a bus error occurred as described in Bits \\[25:23\\]. When this bit is set the corresponding DMA engine disables all of its bus accesses."]
pub type FATAL_BUS_ERR_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EARLY_RECV_INT` reader - This bit indicates that the DMA filled the first data buffer of the packet. This bit is cleared when the software writes 1 to this bit or when Bit\\[6\\] (RI) of this register is set (whichever occurs earlier)."]
pub type EARLY_RECV_INT_R = crate::BitReader;
#[doc = "Field `EARLY_RECV_INT` writer - This bit indicates that the DMA filled the first data buffer of the packet. This bit is cleared when the software writes 1 to this bit or when Bit\\[6\\] (RI) of this register is set (whichever occurs earlier)."]
pub type EARLY_RECV_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABN_INT_SUMM` reader - Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Interrupt Enable Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive FIFO Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error. Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit which causes AIS to be set is cleared."]
pub type ABN_INT_SUMM_R = crate::BitReader;
#[doc = "Field `ABN_INT_SUMM` writer - Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Interrupt Enable Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive FIFO Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error. Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit which causes AIS to be set is cleared."]
pub type ABN_INT_SUMM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NORM_INT_SUMM` reader - Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Interrupt Enable Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt. Only unmasked bits affect the Normal Interrupt Summary bit.This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit which causes NIS to be set is cleared."]
pub type NORM_INT_SUMM_R = crate::BitReader;
#[doc = "Field `NORM_INT_SUMM` writer - Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Interrupt Enable Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt. Only unmasked bits affect the Normal Interrupt Summary bit.This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit which causes NIS to be set is cleared."]
pub type NORM_INT_SUMM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RECV_PROC_STATE` reader - This field indicates the Receive DMA FSM state. This field does not generate an interrupt. 3'b000: Stopped. Reset or Stop Receive Command issued. 3'b001: Running. Fetching Receive Transfer Descriptor. 3'b010: Reserved for future use. 3'b011: Running. Waiting for RX packets. 3'b100: Suspended. Receive Descriptor Unavailable. 3'b101: Running. Closing Receive Descriptor. 3'b110: TIME_STAMP write state. 3'b111: Running. Transferring the TX packets data from receive buffer to host memory."]
pub type RECV_PROC_STATE_R = crate::FieldReader;
#[doc = "Field `RECV_PROC_STATE` writer - This field indicates the Receive DMA FSM state. This field does not generate an interrupt. 3'b000: Stopped. Reset or Stop Receive Command issued. 3'b001: Running. Fetching Receive Transfer Descriptor. 3'b010: Reserved for future use. 3'b011: Running. Waiting for RX packets. 3'b100: Suspended. Receive Descriptor Unavailable. 3'b101: Running. Closing Receive Descriptor. 3'b110: TIME_STAMP write state. 3'b111: Running. Transferring the TX packets data from receive buffer to host memory."]
pub type RECV_PROC_STATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TRANS_PROC_STATE` reader - This field indicates the Transmit DMA FSM state. This field does not generate an interrupt. 3'b000: Stopped. Reset or Stop Transmit Command issued. 3'b001: Running. Fetching Transmit Transfer Descriptor. 3'b010: Reserved for future use. 3'b011: Running. Waiting for TX packets. 3'b100: Suspended. Receive Descriptor Unavailable. 3'b101: Running. Closing Transmit Descriptor. 3'b110: TIME_STAMP write state. 3'b111: Running. Transferring the TX packets data from transmit buffer to host memory."]
pub type TRANS_PROC_STATE_R = crate::FieldReader;
#[doc = "Field `TRANS_PROC_STATE` writer - This field indicates the Transmit DMA FSM state. This field does not generate an interrupt. 3'b000: Stopped. Reset or Stop Transmit Command issued. 3'b001: Running. Fetching Transmit Transfer Descriptor. 3'b010: Reserved for future use. 3'b011: Running. Waiting for TX packets. 3'b100: Suspended. Receive Descriptor Unavailable. 3'b101: Running. Closing Transmit Descriptor. 3'b110: TIME_STAMP write state. 3'b111: Running. Transferring the TX packets data from transmit buffer to host memory."]
pub type TRANS_PROC_STATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ERROR_BITS` reader - This field indicates the type of error that caused a Bus Error for example error response on the AHB interface. This field is valid only when Bit\\[13\\] (FBI) is set. This field does not generate an interrupt. 3'b000: Error during Rx DMA Write Data Transfer. 3'b011: Error during Tx DMA Read Data Transfer. 3'b100: Error during Rx DMA Descriptor Write Access. 3'b101: Error during Tx DMA Descriptor Write Access. 3'b110: Error during Rx DMA Descriptor Read Access. 3'b111: Error during Tx DMA Descriptor Read Access."]
pub type ERROR_BITS_R = crate::FieldReader;
#[doc = "Field `ERROR_BITS` writer - This field indicates the type of error that caused a Bus Error for example error response on the AHB interface. This field is valid only when Bit\\[13\\] (FBI) is set. This field does not generate an interrupt. 3'b000: Error during Rx DMA Write Data Transfer. 3'b011: Error during Tx DMA Read Data Transfer. 3'b100: Error during Rx DMA Descriptor Write Access. 3'b101: Error during Tx DMA Descriptor Write Access. 3'b110: Error during Rx DMA Descriptor Read Access. 3'b111: Error during Tx DMA Descriptor Read Access."]
pub type ERROR_BITS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PMT_INT` reader - This bit indicates an interrupt event in the PMT module of the ETH_MAC. The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1'b0."]
pub type PMT_INT_R = crate::BitReader;
#[doc = "Field `PMT_INT` writer - This bit indicates an interrupt event in the PMT module of the ETH_MAC. The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1'b0."]
pub type PMT_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TS_TRI_INT` reader - This bit indicates an interrupt event in the Timestamp Generator block of the ETH_MAC.The software must read the corresponding registers in the ETH_MAC to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0."]
pub type TS_TRI_INT_R = crate::BitReader;
#[doc = "Field `TS_TRI_INT` writer - This bit indicates an interrupt event in the Timestamp Generator block of the ETH_MAC.The software must read the corresponding registers in the ETH_MAC to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0."]
pub type TS_TRI_INT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - This bit indicates that the frame transmission is complete. When transmission is complete Bit\\[31\\] (OWN) of TDES0 is reset and the specific frame status information is updated in the Descriptor."]
    #[inline(always)]
    pub fn trans_int(&self) -> TRANS_INT_R {
        TRANS_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set when the transmission is stopped."]
    #[inline(always)]
    pub fn trans_proc_stop(&self) -> TRANS_PROC_STOP_R {
        TRANS_PROC_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it. Transmission is suspended. Bits\\[22:20\\] explain the Transmit Process state transitions. To resume processing Transmit descriptors the host should change the ownership of the descriptor by setting TDES0\\[31\\] and then issue a Transmit Poll Demand Command."]
    #[inline(always)]
    pub fn trans_buf_unavail(&self) -> TRANS_BUF_UNAVAIL_R {
        TRANS_BUF_UNAVAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates that the Transmit Jabber Timer expired which happens when the frame size exceeds 2 048 (10 240 bytes when the Jumbo frame is enabled). When the Jabber Timeout occurs the transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\] flag to assert."]
    #[inline(always)]
    pub fn trans_jabber_to(&self) -> TRANS_JABBER_TO_R {
        TRANS_JABBER_TO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to the application the overflow status is set in RDES0\\[11\\]."]
    #[inline(always)]
    pub fn recv_ovflow(&self) -> RECV_OVFLOW_R {
        RECV_OVFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\] is set."]
    #[inline(always)]
    pub fn trans_undflow(&self) -> TRANS_UNDFLOW_R {
        TRANS_UNDFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit indicates that the frame reception is complete. When reception is complete the Bit\\[31\\] of RDES1 (Disable Interrupt on Completion) is reset in the last Descriptor and the specific frame status information is updated in the descriptor. The reception remains in the Running state."]
    #[inline(always)]
    pub fn recv_int(&self) -> RECV_INT_R {
        RECV_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it. The Receive Process is suspended. To resume processing Receive descriptors the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued the Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor is owned by the DMA."]
    #[inline(always)]
    pub fn recv_buf_unavail(&self) -> RECV_BUF_UNAVAIL_R {
        RECV_BUF_UNAVAIL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit is asserted when the Receive Process enters the Stopped state."]
    #[inline(always)]
    pub fn recv_proc_stop(&self) -> RECV_PROC_STOP_R {
        RECV_PROC_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When set this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout."]
    #[inline(always)]
    pub fn recv_wdt_to(&self) -> RECV_WDT_TO_R {
        RECV_WDT_TO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO."]
    #[inline(always)]
    pub fn early_trans_int(&self) -> EARLY_TRANS_INT_R {
        EARLY_TRANS_INT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit indicates that a bus error occurred as described in Bits \\[25:23\\]. When this bit is set the corresponding DMA engine disables all of its bus accesses."]
    #[inline(always)]
    pub fn fatal_bus_err_int(&self) -> FATAL_BUS_ERR_INT_R {
        FATAL_BUS_ERR_INT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit indicates that the DMA filled the first data buffer of the packet. This bit is cleared when the software writes 1 to this bit or when Bit\\[6\\] (RI) of this register is set (whichever occurs earlier)."]
    #[inline(always)]
    pub fn early_recv_int(&self) -> EARLY_RECV_INT_R {
        EARLY_RECV_INT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Interrupt Enable Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive FIFO Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error. Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit which causes AIS to be set is cleared."]
    #[inline(always)]
    pub fn abn_int_summ(&self) -> ABN_INT_SUMM_R {
        ABN_INT_SUMM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Interrupt Enable Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt. Only unmasked bits affect the Normal Interrupt Summary bit.This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit which causes NIS to be set is cleared."]
    #[inline(always)]
    pub fn norm_int_summ(&self) -> NORM_INT_SUMM_R {
        NORM_INT_SUMM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - This field indicates the Receive DMA FSM state. This field does not generate an interrupt. 3'b000: Stopped. Reset or Stop Receive Command issued. 3'b001: Running. Fetching Receive Transfer Descriptor. 3'b010: Reserved for future use. 3'b011: Running. Waiting for RX packets. 3'b100: Suspended. Receive Descriptor Unavailable. 3'b101: Running. Closing Receive Descriptor. 3'b110: TIME_STAMP write state. 3'b111: Running. Transferring the TX packets data from receive buffer to host memory."]
    #[inline(always)]
    pub fn recv_proc_state(&self) -> RECV_PROC_STATE_R {
        RECV_PROC_STATE_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - This field indicates the Transmit DMA FSM state. This field does not generate an interrupt. 3'b000: Stopped. Reset or Stop Transmit Command issued. 3'b001: Running. Fetching Transmit Transfer Descriptor. 3'b010: Reserved for future use. 3'b011: Running. Waiting for TX packets. 3'b100: Suspended. Receive Descriptor Unavailable. 3'b101: Running. Closing Transmit Descriptor. 3'b110: TIME_STAMP write state. 3'b111: Running. Transferring the TX packets data from transmit buffer to host memory."]
    #[inline(always)]
    pub fn trans_proc_state(&self) -> TRANS_PROC_STATE_R {
        TRANS_PROC_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - This field indicates the type of error that caused a Bus Error for example error response on the AHB interface. This field is valid only when Bit\\[13\\] (FBI) is set. This field does not generate an interrupt. 3'b000: Error during Rx DMA Write Data Transfer. 3'b011: Error during Tx DMA Read Data Transfer. 3'b100: Error during Rx DMA Descriptor Write Access. 3'b101: Error during Tx DMA Descriptor Write Access. 3'b110: Error during Rx DMA Descriptor Read Access. 3'b111: Error during Tx DMA Descriptor Read Access."]
    #[inline(always)]
    pub fn error_bits(&self) -> ERROR_BITS_R {
        ERROR_BITS_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 28 - This bit indicates an interrupt event in the PMT module of the ETH_MAC. The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1'b0."]
    #[inline(always)]
    pub fn pmt_int(&self) -> PMT_INT_R {
        PMT_INT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit indicates an interrupt event in the Timestamp Generator block of the ETH_MAC.The software must read the corresponding registers in the ETH_MAC to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0."]
    #[inline(always)]
    pub fn ts_tri_int(&self) -> TS_TRI_INT_R {
        TS_TRI_INT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASTATUS")
            .field("trans_int", &format_args!("{}", self.trans_int().bit()))
            .field(
                "trans_proc_stop",
                &format_args!("{}", self.trans_proc_stop().bit()),
            )
            .field(
                "trans_buf_unavail",
                &format_args!("{}", self.trans_buf_unavail().bit()),
            )
            .field(
                "trans_jabber_to",
                &format_args!("{}", self.trans_jabber_to().bit()),
            )
            .field("recv_ovflow", &format_args!("{}", self.recv_ovflow().bit()))
            .field(
                "trans_undflow",
                &format_args!("{}", self.trans_undflow().bit()),
            )
            .field("recv_int", &format_args!("{}", self.recv_int().bit()))
            .field(
                "recv_buf_unavail",
                &format_args!("{}", self.recv_buf_unavail().bit()),
            )
            .field(
                "recv_proc_stop",
                &format_args!("{}", self.recv_proc_stop().bit()),
            )
            .field("recv_wdt_to", &format_args!("{}", self.recv_wdt_to().bit()))
            .field(
                "early_trans_int",
                &format_args!("{}", self.early_trans_int().bit()),
            )
            .field(
                "fatal_bus_err_int",
                &format_args!("{}", self.fatal_bus_err_int().bit()),
            )
            .field(
                "early_recv_int",
                &format_args!("{}", self.early_recv_int().bit()),
            )
            .field(
                "abn_int_summ",
                &format_args!("{}", self.abn_int_summ().bit()),
            )
            .field(
                "norm_int_summ",
                &format_args!("{}", self.norm_int_summ().bit()),
            )
            .field(
                "recv_proc_state",
                &format_args!("{}", self.recv_proc_state().bits()),
            )
            .field(
                "trans_proc_state",
                &format_args!("{}", self.trans_proc_state().bits()),
            )
            .field("error_bits", &format_args!("{}", self.error_bits().bits()))
            .field("pmt_int", &format_args!("{}", self.pmt_int().bit()))
            .field("ts_tri_int", &format_args!("{}", self.ts_tri_int().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMASTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates that the frame transmission is complete. When transmission is complete Bit\\[31\\] (OWN) of TDES0 is reset and the specific frame status information is updated in the Descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn trans_int(&mut self) -> TRANS_INT_W<DMASTATUS_SPEC, 0> {
        TRANS_INT_W::new(self)
    }
    #[doc = "Bit 1 - This bit is set when the transmission is stopped."]
    #[inline(always)]
    #[must_use]
    pub fn trans_proc_stop(&mut self) -> TRANS_PROC_STOP_W<DMASTATUS_SPEC, 1> {
        TRANS_PROC_STOP_W::new(self)
    }
    #[doc = "Bit 2 - This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it. Transmission is suspended. Bits\\[22:20\\] explain the Transmit Process state transitions. To resume processing Transmit descriptors the host should change the ownership of the descriptor by setting TDES0\\[31\\] and then issue a Transmit Poll Demand Command."]
    #[inline(always)]
    #[must_use]
    pub fn trans_buf_unavail(&mut self) -> TRANS_BUF_UNAVAIL_W<DMASTATUS_SPEC, 2> {
        TRANS_BUF_UNAVAIL_W::new(self)
    }
    #[doc = "Bit 3 - This bit indicates that the Transmit Jabber Timer expired which happens when the frame size exceeds 2 048 (10 240 bytes when the Jumbo frame is enabled). When the Jabber Timeout occurs the transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\] flag to assert."]
    #[inline(always)]
    #[must_use]
    pub fn trans_jabber_to(&mut self) -> TRANS_JABBER_TO_W<DMASTATUS_SPEC, 3> {
        TRANS_JABBER_TO_W::new(self)
    }
    #[doc = "Bit 4 - This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to the application the overflow status is set in RDES0\\[11\\]."]
    #[inline(always)]
    #[must_use]
    pub fn recv_ovflow(&mut self) -> RECV_OVFLOW_W<DMASTATUS_SPEC, 4> {
        RECV_OVFLOW_W::new(self)
    }
    #[doc = "Bit 5 - This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\] is set."]
    #[inline(always)]
    #[must_use]
    pub fn trans_undflow(&mut self) -> TRANS_UNDFLOW_W<DMASTATUS_SPEC, 5> {
        TRANS_UNDFLOW_W::new(self)
    }
    #[doc = "Bit 6 - This bit indicates that the frame reception is complete. When reception is complete the Bit\\[31\\] of RDES1 (Disable Interrupt on Completion) is reset in the last Descriptor and the specific frame status information is updated in the descriptor. The reception remains in the Running state."]
    #[inline(always)]
    #[must_use]
    pub fn recv_int(&mut self) -> RECV_INT_W<DMASTATUS_SPEC, 6> {
        RECV_INT_W::new(self)
    }
    #[doc = "Bit 7 - This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it. The Receive Process is suspended. To resume processing Receive descriptors the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued the Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor is owned by the DMA."]
    #[inline(always)]
    #[must_use]
    pub fn recv_buf_unavail(&mut self) -> RECV_BUF_UNAVAIL_W<DMASTATUS_SPEC, 7> {
        RECV_BUF_UNAVAIL_W::new(self)
    }
    #[doc = "Bit 8 - This bit is asserted when the Receive Process enters the Stopped state."]
    #[inline(always)]
    #[must_use]
    pub fn recv_proc_stop(&mut self) -> RECV_PROC_STOP_W<DMASTATUS_SPEC, 8> {
        RECV_PROC_STOP_W::new(self)
    }
    #[doc = "Bit 9 - When set this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout."]
    #[inline(always)]
    #[must_use]
    pub fn recv_wdt_to(&mut self) -> RECV_WDT_TO_W<DMASTATUS_SPEC, 9> {
        RECV_WDT_TO_W::new(self)
    }
    #[doc = "Bit 10 - This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn early_trans_int(&mut self) -> EARLY_TRANS_INT_W<DMASTATUS_SPEC, 10> {
        EARLY_TRANS_INT_W::new(self)
    }
    #[doc = "Bit 13 - This bit indicates that a bus error occurred as described in Bits \\[25:23\\]. When this bit is set the corresponding DMA engine disables all of its bus accesses."]
    #[inline(always)]
    #[must_use]
    pub fn fatal_bus_err_int(&mut self) -> FATAL_BUS_ERR_INT_W<DMASTATUS_SPEC, 13> {
        FATAL_BUS_ERR_INT_W::new(self)
    }
    #[doc = "Bit 14 - This bit indicates that the DMA filled the first data buffer of the packet. This bit is cleared when the software writes 1 to this bit or when Bit\\[6\\] (RI) of this register is set (whichever occurs earlier)."]
    #[inline(always)]
    #[must_use]
    pub fn early_recv_int(&mut self) -> EARLY_RECV_INT_W<DMASTATUS_SPEC, 14> {
        EARLY_RECV_INT_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Interrupt Enable Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive FIFO Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error. Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit which causes AIS to be set is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn abn_int_summ(&mut self) -> ABN_INT_SUMM_W<DMASTATUS_SPEC, 15> {
        ABN_INT_SUMM_W::new(self)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Interrupt Enable Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt. Only unmasked bits affect the Normal Interrupt Summary bit.This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit which causes NIS to be set is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn norm_int_summ(&mut self) -> NORM_INT_SUMM_W<DMASTATUS_SPEC, 16> {
        NORM_INT_SUMM_W::new(self)
    }
    #[doc = "Bits 17:19 - This field indicates the Receive DMA FSM state. This field does not generate an interrupt. 3'b000: Stopped. Reset or Stop Receive Command issued. 3'b001: Running. Fetching Receive Transfer Descriptor. 3'b010: Reserved for future use. 3'b011: Running. Waiting for RX packets. 3'b100: Suspended. Receive Descriptor Unavailable. 3'b101: Running. Closing Receive Descriptor. 3'b110: TIME_STAMP write state. 3'b111: Running. Transferring the TX packets data from receive buffer to host memory."]
    #[inline(always)]
    #[must_use]
    pub fn recv_proc_state(&mut self) -> RECV_PROC_STATE_W<DMASTATUS_SPEC, 17> {
        RECV_PROC_STATE_W::new(self)
    }
    #[doc = "Bits 20:22 - This field indicates the Transmit DMA FSM state. This field does not generate an interrupt. 3'b000: Stopped. Reset or Stop Transmit Command issued. 3'b001: Running. Fetching Transmit Transfer Descriptor. 3'b010: Reserved for future use. 3'b011: Running. Waiting for TX packets. 3'b100: Suspended. Receive Descriptor Unavailable. 3'b101: Running. Closing Transmit Descriptor. 3'b110: TIME_STAMP write state. 3'b111: Running. Transferring the TX packets data from transmit buffer to host memory."]
    #[inline(always)]
    #[must_use]
    pub fn trans_proc_state(&mut self) -> TRANS_PROC_STATE_W<DMASTATUS_SPEC, 20> {
        TRANS_PROC_STATE_W::new(self)
    }
    #[doc = "Bits 23:25 - This field indicates the type of error that caused a Bus Error for example error response on the AHB interface. This field is valid only when Bit\\[13\\] (FBI) is set. This field does not generate an interrupt. 3'b000: Error during Rx DMA Write Data Transfer. 3'b011: Error during Tx DMA Read Data Transfer. 3'b100: Error during Rx DMA Descriptor Write Access. 3'b101: Error during Tx DMA Descriptor Write Access. 3'b110: Error during Rx DMA Descriptor Read Access. 3'b111: Error during Tx DMA Descriptor Read Access."]
    #[inline(always)]
    #[must_use]
    pub fn error_bits(&mut self) -> ERROR_BITS_W<DMASTATUS_SPEC, 23> {
        ERROR_BITS_W::new(self)
    }
    #[doc = "Bit 28 - This bit indicates an interrupt event in the PMT module of the ETH_MAC. The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1'b0."]
    #[inline(always)]
    #[must_use]
    pub fn pmt_int(&mut self) -> PMT_INT_W<DMASTATUS_SPEC, 28> {
        PMT_INT_W::new(self)
    }
    #[doc = "Bit 29 - This bit indicates an interrupt event in the Timestamp Generator block of the ETH_MAC.The software must read the corresponding registers in the ETH_MAC to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0."]
    #[inline(always)]
    #[must_use]
    pub fn ts_tri_int(&mut self) -> TS_TRI_INT_W<DMASTATUS_SPEC, 29> {
        TS_TRI_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "State of interrupts, errors and other events\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASTATUS_SPEC;
impl crate::RegisterSpec for DMASTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmastatus::R`](R) reader structure"]
impl crate::Readable for DMASTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmastatus::W`](W) writer structure"]
impl crate::Writable for DMASTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMASTATUS to value 0"]
impl crate::Resettable for DMASTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
