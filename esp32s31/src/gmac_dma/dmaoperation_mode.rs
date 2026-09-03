#[doc = "Register `DMAOPERATION_MODE` reader"]
pub type R = crate::R<DMAOPERATION_MODE_SPEC>;
#[doc = "Register `DMAOPERATION_MODE` writer"]
pub type W = crate::W<DMAOPERATION_MODE_SPEC>;
#[doc = "Field `START_STOP_RX` reader - Start or Stop Receive When this bit is set, the Receive process is placed in the Running state The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames The descriptor acquisition is attempted from the current position in the list, which is the address set by the Register 3 _Receive Descriptor List Address Register_ or the position retained when the Receive process was previously stopped If the DMA does not own the descriptor, reception is suspended and Bit 7 _Receive Buffer Unavailable_ of Register 5 _Status Register_ is set The Start Receive command is effective only when the reception has stopped If the command is issued before setting Register 3 _Receive Descriptor List Address Register_, the DMA behavior is unpredictable When this bit is cleared, the Rx DMA operation is stopped after the transfer of the current frame The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted The Stop Receive command is effective only when the Receive process is in either the Running _waiting for receive packet_ or in the Suspended state Note: For information about how to pause the transmission, see “Stopping and Starting Transmission” on page 715"]
pub type START_STOP_RX_R = crate::BitReader;
#[doc = "Field `START_STOP_RX` writer - Start or Stop Receive When this bit is set, the Receive process is placed in the Running state The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames The descriptor acquisition is attempted from the current position in the list, which is the address set by the Register 3 _Receive Descriptor List Address Register_ or the position retained when the Receive process was previously stopped If the DMA does not own the descriptor, reception is suspended and Bit 7 _Receive Buffer Unavailable_ of Register 5 _Status Register_ is set The Start Receive command is effective only when the reception has stopped If the command is issued before setting Register 3 _Receive Descriptor List Address Register_, the DMA behavior is unpredictable When this bit is cleared, the Rx DMA operation is stopped after the transfer of the current frame The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted The Stop Receive command is effective only when the Receive process is in either the Running _waiting for receive packet_ or in the Suspended state Note: For information about how to pause the transmission, see “Stopping and Starting Transmission” on page 715"]
pub type START_STOP_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPT_SECOND_FRAME` reader - Operate on Second Frame When this bit is set, it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained"]
pub type OPT_SECOND_FRAME_R = crate::BitReader;
#[doc = "Field `OPT_SECOND_FRAME` writer - Operate on Second Frame When this bit is set, it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained"]
pub type OPT_SECOND_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THRESH_CTRL` reader - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO Transfer _request_ to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold In addition, full frames with length less than the threshold are automatically transferred The value of 11 is not applicable if the configured Receive FIFO size is 128 bytes These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1 00: 64 01: 32 10: 96 11: 128"]
pub type RX_THRESH_CTRL_R = crate::FieldReader;
#[doc = "Field `RX_THRESH_CTRL` writer - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO Transfer _request_ to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold In addition, full frames with length less than the threshold are automatically transferred The value of 11 is not applicable if the configured Receive FIFO size is 128 bytes These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1 00: 64 01: 32 10: 96 11: 128"]
pub type RX_THRESH_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DROP_GFRM` reader - Drop Giant Frames When set, the MAC drops the received giant frames in the Rx FIFO, that is, frames that are larger than the computed giant frame limit When reset, the MAC does not drop the giant frames in the Rx FIFO Note: This bit is available in the following configurations in which the giant frame status is not provided in Rx status and giant frames are not dropped by default: Configurations in which IP Checksum Offload _Type 1_ is selected in Rx Configurations in which the IPC Full Checksum Offload Engine _Type 2_ is selected in Rx with normal descriptor format Configurations in which the Advanced Timestamp feature is selected In all other configurations, this bit is not used _reserved and always reset_"]
pub type DROP_GFRM_R = crate::BitReader;
#[doc = "Field `DROP_GFRM` writer - Drop Giant Frames When set, the MAC drops the received giant frames in the Rx FIFO, that is, frames that are larger than the computed giant frame limit When reset, the MAC does not drop the giant frames in the Rx FIFO Note: This bit is available in the following configurations in which the giant frame status is not provided in Rx status and giant frames are not dropped by default: Configurations in which IP Checksum Offload _Type 1_ is selected in Rx Configurations in which the IPC Full Checksum Offload Engine _Type 2_ is selected in Rx with normal descriptor format Configurations in which the Advanced Timestamp feature is selected In all other configurations, this bit is not used _reserved and always reset_"]
pub type DROP_GFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWD_UNDER_GF` reader - Forward Undersized Good Frames When set, the Rx FIFO forwards Undersized frames _that is, frames with no Error and length less than 64 bytes_ including padbytes and CRC When reset, the Rx FIFO drops all frames of less than 64 bytes, unless a frame is already transferred because of the lower value of Receive Threshold, for example, RTC = 01"]
pub type FWD_UNDER_GF_R = crate::BitReader;
#[doc = "Field `FWD_UNDER_GF` writer - Forward Undersized Good Frames When set, the Rx FIFO forwards Undersized frames _that is, frames with no Error and length less than 64 bytes_ including padbytes and CRC When reset, the Rx FIFO drops all frames of less than 64 bytes, unless a frame is already transferred because of the lower value of Receive Threshold, for example, RTC = 01"]
pub type FWD_UNDER_GF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWD_ERR_FRAME` reader - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status _CRC error, collision error, GMII_ER, giant frame, watchdog timeout, or overflow_ However, if the start byte _write_ pointer of a frame is already transferred to the read controller side _in Threshold mode_, then the frame is not dropped In the GMACMTL configuration in which the Frame Length FIFO is also enabled during core configuration, the Rx FIFO drops the error frames if that frame's start byte is not transferred _output_ on the ARI bus When the FEF bit is set, all frames except runt error frames are forwarded to the DMA If the Bit 25 _RSF_ is set and the Rx FIFO overflows when a partial frame is written, then the frame is dropped irrespective of the FEF bit setting However, if the Bit 25 _RSF_ is reset and the Rx FIFO overflows when a partial frame is written, then a partial frame may be forwarded to the DMA Note: When FEF bit is reset, the giant frames are dropped if the giant frame status is given in Rx Status _in Table 86 or Table 823_ in the following configurations: The IP checksum engine _Type 1_ and full checksum offload engine _Type 2_ are not selected The advanced timestamp feature is not selected but the extended status is selected The extended status is available with the following features: L3L4 filter in GMACCORE or GMACMTL configurations Full checksum offload engine _Type 2_ with enhanced descriptor format in the GMACDMA, GMACAHB, or GMACAXI configurations"]
pub type FWD_ERR_FRAME_R = crate::BitReader;
#[doc = "Field `FWD_ERR_FRAME` writer - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status _CRC error, collision error, GMII_ER, giant frame, watchdog timeout, or overflow_ However, if the start byte _write_ pointer of a frame is already transferred to the read controller side _in Threshold mode_, then the frame is not dropped In the GMACMTL configuration in which the Frame Length FIFO is also enabled during core configuration, the Rx FIFO drops the error frames if that frame's start byte is not transferred _output_ on the ARI bus When the FEF bit is set, all frames except runt error frames are forwarded to the DMA If the Bit 25 _RSF_ is set and the Rx FIFO overflows when a partial frame is written, then the frame is dropped irrespective of the FEF bit setting However, if the Bit 25 _RSF_ is reset and the Rx FIFO overflows when a partial frame is written, then a partial frame may be forwarded to the DMA Note: When FEF bit is reset, the giant frames are dropped if the giant frame status is given in Rx Status _in Table 86 or Table 823_ in the following configurations: The IP checksum engine _Type 1_ and full checksum offload engine _Type 2_ are not selected The advanced timestamp feature is not selected but the extended status is selected The extended status is available with the following features: L3L4 filter in GMACCORE or GMACMTL configurations Full checksum offload engine _Type 2_ with enhanced descriptor format in the GMACDMA, GMACAHB, or GMACAXI configurations"]
pub type FWD_ERR_FRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFC` reader - Enable HW Flow Control When this bit is set, the flow control signal operation based on the filllevel of Rx FIFO is enabled When reset, the flow control operation is disabled This bit is not used _reserved and always reset_ when the Rx FIFO is less than 4 KB"]
pub type EFC_R = crate::BitReader;
#[doc = "Field `EFC` writer - Enable HW Flow Control When this bit is set, the flow control signal operation based on the filllevel of Rx FIFO is enabled When reset, the flow control operation is disabled This bit is not used _reserved and always reset_ when the Rx FIFO is less than 4 KB"]
pub type EFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFA` reader - Threshold for Activating Flow Control _in halfduplex and fullduplex modes_ These bits control the threshold _Fill level of Rx FIFO_ at which the flow control is activated 00: Full minus 1 KB, that is, FULL—1KB 01: Full minus 2 KB, that is, FULL—2KB 10: Full minus 3 KB, that is, FULL—3KB 11: Full minus 4 KB, that is, FULL—4KB These values are applicable only to Rx FIFOs of 4 KB or more and when Bit 8 _EFC_ is set high If the Rx FIFO is 8 KB or more, an additional Bit _RFA_2_ is used for more threshold levels as described in Bit 23 These bits are reserved and readonly when the depth of Rx FIFO is less than 4 KB Note: When FIFO size is exactly 4 KB, although the DWC_gmac allows you to program the value of these bits to 11, the software should not program these bits to 2'b11 The value 2'b11 means flow control on FIFO empty condition"]
pub type RFA_R = crate::FieldReader;
#[doc = "Field `RFA` writer - Threshold for Activating Flow Control _in halfduplex and fullduplex modes_ These bits control the threshold _Fill level of Rx FIFO_ at which the flow control is activated 00: Full minus 1 KB, that is, FULL—1KB 01: Full minus 2 KB, that is, FULL—2KB 10: Full minus 3 KB, that is, FULL—3KB 11: Full minus 4 KB, that is, FULL—4KB These values are applicable only to Rx FIFOs of 4 KB or more and when Bit 8 _EFC_ is set high If the Rx FIFO is 8 KB or more, an additional Bit _RFA_2_ is used for more threshold levels as described in Bit 23 These bits are reserved and readonly when the depth of Rx FIFO is less than 4 KB Note: When FIFO size is exactly 4 KB, although the DWC_gmac allows you to program the value of these bits to 11, the software should not program these bits to 2'b11 The value 2'b11 means flow control on FIFO empty condition"]
pub type RFA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RFD` reader - Threshold for Deactivating Flow Control _in halfduplex and fullduplex modes_ These bits control the threshold _Filllevel of Rx FIFO_ at which the flow control is deasserted after activation 00: Full minus 1 KB, that is, FULL — 1 KB 01: Full minus 2 KB, that is, FULL — 2 KB 10: Full minus 3 KB, that is, FULL — 3 KB 11: Full minus 4 KB, that is, FULL — 4 KB The deassertion is effective only after flow control is asserted If the Rx FIFO is 8 KB or more, an additional Bit _RFD_2_ is used for more threshold levels as described in Bit 22 These bits are reserved and readonly when the Rx FIFO depth is less than 4 KB Note: For proper flow control, the value programmed in the “RFD_2, RFD” fields should be equal to or more than the value programmed in the “RFA_2, RFA” fields"]
pub type RFD_R = crate::FieldReader;
#[doc = "Field `RFD` writer - Threshold for Deactivating Flow Control _in halfduplex and fullduplex modes_ These bits control the threshold _Filllevel of Rx FIFO_ at which the flow control is deasserted after activation 00: Full minus 1 KB, that is, FULL — 1 KB 01: Full minus 2 KB, that is, FULL — 2 KB 10: Full minus 3 KB, that is, FULL — 3 KB 11: Full minus 4 KB, that is, FULL — 4 KB The deassertion is effective only after flow control is asserted If the Rx FIFO is 8 KB or more, an additional Bit _RFD_2_ is used for more threshold levels as described in Bit 22 These bits are reserved and readonly when the Rx FIFO depth is less than 4 KB Note: For proper flow control, the value programmed in the “RFD_2, RFD” fields should be equal to or more than the value programmed in the “RFA_2, RFA” fields"]
pub type RFD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `START_STOP_TRANSMISSION_COMMAND` reader - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register 4 _Transmit Descriptor List Address Register_, or from the position retained when transmission was stopped previously If the DMA does not own the current descriptor, transmission enters the Suspended state and Bit 2 _Transmit Buffer Unavailable_ of Register 5 _Status Register_ is set The Start Transmission command is effective only when transmission is stopped If the command is issued before setting Register 4 _Transmit Descriptor List Address Register_, then the DMA behavior is unpredictable When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame The Next Descriptor position in the Transmit List is saved, and it becomes the current position when transmission is restarted To change the list address, you need to program Register 4 _Transmit Descriptor List Address Register_ with a new value when this bit is reset The new value is considered when this bit is set again The stop transmission command is effective only when the transmission of the current frame is complete or the transmission is in the Suspended state Note: For information about how to pause the transmission, see “Stopping and Starting Transmission” on page 715"]
pub type START_STOP_TRANSMISSION_COMMAND_R = crate::BitReader;
#[doc = "Field `START_STOP_TRANSMISSION_COMMAND` writer - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register 4 _Transmit Descriptor List Address Register_, or from the position retained when transmission was stopped previously If the DMA does not own the current descriptor, transmission enters the Suspended state and Bit 2 _Transmit Buffer Unavailable_ of Register 5 _Status Register_ is set The Start Transmission command is effective only when transmission is stopped If the command is issued before setting Register 4 _Transmit Descriptor List Address Register_, then the DMA behavior is unpredictable When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame The Next Descriptor position in the Transmit List is saved, and it becomes the current position when transmission is restarted To change the list address, you need to program Register 4 _Transmit Descriptor List Address Register_ with a new value when this bit is reset The new value is considered when this bit is set again The stop transmission command is effective only when the transmission of the current frame is complete or the transmission is in the Suspended state Note: For information about how to pause the transmission, see “Stopping and Starting Transmission” on page 715"]
pub type START_STOP_TRANSMISSION_COMMAND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_THRESH_CTRL` reader - Transmit Threshold Control These bits control the threshold level of the MTL Transmit FIFO Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold In addition, full frames with a length less than the threshold are also transmitted These bits are used only when Bit 21 _TSF_ is reset 000: 64 001: 128 010: 192 011: 256 100: 40 101: 32 110: 24 111: 16"]
pub type TX_THRESH_CTRL_R = crate::FieldReader;
#[doc = "Field `TX_THRESH_CTRL` writer - Transmit Threshold Control These bits control the threshold level of the MTL Transmit FIFO Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold In addition, full frames with a length less than the threshold are also transmitted These bits are used only when Bit 21 _TSF_ is reset 000: 64 001: 128 010: 192 011: 256 100: 40 101: 32 110: 24 111: 16"]
pub type TX_THRESH_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLUSH_TX_FIFO` reader - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed This bit is cleared internally when the flushing operation is complete The Operation Mode register should not be written to until this bit is cleared The data which is already accepted by the MAC transmitter is not flushed It is scheduled for transmission and results in underflow and runt frame transmission Note: The flush operation is complete only when the Tx FIFO is emptied of its contents and all the pending Transmit Status of the transmitted frames are accepted by the host In order to complete this flush operation, the PHY transmit clock _clk_tx_i_ is required to be active 19:17 Reserved 000 RO"]
pub type FLUSH_TX_FIFO_R = crate::BitReader;
#[doc = "Field `FLUSH_TX_FIFO` writer - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed This bit is cleared internally when the flushing operation is complete The Operation Mode register should not be written to until this bit is cleared The data which is already accepted by the MAC transmitter is not flushed It is scheduled for transmission and results in underflow and runt frame transmission Note: The flush operation is complete only when the Tx FIFO is emptied of its contents and all the pending Transmit Status of the transmitted frames are accepted by the host In order to complete this flush operation, the PHY transmit clock _clk_tx_i_ is required to be active 19:17 Reserved 000 RO"]
pub type FLUSH_TX_FIFO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STR_FWD` reader - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO When this bit is set, the TTC values specified in Bits \\[16:14\\] are ignored This bit should be changed only when the transmission is stopped"]
pub type TX_STR_FWD_R = crate::BitReader;
#[doc = "Field `TX_STR_FWD` writer - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO When this bit is set, the TTC values specified in Bits \\[16:14\\] are ignored This bit should be changed only when the transmission is stopped"]
pub type TX_STR_FWD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFD_2` reader - MSB of Threshold for Deactivating Flow Control If the DWC_gmac is configured for Rx FIFO size of 8 KB or more, this bit _when set_ provides additional threshold levels for deactivating the flow control in both halfduplex and fullduplex modes This bit _as Most Significant Bit_ along with the RFD _Bits \\[12:11\\]_ gives the following thresholds for deactivating flow control: 100: Full minus 5 KB, that is, FULL — 5 KB 101: Full minus 6 KB, that is, FULL — 6 KB 110: Full minus 7 KB, that is, FULL — 7 KB 111: Reserved This bit is reserved _and RO_ if the Rx FIFO is 4 KB or less deep"]
pub type RFD_2_R = crate::BitReader;
#[doc = "Field `RFD_2` writer - MSB of Threshold for Deactivating Flow Control If the DWC_gmac is configured for Rx FIFO size of 8 KB or more, this bit _when set_ provides additional threshold levels for deactivating the flow control in both halfduplex and fullduplex modes This bit _as Most Significant Bit_ along with the RFD _Bits \\[12:11\\]_ gives the following thresholds for deactivating flow control: 100: Full minus 5 KB, that is, FULL — 5 KB 101: Full minus 6 KB, that is, FULL — 6 KB 110: Full minus 7 KB, that is, FULL — 7 KB 111: Reserved This bit is reserved _and RO_ if the Rx FIFO is 4 KB or less deep"]
pub type RFD_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFA_2` reader - MSB of Threshold for Activating Flow Control If the DWC_gmac is configured for an Rx FIFO size of 8 KB or more, this bit _when set_ provides additional threshold levels for activating the flow control in both half duplex and fullduplex modes This bit _as Most Significant Bit_, along with the RFA _Bits \\[10:9\\]_, gives the following thresholds for activating flow control: 100: Full minus 5 KB, that is, FULL — 5 KB 101: Full minus 6 KB, that is, FULL — 6 KB 110: Full minus 7 KB, that is, FULL — 7 KB 111: Reserved This bit is reserved _and RO_ if the Rx FIFO is 4 KB or less deep"]
pub type RFA_2_R = crate::BitReader;
#[doc = "Field `RFA_2` writer - MSB of Threshold for Activating Flow Control If the DWC_gmac is configured for an Rx FIFO size of 8 KB or more, this bit _when set_ provides additional threshold levels for activating the flow control in both half duplex and fullduplex modes This bit _as Most Significant Bit_, along with the RFA _Bits \\[10:9\\]_, gives the following thresholds for activating flow control: 100: Full minus 5 KB, that is, FULL — 5 KB 101: Full minus 6 KB, that is, FULL — 6 KB 110: Full minus 7 KB, that is, FULL — 7 KB 111: Reserved This bit is reserved _and RO_ if the Rx FIFO is 4 KB or less deep"]
pub type RFA_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_FLUSH_RECV_FRAMES` reader - Disable Flushing of Received Frames When this bit is set, the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers as it does normally when this bit is reset _See “Receive Process Suspended” on page 83_ This bit is reserved _and RO_ in the GMACMTL configuration"]
pub type DIS_FLUSH_RECV_FRAMES_R = crate::BitReader;
#[doc = "Field `DIS_FLUSH_RECV_FRAMES` writer - Disable Flushing of Received Frames When this bit is set, the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers as it does normally when this bit is reset _See “Receive Process Suspended” on page 83_ This bit is reserved _and RO_ in the GMACMTL configuration"]
pub type DIS_FLUSH_RECV_FRAMES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_STORE_FORWARD` reader - Receive Store and Forward When this bit is set, the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it, ignoring the RTC bits When this bit is reset, the Rx FIFO operates in the cutthrough mode, subject to the threshold specified by the RTC bits"]
pub type RX_STORE_FORWARD_R = crate::BitReader;
#[doc = "Field `RX_STORE_FORWARD` writer - Receive Store and Forward When this bit is set, the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it, ignoring the RTC bits When this bit is reset, the Rx FIFO operates in the cutthrough mode, subject to the threshold specified by the RTC bits"]
pub type RX_STORE_FORWARD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIS_DROP_TCPIP_ERR_FRAM` reader - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the MAC does not drop the frames which only have errors detected by the Receive Checksum Offload engine Such frames do not have any errors _including FCS error_ in the Ethernet frame received by the MAC but have errors only in the encapsulated payload When this bit is reset, all error frames are dropped if the FEF bit is reset If the IPC Full Checksum Offload Engine _Type 2_ is disabled, this bit is reserved _RO with value 1'b0_"]
pub type DIS_DROP_TCPIP_ERR_FRAM_R = crate::BitReader;
#[doc = "Field `DIS_DROP_TCPIP_ERR_FRAM` writer - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the MAC does not drop the frames which only have errors detected by the Receive Checksum Offload engine Such frames do not have any errors _including FCS error_ in the Ethernet frame received by the MAC but have errors only in the encapsulated payload When this bit is reset, all error frames are dropped if the FEF bit is reset If the IPC Full Checksum Offload Engine _Type 2_ is disabled, this bit is reserved _RO with value 1'b0_"]
pub type DIS_DROP_TCPIP_ERR_FRAM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Start or Stop Receive When this bit is set, the Receive process is placed in the Running state The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames The descriptor acquisition is attempted from the current position in the list, which is the address set by the Register 3 _Receive Descriptor List Address Register_ or the position retained when the Receive process was previously stopped If the DMA does not own the descriptor, reception is suspended and Bit 7 _Receive Buffer Unavailable_ of Register 5 _Status Register_ is set The Start Receive command is effective only when the reception has stopped If the command is issued before setting Register 3 _Receive Descriptor List Address Register_, the DMA behavior is unpredictable When this bit is cleared, the Rx DMA operation is stopped after the transfer of the current frame The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted The Stop Receive command is effective only when the Receive process is in either the Running _waiting for receive packet_ or in the Suspended state Note: For information about how to pause the transmission, see “Stopping and Starting Transmission” on page 715"]
    #[inline(always)]
    pub fn start_stop_rx(&self) -> START_STOP_RX_R {
        START_STOP_RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on Second Frame When this bit is set, it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained"]
    #[inline(always)]
    pub fn opt_second_frame(&self) -> OPT_SECOND_FRAME_R {
        OPT_SECOND_FRAME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO Transfer _request_ to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold In addition, full frames with length less than the threshold are automatically transferred The value of 11 is not applicable if the configured Receive FIFO size is 128 bytes These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1 00: 64 01: 32 10: 96 11: 128"]
    #[inline(always)]
    pub fn rx_thresh_ctrl(&self) -> RX_THRESH_CTRL_R {
        RX_THRESH_CTRL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Drop Giant Frames When set, the MAC drops the received giant frames in the Rx FIFO, that is, frames that are larger than the computed giant frame limit When reset, the MAC does not drop the giant frames in the Rx FIFO Note: This bit is available in the following configurations in which the giant frame status is not provided in Rx status and giant frames are not dropped by default: Configurations in which IP Checksum Offload _Type 1_ is selected in Rx Configurations in which the IPC Full Checksum Offload Engine _Type 2_ is selected in Rx with normal descriptor format Configurations in which the Advanced Timestamp feature is selected In all other configurations, this bit is not used _reserved and always reset_"]
    #[inline(always)]
    pub fn drop_gfrm(&self) -> DROP_GFRM_R {
        DROP_GFRM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames When set, the Rx FIFO forwards Undersized frames _that is, frames with no Error and length less than 64 bytes_ including padbytes and CRC When reset, the Rx FIFO drops all frames of less than 64 bytes, unless a frame is already transferred because of the lower value of Receive Threshold, for example, RTC = 01"]
    #[inline(always)]
    pub fn fwd_under_gf(&self) -> FWD_UNDER_GF_R {
        FWD_UNDER_GF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status _CRC error, collision error, GMII_ER, giant frame, watchdog timeout, or overflow_ However, if the start byte _write_ pointer of a frame is already transferred to the read controller side _in Threshold mode_, then the frame is not dropped In the GMACMTL configuration in which the Frame Length FIFO is also enabled during core configuration, the Rx FIFO drops the error frames if that frame's start byte is not transferred _output_ on the ARI bus When the FEF bit is set, all frames except runt error frames are forwarded to the DMA If the Bit 25 _RSF_ is set and the Rx FIFO overflows when a partial frame is written, then the frame is dropped irrespective of the FEF bit setting However, if the Bit 25 _RSF_ is reset and the Rx FIFO overflows when a partial frame is written, then a partial frame may be forwarded to the DMA Note: When FEF bit is reset, the giant frames are dropped if the giant frame status is given in Rx Status _in Table 86 or Table 823_ in the following configurations: The IP checksum engine _Type 1_ and full checksum offload engine _Type 2_ are not selected The advanced timestamp feature is not selected but the extended status is selected The extended status is available with the following features: L3L4 filter in GMACCORE or GMACMTL configurations Full checksum offload engine _Type 2_ with enhanced descriptor format in the GMACDMA, GMACAHB, or GMACAXI configurations"]
    #[inline(always)]
    pub fn fwd_err_frame(&self) -> FWD_ERR_FRAME_R {
        FWD_ERR_FRAME_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable HW Flow Control When this bit is set, the flow control signal operation based on the filllevel of Rx FIFO is enabled When reset, the flow control operation is disabled This bit is not used _reserved and always reset_ when the Rx FIFO is less than 4 KB"]
    #[inline(always)]
    pub fn efc(&self) -> EFC_R {
        EFC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Threshold for Activating Flow Control _in halfduplex and fullduplex modes_ These bits control the threshold _Fill level of Rx FIFO_ at which the flow control is activated 00: Full minus 1 KB, that is, FULL—1KB 01: Full minus 2 KB, that is, FULL—2KB 10: Full minus 3 KB, that is, FULL—3KB 11: Full minus 4 KB, that is, FULL—4KB These values are applicable only to Rx FIFOs of 4 KB or more and when Bit 8 _EFC_ is set high If the Rx FIFO is 8 KB or more, an additional Bit _RFA_2_ is used for more threshold levels as described in Bit 23 These bits are reserved and readonly when the depth of Rx FIFO is less than 4 KB Note: When FIFO size is exactly 4 KB, although the DWC_gmac allows you to program the value of these bits to 11, the software should not program these bits to 2'b11 The value 2'b11 means flow control on FIFO empty condition"]
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Threshold for Deactivating Flow Control _in halfduplex and fullduplex modes_ These bits control the threshold _Filllevel of Rx FIFO_ at which the flow control is deasserted after activation 00: Full minus 1 KB, that is, FULL — 1 KB 01: Full minus 2 KB, that is, FULL — 2 KB 10: Full minus 3 KB, that is, FULL — 3 KB 11: Full minus 4 KB, that is, FULL — 4 KB The deassertion is effective only after flow control is asserted If the Rx FIFO is 8 KB or more, an additional Bit _RFD_2_ is used for more threshold levels as described in Bit 22 These bits are reserved and readonly when the Rx FIFO depth is less than 4 KB Note: For proper flow control, the value programmed in the “RFD_2, RFD” fields should be equal to or more than the value programmed in the “RFA_2, RFA” fields"]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register 4 _Transmit Descriptor List Address Register_, or from the position retained when transmission was stopped previously If the DMA does not own the current descriptor, transmission enters the Suspended state and Bit 2 _Transmit Buffer Unavailable_ of Register 5 _Status Register_ is set The Start Transmission command is effective only when transmission is stopped If the command is issued before setting Register 4 _Transmit Descriptor List Address Register_, then the DMA behavior is unpredictable When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame The Next Descriptor position in the Transmit List is saved, and it becomes the current position when transmission is restarted To change the list address, you need to program Register 4 _Transmit Descriptor List Address Register_ with a new value when this bit is reset The new value is considered when this bit is set again The stop transmission command is effective only when the transmission of the current frame is complete or the transmission is in the Suspended state Note: For information about how to pause the transmission, see “Stopping and Starting Transmission” on page 715"]
    #[inline(always)]
    pub fn start_stop_transmission_command(&self) -> START_STOP_TRANSMISSION_COMMAND_R {
        START_STOP_TRANSMISSION_COMMAND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control These bits control the threshold level of the MTL Transmit FIFO Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold In addition, full frames with a length less than the threshold are also transmitted These bits are used only when Bit 21 _TSF_ is reset 000: 64 001: 128 010: 192 011: 256 100: 40 101: 32 110: 24 111: 16"]
    #[inline(always)]
    pub fn tx_thresh_ctrl(&self) -> TX_THRESH_CTRL_R {
        TX_THRESH_CTRL_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed This bit is cleared internally when the flushing operation is complete The Operation Mode register should not be written to until this bit is cleared The data which is already accepted by the MAC transmitter is not flushed It is scheduled for transmission and results in underflow and runt frame transmission Note: The flush operation is complete only when the Tx FIFO is emptied of its contents and all the pending Transmit Status of the transmitted frames are accepted by the host In order to complete this flush operation, the PHY transmit clock _clk_tx_i_ is required to be active 19:17 Reserved 000 RO"]
    #[inline(always)]
    pub fn flush_tx_fifo(&self) -> FLUSH_TX_FIFO_R {
        FLUSH_TX_FIFO_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO When this bit is set, the TTC values specified in Bits \\[16:14\\] are ignored This bit should be changed only when the transmission is stopped"]
    #[inline(always)]
    pub fn tx_str_fwd(&self) -> TX_STR_FWD_R {
        TX_STR_FWD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MSB of Threshold for Deactivating Flow Control If the DWC_gmac is configured for Rx FIFO size of 8 KB or more, this bit _when set_ provides additional threshold levels for deactivating the flow control in both halfduplex and fullduplex modes This bit _as Most Significant Bit_ along with the RFD _Bits \\[12:11\\]_ gives the following thresholds for deactivating flow control: 100: Full minus 5 KB, that is, FULL — 5 KB 101: Full minus 6 KB, that is, FULL — 6 KB 110: Full minus 7 KB, that is, FULL — 7 KB 111: Reserved This bit is reserved _and RO_ if the Rx FIFO is 4 KB or less deep"]
    #[inline(always)]
    pub fn rfd_2(&self) -> RFD_2_R {
        RFD_2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MSB of Threshold for Activating Flow Control If the DWC_gmac is configured for an Rx FIFO size of 8 KB or more, this bit _when set_ provides additional threshold levels for activating the flow control in both half duplex and fullduplex modes This bit _as Most Significant Bit_, along with the RFA _Bits \\[10:9\\]_, gives the following thresholds for activating flow control: 100: Full minus 5 KB, that is, FULL — 5 KB 101: Full minus 6 KB, that is, FULL — 6 KB 110: Full minus 7 KB, that is, FULL — 7 KB 111: Reserved This bit is reserved _and RO_ if the Rx FIFO is 4 KB or less deep"]
    #[inline(always)]
    pub fn rfa_2(&self) -> RFA_2_R {
        RFA_2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames When this bit is set, the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers as it does normally when this bit is reset _See “Receive Process Suspended” on page 83_ This bit is reserved _and RO_ in the GMACMTL configuration"]
    #[inline(always)]
    pub fn dis_flush_recv_frames(&self) -> DIS_FLUSH_RECV_FRAMES_R {
        DIS_FLUSH_RECV_FRAMES_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Store and Forward When this bit is set, the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it, ignoring the RTC bits When this bit is reset, the Rx FIFO operates in the cutthrough mode, subject to the threshold specified by the RTC bits"]
    #[inline(always)]
    pub fn rx_store_forward(&self) -> RX_STORE_FORWARD_R {
        RX_STORE_FORWARD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the MAC does not drop the frames which only have errors detected by the Receive Checksum Offload engine Such frames do not have any errors _including FCS error_ in the Ethernet frame received by the MAC but have errors only in the encapsulated payload When this bit is reset, all error frames are dropped if the FEF bit is reset If the IPC Full Checksum Offload Engine _Type 2_ is disabled, this bit is reserved _RO with value 1'b0_"]
    #[inline(always)]
    pub fn dis_drop_tcpip_err_fram(&self) -> DIS_DROP_TCPIP_ERR_FRAM_R {
        DIS_DROP_TCPIP_ERR_FRAM_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAOPERATION_MODE")
            .field("start_stop_rx", &self.start_stop_rx())
            .field("opt_second_frame", &self.opt_second_frame())
            .field("rx_thresh_ctrl", &self.rx_thresh_ctrl())
            .field("drop_gfrm", &self.drop_gfrm())
            .field("fwd_under_gf", &self.fwd_under_gf())
            .field("fwd_err_frame", &self.fwd_err_frame())
            .field("efc", &self.efc())
            .field("rfa", &self.rfa())
            .field("rfd", &self.rfd())
            .field(
                "start_stop_transmission_command",
                &self.start_stop_transmission_command(),
            )
            .field("tx_thresh_ctrl", &self.tx_thresh_ctrl())
            .field("flush_tx_fifo", &self.flush_tx_fifo())
            .field("tx_str_fwd", &self.tx_str_fwd())
            .field("rfd_2", &self.rfd_2())
            .field("rfa_2", &self.rfa_2())
            .field("dis_flush_recv_frames", &self.dis_flush_recv_frames())
            .field("rx_store_forward", &self.rx_store_forward())
            .field("dis_drop_tcpip_err_fram", &self.dis_drop_tcpip_err_fram())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Start or Stop Receive When this bit is set, the Receive process is placed in the Running state The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames The descriptor acquisition is attempted from the current position in the list, which is the address set by the Register 3 _Receive Descriptor List Address Register_ or the position retained when the Receive process was previously stopped If the DMA does not own the descriptor, reception is suspended and Bit 7 _Receive Buffer Unavailable_ of Register 5 _Status Register_ is set The Start Receive command is effective only when the reception has stopped If the command is issued before setting Register 3 _Receive Descriptor List Address Register_, the DMA behavior is unpredictable When this bit is cleared, the Rx DMA operation is stopped after the transfer of the current frame The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted The Stop Receive command is effective only when the Receive process is in either the Running _waiting for receive packet_ or in the Suspended state Note: For information about how to pause the transmission, see “Stopping and Starting Transmission” on page 715"]
    #[inline(always)]
    pub fn start_stop_rx(&mut self) -> START_STOP_RX_W<'_, DMAOPERATION_MODE_SPEC> {
        START_STOP_RX_W::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on Second Frame When this bit is set, it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained"]
    #[inline(always)]
    pub fn opt_second_frame(&mut self) -> OPT_SECOND_FRAME_W<'_, DMAOPERATION_MODE_SPEC> {
        OPT_SECOND_FRAME_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO Transfer _request_ to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold In addition, full frames with length less than the threshold are automatically transferred The value of 11 is not applicable if the configured Receive FIFO size is 128 bytes These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1 00: 64 01: 32 10: 96 11: 128"]
    #[inline(always)]
    pub fn rx_thresh_ctrl(&mut self) -> RX_THRESH_CTRL_W<'_, DMAOPERATION_MODE_SPEC> {
        RX_THRESH_CTRL_W::new(self, 3)
    }
    #[doc = "Bit 5 - Drop Giant Frames When set, the MAC drops the received giant frames in the Rx FIFO, that is, frames that are larger than the computed giant frame limit When reset, the MAC does not drop the giant frames in the Rx FIFO Note: This bit is available in the following configurations in which the giant frame status is not provided in Rx status and giant frames are not dropped by default: Configurations in which IP Checksum Offload _Type 1_ is selected in Rx Configurations in which the IPC Full Checksum Offload Engine _Type 2_ is selected in Rx with normal descriptor format Configurations in which the Advanced Timestamp feature is selected In all other configurations, this bit is not used _reserved and always reset_"]
    #[inline(always)]
    pub fn drop_gfrm(&mut self) -> DROP_GFRM_W<'_, DMAOPERATION_MODE_SPEC> {
        DROP_GFRM_W::new(self, 5)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames When set, the Rx FIFO forwards Undersized frames _that is, frames with no Error and length less than 64 bytes_ including padbytes and CRC When reset, the Rx FIFO drops all frames of less than 64 bytes, unless a frame is already transferred because of the lower value of Receive Threshold, for example, RTC = 01"]
    #[inline(always)]
    pub fn fwd_under_gf(&mut self) -> FWD_UNDER_GF_W<'_, DMAOPERATION_MODE_SPEC> {
        FWD_UNDER_GF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status _CRC error, collision error, GMII_ER, giant frame, watchdog timeout, or overflow_ However, if the start byte _write_ pointer of a frame is already transferred to the read controller side _in Threshold mode_, then the frame is not dropped In the GMACMTL configuration in which the Frame Length FIFO is also enabled during core configuration, the Rx FIFO drops the error frames if that frame's start byte is not transferred _output_ on the ARI bus When the FEF bit is set, all frames except runt error frames are forwarded to the DMA If the Bit 25 _RSF_ is set and the Rx FIFO overflows when a partial frame is written, then the frame is dropped irrespective of the FEF bit setting However, if the Bit 25 _RSF_ is reset and the Rx FIFO overflows when a partial frame is written, then a partial frame may be forwarded to the DMA Note: When FEF bit is reset, the giant frames are dropped if the giant frame status is given in Rx Status _in Table 86 or Table 823_ in the following configurations: The IP checksum engine _Type 1_ and full checksum offload engine _Type 2_ are not selected The advanced timestamp feature is not selected but the extended status is selected The extended status is available with the following features: L3L4 filter in GMACCORE or GMACMTL configurations Full checksum offload engine _Type 2_ with enhanced descriptor format in the GMACDMA, GMACAHB, or GMACAXI configurations"]
    #[inline(always)]
    pub fn fwd_err_frame(&mut self) -> FWD_ERR_FRAME_W<'_, DMAOPERATION_MODE_SPEC> {
        FWD_ERR_FRAME_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable HW Flow Control When this bit is set, the flow control signal operation based on the filllevel of Rx FIFO is enabled When reset, the flow control operation is disabled This bit is not used _reserved and always reset_ when the Rx FIFO is less than 4 KB"]
    #[inline(always)]
    pub fn efc(&mut self) -> EFC_W<'_, DMAOPERATION_MODE_SPEC> {
        EFC_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Threshold for Activating Flow Control _in halfduplex and fullduplex modes_ These bits control the threshold _Fill level of Rx FIFO_ at which the flow control is activated 00: Full minus 1 KB, that is, FULL—1KB 01: Full minus 2 KB, that is, FULL—2KB 10: Full minus 3 KB, that is, FULL—3KB 11: Full minus 4 KB, that is, FULL—4KB These values are applicable only to Rx FIFOs of 4 KB or more and when Bit 8 _EFC_ is set high If the Rx FIFO is 8 KB or more, an additional Bit _RFA_2_ is used for more threshold levels as described in Bit 23 These bits are reserved and readonly when the depth of Rx FIFO is less than 4 KB Note: When FIFO size is exactly 4 KB, although the DWC_gmac allows you to program the value of these bits to 11, the software should not program these bits to 2'b11 The value 2'b11 means flow control on FIFO empty condition"]
    #[inline(always)]
    pub fn rfa(&mut self) -> RFA_W<'_, DMAOPERATION_MODE_SPEC> {
        RFA_W::new(self, 9)
    }
    #[doc = "Bits 11:12 - Threshold for Deactivating Flow Control _in halfduplex and fullduplex modes_ These bits control the threshold _Filllevel of Rx FIFO_ at which the flow control is deasserted after activation 00: Full minus 1 KB, that is, FULL — 1 KB 01: Full minus 2 KB, that is, FULL — 2 KB 10: Full minus 3 KB, that is, FULL — 3 KB 11: Full minus 4 KB, that is, FULL — 4 KB The deassertion is effective only after flow control is asserted If the Rx FIFO is 8 KB or more, an additional Bit _RFD_2_ is used for more threshold levels as described in Bit 22 These bits are reserved and readonly when the Rx FIFO depth is less than 4 KB Note: For proper flow control, the value programmed in the “RFD_2, RFD” fields should be equal to or more than the value programmed in the “RFA_2, RFA” fields"]
    #[inline(always)]
    pub fn rfd(&mut self) -> RFD_W<'_, DMAOPERATION_MODE_SPEC> {
        RFD_W::new(self, 11)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register 4 _Transmit Descriptor List Address Register_, or from the position retained when transmission was stopped previously If the DMA does not own the current descriptor, transmission enters the Suspended state and Bit 2 _Transmit Buffer Unavailable_ of Register 5 _Status Register_ is set The Start Transmission command is effective only when transmission is stopped If the command is issued before setting Register 4 _Transmit Descriptor List Address Register_, then the DMA behavior is unpredictable When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame The Next Descriptor position in the Transmit List is saved, and it becomes the current position when transmission is restarted To change the list address, you need to program Register 4 _Transmit Descriptor List Address Register_ with a new value when this bit is reset The new value is considered when this bit is set again The stop transmission command is effective only when the transmission of the current frame is complete or the transmission is in the Suspended state Note: For information about how to pause the transmission, see “Stopping and Starting Transmission” on page 715"]
    #[inline(always)]
    pub fn start_stop_transmission_command(
        &mut self,
    ) -> START_STOP_TRANSMISSION_COMMAND_W<'_, DMAOPERATION_MODE_SPEC> {
        START_STOP_TRANSMISSION_COMMAND_W::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control These bits control the threshold level of the MTL Transmit FIFO Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold In addition, full frames with a length less than the threshold are also transmitted These bits are used only when Bit 21 _TSF_ is reset 000: 64 001: 128 010: 192 011: 256 100: 40 101: 32 110: 24 111: 16"]
    #[inline(always)]
    pub fn tx_thresh_ctrl(&mut self) -> TX_THRESH_CTRL_W<'_, DMAOPERATION_MODE_SPEC> {
        TX_THRESH_CTRL_W::new(self, 14)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed This bit is cleared internally when the flushing operation is complete The Operation Mode register should not be written to until this bit is cleared The data which is already accepted by the MAC transmitter is not flushed It is scheduled for transmission and results in underflow and runt frame transmission Note: The flush operation is complete only when the Tx FIFO is emptied of its contents and all the pending Transmit Status of the transmitted frames are accepted by the host In order to complete this flush operation, the PHY transmit clock _clk_tx_i_ is required to be active 19:17 Reserved 000 RO"]
    #[inline(always)]
    pub fn flush_tx_fifo(&mut self) -> FLUSH_TX_FIFO_W<'_, DMAOPERATION_MODE_SPEC> {
        FLUSH_TX_FIFO_W::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO When this bit is set, the TTC values specified in Bits \\[16:14\\] are ignored This bit should be changed only when the transmission is stopped"]
    #[inline(always)]
    pub fn tx_str_fwd(&mut self) -> TX_STR_FWD_W<'_, DMAOPERATION_MODE_SPEC> {
        TX_STR_FWD_W::new(self, 21)
    }
    #[doc = "Bit 22 - MSB of Threshold for Deactivating Flow Control If the DWC_gmac is configured for Rx FIFO size of 8 KB or more, this bit _when set_ provides additional threshold levels for deactivating the flow control in both halfduplex and fullduplex modes This bit _as Most Significant Bit_ along with the RFD _Bits \\[12:11\\]_ gives the following thresholds for deactivating flow control: 100: Full minus 5 KB, that is, FULL — 5 KB 101: Full minus 6 KB, that is, FULL — 6 KB 110: Full minus 7 KB, that is, FULL — 7 KB 111: Reserved This bit is reserved _and RO_ if the Rx FIFO is 4 KB or less deep"]
    #[inline(always)]
    pub fn rfd_2(&mut self) -> RFD_2_W<'_, DMAOPERATION_MODE_SPEC> {
        RFD_2_W::new(self, 22)
    }
    #[doc = "Bit 23 - MSB of Threshold for Activating Flow Control If the DWC_gmac is configured for an Rx FIFO size of 8 KB or more, this bit _when set_ provides additional threshold levels for activating the flow control in both half duplex and fullduplex modes This bit _as Most Significant Bit_, along with the RFA _Bits \\[10:9\\]_, gives the following thresholds for activating flow control: 100: Full minus 5 KB, that is, FULL — 5 KB 101: Full minus 6 KB, that is, FULL — 6 KB 110: Full minus 7 KB, that is, FULL — 7 KB 111: Reserved This bit is reserved _and RO_ if the Rx FIFO is 4 KB or less deep"]
    #[inline(always)]
    pub fn rfa_2(&mut self) -> RFA_2_W<'_, DMAOPERATION_MODE_SPEC> {
        RFA_2_W::new(self, 23)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames When this bit is set, the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers as it does normally when this bit is reset _See “Receive Process Suspended” on page 83_ This bit is reserved _and RO_ in the GMACMTL configuration"]
    #[inline(always)]
    pub fn dis_flush_recv_frames(&mut self) -> DIS_FLUSH_RECV_FRAMES_W<'_, DMAOPERATION_MODE_SPEC> {
        DIS_FLUSH_RECV_FRAMES_W::new(self, 24)
    }
    #[doc = "Bit 25 - Receive Store and Forward When this bit is set, the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it, ignoring the RTC bits When this bit is reset, the Rx FIFO operates in the cutthrough mode, subject to the threshold specified by the RTC bits"]
    #[inline(always)]
    pub fn rx_store_forward(&mut self) -> RX_STORE_FORWARD_W<'_, DMAOPERATION_MODE_SPEC> {
        RX_STORE_FORWARD_W::new(self, 25)
    }
    #[doc = "Bit 26 - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the MAC does not drop the frames which only have errors detected by the Receive Checksum Offload engine Such frames do not have any errors _including FCS error_ in the Ethernet frame received by the MAC but have errors only in the encapsulated payload When this bit is reset, all error frames are dropped if the FEF bit is reset If the IPC Full Checksum Offload Engine _Type 2_ is disabled, this bit is reserved _RO with value 1'b0_"]
    #[inline(always)]
    pub fn dis_drop_tcpip_err_fram(
        &mut self,
    ) -> DIS_DROP_TCPIP_ERR_FRAM_W<'_, DMAOPERATION_MODE_SPEC> {
        DIS_DROP_TCPIP_ERR_FRAM_W::new(self, 26)
    }
}
#[doc = "Receive and Transmit operating modes and command\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaoperation_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaoperation_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAOPERATION_MODE_SPEC;
impl crate::RegisterSpec for DMAOPERATION_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaoperation_mode::R`](R) reader structure"]
impl crate::Readable for DMAOPERATION_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaoperation_mode::W`](W) writer structure"]
impl crate::Writable for DMAOPERATION_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAOPERATION_MODE to value 0"]
impl crate::Resettable for DMAOPERATION_MODE_SPEC {}
