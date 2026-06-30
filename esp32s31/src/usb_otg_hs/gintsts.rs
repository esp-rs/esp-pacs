#[doc = "Register `GINTSTS` reader"]
pub type R = crate::R<GINTSTS_SPEC>;
#[doc = "Register `GINTSTS` writer"]
pub type W = crate::W<GINTSTS_SPEC>;
#[doc = "Field `CURMOD` reader - Mode: Host and Device Current Mode of Operation (CurMod) Indicates the current mode. - 1'b0: Device mode - 1'b1: Host mode Note: The reset value of this register field can be read only after the PHY clock is stable, or if IDDIG_FILTER is enabled, wait for the filter timer to expire to read the correct reset value which ever event is later."]
pub type CURMOD_R = crate::BitReader;
#[doc = "Field `MODEMIS` reader - Mode: Host and Device Mode Mismatch Interrupt (ModeMis) The core sets this bit when the application is trying to access: - A Host mode register, when the controller is operating in Device mode - A Device mode register, when the controller is operating in Host mode The register access is completed on the AHB with an OKAY response, but is ignored by the controller internally and does not affect the operation of the controller. This bit can be set only by the core and the application must write 1 to clear it."]
pub type MODEMIS_R = crate::BitReader;
#[doc = "Field `MODEMIS` writer - Mode: Host and Device Mode Mismatch Interrupt (ModeMis) The core sets this bit when the application is trying to access: - A Host mode register, when the controller is operating in Device mode - A Device mode register, when the controller is operating in Host mode The register access is completed on the AHB with an OKAY response, but is ignored by the controller internally and does not affect the operation of the controller. This bit can be set only by the core and the application must write 1 to clear it."]
pub type MODEMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINT` reader - Mode: Host and Device OTG Interrupt (OTGInt) The controller sets this bit to indicate an OTG protocol event. The application must read the OTG Interrupt Status (GOTGINT) register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the GOTGINT register to clear this bit."]
pub type OTGINT_R = crate::BitReader;
#[doc = "Field `SOF` reader - Mode: Host and Device Start of (micro)Frame (Sof) In Host mode, the core sets this bit to indicate that an SOF (FS), micro-SOF (HS), or Keep-Alive (LS) is transmitted on the USB. The application must write a 1 to this bit to clear the interrupt. In Device mode, the controller sets this bit to indicate that an SOF token has been received on the USB. The application can read the Device Status register to get the current (micro)Frame number. This interrupt is seen only when the core is operating at either HS or FS. This bit can be set only by the core and the application must write 1 to clear it. Note: This register may return 1'b1 if read immediately after power-on reset. If the register bit reads 1'b1 immediately after power-on reset, it does not indicate that an SOF has been sent (in case of host mode) or SOF has been received (in case of device mode). The read value of this interrupt is valid only after a valid connection between host and device is established. If the bit is set after power on reset the application can clear the bit."]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - Mode: Host and Device Start of (micro)Frame (Sof) In Host mode, the core sets this bit to indicate that an SOF (FS), micro-SOF (HS), or Keep-Alive (LS) is transmitted on the USB. The application must write a 1 to this bit to clear the interrupt. In Device mode, the controller sets this bit to indicate that an SOF token has been received on the USB. The application can read the Device Status register to get the current (micro)Frame number. This interrupt is seen only when the core is operating at either HS or FS. This bit can be set only by the core and the application must write 1 to clear it. Note: This register may return 1'b1 if read immediately after power-on reset. If the register bit reads 1'b1 immediately after power-on reset, it does not indicate that an SOF has been sent (in case of host mode) or SOF has been received (in case of device mode). The read value of this interrupt is valid only after a valid connection between host and device is established. If the bit is set after power on reset the application can clear the bit."]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVL` reader - Mode: Host and Device RxFIFO Non-Empty (RxFLvl) Indicates that there is at least one packet pending to be read from the RxFIFO."]
pub type RXFLVL_R = crate::BitReader;
#[doc = "Field `NPTXFEMP` reader - Mode: Host and Device Non-periodic TxFIFO Empty (NPTxFEmp) This interrupt is asserted when the Non-periodic TxFIFO is either half or completely empty, and there is space for at least one Entry to be written to the Non-periodic Transmit Request Queue. The half or completely empty status is determined by the Non-periodic TxFIFO Empty Level bit in the Core AHB Configuration register (GAHBCFG.NPTxFEmpLvl). In host mode, the application can use GINTSTS.NPTxFEmp with the OTG_EN_DED_TX_FIFO parameter set to either 1 or 0. In device mode, the application uses GINTSTS.NPTxFEmp when OTG_EN_DED_TX_FIFO=0. When OTG_EN_DED_TX_FIFO=1, the application uses DIEPINTn.TxFEmp."]
pub type NPTXFEMP_R = crate::BitReader;
#[doc = "Field `GINNAKEFF` reader - Mode: Device only Global IN Non-periodic NAK Effective (GINNakEff) Indicates that the Set Global Non-periodic IN NAK bit in the Device Control register (DCTL.SGNPInNak) set by the application, has taken effect in the core. That is, the core has sampled the Global IN NAK bit Set by the application. This bit can be cleared by clearing the Clear Global Non-periodic IN NAK bit in the Device Control register (DCTL.CGNPInNak). This interrupt does not necessarily mean that a NAK handshake is sent out on the USB. The STALL bit takes precedence over the NAK bit."]
pub type GINNAKEFF_R = crate::BitReader;
#[doc = "Field `GOUTNAKEFF` reader - Mode: Device only Global OUT NAK Effective (GOUTNakEff) Indicates that the Set Global OUT NAK bit in the Device Control register (DCTL.SGOUTNak), Set by the application, has taken effect in the core. This bit can be cleared by writing the Clear Global OUT NAK bit in the Device Control register (DCTL.CGOUTNak)."]
pub type GOUTNAKEFF_R = crate::BitReader;
#[doc = "Field `ERLYSUSP` reader - Mode: Device only Early Suspend (ErlySusp) The controller sets this bit to indicate that an Idle state has been detected on the USB for 3 ms."]
pub type ERLYSUSP_R = crate::BitReader;
#[doc = "Field `ERLYSUSP` writer - Mode: Device only Early Suspend (ErlySusp) The controller sets this bit to indicate that an Idle state has been detected on the USB for 3 ms."]
pub type ERLYSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSP` reader - Mode: Device only USB Suspend (USBSusp) The controller sets this bit to indicate that a suspend was detected on the USB. The controller enters the Suspended state when there is no activity on the linestate signal for an extended period of time."]
pub type USBSUSP_R = crate::BitReader;
#[doc = "Field `USBSUSP` writer - Mode: Device only USB Suspend (USBSusp) The controller sets this bit to indicate that a suspend was detected on the USB. The controller enters the Suspended state when there is no activity on the linestate signal for an extended period of time."]
pub type USBSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - Mode: Device only USB Reset (USBRst) The controller sets this bit to indicate that a reset is detected on the USB."]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - Mode: Device only USB Reset (USBRst) The controller sets this bit to indicate that a reset is detected on the USB."]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONE` reader - Mode: Device only Enumeration Done (EnumDone) The core sets this bit to indicate that speed enumeration is complete. The application must read the Device Status (DSTS) register to obtain the enumerated speed."]
pub type ENUMDONE_R = crate::BitReader;
#[doc = "Field `ENUMDONE` writer - Mode: Device only Enumeration Done (EnumDone) The core sets this bit to indicate that speed enumeration is complete. The application must read the Device Status (DSTS) register to obtain the enumerated speed."]
pub type ENUMDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROP` reader - Mode: Device only Isochronous OUT Packet Dropped Interrupt (ISOOutDrop) The controller sets this bit when it fails to write an isochronous OUT packet into the RxFIFO because the RxFIFO does not have enough space to accommodate a maximum packet size packet for the isochronous OUT endpoint."]
pub type ISOOUTDROP_R = crate::BitReader;
#[doc = "Field `ISOOUTDROP` writer - Mode: Device only Isochronous OUT Packet Dropped Interrupt (ISOOutDrop) The controller sets this bit when it fails to write an isochronous OUT packet into the RxFIFO because the RxFIFO does not have enough space to accommodate a maximum packet size packet for the isochronous OUT endpoint."]
pub type ISOOUTDROP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - Mode: Device only End of Periodic Frame Interrupt (EOPF) Indicates that the period specified in the Periodic Frame Interval field of the Device Configuration register (DCFG.PerFrInt) has been reached in the current microframe. In case of Non-Ignore Frame Number Scatter/Gather (Descriptor DMA) mode, the controller internally handles the following scenarios based on EOPF: Read Flush: At the EOPF, the controller checks if there are any pending packets in the FIFO corresponding to the current (micro)Frame. - If there are any pending packets, then the controller initiates read flush, due to which the read pointer is updated to the starting location of the next micro-frame packet. - If there are no pending packets corresponding to the current (micro)Frame, the controller does not take any action. Write Flush: At the EOPF, if the controller is still fetching the current micro-frame data, then the controller stops pushing data into the TXFIFO but keeps fetching the complete packet from the System Memory. After completing the scheduled packet size fetch, the controller updates the Status Quadlet Fields (Transmit Status to BUFFLUSH) and closes the Descriptor. During the descriptor close, the controller initiates write flush, due to which the write pointer is updated to the starting location of the next micro-frame packet. Because the controller stops pushing the packet to the TxFIFO after EOPF, to bring the write pointer to the starting location of the next micro-frame, write flush is done."]
pub type EOPF_R = crate::BitReader;
#[doc = "Field `EOPF` writer - Mode: Device only End of Periodic Frame Interrupt (EOPF) Indicates that the period specified in the Periodic Frame Interval field of the Device Configuration register (DCFG.PerFrInt) has been reached in the current microframe. In case of Non-Ignore Frame Number Scatter/Gather (Descriptor DMA) mode, the controller internally handles the following scenarios based on EOPF: Read Flush: At the EOPF, the controller checks if there are any pending packets in the FIFO corresponding to the current (micro)Frame. - If there are any pending packets, then the controller initiates read flush, due to which the read pointer is updated to the starting location of the next micro-frame packet. - If there are no pending packets corresponding to the current (micro)Frame, the controller does not take any action. Write Flush: At the EOPF, if the controller is still fetching the current micro-frame data, then the controller stops pushing data into the TXFIFO but keeps fetching the complete packet from the System Memory. After completing the scheduled packet size fetch, the controller updates the Status Quadlet Fields (Transmit Status to BUFFLUSH) and closes the Descriptor. During the descriptor close, the controller initiates write flush, due to which the write pointer is updated to the starting location of the next micro-frame packet. Because the controller stops pushing the packet to the TxFIFO after EOPF, to bring the write pointer to the starting location of the next micro-frame, write flush is done."]
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPMIS` reader - Mode: Device only Endpoint Mismatch Interrupt (EPMis) Note: This interrupt is valid only in shared FIFO operation. Indicates that an IN token has been received for a non-periodic endpoint, but the data for another endpoint is present in the top of the Non-periodic Transmit FIFO and the IN endpoint mismatch count programmed by the application has expired."]
pub type EPMIS_R = crate::BitReader;
#[doc = "Field `EPMIS` writer - Mode: Device only Endpoint Mismatch Interrupt (EPMis) Note: This interrupt is valid only in shared FIFO operation. Indicates that an IN token has been received for a non-periodic endpoint, but the data for another endpoint is present in the top of the Non-periodic Transmit FIFO and the IN endpoint mismatch count programmed by the application has expired."]
pub type EPMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - Mode: Device only IN Endpoints Interrupt (IEPInt) The core sets this bit to indicate that an interrupt is pending on one of the IN endpoints of the core (in Device mode). The application must read the Device All Endpoints Interrupt (DAINT) register to determine the exact number of the IN endpoint on Device IN Endpoint-n Interrupt (DIEPINTn) register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding DIEPINTn register to clear this bit."]
pub type IEPINT_R = crate::BitReader;
#[doc = "Field `OEPINT` reader - Mode: Device only OUT Endpoints Interrupt (OEPInt) The controller sets this bit to indicate that an interrupt is pending on one of the OUT endpoints of the core (in Device mode). The application must read the Device All Endpoints Interrupt (DAINT) register to determine the exact number of the OUT endpoint on which the interrupt occurred, and then read the corresponding Device OUT Endpoint-n Interrupt (DOEPINTn) register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding DOEPINTn register to clear this bit."]
pub type OEPINT_R = crate::BitReader;
#[doc = "Field `INCOMPISOIN` reader - Mode: Device only Incomplete Isochronous IN Transfer (incompISOIN) The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on which the transfer is not completed in the current microframe. This interrupt is asserted along with the End of Periodic Frame Interrupt (EOPF) bit in this register. Note: This interrupt is not asserted in Scatter/Gather DMA mode."]
pub type INCOMPISOIN_R = crate::BitReader;
#[doc = "Field `INCOMPISOIN` writer - Mode: Device only Incomplete Isochronous IN Transfer (incompISOIN) The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on which the transfer is not completed in the current microframe. This interrupt is asserted along with the End of Periodic Frame Interrupt (EOPF) bit in this register. Note: This interrupt is not asserted in Scatter/Gather DMA mode."]
pub type INCOMPISOIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPLP` reader - Incomplete Periodic Transfer (incomplP) Mode: Host only In Host mode, the core sets this interrupt bit when there are incomplete periodic transactions still pending which are scheduled for the current microframe. Incomplete Isochronous OUT Transfer (incompISOOUT) Mode: Device only The Device mode, the core sets this interrupt to indicate that there is at least one isochronous OUT endpoint on which the transfer is not completed in the current microframe. This interrupt is asserted along with the End of Periodic Frame Interrupt (EOPF) bit in this register."]
pub type INCOMPLP_R = crate::BitReader;
#[doc = "Field `INCOMPLP` writer - Incomplete Periodic Transfer (incomplP) Mode: Host only In Host mode, the core sets this interrupt bit when there are incomplete periodic transactions still pending which are scheduled for the current microframe. Incomplete Isochronous OUT Transfer (incompISOOUT) Mode: Device only The Device mode, the core sets this interrupt to indicate that there is at least one isochronous OUT endpoint on which the transfer is not completed in the current microframe. This interrupt is asserted along with the End of Periodic Frame Interrupt (EOPF) bit in this register."]
pub type INCOMPLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETSUSP` reader - Mode: Device only Data Fetch Suspended (FetSusp) This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped fetching data. For IN endpoints due to the unavailability of TxFIFO space or Request Queue space. This interrupt is used by the application for an endpoint mismatch algorithm. For example, after detecting an endpoint mismatch, the application: - Sets a Global non-periodic IN NAK handshake - Disables IN endpoints - Flushes the FIFO - Determines the token sequence from the IN Token Sequence Learning Queue - Re-enables the endpoints - Clears the Global non-periodic IN NAK handshake If the Global non-periodic IN NAK is cleared, the core has not yet fetched data for the IN endpoint, and the IN token is received. The core generates an 'IN token received when FIFO empty' interrupt. It then sends the host a NAK response. To avoid this scenario, the application can check the GINTSTS.FetSusp interrupt, which ensures that the FIFO is full before clearing a Global NAK handshake. Alternatively, the application can mask the IN token received when FIFO empty interrupt when clearing a Global IN NAK handshake."]
pub type FETSUSP_R = crate::BitReader;
#[doc = "Field `FETSUSP` writer - Mode: Device only Data Fetch Suspended (FetSusp) This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped fetching data. For IN endpoints due to the unavailability of TxFIFO space or Request Queue space. This interrupt is used by the application for an endpoint mismatch algorithm. For example, after detecting an endpoint mismatch, the application: - Sets a Global non-periodic IN NAK handshake - Disables IN endpoints - Flushes the FIFO - Determines the token sequence from the IN Token Sequence Learning Queue - Re-enables the endpoints - Clears the Global non-periodic IN NAK handshake If the Global non-periodic IN NAK is cleared, the core has not yet fetched data for the IN endpoint, and the IN token is received. The core generates an 'IN token received when FIFO empty' interrupt. It then sends the host a NAK response. To avoid this scenario, the application can check the GINTSTS.FetSusp interrupt, which ensures that the FIFO is full before clearing a Global NAK handshake. Alternatively, the application can mask the IN token received when FIFO empty interrupt when clearing a Global IN NAK handshake."]
pub type FETSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDET` reader - Mode: Device only Reset detected Interrupt (ResetDet) In Device mode, this interrupt is asserted when a reset is detected on the USB in partial power-down mode when the device is in Suspend. In Host mode, this interrupt is not asserted."]
pub type RESETDET_R = crate::BitReader;
#[doc = "Field `RESETDET` writer - Mode: Device only Reset detected Interrupt (ResetDet) In Device mode, this interrupt is asserted when a reset is detected on the USB in partial power-down mode when the device is in Suspend. In Host mode, this interrupt is not asserted."]
pub type RESETDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTINT` reader - Mode: Host only Host Port Interrupt (PrtInt) The core sets this bit to indicate a change in port status of one of the controller ports in Host mode. The application must read the Host Port Control and Status (HPRT) register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the Host Port Control and Status register to clear this bit."]
pub type PRTINT_R = crate::BitReader;
#[doc = "Field `HCHINT` reader - Mode: Host only Host Channels Interrupt (HChInt) The core sets this bit to indicate that an interrupt is pending on one of the channels of the core (in Host mode). The application must read the Host All Channels Interrupt (HAINT) register to determine the exact number of the channel on which the interrupt occurred, and Then read the corresponding Host Channel-n Interrupt (HCINTn) register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the HCINTn register to clear this bit."]
pub type HCHINT_R = crate::BitReader;
#[doc = "Field `PTXFEMP` reader - Mode: Host only Periodic TxFIFO Empty (PTxFEmp) This interrupt is asserted when the Periodic Transmit FIFO is either half or completely empty and there is space for at least one entry to be written in the Periodic Request Queue. The half or completely empty status is determined by the Periodic TxFIFO Empty Level bit in the Core AHB Configuration register (GAHBCFG.PTxFEmpLvl)."]
pub type PTXFEMP_R = crate::BitReader;
#[doc = "Field `CONIDSTSCHNG` reader - Mode: Host and Device Connector ID Status Change (ConIDStsChng) The core sets this bit when there is a change in connector ID status."]
pub type CONIDSTSCHNG_R = crate::BitReader;
#[doc = "Field `CONIDSTSCHNG` writer - Mode: Host and Device Connector ID Status Change (ConIDStsChng) The core sets this bit when there is a change in connector ID status."]
pub type CONIDSTSCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCONNINT` reader - Mode: Host only Disconnect Detected Interrupt (DisconnInt) Asserted when a device disconnect is detected."]
pub type DISCONNINT_R = crate::BitReader;
#[doc = "Field `DISCONNINT` writer - Mode: Host only Disconnect Detected Interrupt (DisconnInt) Asserted when a device disconnect is detected."]
pub type DISCONNINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESSREQINT` reader - Mode: Host and Device Session Request/New Session Detected Interrupt (SessReqInt) In Host mode, this interrupt is asserted when a session request is detected from the device. In Host mode, this interrupt is asserted when a session request is detected from the device. In Device mode, this interrupt is asserted when the utmisrp_bvalid signal goes high. For more information on how to use this interrupt, see 'Partial Power-Down and Clock Gating Programming Model' in the Programming Guide."]
pub type SESSREQINT_R = crate::BitReader;
#[doc = "Field `SESSREQINT` writer - Mode: Host and Device Session Request/New Session Detected Interrupt (SessReqInt) In Host mode, this interrupt is asserted when a session request is detected from the device. In Host mode, this interrupt is asserted when a session request is detected from the device. In Device mode, this interrupt is asserted when the utmisrp_bvalid signal goes high. For more information on how to use this interrupt, see 'Partial Power-Down and Clock Gating Programming Model' in the Programming Guide."]
pub type SESSREQINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINT` reader - Mode: Host and Device Resume/Remote Wakeup Detected Interrupt (WkUpInt) Wakeup Interrupt during Suspend(L2) or LPM(L1) state. - During Suspend(L2): -- Device Mode: This interrupt is asserted only when Host Initiated Resume is detected on USB. -- Host Mode: This interrupt is asserted only when Device Initiated Remote Wakeup is detected on USB. For more information, see 'Partial Power-Down and Clock Gating Programming Model' in the Programming Guide. - During LPM(L1): -- Device Mode: This interrupt is asserted for either Host Initiated Resume or Device Initiated Remote Wakeup on USB. -- Host Mode: This interrupt is asserted for either Host Initiated Resume or Device Initiated Remote Wakeup on USB. For more information, see 'LPM Entry and Exit Programming Model' in the Programming Guide."]
pub type WKUPINT_R = crate::BitReader;
#[doc = "Field `WKUPINT` writer - Mode: Host and Device Resume/Remote Wakeup Detected Interrupt (WkUpInt) Wakeup Interrupt during Suspend(L2) or LPM(L1) state. - During Suspend(L2): -- Device Mode: This interrupt is asserted only when Host Initiated Resume is detected on USB. -- Host Mode: This interrupt is asserted only when Device Initiated Remote Wakeup is detected on USB. For more information, see 'Partial Power-Down and Clock Gating Programming Model' in the Programming Guide. - During LPM(L1): -- Device Mode: This interrupt is asserted for either Host Initiated Resume or Device Initiated Remote Wakeup on USB. -- Host Mode: This interrupt is asserted for either Host Initiated Resume or Device Initiated Remote Wakeup on USB. For more information, see 'LPM Entry and Exit Programming Model' in the Programming Guide."]
pub type WKUPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mode: Host and Device Current Mode of Operation (CurMod) Indicates the current mode. - 1'b0: Device mode - 1'b1: Host mode Note: The reset value of this register field can be read only after the PHY clock is stable, or if IDDIG_FILTER is enabled, wait for the filter timer to expire to read the correct reset value which ever event is later."]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode: Host and Device Mode Mismatch Interrupt (ModeMis) The core sets this bit when the application is trying to access: - A Host mode register, when the controller is operating in Device mode - A Device mode register, when the controller is operating in Host mode The register access is completed on the AHB with an OKAY response, but is ignored by the controller internally and does not affect the operation of the controller. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn modemis(&self) -> MODEMIS_R {
        MODEMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode: Host and Device OTG Interrupt (OTGInt) The controller sets this bit to indicate an OTG protocol event. The application must read the OTG Interrupt Status (GOTGINT) register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the GOTGINT register to clear this bit."]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mode: Host and Device Start of (micro)Frame (Sof) In Host mode, the core sets this bit to indicate that an SOF (FS), micro-SOF (HS), or Keep-Alive (LS) is transmitted on the USB. The application must write a 1 to this bit to clear the interrupt. In Device mode, the controller sets this bit to indicate that an SOF token has been received on the USB. The application can read the Device Status register to get the current (micro)Frame number. This interrupt is seen only when the core is operating at either HS or FS. This bit can be set only by the core and the application must write 1 to clear it. Note: This register may return 1'b1 if read immediately after power-on reset. If the register bit reads 1'b1 immediately after power-on reset, it does not indicate that an SOF has been sent (in case of host mode) or SOF has been received (in case of device mode). The read value of this interrupt is valid only after a valid connection between host and device is established. If the bit is set after power on reset the application can clear the bit."]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode: Host and Device RxFIFO Non-Empty (RxFLvl) Indicates that there is at least one packet pending to be read from the RxFIFO."]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode: Host and Device Non-periodic TxFIFO Empty (NPTxFEmp) This interrupt is asserted when the Non-periodic TxFIFO is either half or completely empty, and there is space for at least one Entry to be written to the Non-periodic Transmit Request Queue. The half or completely empty status is determined by the Non-periodic TxFIFO Empty Level bit in the Core AHB Configuration register (GAHBCFG.NPTxFEmpLvl). In host mode, the application can use GINTSTS.NPTxFEmp with the OTG_EN_DED_TX_FIFO parameter set to either 1 or 0. In device mode, the application uses GINTSTS.NPTxFEmp when OTG_EN_DED_TX_FIFO=0. When OTG_EN_DED_TX_FIFO=1, the application uses DIEPINTn.TxFEmp."]
    #[inline(always)]
    pub fn nptxfemp(&self) -> NPTXFEMP_R {
        NPTXFEMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mode: Device only Global IN Non-periodic NAK Effective (GINNakEff) Indicates that the Set Global Non-periodic IN NAK bit in the Device Control register (DCTL.SGNPInNak) set by the application, has taken effect in the core. That is, the core has sampled the Global IN NAK bit Set by the application. This bit can be cleared by clearing the Clear Global Non-periodic IN NAK bit in the Device Control register (DCTL.CGNPInNak). This interrupt does not necessarily mean that a NAK handshake is sent out on the USB. The STALL bit takes precedence over the NAK bit."]
    #[inline(always)]
    pub fn ginnakeff(&self) -> GINNAKEFF_R {
        GINNAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mode: Device only Global OUT NAK Effective (GOUTNakEff) Indicates that the Set Global OUT NAK bit in the Device Control register (DCTL.SGOUTNak), Set by the application, has taken effect in the core. This bit can be cleared by writing the Clear Global OUT NAK bit in the Device Control register (DCTL.CGOUTNak)."]
    #[inline(always)]
    pub fn goutnakeff(&self) -> GOUTNAKEFF_R {
        GOUTNAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Mode: Device only Early Suspend (ErlySusp) The controller sets this bit to indicate that an Idle state has been detected on the USB for 3 ms."]
    #[inline(always)]
    pub fn erlysusp(&self) -> ERLYSUSP_R {
        ERLYSUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mode: Device only USB Suspend (USBSusp) The controller sets this bit to indicate that a suspend was detected on the USB. The controller enters the Suspended state when there is no activity on the linestate signal for an extended period of time."]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mode: Device only USB Reset (USBRst) The controller sets this bit to indicate that a reset is detected on the USB."]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mode: Device only Enumeration Done (EnumDone) The core sets this bit to indicate that speed enumeration is complete. The application must read the Device Status (DSTS) register to obtain the enumerated speed."]
    #[inline(always)]
    pub fn enumdone(&self) -> ENUMDONE_R {
        ENUMDONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mode: Device only Isochronous OUT Packet Dropped Interrupt (ISOOutDrop) The controller sets this bit when it fails to write an isochronous OUT packet into the RxFIFO because the RxFIFO does not have enough space to accommodate a maximum packet size packet for the isochronous OUT endpoint."]
    #[inline(always)]
    pub fn isooutdrop(&self) -> ISOOUTDROP_R {
        ISOOUTDROP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mode: Device only End of Periodic Frame Interrupt (EOPF) Indicates that the period specified in the Periodic Frame Interval field of the Device Configuration register (DCFG.PerFrInt) has been reached in the current microframe. In case of Non-Ignore Frame Number Scatter/Gather (Descriptor DMA) mode, the controller internally handles the following scenarios based on EOPF: Read Flush: At the EOPF, the controller checks if there are any pending packets in the FIFO corresponding to the current (micro)Frame. - If there are any pending packets, then the controller initiates read flush, due to which the read pointer is updated to the starting location of the next micro-frame packet. - If there are no pending packets corresponding to the current (micro)Frame, the controller does not take any action. Write Flush: At the EOPF, if the controller is still fetching the current micro-frame data, then the controller stops pushing data into the TXFIFO but keeps fetching the complete packet from the System Memory. After completing the scheduled packet size fetch, the controller updates the Status Quadlet Fields (Transmit Status to BUFFLUSH) and closes the Descriptor. During the descriptor close, the controller initiates write flush, due to which the write pointer is updated to the starting location of the next micro-frame packet. Because the controller stops pushing the packet to the TxFIFO after EOPF, to bring the write pointer to the starting location of the next micro-frame, write flush is done."]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Mode: Device only Endpoint Mismatch Interrupt (EPMis) Note: This interrupt is valid only in shared FIFO operation. Indicates that an IN token has been received for a non-periodic endpoint, but the data for another endpoint is present in the top of the Non-periodic Transmit FIFO and the IN endpoint mismatch count programmed by the application has expired."]
    #[inline(always)]
    pub fn epmis(&self) -> EPMIS_R {
        EPMIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mode: Device only IN Endpoints Interrupt (IEPInt) The core sets this bit to indicate that an interrupt is pending on one of the IN endpoints of the core (in Device mode). The application must read the Device All Endpoints Interrupt (DAINT) register to determine the exact number of the IN endpoint on Device IN Endpoint-n Interrupt (DIEPINTn) register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding DIEPINTn register to clear this bit."]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mode: Device only OUT Endpoints Interrupt (OEPInt) The controller sets this bit to indicate that an interrupt is pending on one of the OUT endpoints of the core (in Device mode). The application must read the Device All Endpoints Interrupt (DAINT) register to determine the exact number of the OUT endpoint on which the interrupt occurred, and then read the corresponding Device OUT Endpoint-n Interrupt (DOEPINTn) register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the corresponding DOEPINTn register to clear this bit."]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Mode: Device only Incomplete Isochronous IN Transfer (incompISOIN) The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on which the transfer is not completed in the current microframe. This interrupt is asserted along with the End of Periodic Frame Interrupt (EOPF) bit in this register. Note: This interrupt is not asserted in Scatter/Gather DMA mode."]
    #[inline(always)]
    pub fn incompisoin(&self) -> INCOMPISOIN_R {
        INCOMPISOIN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer (incomplP) Mode: Host only In Host mode, the core sets this interrupt bit when there are incomplete periodic transactions still pending which are scheduled for the current microframe. Incomplete Isochronous OUT Transfer (incompISOOUT) Mode: Device only The Device mode, the core sets this interrupt to indicate that there is at least one isochronous OUT endpoint on which the transfer is not completed in the current microframe. This interrupt is asserted along with the End of Periodic Frame Interrupt (EOPF) bit in this register."]
    #[inline(always)]
    pub fn incomplp(&self) -> INCOMPLP_R {
        INCOMPLP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mode: Device only Data Fetch Suspended (FetSusp) This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped fetching data. For IN endpoints due to the unavailability of TxFIFO space or Request Queue space. This interrupt is used by the application for an endpoint mismatch algorithm. For example, after detecting an endpoint mismatch, the application: - Sets a Global non-periodic IN NAK handshake - Disables IN endpoints - Flushes the FIFO - Determines the token sequence from the IN Token Sequence Learning Queue - Re-enables the endpoints - Clears the Global non-periodic IN NAK handshake If the Global non-periodic IN NAK is cleared, the core has not yet fetched data for the IN endpoint, and the IN token is received. The core generates an 'IN token received when FIFO empty' interrupt. It then sends the host a NAK response. To avoid this scenario, the application can check the GINTSTS.FetSusp interrupt, which ensures that the FIFO is full before clearing a Global NAK handshake. Alternatively, the application can mask the IN token received when FIFO empty interrupt when clearing a Global IN NAK handshake."]
    #[inline(always)]
    pub fn fetsusp(&self) -> FETSUSP_R {
        FETSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mode: Device only Reset detected Interrupt (ResetDet) In Device mode, this interrupt is asserted when a reset is detected on the USB in partial power-down mode when the device is in Suspend. In Host mode, this interrupt is not asserted."]
    #[inline(always)]
    pub fn resetdet(&self) -> RESETDET_R {
        RESETDET_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mode: Host only Host Port Interrupt (PrtInt) The core sets this bit to indicate a change in port status of one of the controller ports in Host mode. The application must read the Host Port Control and Status (HPRT) register to determine the exact event that caused this interrupt. The application must clear the appropriate status bit in the Host Port Control and Status register to clear this bit."]
    #[inline(always)]
    pub fn prtint(&self) -> PRTINT_R {
        PRTINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mode: Host only Host Channels Interrupt (HChInt) The core sets this bit to indicate that an interrupt is pending on one of the channels of the core (in Host mode). The application must read the Host All Channels Interrupt (HAINT) register to determine the exact number of the channel on which the interrupt occurred, and Then read the corresponding Host Channel-n Interrupt (HCINTn) register to determine the exact cause of the interrupt. The application must clear the appropriate status bit in the HCINTn register to clear this bit."]
    #[inline(always)]
    pub fn hchint(&self) -> HCHINT_R {
        HCHINT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mode: Host only Periodic TxFIFO Empty (PTxFEmp) This interrupt is asserted when the Periodic Transmit FIFO is either half or completely empty and there is space for at least one entry to be written in the Periodic Request Queue. The half or completely empty status is determined by the Periodic TxFIFO Empty Level bit in the Core AHB Configuration register (GAHBCFG.PTxFEmpLvl)."]
    #[inline(always)]
    pub fn ptxfemp(&self) -> PTXFEMP_R {
        PTXFEMP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Mode: Host and Device Connector ID Status Change (ConIDStsChng) The core sets this bit when there is a change in connector ID status."]
    #[inline(always)]
    pub fn conidstschng(&self) -> CONIDSTSCHNG_R {
        CONIDSTSCHNG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Mode: Host only Disconnect Detected Interrupt (DisconnInt) Asserted when a device disconnect is detected."]
    #[inline(always)]
    pub fn disconnint(&self) -> DISCONNINT_R {
        DISCONNINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Mode: Host and Device Session Request/New Session Detected Interrupt (SessReqInt) In Host mode, this interrupt is asserted when a session request is detected from the device. In Host mode, this interrupt is asserted when a session request is detected from the device. In Device mode, this interrupt is asserted when the utmisrp_bvalid signal goes high. For more information on how to use this interrupt, see 'Partial Power-Down and Clock Gating Programming Model' in the Programming Guide."]
    #[inline(always)]
    pub fn sessreqint(&self) -> SESSREQINT_R {
        SESSREQINT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mode: Host and Device Resume/Remote Wakeup Detected Interrupt (WkUpInt) Wakeup Interrupt during Suspend(L2) or LPM(L1) state. - During Suspend(L2): -- Device Mode: This interrupt is asserted only when Host Initiated Resume is detected on USB. -- Host Mode: This interrupt is asserted only when Device Initiated Remote Wakeup is detected on USB. For more information, see 'Partial Power-Down and Clock Gating Programming Model' in the Programming Guide. - During LPM(L1): -- Device Mode: This interrupt is asserted for either Host Initiated Resume or Device Initiated Remote Wakeup on USB. -- Host Mode: This interrupt is asserted for either Host Initiated Resume or Device Initiated Remote Wakeup on USB. For more information, see 'LPM Entry and Exit Programming Model' in the Programming Guide."]
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTSTS")
            .field("curmod", &self.curmod())
            .field("modemis", &self.modemis())
            .field("otgint", &self.otgint())
            .field("sof", &self.sof())
            .field("rxflvl", &self.rxflvl())
            .field("nptxfemp", &self.nptxfemp())
            .field("ginnakeff", &self.ginnakeff())
            .field("goutnakeff", &self.goutnakeff())
            .field("erlysusp", &self.erlysusp())
            .field("usbsusp", &self.usbsusp())
            .field("usbrst", &self.usbrst())
            .field("enumdone", &self.enumdone())
            .field("isooutdrop", &self.isooutdrop())
            .field("eopf", &self.eopf())
            .field("epmis", &self.epmis())
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .field("incompisoin", &self.incompisoin())
            .field("incomplp", &self.incomplp())
            .field("fetsusp", &self.fetsusp())
            .field("resetdet", &self.resetdet())
            .field("prtint", &self.prtint())
            .field("hchint", &self.hchint())
            .field("ptxfemp", &self.ptxfemp())
            .field("conidstschng", &self.conidstschng())
            .field("disconnint", &self.disconnint())
            .field("sessreqint", &self.sessreqint())
            .field("wkupint", &self.wkupint())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Mode: Host and Device Mode Mismatch Interrupt (ModeMis) The core sets this bit when the application is trying to access: - A Host mode register, when the controller is operating in Device mode - A Device mode register, when the controller is operating in Host mode The register access is completed on the AHB with an OKAY response, but is ignored by the controller internally and does not affect the operation of the controller. This bit can be set only by the core and the application must write 1 to clear it."]
    #[inline(always)]
    pub fn modemis(&mut self) -> MODEMIS_W<'_, GINTSTS_SPEC> {
        MODEMIS_W::new(self, 1)
    }
    #[doc = "Bit 3 - Mode: Host and Device Start of (micro)Frame (Sof) In Host mode, the core sets this bit to indicate that an SOF (FS), micro-SOF (HS), or Keep-Alive (LS) is transmitted on the USB. The application must write a 1 to this bit to clear the interrupt. In Device mode, the controller sets this bit to indicate that an SOF token has been received on the USB. The application can read the Device Status register to get the current (micro)Frame number. This interrupt is seen only when the core is operating at either HS or FS. This bit can be set only by the core and the application must write 1 to clear it. Note: This register may return 1'b1 if read immediately after power-on reset. If the register bit reads 1'b1 immediately after power-on reset, it does not indicate that an SOF has been sent (in case of host mode) or SOF has been received (in case of device mode). The read value of this interrupt is valid only after a valid connection between host and device is established. If the bit is set after power on reset the application can clear the bit."]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<'_, GINTSTS_SPEC> {
        SOF_W::new(self, 3)
    }
    #[doc = "Bit 10 - Mode: Device only Early Suspend (ErlySusp) The controller sets this bit to indicate that an Idle state has been detected on the USB for 3 ms."]
    #[inline(always)]
    pub fn erlysusp(&mut self) -> ERLYSUSP_W<'_, GINTSTS_SPEC> {
        ERLYSUSP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Mode: Device only USB Suspend (USBSusp) The controller sets this bit to indicate that a suspend was detected on the USB. The controller enters the Suspended state when there is no activity on the linestate signal for an extended period of time."]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W<'_, GINTSTS_SPEC> {
        USBSUSP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Mode: Device only USB Reset (USBRst) The controller sets this bit to indicate that a reset is detected on the USB."]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<'_, GINTSTS_SPEC> {
        USBRST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Mode: Device only Enumeration Done (EnumDone) The core sets this bit to indicate that speed enumeration is complete. The application must read the Device Status (DSTS) register to obtain the enumerated speed."]
    #[inline(always)]
    pub fn enumdone(&mut self) -> ENUMDONE_W<'_, GINTSTS_SPEC> {
        ENUMDONE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Mode: Device only Isochronous OUT Packet Dropped Interrupt (ISOOutDrop) The controller sets this bit when it fails to write an isochronous OUT packet into the RxFIFO because the RxFIFO does not have enough space to accommodate a maximum packet size packet for the isochronous OUT endpoint."]
    #[inline(always)]
    pub fn isooutdrop(&mut self) -> ISOOUTDROP_W<'_, GINTSTS_SPEC> {
        ISOOUTDROP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Mode: Device only End of Periodic Frame Interrupt (EOPF) Indicates that the period specified in the Periodic Frame Interval field of the Device Configuration register (DCFG.PerFrInt) has been reached in the current microframe. In case of Non-Ignore Frame Number Scatter/Gather (Descriptor DMA) mode, the controller internally handles the following scenarios based on EOPF: Read Flush: At the EOPF, the controller checks if there are any pending packets in the FIFO corresponding to the current (micro)Frame. - If there are any pending packets, then the controller initiates read flush, due to which the read pointer is updated to the starting location of the next micro-frame packet. - If there are no pending packets corresponding to the current (micro)Frame, the controller does not take any action. Write Flush: At the EOPF, if the controller is still fetching the current micro-frame data, then the controller stops pushing data into the TXFIFO but keeps fetching the complete packet from the System Memory. After completing the scheduled packet size fetch, the controller updates the Status Quadlet Fields (Transmit Status to BUFFLUSH) and closes the Descriptor. During the descriptor close, the controller initiates write flush, due to which the write pointer is updated to the starting location of the next micro-frame packet. Because the controller stops pushing the packet to the TxFIFO after EOPF, to bring the write pointer to the starting location of the next micro-frame, write flush is done."]
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W<'_, GINTSTS_SPEC> {
        EOPF_W::new(self, 15)
    }
    #[doc = "Bit 17 - Mode: Device only Endpoint Mismatch Interrupt (EPMis) Note: This interrupt is valid only in shared FIFO operation. Indicates that an IN token has been received for a non-periodic endpoint, but the data for another endpoint is present in the top of the Non-periodic Transmit FIFO and the IN endpoint mismatch count programmed by the application has expired."]
    #[inline(always)]
    pub fn epmis(&mut self) -> EPMIS_W<'_, GINTSTS_SPEC> {
        EPMIS_W::new(self, 17)
    }
    #[doc = "Bit 20 - Mode: Device only Incomplete Isochronous IN Transfer (incompISOIN) The core sets this interrupt to indicate that there is at least one isochronous IN endpoint on which the transfer is not completed in the current microframe. This interrupt is asserted along with the End of Periodic Frame Interrupt (EOPF) bit in this register. Note: This interrupt is not asserted in Scatter/Gather DMA mode."]
    #[inline(always)]
    pub fn incompisoin(&mut self) -> INCOMPISOIN_W<'_, GINTSTS_SPEC> {
        INCOMPISOIN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer (incomplP) Mode: Host only In Host mode, the core sets this interrupt bit when there are incomplete periodic transactions still pending which are scheduled for the current microframe. Incomplete Isochronous OUT Transfer (incompISOOUT) Mode: Device only The Device mode, the core sets this interrupt to indicate that there is at least one isochronous OUT endpoint on which the transfer is not completed in the current microframe. This interrupt is asserted along with the End of Periodic Frame Interrupt (EOPF) bit in this register."]
    #[inline(always)]
    pub fn incomplp(&mut self) -> INCOMPLP_W<'_, GINTSTS_SPEC> {
        INCOMPLP_W::new(self, 21)
    }
    #[doc = "Bit 22 - Mode: Device only Data Fetch Suspended (FetSusp) This interrupt is valid only in DMA mode. This interrupt indicates that the core has stopped fetching data. For IN endpoints due to the unavailability of TxFIFO space or Request Queue space. This interrupt is used by the application for an endpoint mismatch algorithm. For example, after detecting an endpoint mismatch, the application: - Sets a Global non-periodic IN NAK handshake - Disables IN endpoints - Flushes the FIFO - Determines the token sequence from the IN Token Sequence Learning Queue - Re-enables the endpoints - Clears the Global non-periodic IN NAK handshake If the Global non-periodic IN NAK is cleared, the core has not yet fetched data for the IN endpoint, and the IN token is received. The core generates an 'IN token received when FIFO empty' interrupt. It then sends the host a NAK response. To avoid this scenario, the application can check the GINTSTS.FetSusp interrupt, which ensures that the FIFO is full before clearing a Global NAK handshake. Alternatively, the application can mask the IN token received when FIFO empty interrupt when clearing a Global IN NAK handshake."]
    #[inline(always)]
    pub fn fetsusp(&mut self) -> FETSUSP_W<'_, GINTSTS_SPEC> {
        FETSUSP_W::new(self, 22)
    }
    #[doc = "Bit 23 - Mode: Device only Reset detected Interrupt (ResetDet) In Device mode, this interrupt is asserted when a reset is detected on the USB in partial power-down mode when the device is in Suspend. In Host mode, this interrupt is not asserted."]
    #[inline(always)]
    pub fn resetdet(&mut self) -> RESETDET_W<'_, GINTSTS_SPEC> {
        RESETDET_W::new(self, 23)
    }
    #[doc = "Bit 28 - Mode: Host and Device Connector ID Status Change (ConIDStsChng) The core sets this bit when there is a change in connector ID status."]
    #[inline(always)]
    pub fn conidstschng(&mut self) -> CONIDSTSCHNG_W<'_, GINTSTS_SPEC> {
        CONIDSTSCHNG_W::new(self, 28)
    }
    #[doc = "Bit 29 - Mode: Host only Disconnect Detected Interrupt (DisconnInt) Asserted when a device disconnect is detected."]
    #[inline(always)]
    pub fn disconnint(&mut self) -> DISCONNINT_W<'_, GINTSTS_SPEC> {
        DISCONNINT_W::new(self, 29)
    }
    #[doc = "Bit 30 - Mode: Host and Device Session Request/New Session Detected Interrupt (SessReqInt) In Host mode, this interrupt is asserted when a session request is detected from the device. In Host mode, this interrupt is asserted when a session request is detected from the device. In Device mode, this interrupt is asserted when the utmisrp_bvalid signal goes high. For more information on how to use this interrupt, see 'Partial Power-Down and Clock Gating Programming Model' in the Programming Guide."]
    #[inline(always)]
    pub fn sessreqint(&mut self) -> SESSREQINT_W<'_, GINTSTS_SPEC> {
        SESSREQINT_W::new(self, 30)
    }
    #[doc = "Bit 31 - Mode: Host and Device Resume/Remote Wakeup Detected Interrupt (WkUpInt) Wakeup Interrupt during Suspend(L2) or LPM(L1) state. - During Suspend(L2): -- Device Mode: This interrupt is asserted only when Host Initiated Resume is detected on USB. -- Host Mode: This interrupt is asserted only when Device Initiated Remote Wakeup is detected on USB. For more information, see 'Partial Power-Down and Clock Gating Programming Model' in the Programming Guide. - During LPM(L1): -- Device Mode: This interrupt is asserted for either Host Initiated Resume or Device Initiated Remote Wakeup on USB. -- Host Mode: This interrupt is asserted for either Host Initiated Resume or Device Initiated Remote Wakeup on USB. For more information, see 'LPM Entry and Exit Programming Model' in the Programming Guide."]
    #[inline(always)]
    pub fn wkupint(&mut self) -> WKUPINT_W<'_, GINTSTS_SPEC> {
        WKUPINT_W::new(self, 31)
    }
}
#[doc = "This register interrupts the application for system-level events in the current mode (Device mode or Host mode). Some of the bits in this register are valid only in Host mode, while others are valid in Device mode only. This register also indicates the current mode. To clear the interrupt status bits of type R_SS_WC, the application must write 1'b1 to the bit. The FIFO status interrupts are read only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization. Note: Read the reset value of GINTSTS.CurMod only after the following conditions: - If IDDIG_FILTER is disabled, read only after PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires.\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTSTS_SPEC;
impl crate::RegisterSpec for GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts::R`](R) reader structure"]
impl crate::Readable for GINTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintsts::W`](W) writer structure"]
impl crate::Writable for GINTSTS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GINTSTS to value 0x0400_0020"]
impl crate::Resettable for GINTSTS_SPEC {
    const RESET_VALUE: u32 = 0x0400_0020;
}
