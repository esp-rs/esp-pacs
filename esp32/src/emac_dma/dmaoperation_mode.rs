#[doc = "Register `DMAOPERATION_MODE` reader"]
pub type R = crate::R<DMAOPERATION_MODE_SPEC>;
#[doc = "Register `DMAOPERATION_MODE` writer"]
pub type W = crate::W<DMAOPERATION_MODE_SPEC>;
#[doc = "Field `START_STOP_RX` reader - When this bit is set the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames.When this bit is cleared the Rx DMA operation is stopped after the transfer of the current frame."]
pub type START_STOP_RX_R = crate::BitReader;
#[doc = "Field `START_STOP_RX` writer - When this bit is set the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames.When this bit is cleared the Rx DMA operation is stopped after the transfer of the current frame."]
pub type START_STOP_RX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OPT_SECOND_FRAME` reader - When this bit is set it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained."]
pub type OPT_SECOND_FRAME_R = crate::BitReader;
#[doc = "Field `OPT_SECOND_FRAME` writer - When this bit is set it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained."]
pub type OPT_SECOND_FRAME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_THRESH_CTRL` reader - These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. 2'b00: 64， 2'b01: 32， 2'b10: 96， 2'b11: 128 ."]
pub type RX_THRESH_CTRL_R = crate::FieldReader;
#[doc = "Field `RX_THRESH_CTRL` writer - These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. 2'b00: 64， 2'b01: 32， 2'b10: 96， 2'b11: 128 ."]
pub type RX_THRESH_CTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DROP_GFRM` reader - When set the MAC drops the received giant frames in the Rx FIFO that is frames that are larger than the computed giant frame limit."]
pub type DROP_GFRM_R = crate::BitReader;
#[doc = "Field `DROP_GFRM` writer - When set the MAC drops the received giant frames in the Rx FIFO that is frames that are larger than the computed giant frame limit."]
pub type DROP_GFRM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FWD_UNDER_GF` reader - When set the Rx FIFO forwards Undersized frames (that is frames with no Error and length less than 64 bytes) including pad-bytes and CRC."]
pub type FWD_UNDER_GF_R = crate::BitReader;
#[doc = "Field `FWD_UNDER_GF` writer - When set the Rx FIFO forwards Undersized frames (that is frames with no Error and length less than 64 bytes) including pad-bytes and CRC."]
pub type FWD_UNDER_GF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FWD_ERR_FRAME` reader - When this bit is reset the Rx FIFO drops frames with error status (CRC error collision error giant frame watchdog timeout or overflow)."]
pub type FWD_ERR_FRAME_R = crate::BitReader;
#[doc = "Field `FWD_ERR_FRAME` writer - When this bit is reset the Rx FIFO drops frames with error status (CRC error collision error giant frame watchdog timeout or overflow)."]
pub type FWD_ERR_FRAME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `START_STOP_TRANSMISSION_COMMAND` reader - When this bit is set transmission is placed in the Running state and the DMA checks the Transmit List at the current position for a frame to be transmitted.When this bit is reset the transmission process is placed in the Stopped state after completing the transmission of the current frame."]
pub type START_STOP_TRANSMISSION_COMMAND_R = crate::BitReader;
#[doc = "Field `START_STOP_TRANSMISSION_COMMAND` writer - When this bit is set transmission is placed in the Running state and the DMA checks the Transmit List at the current position for a frame to be transmitted.When this bit is reset the transmission process is placed in the Stopped state after completing the transmission of the current frame."]
pub type START_STOP_TRANSMISSION_COMMAND_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_THRESH_CTRL` reader - These bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition full frames with a length less than the threshold are also transmitted. These bits are used only when Tx_Str_fwd is reset. 3'b000: 64 3'b001: 128 3'b010: 192 3'b011: 256 3'b100: 40 3'b101: 32 3'b110: 24 3'b111: 16 ."]
pub type TX_THRESH_CTRL_R = crate::FieldReader;
#[doc = "Field `TX_THRESH_CTRL` writer - These bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition full frames with a length less than the threshold are also transmitted. These bits are used only when Tx_Str_fwd is reset. 3'b000: 64 3'b001: 128 3'b010: 192 3'b011: 256 3'b100: 40 3'b101: 32 3'b110: 24 3'b111: 16 ."]
pub type TX_THRESH_CTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `FLUSH_TX_FIFO` reader - When this bit is set the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed. This bit is cleared internally when the flushing operation is complete."]
pub type FLUSH_TX_FIFO_R = crate::BitReader;
#[doc = "Field `FLUSH_TX_FIFO` writer - When this bit is set the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed. This bit is cleared internally when the flushing operation is complete."]
pub type FLUSH_TX_FIFO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_STR_FWD` reader - When this bit is set transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set the Tx_Thresh_Ctrl values specified in Tx_Thresh_Ctrl are ignored."]
pub type TX_STR_FWD_R = crate::BitReader;
#[doc = "Field `TX_STR_FWD` writer - When this bit is set transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set the Tx_Thresh_Ctrl values specified in Tx_Thresh_Ctrl are ignored."]
pub type TX_STR_FWD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIS_FLUSH_RECV_FRAMES` reader - When this bit is set the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers."]
pub type DIS_FLUSH_RECV_FRAMES_R = crate::BitReader;
#[doc = "Field `DIS_FLUSH_RECV_FRAMES` writer - When this bit is set the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers."]
pub type DIS_FLUSH_RECV_FRAMES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_STORE_FORWARD` reader - When this bit is set the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it."]
pub type RX_STORE_FORWARD_R = crate::BitReader;
#[doc = "Field `RX_STORE_FORWARD` writer - When this bit is set the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it."]
pub type RX_STORE_FORWARD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIS_DROP_TCPIP_ERR_FRAM` reader - When this bit is set the MAC does not drop the frames which only have errors detected by the Receive Checksum engine.When this bit is reset all error frames are dropped if the Fwd_Err_Frame bit is reset."]
pub type DIS_DROP_TCPIP_ERR_FRAM_R = crate::BitReader;
#[doc = "Field `DIS_DROP_TCPIP_ERR_FRAM` writer - When this bit is set the MAC does not drop the frames which only have errors detected by the Receive Checksum engine.When this bit is reset all error frames are dropped if the Fwd_Err_Frame bit is reset."]
pub type DIS_DROP_TCPIP_ERR_FRAM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - When this bit is set the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames.When this bit is cleared the Rx DMA operation is stopped after the transfer of the current frame."]
    #[inline(always)]
    pub fn start_stop_rx(&self) -> START_STOP_RX_R {
        START_STOP_RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is set it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained."]
    #[inline(always)]
    pub fn opt_second_frame(&self) -> OPT_SECOND_FRAME_R {
        OPT_SECOND_FRAME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. 2'b00: 64， 2'b01: 32， 2'b10: 96， 2'b11: 128 ."]
    #[inline(always)]
    pub fn rx_thresh_ctrl(&self) -> RX_THRESH_CTRL_R {
        RX_THRESH_CTRL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - When set the MAC drops the received giant frames in the Rx FIFO that is frames that are larger than the computed giant frame limit."]
    #[inline(always)]
    pub fn drop_gfrm(&self) -> DROP_GFRM_R {
        DROP_GFRM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set the Rx FIFO forwards Undersized frames (that is frames with no Error and length less than 64 bytes) including pad-bytes and CRC."]
    #[inline(always)]
    pub fn fwd_under_gf(&self) -> FWD_UNDER_GF_R {
        FWD_UNDER_GF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When this bit is reset the Rx FIFO drops frames with error status (CRC error collision error giant frame watchdog timeout or overflow)."]
    #[inline(always)]
    pub fn fwd_err_frame(&self) -> FWD_ERR_FRAME_R {
        FWD_ERR_FRAME_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - When this bit is set transmission is placed in the Running state and the DMA checks the Transmit List at the current position for a frame to be transmitted.When this bit is reset the transmission process is placed in the Stopped state after completing the transmission of the current frame."]
    #[inline(always)]
    pub fn start_stop_transmission_command(&self) -> START_STOP_TRANSMISSION_COMMAND_R {
        START_STOP_TRANSMISSION_COMMAND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - These bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition full frames with a length less than the threshold are also transmitted. These bits are used only when Tx_Str_fwd is reset. 3'b000: 64 3'b001: 128 3'b010: 192 3'b011: 256 3'b100: 40 3'b101: 32 3'b110: 24 3'b111: 16 ."]
    #[inline(always)]
    pub fn tx_thresh_ctrl(&self) -> TX_THRESH_CTRL_R {
        TX_THRESH_CTRL_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - When this bit is set the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed. This bit is cleared internally when the flushing operation is complete."]
    #[inline(always)]
    pub fn flush_tx_fifo(&self) -> FLUSH_TX_FIFO_R {
        FLUSH_TX_FIFO_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When this bit is set transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set the Tx_Thresh_Ctrl values specified in Tx_Thresh_Ctrl are ignored."]
    #[inline(always)]
    pub fn tx_str_fwd(&self) -> TX_STR_FWD_R {
        TX_STR_FWD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - When this bit is set the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers."]
    #[inline(always)]
    pub fn dis_flush_recv_frames(&self) -> DIS_FLUSH_RECV_FRAMES_R {
        DIS_FLUSH_RECV_FRAMES_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When this bit is set the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it."]
    #[inline(always)]
    pub fn rx_store_forward(&self) -> RX_STORE_FORWARD_R {
        RX_STORE_FORWARD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When this bit is set the MAC does not drop the frames which only have errors detected by the Receive Checksum engine.When this bit is reset all error frames are dropped if the Fwd_Err_Frame bit is reset."]
    #[inline(always)]
    pub fn dis_drop_tcpip_err_fram(&self) -> DIS_DROP_TCPIP_ERR_FRAM_R {
        DIS_DROP_TCPIP_ERR_FRAM_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAOPERATION_MODE")
            .field(
                "start_stop_rx",
                &format_args!("{}", self.start_stop_rx().bit()),
            )
            .field(
                "opt_second_frame",
                &format_args!("{}", self.opt_second_frame().bit()),
            )
            .field(
                "rx_thresh_ctrl",
                &format_args!("{}", self.rx_thresh_ctrl().bits()),
            )
            .field("drop_gfrm", &format_args!("{}", self.drop_gfrm().bit()))
            .field(
                "fwd_under_gf",
                &format_args!("{}", self.fwd_under_gf().bit()),
            )
            .field(
                "fwd_err_frame",
                &format_args!("{}", self.fwd_err_frame().bit()),
            )
            .field(
                "start_stop_transmission_command",
                &format_args!("{}", self.start_stop_transmission_command().bit()),
            )
            .field(
                "tx_thresh_ctrl",
                &format_args!("{}", self.tx_thresh_ctrl().bits()),
            )
            .field(
                "flush_tx_fifo",
                &format_args!("{}", self.flush_tx_fifo().bit()),
            )
            .field("tx_str_fwd", &format_args!("{}", self.tx_str_fwd().bit()))
            .field(
                "dis_flush_recv_frames",
                &format_args!("{}", self.dis_flush_recv_frames().bit()),
            )
            .field(
                "rx_store_forward",
                &format_args!("{}", self.rx_store_forward().bit()),
            )
            .field(
                "dis_drop_tcpip_err_fram",
                &format_args!("{}", self.dis_drop_tcpip_err_fram().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMAOPERATION_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - When this bit is set the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames.When this bit is cleared the Rx DMA operation is stopped after the transfer of the current frame."]
    #[inline(always)]
    #[must_use]
    pub fn start_stop_rx(&mut self) -> START_STOP_RX_W<DMAOPERATION_MODE_SPEC, 1> {
        START_STOP_RX_W::new(self)
    }
    #[doc = "Bit 2 - When this bit is set it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained."]
    #[inline(always)]
    #[must_use]
    pub fn opt_second_frame(&mut self) -> OPT_SECOND_FRAME_W<DMAOPERATION_MODE_SPEC, 2> {
        OPT_SECOND_FRAME_W::new(self)
    }
    #[doc = "Bits 3:4 - These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. 2'b00: 64， 2'b01: 32， 2'b10: 96， 2'b11: 128 ."]
    #[inline(always)]
    #[must_use]
    pub fn rx_thresh_ctrl(&mut self) -> RX_THRESH_CTRL_W<DMAOPERATION_MODE_SPEC, 3> {
        RX_THRESH_CTRL_W::new(self)
    }
    #[doc = "Bit 5 - When set the MAC drops the received giant frames in the Rx FIFO that is frames that are larger than the computed giant frame limit."]
    #[inline(always)]
    #[must_use]
    pub fn drop_gfrm(&mut self) -> DROP_GFRM_W<DMAOPERATION_MODE_SPEC, 5> {
        DROP_GFRM_W::new(self)
    }
    #[doc = "Bit 6 - When set the Rx FIFO forwards Undersized frames (that is frames with no Error and length less than 64 bytes) including pad-bytes and CRC."]
    #[inline(always)]
    #[must_use]
    pub fn fwd_under_gf(&mut self) -> FWD_UNDER_GF_W<DMAOPERATION_MODE_SPEC, 6> {
        FWD_UNDER_GF_W::new(self)
    }
    #[doc = "Bit 7 - When this bit is reset the Rx FIFO drops frames with error status (CRC error collision error giant frame watchdog timeout or overflow)."]
    #[inline(always)]
    #[must_use]
    pub fn fwd_err_frame(&mut self) -> FWD_ERR_FRAME_W<DMAOPERATION_MODE_SPEC, 7> {
        FWD_ERR_FRAME_W::new(self)
    }
    #[doc = "Bit 13 - When this bit is set transmission is placed in the Running state and the DMA checks the Transmit List at the current position for a frame to be transmitted.When this bit is reset the transmission process is placed in the Stopped state after completing the transmission of the current frame."]
    #[inline(always)]
    #[must_use]
    pub fn start_stop_transmission_command(
        &mut self,
    ) -> START_STOP_TRANSMISSION_COMMAND_W<DMAOPERATION_MODE_SPEC, 13> {
        START_STOP_TRANSMISSION_COMMAND_W::new(self)
    }
    #[doc = "Bits 14:16 - These bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition full frames with a length less than the threshold are also transmitted. These bits are used only when Tx_Str_fwd is reset. 3'b000: 64 3'b001: 128 3'b010: 192 3'b011: 256 3'b100: 40 3'b101: 32 3'b110: 24 3'b111: 16 ."]
    #[inline(always)]
    #[must_use]
    pub fn tx_thresh_ctrl(&mut self) -> TX_THRESH_CTRL_W<DMAOPERATION_MODE_SPEC, 14> {
        TX_THRESH_CTRL_W::new(self)
    }
    #[doc = "Bit 20 - When this bit is set the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed. This bit is cleared internally when the flushing operation is complete."]
    #[inline(always)]
    #[must_use]
    pub fn flush_tx_fifo(&mut self) -> FLUSH_TX_FIFO_W<DMAOPERATION_MODE_SPEC, 20> {
        FLUSH_TX_FIFO_W::new(self)
    }
    #[doc = "Bit 21 - When this bit is set transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set the Tx_Thresh_Ctrl values specified in Tx_Thresh_Ctrl are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn tx_str_fwd(&mut self) -> TX_STR_FWD_W<DMAOPERATION_MODE_SPEC, 21> {
        TX_STR_FWD_W::new(self)
    }
    #[doc = "Bit 24 - When this bit is set the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers."]
    #[inline(always)]
    #[must_use]
    pub fn dis_flush_recv_frames(&mut self) -> DIS_FLUSH_RECV_FRAMES_W<DMAOPERATION_MODE_SPEC, 24> {
        DIS_FLUSH_RECV_FRAMES_W::new(self)
    }
    #[doc = "Bit 25 - When this bit is set the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it."]
    #[inline(always)]
    #[must_use]
    pub fn rx_store_forward(&mut self) -> RX_STORE_FORWARD_W<DMAOPERATION_MODE_SPEC, 25> {
        RX_STORE_FORWARD_W::new(self)
    }
    #[doc = "Bit 26 - When this bit is set the MAC does not drop the frames which only have errors detected by the Receive Checksum engine.When this bit is reset all error frames are dropped if the Fwd_Err_Frame bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn dis_drop_tcpip_err_fram(
        &mut self,
    ) -> DIS_DROP_TCPIP_ERR_FRAM_W<DMAOPERATION_MODE_SPEC, 26> {
        DIS_DROP_TCPIP_ERR_FRAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receive and Transmit operating modes and command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaoperation_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaoperation_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAOPERATION_MODE_SPEC;
impl crate::RegisterSpec for DMAOPERATION_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaoperation_mode::R`](R) reader structure"]
impl crate::Readable for DMAOPERATION_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaoperation_mode::W`](W) writer structure"]
impl crate::Writable for DMAOPERATION_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAOPERATION_MODE to value 0"]
impl crate::Resettable for DMAOPERATION_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
