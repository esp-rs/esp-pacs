#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DCTL_SPEC>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DCTL_SPEC>;
#[doc = "Field `RMTWKUPSIG` reader - Remote Wakeup Signaling (RmtWkUpSig) When the application sets this bit, the core initiates remote signaling to wake up the USB host. The application must Set this bit to instruct the core to exit the Suspend state. As specified in the USB 2.0 specification, the application must clear this bit 1-15 ms after setting it. If LPM is enabled and the core is in the L1 (Sleep) state, when the application sets this bit, the core initiates L1 remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the Sleep state. As specified in the LPM specification, the hardware automatically clears this bit 50 microseconds (TL1DevDrvResume) after being set by the application. The application must not set this bit when GLPMCFG bRemoteWake from the previous LPM transaction is zero."]
pub type RMTWKUPSIG_R = crate::BitReader;
#[doc = "Field `RMTWKUPSIG` writer - Remote Wakeup Signaling (RmtWkUpSig) When the application sets this bit, the core initiates remote signaling to wake up the USB host. The application must Set this bit to instruct the core to exit the Suspend state. As specified in the USB 2.0 specification, the application must clear this bit 1-15 ms after setting it. If LPM is enabled and the core is in the L1 (Sleep) state, when the application sets this bit, the core initiates L1 remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the Sleep state. As specified in the LPM specification, the hardware automatically clears this bit 50 microseconds (TL1DevDrvResume) after being set by the application. The application must not set this bit when GLPMCFG bRemoteWake from the previous LPM transaction is zero."]
pub type RMTWKUPSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTDISCON` reader - Soft Disconnect (SftDiscon) The application uses this bit to signal the controller to do a soft disconnect. As long as this bit is set, the host does not see that the device is connected, and the device does not receive signals on the USB. The core stays in the disconnected state until the application clears this bit. - 1'b0: Normal operation. When this bit is cleared after a soft disconnect, the core drives the phy_opmode_o signal on the UTMI+ to 2'b00, which generates a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration. - 1'b1: The core drives the phy_opmode_o signal on the UTMI+ to 2'b01, which generates a device disconnect event to the USB host. The following is the minimum duration under various conditions for which this bit must be set for the USB host to detect a device disconnect. To accommodate clock jitter, it is recommended that the application adds some extra delay to the specified minimum duration. For high speed, if the device state is, - Suspended, the minimum duration is 1ms + 2.5us - Idle, the minimum duration is 3ms + 2.5us - Not Idle or Suspended (performing transactions), the minimum duration 125 us For full speed/low speed, if the device state is, - Suspended, the minimum duration is 1ms + 2.5us - Idle, the minimum duration is 2.5us - Not Idle or Suspended (performing transactions), the minimum duration 125 us Note: - This bit can be also used for ULPI/FS Serial interfaces. - This bit is not impacted by a soft reset."]
pub type SFTDISCON_R = crate::BitReader;
#[doc = "Field `SFTDISCON` writer - Soft Disconnect (SftDiscon) The application uses this bit to signal the controller to do a soft disconnect. As long as this bit is set, the host does not see that the device is connected, and the device does not receive signals on the USB. The core stays in the disconnected state until the application clears this bit. - 1'b0: Normal operation. When this bit is cleared after a soft disconnect, the core drives the phy_opmode_o signal on the UTMI+ to 2'b00, which generates a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration. - 1'b1: The core drives the phy_opmode_o signal on the UTMI+ to 2'b01, which generates a device disconnect event to the USB host. The following is the minimum duration under various conditions for which this bit must be set for the USB host to detect a device disconnect. To accommodate clock jitter, it is recommended that the application adds some extra delay to the specified minimum duration. For high speed, if the device state is, - Suspended, the minimum duration is 1ms + 2.5us - Idle, the minimum duration is 3ms + 2.5us - Not Idle or Suspended (performing transactions), the minimum duration 125 us For full speed/low speed, if the device state is, - Suspended, the minimum duration is 1ms + 2.5us - Idle, the minimum duration is 2.5us - Not Idle or Suspended (performing transactions), the minimum duration 125 us Note: - This bit can be also used for ULPI/FS Serial interfaces. - This bit is not impacted by a soft reset."]
pub type SFTDISCON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GNPINNAKSTS` reader - Global Non-periodic IN NAK Status (GNPINNakSts) - 1'b0: A handshake is sent out based on the data availability in the transmit FIFO. - 1'b1: A NAK handshake is sent out on all non-periodic IN endpoints, irrespective of the data availability in the transmit FIFO."]
pub type GNPINNAKSTS_R = crate::BitReader;
#[doc = "Field `GOUTNAKSTS` reader - Global OUT NAK Status (GOUTNakSts) - 1'b0: A handshake is sent based on the FIFO Status and the NAK and STALL bit settings. - 1'b1: No data is written to the RxFIFO, irrespective of space availability. Sends a NAK handshake on all packets, except on SETUP transactions. All isochronous OUT packets are dropped."]
pub type GOUTNAKSTS_R = crate::BitReader;
#[doc = "Field `TSTCTL` reader - Test Control (TstCtl) - 3'b000: Test mode disabled - 3'b001: Test_J mode - 3'b010: Test_K mode - 3'b011: Test_SE0_NAK mode - 3'b100: Test_Packet mode - 3'b101: Test_Force_Enable - Others: Reserved"]
pub type TSTCTL_R = crate::FieldReader;
#[doc = "Field `TSTCTL` writer - Test Control (TstCtl) - 3'b000: Test mode disabled - 3'b001: Test_J mode - 3'b010: Test_K mode - 3'b011: Test_SE0_NAK mode - 3'b100: Test_Packet mode - 3'b101: Test_Force_Enable - Others: Reserved"]
pub type TSTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SGNPINNAK` reader - Set Global Non-periodic IN NAK (SGNPInNak) A write to this field sets the Global Non-periodic IN NAK.The application uses this bit to send a NAK handshake on all non-periodic IN endpoints. The core can also Set this bit when a timeout condition is detected on a non-periodic endpoint in shared FIFO operation. The application must Set this bit only after making sure that the Global IN NAK Effective bit in the Core Interrupt Register (GINTSTS.GINNakEff) is cleared"]
pub type SGNPINNAK_R = crate::BitReader;
#[doc = "Field `SGNPINNAK` writer - Set Global Non-periodic IN NAK (SGNPInNak) A write to this field sets the Global Non-periodic IN NAK.The application uses this bit to send a NAK handshake on all non-periodic IN endpoints. The core can also Set this bit when a timeout condition is detected on a non-periodic endpoint in shared FIFO operation. The application must Set this bit only after making sure that the Global IN NAK Effective bit in the Core Interrupt Register (GINTSTS.GINNakEff) is cleared"]
pub type SGNPINNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGNPINNAK` reader - Clear Global Non-periodic IN NAK (CGNPInNak) A write to this field clears the Global Non-periodic IN NAK."]
pub type CGNPINNAK_R = crate::BitReader;
#[doc = "Field `CGNPINNAK` writer - Clear Global Non-periodic IN NAK (CGNPInNak) A write to this field clears the Global Non-periodic IN NAK."]
pub type CGNPINNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGOUTNAK` reader - Set Global OUT NAK (SGOUTNak) A write to this field sets the Global OUT NAK. The application uses this bit to send a NAK handshake on all OUT endpoints. The application must set the this bit only after making sure that the Global OUT NAK Effective bit in the Core Interrupt Register (GINTSTS.GOUTNakEff) is cleared."]
pub type SGOUTNAK_R = crate::BitReader;
#[doc = "Field `SGOUTNAK` writer - Set Global OUT NAK (SGOUTNak) A write to this field sets the Global OUT NAK. The application uses this bit to send a NAK handshake on all OUT endpoints. The application must set the this bit only after making sure that the Global OUT NAK Effective bit in the Core Interrupt Register (GINTSTS.GOUTNakEff) is cleared."]
pub type SGOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGOUTNAK` reader - Clear Global OUT NAK (CGOUTNak) A write to this field clears the Global OUT NAK."]
pub type CGOUTNAK_R = crate::BitReader;
#[doc = "Field `CGOUTNAK` writer - Clear Global OUT NAK (CGOUTNak) A write to this field clears the Global OUT NAK."]
pub type CGOUTNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRONPRGDONE` reader - Power-On Programming Done (PWROnPrgDone) The application uses this bit to indicate that register programming is completed after a wake-up from Power Down mode."]
pub type PWRONPRGDONE_R = crate::BitReader;
#[doc = "Field `PWRONPRGDONE` writer - Power-On Programming Done (PWROnPrgDone) The application uses this bit to indicate that register programming is completed after a wake-up from Power Down mode."]
pub type PWRONPRGDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMC` reader - Global Multi Count (GMC) GMC must be programmed only once after initialization. Applicable only for Scatter/Gather DMA mode. This indicates the number of packets to be serviced for that end point before moving to the next end point. It is only for non-periodic endpoints. - 2'b00: Invalid. - 2'b01: 1 packet. - 2'b10: 2 packets. - 2'b11: 3 packets. The value of this field automatically changes to 2'h1 when DCFG.DescDMA is set to 1. When Scatter/Gather DMA mode is disabled, this field is reserved. and reads 2'b00."]
pub type GMC_R = crate::FieldReader;
#[doc = "Field `GMC` writer - Global Multi Count (GMC) GMC must be programmed only once after initialization. Applicable only for Scatter/Gather DMA mode. This indicates the number of packets to be serviced for that end point before moving to the next end point. It is only for non-periodic endpoints. - 2'b00: Invalid. - 2'b01: 1 packet. - 2'b10: 2 packets. - 2'b11: 3 packets. The value of this field automatically changes to 2'h1 when DCFG.DescDMA is set to 1. When Scatter/Gather DMA mode is disabled, this field is reserved. and reads 2'b00."]
pub type GMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IGNRFRMNUM` reader - Ignore Frame Number Feature for Isochronous Endpoints (IgnrFrmNum) This field is also used to control the Periodic Transfer Interrupt (PTI) feature. Note: Do not program IgnrFrmNum bit to 1'b1 when the core is operating in threshold mode. Slave Mode (GAHBCFG.DMAEn=0): This bit is not valid in Slave mode and must not be programmed to 1. Scatter/Gather DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=1): Note: When Scatter/Gather DMA mode is enabled this feature is not applicable to High Speed, High bandwidth transfers. When this bit is enabled, there must be only one packet per descriptor. - 0: The core transmits the packets only in the frame number in which they are intended to be transmitted. - 1: The core ignores the frame number, sending packets immediately as the packets are ready. In Scatter/Gather DMA mode, if this bit is enabled, the packets are not flushed when a ISOC IN token is received for an elapsed frame. Non-Scatter/Gather DMA Mode, that is, Buffer DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=0): When Scatter/Gather DMA mode is disabled, this field is used by the application to enable Periodic Transfer Interrupt (PTI) Mode. The application can program Periodic Endpoint transfers for multiple (micro)Frames. - 0: Periodic Transfer Interrupt feature is disabled, application needs to program transfers for periodic endpoints every (micro)Frame - 1: Periodic Transfer Interrupt feature is enabled, application can program transfers for multiple (micro)Frames for periodic endpoints. In the PTI mode, the application receives Transfer Complete Interrupt after transfers for multiple (micro)Frames are completed."]
pub type IGNRFRMNUM_R = crate::BitReader;
#[doc = "Field `IGNRFRMNUM` writer - Ignore Frame Number Feature for Isochronous Endpoints (IgnrFrmNum) This field is also used to control the Periodic Transfer Interrupt (PTI) feature. Note: Do not program IgnrFrmNum bit to 1'b1 when the core is operating in threshold mode. Slave Mode (GAHBCFG.DMAEn=0): This bit is not valid in Slave mode and must not be programmed to 1. Scatter/Gather DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=1): Note: When Scatter/Gather DMA mode is enabled this feature is not applicable to High Speed, High bandwidth transfers. When this bit is enabled, there must be only one packet per descriptor. - 0: The core transmits the packets only in the frame number in which they are intended to be transmitted. - 1: The core ignores the frame number, sending packets immediately as the packets are ready. In Scatter/Gather DMA mode, if this bit is enabled, the packets are not flushed when a ISOC IN token is received for an elapsed frame. Non-Scatter/Gather DMA Mode, that is, Buffer DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=0): When Scatter/Gather DMA mode is disabled, this field is used by the application to enable Periodic Transfer Interrupt (PTI) Mode. The application can program Periodic Endpoint transfers for multiple (micro)Frames. - 0: Periodic Transfer Interrupt feature is disabled, application needs to program transfers for periodic endpoints every (micro)Frame - 1: Periodic Transfer Interrupt feature is enabled, application can program transfers for multiple (micro)Frames for periodic endpoints. In the PTI mode, the application receives Transfer Complete Interrupt after transfers for multiple (micro)Frames are completed."]
pub type IGNRFRMNUM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKONBBLE` reader - NAK on Babble Error (NakOnBble) Set NAK automatically on babble (NakOnBble). The core sets NAK automatically for the endpoint on which babble is received."]
pub type NAKONBBLE_R = crate::BitReader;
#[doc = "Field `NAKONBBLE` writer - NAK on Babble Error (NakOnBble) Set NAK automatically on babble (NakOnBble). The core sets NAK automatically for the endpoint on which babble is received."]
pub type NAKONBBLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCONTONBNA` reader - Enable Continue on BNA (EnContOnBNA) This bit enables the core to continue on BNA for Bulk OUT endpoints. With this feature enabled, when a Bulk OUT or INTR OUT endpoint receives a BNA interrupt the core starts processing the descriptor that caused the BNA interrupt after the endpoint re-enables the endpoint. - 1'b0: After receiving BNA interrupt,the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor. - 1'b1: After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt. This bit is valid only when OTG_EN_DESC_DMA == 1'b1. It is a one-time programmable after reset bit like any other DCTL register bits."]
pub type ENCONTONBNA_R = crate::BitReader;
#[doc = "Field `ENCONTONBNA` writer - Enable Continue on BNA (EnContOnBNA) This bit enables the core to continue on BNA for Bulk OUT endpoints. With this feature enabled, when a Bulk OUT or INTR OUT endpoint receives a BNA interrupt the core starts processing the descriptor that caused the BNA interrupt after the endpoint re-enables the endpoint. - 1'b0: After receiving BNA interrupt,the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor. - 1'b1: After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt. This bit is valid only when OTG_EN_DESC_DMA == 1'b1. It is a one-time programmable after reset bit like any other DCTL register bits."]
pub type ENCONTONBNA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERVINT` reader - Service Interval based scheduling for Isochronous IN Endpoints This field is used to enable service interval based scheduling feature for ISOC IN EPs. Note: This bit is applicable only in device mode and when Scatter/Gather DMA mode is used. This feature must not be enabled along with DCTL.IgnrFrmNum. Scatter/Gather DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=1): When this bit is enabled, the frame number field in the ISOC IN descriptor structure is interpreted as the last frame of the service interval. In Scatter/Gather DMA mode, if this bit is enabled, the pending packets are flushed by the controller at the last frame of the service interval."]
pub type SERVINT_R = crate::BitReader;
#[doc = "Field `SERVINT` writer - Service Interval based scheduling for Isochronous IN Endpoints This field is used to enable service interval based scheduling feature for ISOC IN EPs. Note: This bit is applicable only in device mode and when Scatter/Gather DMA mode is used. This feature must not be enabled along with DCTL.IgnrFrmNum. Scatter/Gather DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=1): When this bit is enabled, the frame number field in the ISOC IN descriptor structure is interpreted as the last frame of the service interval. In Scatter/Gather DMA mode, if this bit is enabled, the pending packets are flushed by the controller at the last frame of the service interval."]
pub type SERVINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTMI_TXVLD_CORR_DIS` reader - Disable the correction to OpMode/XcvrSel/TermSel on UTMI Interface. When Soft disconnect (DCTL.SftDiscon=1'b1) is set by the application, the Device controller immediately changes the Opmode to Non-Driving (2'b01), XcvrSel to FS_XCVR (2'b01) and TermSel to HS_TERM (1'b1). If the Device controller is disconnected (DCTL.SftDiscon=1'b1) while transmitting a packet or a chirp, then the Device controller immediately changes the Opmode to Non-Driving (2'b01) while TxValid being High (1'b1). Certain PHYs may not respond to the TxValid by interpreting the change in OpMode to Non-Driving. The behavior of OpMode, XcvrSel and TermSel have been corrected to handle Soft disconnect during transmission. The application can set this register bit to 1'b1 to fall back to the original behavior of OpMode, XcvrSel, TermSel when Soft disconnect is set during transmission. - 1'b0: Opmode, XcvrSel, TermSel are changed by the Device Controller after TxValid goes LOW (1'b0). When this bit is set to 1'b0, the Device controller changes Opmode, XcvrSel and TermSel when TxVlaid is low after assertion of Soft Disconnect. - 1'b1: Opmode, XcvrSel, TermSel are changed by the Device Controller immediately. When this bit is set to 1'b1, the Device controller changes Opmode, XcvrSel and TermSel immediately after assertion of Soft Disconnect. Note: The application must program this register during Device Initialization before USB Reset and this value cannot be changed afterwards."]
pub type UTMI_TXVLD_CORR_DIS_R = crate::BitReader;
#[doc = "Field `UTMI_TXVLD_CORR_DIS` writer - Disable the correction to OpMode/XcvrSel/TermSel on UTMI Interface. When Soft disconnect (DCTL.SftDiscon=1'b1) is set by the application, the Device controller immediately changes the Opmode to Non-Driving (2'b01), XcvrSel to FS_XCVR (2'b01) and TermSel to HS_TERM (1'b1). If the Device controller is disconnected (DCTL.SftDiscon=1'b1) while transmitting a packet or a chirp, then the Device controller immediately changes the Opmode to Non-Driving (2'b01) while TxValid being High (1'b1). Certain PHYs may not respond to the TxValid by interpreting the change in OpMode to Non-Driving. The behavior of OpMode, XcvrSel and TermSel have been corrected to handle Soft disconnect during transmission. The application can set this register bit to 1'b1 to fall back to the original behavior of OpMode, XcvrSel, TermSel when Soft disconnect is set during transmission. - 1'b0: Opmode, XcvrSel, TermSel are changed by the Device Controller after TxValid goes LOW (1'b0). When this bit is set to 1'b0, the Device controller changes Opmode, XcvrSel and TermSel when TxVlaid is low after assertion of Soft Disconnect. - 1'b1: Opmode, XcvrSel, TermSel are changed by the Device Controller immediately. When this bit is set to 1'b1, the Device controller changes Opmode, XcvrSel and TermSel immediately after assertion of Soft Disconnect. Note: The application must program this register during Device Initialization before USB Reset and this value cannot be changed afterwards."]
pub type UTMI_TXVLD_CORR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTMI_TERMSEL_CORR_DIS` reader - Disable the correction of TermSel on UTMI Interface. When the application sets Soft disconnect (DCTL.SftDiscon=1'b1), the behavior of TermSel has been corrected to drive the valid combination of XcvrSel (FS_XCVR) and TermSel (FS_TERM). The application can set this register bit to 1'b1 to fall back to the original behavior of the Device controller driving XcvrSel (FS_XCVR) and TermSel (HS_TERM) when Soft disconnect is set (DCTL.SftDiscon=1'b1). - 1'b0: Valid Combination of XcvrSel and TermSel is driven by the Device Controller. When this bit is set to 1'b0, the Device controller drives TermSel to 1 upon assertion of Soft Disconnect which enables FS Termination. - 1'b1: Invalid Combination of XcvrSel and TermSel is driven by the Device Controller. When this bit is set to 1'b1, the Device controller drives TermSel to 0 upon assertion of Soft Disconnect which enables HS Termination. Note: The application must program this register during Device Initialization before USB Reset and this value cannot be changed afterwards."]
pub type UTMI_TERMSEL_CORR_DIS_R = crate::BitReader;
#[doc = "Field `UTMI_TERMSEL_CORR_DIS` writer - Disable the correction of TermSel on UTMI Interface. When the application sets Soft disconnect (DCTL.SftDiscon=1'b1), the behavior of TermSel has been corrected to drive the valid combination of XcvrSel (FS_XCVR) and TermSel (FS_TERM). The application can set this register bit to 1'b1 to fall back to the original behavior of the Device controller driving XcvrSel (FS_XCVR) and TermSel (HS_TERM) when Soft disconnect is set (DCTL.SftDiscon=1'b1). - 1'b0: Valid Combination of XcvrSel and TermSel is driven by the Device Controller. When this bit is set to 1'b0, the Device controller drives TermSel to 1 upon assertion of Soft Disconnect which enables FS Termination. - 1'b1: Invalid Combination of XcvrSel and TermSel is driven by the Device Controller. When this bit is set to 1'b1, the Device controller drives TermSel to 0 upon assertion of Soft Disconnect which enables HS Termination. Note: The application must program this register during Device Initialization before USB Reset and this value cannot be changed afterwards."]
pub type UTMI_TERMSEL_CORR_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote Wakeup Signaling (RmtWkUpSig) When the application sets this bit, the core initiates remote signaling to wake up the USB host. The application must Set this bit to instruct the core to exit the Suspend state. As specified in the USB 2.0 specification, the application must clear this bit 1-15 ms after setting it. If LPM is enabled and the core is in the L1 (Sleep) state, when the application sets this bit, the core initiates L1 remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the Sleep state. As specified in the LPM specification, the hardware automatically clears this bit 50 microseconds (TL1DevDrvResume) after being set by the application. The application must not set this bit when GLPMCFG bRemoteWake from the previous LPM transaction is zero."]
    #[inline(always)]
    pub fn rmtwkupsig(&self) -> RMTWKUPSIG_R {
        RMTWKUPSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft Disconnect (SftDiscon) The application uses this bit to signal the controller to do a soft disconnect. As long as this bit is set, the host does not see that the device is connected, and the device does not receive signals on the USB. The core stays in the disconnected state until the application clears this bit. - 1'b0: Normal operation. When this bit is cleared after a soft disconnect, the core drives the phy_opmode_o signal on the UTMI+ to 2'b00, which generates a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration. - 1'b1: The core drives the phy_opmode_o signal on the UTMI+ to 2'b01, which generates a device disconnect event to the USB host. The following is the minimum duration under various conditions for which this bit must be set for the USB host to detect a device disconnect. To accommodate clock jitter, it is recommended that the application adds some extra delay to the specified minimum duration. For high speed, if the device state is, - Suspended, the minimum duration is 1ms + 2.5us - Idle, the minimum duration is 3ms + 2.5us - Not Idle or Suspended (performing transactions), the minimum duration 125 us For full speed/low speed, if the device state is, - Suspended, the minimum duration is 1ms + 2.5us - Idle, the minimum duration is 2.5us - Not Idle or Suspended (performing transactions), the minimum duration 125 us Note: - This bit can be also used for ULPI/FS Serial interfaces. - This bit is not impacted by a soft reset."]
    #[inline(always)]
    pub fn sftdiscon(&self) -> SFTDISCON_R {
        SFTDISCON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status (GNPINNakSts) - 1'b0: A handshake is sent out based on the data availability in the transmit FIFO. - 1'b1: A NAK handshake is sent out on all non-periodic IN endpoints, irrespective of the data availability in the transmit FIFO."]
    #[inline(always)]
    pub fn gnpinnaksts(&self) -> GNPINNAKSTS_R {
        GNPINNAKSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK Status (GOUTNakSts) - 1'b0: A handshake is sent based on the FIFO Status and the NAK and STALL bit settings. - 1'b1: No data is written to the RxFIFO, irrespective of space availability. Sends a NAK handshake on all packets, except on SETUP transactions. All isochronous OUT packets are dropped."]
    #[inline(always)]
    pub fn goutnaksts(&self) -> GOUTNAKSTS_R {
        GOUTNAKSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test Control (TstCtl) - 3'b000: Test mode disabled - 3'b001: Test_J mode - 3'b010: Test_K mode - 3'b011: Test_SE0_NAK mode - 3'b100: Test_Packet mode - 3'b101: Test_Force_Enable - Others: Reserved"]
    #[inline(always)]
    pub fn tstctl(&self) -> TSTCTL_R {
        TSTCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK (SGNPInNak) A write to this field sets the Global Non-periodic IN NAK.The application uses this bit to send a NAK handshake on all non-periodic IN endpoints. The core can also Set this bit when a timeout condition is detected on a non-periodic endpoint in shared FIFO operation. The application must Set this bit only after making sure that the Global IN NAK Effective bit in the Core Interrupt Register (GINTSTS.GINNakEff) is cleared"]
    #[inline(always)]
    pub fn sgnpinnak(&self) -> SGNPINNAK_R {
        SGNPINNAK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK (CGNPInNak) A write to this field clears the Global Non-periodic IN NAK."]
    #[inline(always)]
    pub fn cgnpinnak(&self) -> CGNPINNAK_R {
        CGNPINNAK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set Global OUT NAK (SGOUTNak) A write to this field sets the Global OUT NAK. The application uses this bit to send a NAK handshake on all OUT endpoints. The application must set the this bit only after making sure that the Global OUT NAK Effective bit in the Core Interrupt Register (GINTSTS.GOUTNakEff) is cleared."]
    #[inline(always)]
    pub fn sgoutnak(&self) -> SGOUTNAK_R {
        SGOUTNAK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear Global OUT NAK (CGOUTNak) A write to this field clears the Global OUT NAK."]
    #[inline(always)]
    pub fn cgoutnak(&self) -> CGOUTNAK_R {
        CGOUTNAK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power-On Programming Done (PWROnPrgDone) The application uses this bit to indicate that register programming is completed after a wake-up from Power Down mode."]
    #[inline(always)]
    pub fn pwronprgdone(&self) -> PWRONPRGDONE_R {
        PWRONPRGDONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Global Multi Count (GMC) GMC must be programmed only once after initialization. Applicable only for Scatter/Gather DMA mode. This indicates the number of packets to be serviced for that end point before moving to the next end point. It is only for non-periodic endpoints. - 2'b00: Invalid. - 2'b01: 1 packet. - 2'b10: 2 packets. - 2'b11: 3 packets. The value of this field automatically changes to 2'h1 when DCFG.DescDMA is set to 1. When Scatter/Gather DMA mode is disabled, this field is reserved. and reads 2'b00."]
    #[inline(always)]
    pub fn gmc(&self) -> GMC_R {
        GMC_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Ignore Frame Number Feature for Isochronous Endpoints (IgnrFrmNum) This field is also used to control the Periodic Transfer Interrupt (PTI) feature. Note: Do not program IgnrFrmNum bit to 1'b1 when the core is operating in threshold mode. Slave Mode (GAHBCFG.DMAEn=0): This bit is not valid in Slave mode and must not be programmed to 1. Scatter/Gather DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=1): Note: When Scatter/Gather DMA mode is enabled this feature is not applicable to High Speed, High bandwidth transfers. When this bit is enabled, there must be only one packet per descriptor. - 0: The core transmits the packets only in the frame number in which they are intended to be transmitted. - 1: The core ignores the frame number, sending packets immediately as the packets are ready. In Scatter/Gather DMA mode, if this bit is enabled, the packets are not flushed when a ISOC IN token is received for an elapsed frame. Non-Scatter/Gather DMA Mode, that is, Buffer DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=0): When Scatter/Gather DMA mode is disabled, this field is used by the application to enable Periodic Transfer Interrupt (PTI) Mode. The application can program Periodic Endpoint transfers for multiple (micro)Frames. - 0: Periodic Transfer Interrupt feature is disabled, application needs to program transfers for periodic endpoints every (micro)Frame - 1: Periodic Transfer Interrupt feature is enabled, application can program transfers for multiple (micro)Frames for periodic endpoints. In the PTI mode, the application receives Transfer Complete Interrupt after transfers for multiple (micro)Frames are completed."]
    #[inline(always)]
    pub fn ignrfrmnum(&self) -> IGNRFRMNUM_R {
        IGNRFRMNUM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NAK on Babble Error (NakOnBble) Set NAK automatically on babble (NakOnBble). The core sets NAK automatically for the endpoint on which babble is received."]
    #[inline(always)]
    pub fn nakonbble(&self) -> NAKONBBLE_R {
        NAKONBBLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Continue on BNA (EnContOnBNA) This bit enables the core to continue on BNA for Bulk OUT endpoints. With this feature enabled, when a Bulk OUT or INTR OUT endpoint receives a BNA interrupt the core starts processing the descriptor that caused the BNA interrupt after the endpoint re-enables the endpoint. - 1'b0: After receiving BNA interrupt,the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor. - 1'b1: After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt. This bit is valid only when OTG_EN_DESC_DMA == 1'b1. It is a one-time programmable after reset bit like any other DCTL register bits."]
    #[inline(always)]
    pub fn encontonbna(&self) -> ENCONTONBNA_R {
        ENCONTONBNA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Service Interval based scheduling for Isochronous IN Endpoints This field is used to enable service interval based scheduling feature for ISOC IN EPs. Note: This bit is applicable only in device mode and when Scatter/Gather DMA mode is used. This feature must not be enabled along with DCTL.IgnrFrmNum. Scatter/Gather DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=1): When this bit is enabled, the frame number field in the ISOC IN descriptor structure is interpreted as the last frame of the service interval. In Scatter/Gather DMA mode, if this bit is enabled, the pending packets are flushed by the controller at the last frame of the service interval."]
    #[inline(always)]
    pub fn servint(&self) -> SERVINT_R {
        SERVINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 30 - Disable the correction to OpMode/XcvrSel/TermSel on UTMI Interface. When Soft disconnect (DCTL.SftDiscon=1'b1) is set by the application, the Device controller immediately changes the Opmode to Non-Driving (2'b01), XcvrSel to FS_XCVR (2'b01) and TermSel to HS_TERM (1'b1). If the Device controller is disconnected (DCTL.SftDiscon=1'b1) while transmitting a packet or a chirp, then the Device controller immediately changes the Opmode to Non-Driving (2'b01) while TxValid being High (1'b1). Certain PHYs may not respond to the TxValid by interpreting the change in OpMode to Non-Driving. The behavior of OpMode, XcvrSel and TermSel have been corrected to handle Soft disconnect during transmission. The application can set this register bit to 1'b1 to fall back to the original behavior of OpMode, XcvrSel, TermSel when Soft disconnect is set during transmission. - 1'b0: Opmode, XcvrSel, TermSel are changed by the Device Controller after TxValid goes LOW (1'b0). When this bit is set to 1'b0, the Device controller changes Opmode, XcvrSel and TermSel when TxVlaid is low after assertion of Soft Disconnect. - 1'b1: Opmode, XcvrSel, TermSel are changed by the Device Controller immediately. When this bit is set to 1'b1, the Device controller changes Opmode, XcvrSel and TermSel immediately after assertion of Soft Disconnect. Note: The application must program this register during Device Initialization before USB Reset and this value cannot be changed afterwards."]
    #[inline(always)]
    pub fn utmi_txvld_corr_dis(&self) -> UTMI_TXVLD_CORR_DIS_R {
        UTMI_TXVLD_CORR_DIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable the correction of TermSel on UTMI Interface. When the application sets Soft disconnect (DCTL.SftDiscon=1'b1), the behavior of TermSel has been corrected to drive the valid combination of XcvrSel (FS_XCVR) and TermSel (FS_TERM). The application can set this register bit to 1'b1 to fall back to the original behavior of the Device controller driving XcvrSel (FS_XCVR) and TermSel (HS_TERM) when Soft disconnect is set (DCTL.SftDiscon=1'b1). - 1'b0: Valid Combination of XcvrSel and TermSel is driven by the Device Controller. When this bit is set to 1'b0, the Device controller drives TermSel to 1 upon assertion of Soft Disconnect which enables FS Termination. - 1'b1: Invalid Combination of XcvrSel and TermSel is driven by the Device Controller. When this bit is set to 1'b1, the Device controller drives TermSel to 0 upon assertion of Soft Disconnect which enables HS Termination. Note: The application must program this register during Device Initialization before USB Reset and this value cannot be changed afterwards."]
    #[inline(always)]
    pub fn utmi_termsel_corr_dis(&self) -> UTMI_TERMSEL_CORR_DIS_R {
        UTMI_TERMSEL_CORR_DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTL")
            .field("rmtwkupsig", &self.rmtwkupsig())
            .field("sftdiscon", &self.sftdiscon())
            .field("gnpinnaksts", &self.gnpinnaksts())
            .field("goutnaksts", &self.goutnaksts())
            .field("tstctl", &self.tstctl())
            .field("sgnpinnak", &self.sgnpinnak())
            .field("cgnpinnak", &self.cgnpinnak())
            .field("sgoutnak", &self.sgoutnak())
            .field("cgoutnak", &self.cgoutnak())
            .field("pwronprgdone", &self.pwronprgdone())
            .field("gmc", &self.gmc())
            .field("ignrfrmnum", &self.ignrfrmnum())
            .field("nakonbble", &self.nakonbble())
            .field("encontonbna", &self.encontonbna())
            .field("servint", &self.servint())
            .field("utmi_txvld_corr_dis", &self.utmi_txvld_corr_dis())
            .field("utmi_termsel_corr_dis", &self.utmi_termsel_corr_dis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Remote Wakeup Signaling (RmtWkUpSig) When the application sets this bit, the core initiates remote signaling to wake up the USB host. The application must Set this bit to instruct the core to exit the Suspend state. As specified in the USB 2.0 specification, the application must clear this bit 1-15 ms after setting it. If LPM is enabled and the core is in the L1 (Sleep) state, when the application sets this bit, the core initiates L1 remote signaling to wake up the USB host. The application must set this bit to instruct the core to exit the Sleep state. As specified in the LPM specification, the hardware automatically clears this bit 50 microseconds (TL1DevDrvResume) after being set by the application. The application must not set this bit when GLPMCFG bRemoteWake from the previous LPM transaction is zero."]
    #[inline(always)]
    pub fn rmtwkupsig(&mut self) -> RMTWKUPSIG_W<'_, DCTL_SPEC> {
        RMTWKUPSIG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Soft Disconnect (SftDiscon) The application uses this bit to signal the controller to do a soft disconnect. As long as this bit is set, the host does not see that the device is connected, and the device does not receive signals on the USB. The core stays in the disconnected state until the application clears this bit. - 1'b0: Normal operation. When this bit is cleared after a soft disconnect, the core drives the phy_opmode_o signal on the UTMI+ to 2'b00, which generates a device connect event to the USB host. When the device is reconnected, the USB host restarts device enumeration. - 1'b1: The core drives the phy_opmode_o signal on the UTMI+ to 2'b01, which generates a device disconnect event to the USB host. The following is the minimum duration under various conditions for which this bit must be set for the USB host to detect a device disconnect. To accommodate clock jitter, it is recommended that the application adds some extra delay to the specified minimum duration. For high speed, if the device state is, - Suspended, the minimum duration is 1ms + 2.5us - Idle, the minimum duration is 3ms + 2.5us - Not Idle or Suspended (performing transactions), the minimum duration 125 us For full speed/low speed, if the device state is, - Suspended, the minimum duration is 1ms + 2.5us - Idle, the minimum duration is 2.5us - Not Idle or Suspended (performing transactions), the minimum duration 125 us Note: - This bit can be also used for ULPI/FS Serial interfaces. - This bit is not impacted by a soft reset."]
    #[inline(always)]
    pub fn sftdiscon(&mut self) -> SFTDISCON_W<'_, DCTL_SPEC> {
        SFTDISCON_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Test Control (TstCtl) - 3'b000: Test mode disabled - 3'b001: Test_J mode - 3'b010: Test_K mode - 3'b011: Test_SE0_NAK mode - 3'b100: Test_Packet mode - 3'b101: Test_Force_Enable - Others: Reserved"]
    #[inline(always)]
    pub fn tstctl(&mut self) -> TSTCTL_W<'_, DCTL_SPEC> {
        TSTCTL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK (SGNPInNak) A write to this field sets the Global Non-periodic IN NAK.The application uses this bit to send a NAK handshake on all non-periodic IN endpoints. The core can also Set this bit when a timeout condition is detected on a non-periodic endpoint in shared FIFO operation. The application must Set this bit only after making sure that the Global IN NAK Effective bit in the Core Interrupt Register (GINTSTS.GINNakEff) is cleared"]
    #[inline(always)]
    pub fn sgnpinnak(&mut self) -> SGNPINNAK_W<'_, DCTL_SPEC> {
        SGNPINNAK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK (CGNPInNak) A write to this field clears the Global Non-periodic IN NAK."]
    #[inline(always)]
    pub fn cgnpinnak(&mut self) -> CGNPINNAK_W<'_, DCTL_SPEC> {
        CGNPINNAK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set Global OUT NAK (SGOUTNak) A write to this field sets the Global OUT NAK. The application uses this bit to send a NAK handshake on all OUT endpoints. The application must set the this bit only after making sure that the Global OUT NAK Effective bit in the Core Interrupt Register (GINTSTS.GOUTNakEff) is cleared."]
    #[inline(always)]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W<'_, DCTL_SPEC> {
        SGOUTNAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Global OUT NAK (CGOUTNak) A write to this field clears the Global OUT NAK."]
    #[inline(always)]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W<'_, DCTL_SPEC> {
        CGOUTNAK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Power-On Programming Done (PWROnPrgDone) The application uses this bit to indicate that register programming is completed after a wake-up from Power Down mode."]
    #[inline(always)]
    pub fn pwronprgdone(&mut self) -> PWRONPRGDONE_W<'_, DCTL_SPEC> {
        PWRONPRGDONE_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - Global Multi Count (GMC) GMC must be programmed only once after initialization. Applicable only for Scatter/Gather DMA mode. This indicates the number of packets to be serviced for that end point before moving to the next end point. It is only for non-periodic endpoints. - 2'b00: Invalid. - 2'b01: 1 packet. - 2'b10: 2 packets. - 2'b11: 3 packets. The value of this field automatically changes to 2'h1 when DCFG.DescDMA is set to 1. When Scatter/Gather DMA mode is disabled, this field is reserved. and reads 2'b00."]
    #[inline(always)]
    pub fn gmc(&mut self) -> GMC_W<'_, DCTL_SPEC> {
        GMC_W::new(self, 13)
    }
    #[doc = "Bit 15 - Ignore Frame Number Feature for Isochronous Endpoints (IgnrFrmNum) This field is also used to control the Periodic Transfer Interrupt (PTI) feature. Note: Do not program IgnrFrmNum bit to 1'b1 when the core is operating in threshold mode. Slave Mode (GAHBCFG.DMAEn=0): This bit is not valid in Slave mode and must not be programmed to 1. Scatter/Gather DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=1): Note: When Scatter/Gather DMA mode is enabled this feature is not applicable to High Speed, High bandwidth transfers. When this bit is enabled, there must be only one packet per descriptor. - 0: The core transmits the packets only in the frame number in which they are intended to be transmitted. - 1: The core ignores the frame number, sending packets immediately as the packets are ready. In Scatter/Gather DMA mode, if this bit is enabled, the packets are not flushed when a ISOC IN token is received for an elapsed frame. Non-Scatter/Gather DMA Mode, that is, Buffer DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=0): When Scatter/Gather DMA mode is disabled, this field is used by the application to enable Periodic Transfer Interrupt (PTI) Mode. The application can program Periodic Endpoint transfers for multiple (micro)Frames. - 0: Periodic Transfer Interrupt feature is disabled, application needs to program transfers for periodic endpoints every (micro)Frame - 1: Periodic Transfer Interrupt feature is enabled, application can program transfers for multiple (micro)Frames for periodic endpoints. In the PTI mode, the application receives Transfer Complete Interrupt after transfers for multiple (micro)Frames are completed."]
    #[inline(always)]
    pub fn ignrfrmnum(&mut self) -> IGNRFRMNUM_W<'_, DCTL_SPEC> {
        IGNRFRMNUM_W::new(self, 15)
    }
    #[doc = "Bit 16 - NAK on Babble Error (NakOnBble) Set NAK automatically on babble (NakOnBble). The core sets NAK automatically for the endpoint on which babble is received."]
    #[inline(always)]
    pub fn nakonbble(&mut self) -> NAKONBBLE_W<'_, DCTL_SPEC> {
        NAKONBBLE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Continue on BNA (EnContOnBNA) This bit enables the core to continue on BNA for Bulk OUT endpoints. With this feature enabled, when a Bulk OUT or INTR OUT endpoint receives a BNA interrupt the core starts processing the descriptor that caused the BNA interrupt after the endpoint re-enables the endpoint. - 1'b0: After receiving BNA interrupt,the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the DOEPDMA descriptor. - 1'b1: After receiving BNA interrupt, the core disables the endpoint. When the endpoint is re-enabled by the application, the core starts processing from the descriptor that received the BNA interrupt. This bit is valid only when OTG_EN_DESC_DMA == 1'b1. It is a one-time programmable after reset bit like any other DCTL register bits."]
    #[inline(always)]
    pub fn encontonbna(&mut self) -> ENCONTONBNA_W<'_, DCTL_SPEC> {
        ENCONTONBNA_W::new(self, 17)
    }
    #[doc = "Bit 19 - Service Interval based scheduling for Isochronous IN Endpoints This field is used to enable service interval based scheduling feature for ISOC IN EPs. Note: This bit is applicable only in device mode and when Scatter/Gather DMA mode is used. This feature must not be enabled along with DCTL.IgnrFrmNum. Scatter/Gather DMA Mode (GAHBCFG.DMAEn=1,DCFG.DescDMA=1): When this bit is enabled, the frame number field in the ISOC IN descriptor structure is interpreted as the last frame of the service interval. In Scatter/Gather DMA mode, if this bit is enabled, the pending packets are flushed by the controller at the last frame of the service interval."]
    #[inline(always)]
    pub fn servint(&mut self) -> SERVINT_W<'_, DCTL_SPEC> {
        SERVINT_W::new(self, 19)
    }
    #[doc = "Bit 30 - Disable the correction to OpMode/XcvrSel/TermSel on UTMI Interface. When Soft disconnect (DCTL.SftDiscon=1'b1) is set by the application, the Device controller immediately changes the Opmode to Non-Driving (2'b01), XcvrSel to FS_XCVR (2'b01) and TermSel to HS_TERM (1'b1). If the Device controller is disconnected (DCTL.SftDiscon=1'b1) while transmitting a packet or a chirp, then the Device controller immediately changes the Opmode to Non-Driving (2'b01) while TxValid being High (1'b1). Certain PHYs may not respond to the TxValid by interpreting the change in OpMode to Non-Driving. The behavior of OpMode, XcvrSel and TermSel have been corrected to handle Soft disconnect during transmission. The application can set this register bit to 1'b1 to fall back to the original behavior of OpMode, XcvrSel, TermSel when Soft disconnect is set during transmission. - 1'b0: Opmode, XcvrSel, TermSel are changed by the Device Controller after TxValid goes LOW (1'b0). When this bit is set to 1'b0, the Device controller changes Opmode, XcvrSel and TermSel when TxVlaid is low after assertion of Soft Disconnect. - 1'b1: Opmode, XcvrSel, TermSel are changed by the Device Controller immediately. When this bit is set to 1'b1, the Device controller changes Opmode, XcvrSel and TermSel immediately after assertion of Soft Disconnect. Note: The application must program this register during Device Initialization before USB Reset and this value cannot be changed afterwards."]
    #[inline(always)]
    pub fn utmi_txvld_corr_dis(&mut self) -> UTMI_TXVLD_CORR_DIS_W<'_, DCTL_SPEC> {
        UTMI_TXVLD_CORR_DIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Disable the correction of TermSel on UTMI Interface. When the application sets Soft disconnect (DCTL.SftDiscon=1'b1), the behavior of TermSel has been corrected to drive the valid combination of XcvrSel (FS_XCVR) and TermSel (FS_TERM). The application can set this register bit to 1'b1 to fall back to the original behavior of the Device controller driving XcvrSel (FS_XCVR) and TermSel (HS_TERM) when Soft disconnect is set (DCTL.SftDiscon=1'b1). - 1'b0: Valid Combination of XcvrSel and TermSel is driven by the Device Controller. When this bit is set to 1'b0, the Device controller drives TermSel to 1 upon assertion of Soft Disconnect which enables FS Termination. - 1'b1: Invalid Combination of XcvrSel and TermSel is driven by the Device Controller. When this bit is set to 1'b1, the Device controller drives TermSel to 0 upon assertion of Soft Disconnect which enables HS Termination. Note: The application must program this register during Device Initialization before USB Reset and this value cannot be changed afterwards."]
    #[inline(always)]
    pub fn utmi_termsel_corr_dis(&mut self) -> UTMI_TERMSEL_CORR_DIS_W<'_, DCTL_SPEC> {
        UTMI_TERMSEL_CORR_DIS_W::new(self, 31)
    }
}
#[doc = "This register is used to control the characteristics of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCTL to value 0x02"]
impl crate::Resettable for DCTL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
