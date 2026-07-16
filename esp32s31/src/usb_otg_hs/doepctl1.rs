#[doc = "Register `DOEPCTL1` reader"]
pub type R = crate::R<DOEPCTL1_SPEC>;
#[doc = "Register `DOEPCTL1` writer"]
pub type W = crate::W<DOEPCTL1_SPEC>;
#[doc = "Field `MPS` reader - Maximum Packet Size (MPS) The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes."]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size (MPS) The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes."]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBACTEP` reader - USB Active Endpoint (USBActEP) Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit."]
pub type USBACTEP_R = crate::BitReader;
#[doc = "Field `USBACTEP` writer - USB Active Endpoint (USBActEP) Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit."]
pub type USBACTEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPID` reader - Endpoint Data PID (DPID) Applies to interrupt/bulk IN and OUT endpoints only. Contains the PID of the packet to be received or transmitted on this endpoint. The application must program the PID of the first packet to be received or transmitted on this endpoint, after the endpoint is activated. The applications use the SetD1PID and SetD0PID fields of this register to program either DATA0 or DATA1 PID. - 1'b0: DATA0 - 1'b1: DATA1 This field is applicable for both Scatter/Gather DMA mode and non-Scatter/Gather DMA mode. Reset: 1'b0 Even/Odd (Micro)Frame (EO_FrNum) In non-Scatter/Gather DMA mode: - Applies to isochronous IN and OUT endpoints only. - Indicates the (micro)frame number in which the core transmits/receives isochronous data for this endpoint. The application must program the even/odd (micro)frame number in which it intends to transmit/receive isochronous data for this endpoint using the SetEvnFr and SetOddFr fields in this register. -- 1'b0: Even (micro)frame -- 1'b1: Odd (micro)frame - When Scatter/Gather DMA mode is enabled, this field is reserved. The frame number in which to send data is provided in the transmit descriptor structure. The frame in which data is received is updated in receive descriptor structure. Reset: 1'b0"]
pub type DPID_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status (NAKSts) Indicates the following: - 1'b0: The core is transmitting non-NAK handshakes based on the FIFO status. - 1'b1: The core is transmitting NAK handshakes on this endpoint. When either the application or the core sets this bit: - The core stops receiving any data on an OUT endpoint, even if there is space in the RxFIFO to accommodate the incoming packet. - For non-isochronous IN endpoints: The core stops transmitting any data on an IN endpoint, even if there data is available in the TxFIFO. - For isochronous IN endpoints: The core sends out a zero-length data packet, even if there data is available in the TxFIFO. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type (EPType) This is the transfer type supported by this logical endpoint. - 2'b00: Control - 2'b01: Isochronous - 2'b10: Bulk - 2'b11: Interrupt"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint Type (EPType) This is the transfer type supported by this logical endpoint. - 2'b00: Control - 2'b01: Isochronous - 2'b10: Bulk - 2'b11: Interrupt"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNP` reader - RESERVED"]
pub type SNP_R = crate::BitReader;
#[doc = "Field `SNP` writer - RESERVED"]
pub type SNP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL Handshake (Stall) Applies to non-control, non-isochronous IN and OUT endpoints only. The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global Non-periodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core. Applies to control endpoints only. The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global Non-periodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL Handshake (Stall) Applies to non-control, non-isochronous IN and OUT endpoints only. The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global Non-periodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core. Applies to control endpoints only. The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global Non-periodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` reader - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
pub type CNAK_R = crate::BitReader;
#[doc = "Field `CNAK` writer - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` reader - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for an endpoint after a SETUP packet is received on that endpoint."]
pub type SNAK_R = crate::BitReader;
#[doc = "Field `SNAK` writer - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for an endpoint after a SETUP packet is received on that endpoint."]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD0PID` reader - Set DATA0 PID (SetD0PID) - Applies to interrupt/bulk IN and OUT endpoints only. - Writing to this field sets the Endpoint Data PID (DPID) field in this register to DATA0. - This field is applicable both for Scatter/Gather DMA mode and non-Scatter/Gather DMA mode. Reset: 1'b0 In non-Scatter/Gather DMA mode: Set Even (micro)frame (SetEvenFr) - Applies to isochronous IN and OUT endpoints only. - Writing to this field sets the Even/Odd (micro)frame (EO_FrNum) field to even (micro)frame. - When Scatter/Gather DMA mode is enabled, this field is reserved. The frame number in which to send data is in the transmit descriptor structure. The frame in which to receive data is updated in receive descriptor structure. Reset: 1'b0"]
pub type SETD0PID_R = crate::BitReader;
#[doc = "Field `SETD0PID` writer - Set DATA0 PID (SetD0PID) - Applies to interrupt/bulk IN and OUT endpoints only. - Writing to this field sets the Endpoint Data PID (DPID) field in this register to DATA0. - This field is applicable both for Scatter/Gather DMA mode and non-Scatter/Gather DMA mode. Reset: 1'b0 In non-Scatter/Gather DMA mode: Set Even (micro)frame (SetEvenFr) - Applies to isochronous IN and OUT endpoints only. - Writing to this field sets the Even/Odd (micro)frame (EO_FrNum) field to even (micro)frame. - When Scatter/Gather DMA mode is enabled, this field is reserved. The frame number in which to send data is in the transmit descriptor structure. The frame in which to receive data is updated in receive descriptor structure. Reset: 1'b0"]
pub type SETD0PID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD1PID` reader - Set DATA1 PID (SetD1PID) - Applies to interrupt and bulk IN and OUT endpoints only. - Writing to this field sets the Endpoint Data PID (DPID) field in this register to DATA1. - This field is applicable both for scatter-gather DMA mode and non scatter-gather DMA mode. Reset: 1'b0 Set Odd (micro)frame (SetOddFr) - Applies to isochronous IN and OUT endpoints only. - Writing to this field sets the even and odd (micro)frame (EO_FrNum) field to odd (micro)frame. Reset: 1'b0"]
pub type SETD1PID_R = crate::BitReader;
#[doc = "Field `SETD1PID` writer - Set DATA1 PID (SetD1PID) - Applies to interrupt and bulk IN and OUT endpoints only. - Writing to this field sets the Endpoint Data PID (DPID) field in this register to DATA1. - This field is applicable both for scatter-gather DMA mode and non scatter-gather DMA mode. Reset: 1'b0 Set Odd (micro)frame (SetOddFr) - Applies to isochronous IN and OUT endpoints only. - Writing to this field sets the even and odd (micro)frame (EO_FrNum) field to odd (micro)frame. Reset: 1'b0"]
pub type SETD1PID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable (EPDis) Applies to IN and OUT endpoints. The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint Disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint Disabled interrupt. The application must set this bit only if Endpoint Enable is already set for this endpoint."]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint Disable (EPDis) Applies to IN and OUT endpoints. The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint Disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint Disabled interrupt. The application must set this bit only if Endpoint Enable is already set for this endpoint."]
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - Endpoint Enable (EPEna) Applies to IN and OUT endpoints. When Scatter/Gather DMA mode is enabled, - For IN endpoints this bit indicates that the descriptor structure and data buffer with data ready to transmit is setup. - For OUT endpoint it indicates that the descriptor structure and data buffer to receive data is setup. When Scatter/Gather DMA mode is enabled such as for buffer-pointer based DMA mode: - For IN endpoints, this bit indicates that data is ready to be transmitted on the endpoint. - For OUT endpoints, this bit indicates that the application has allocated the memory to start receiving data from the USB. The core clears this bit before setting any of the following interrupts on this endpoint: - SETUP Phase Done - Endpoint Disabled - Transfer Completed Note: For control endpoints in DMA mode, this bit must be set for the controller to transfer SETUP data packets to the memory. This bit is not cleared on Transfer Completed interrupt of the SETUP packet."]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint Enable (EPEna) Applies to IN and OUT endpoints. When Scatter/Gather DMA mode is enabled, - For IN endpoints this bit indicates that the descriptor structure and data buffer with data ready to transmit is setup. - For OUT endpoint it indicates that the descriptor structure and data buffer to receive data is setup. When Scatter/Gather DMA mode is enabled such as for buffer-pointer based DMA mode: - For IN endpoints, this bit indicates that data is ready to be transmitted on the endpoint. - For OUT endpoints, this bit indicates that the application has allocated the memory to start receiving data from the USB. The core clears this bit before setting any of the following interrupts on this endpoint: - SETUP Phase Done - Endpoint Disabled - Transfer Completed Note: For control endpoints in DMA mode, this bit must be set for the controller to transfer SETUP data packets to the memory. This bit is not cleared on Transfer Completed interrupt of the SETUP packet."]
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size (MPS) The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes."]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB Active Endpoint (USBActEP) Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit."]
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Data PID (DPID) Applies to interrupt/bulk IN and OUT endpoints only. Contains the PID of the packet to be received or transmitted on this endpoint. The application must program the PID of the first packet to be received or transmitted on this endpoint, after the endpoint is activated. The applications use the SetD1PID and SetD0PID fields of this register to program either DATA0 or DATA1 PID. - 1'b0: DATA0 - 1'b1: DATA1 This field is applicable for both Scatter/Gather DMA mode and non-Scatter/Gather DMA mode. Reset: 1'b0 Even/Odd (Micro)Frame (EO_FrNum) In non-Scatter/Gather DMA mode: - Applies to isochronous IN and OUT endpoints only. - Indicates the (micro)frame number in which the core transmits/receives isochronous data for this endpoint. The application must program the even/odd (micro)frame number in which it intends to transmit/receive isochronous data for this endpoint using the SetEvnFr and SetOddFr fields in this register. -- 1'b0: Even (micro)frame -- 1'b1: Odd (micro)frame - When Scatter/Gather DMA mode is enabled, this field is reserved. The frame number in which to send data is provided in the transmit descriptor structure. The frame in which data is received is updated in receive descriptor structure. Reset: 1'b0"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status (NAKSts) Indicates the following: - 1'b0: The core is transmitting non-NAK handshakes based on the FIFO status. - 1'b1: The core is transmitting NAK handshakes on this endpoint. When either the application or the core sets this bit: - The core stops receiving any data on an OUT endpoint, even if there is space in the RxFIFO to accommodate the incoming packet. - For non-isochronous IN endpoints: The core stops transmitting any data on an IN endpoint, even if there data is available in the TxFIFO. - For isochronous IN endpoints: The core sends out a zero-length data packet, even if there data is available in the TxFIFO. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type (EPType) This is the transfer type supported by this logical endpoint. - 2'b00: Control - 2'b01: Isochronous - 2'b10: Bulk - 2'b11: Interrupt"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - RESERVED"]
    #[inline(always)]
    pub fn snp(&self) -> SNP_R {
        SNP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL Handshake (Stall) Applies to non-control, non-isochronous IN and OUT endpoints only. The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global Non-periodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core. Applies to control endpoints only. The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global Non-periodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
    #[inline(always)]
    pub fn cnak(&self) -> CNAK_R {
        CNAK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for an endpoint after a SETUP packet is received on that endpoint."]
    #[inline(always)]
    pub fn snak(&self) -> SNAK_R {
        SNAK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set DATA0 PID (SetD0PID) - Applies to interrupt/bulk IN and OUT endpoints only. - Writing to this field sets the Endpoint Data PID (DPID) field in this register to DATA0. - This field is applicable both for Scatter/Gather DMA mode and non-Scatter/Gather DMA mode. Reset: 1'b0 In non-Scatter/Gather DMA mode: Set Even (micro)frame (SetEvenFr) - Applies to isochronous IN and OUT endpoints only. - Writing to this field sets the Even/Odd (micro)frame (EO_FrNum) field to even (micro)frame. - When Scatter/Gather DMA mode is enabled, this field is reserved. The frame number in which to send data is in the transmit descriptor structure. The frame in which to receive data is updated in receive descriptor structure. Reset: 1'b0"]
    #[inline(always)]
    pub fn setd0pid(&self) -> SETD0PID_R {
        SETD0PID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set DATA1 PID (SetD1PID) - Applies to interrupt and bulk IN and OUT endpoints only. - Writing to this field sets the Endpoint Data PID (DPID) field in this register to DATA1. - This field is applicable both for scatter-gather DMA mode and non scatter-gather DMA mode. Reset: 1'b0 Set Odd (micro)frame (SetOddFr) - Applies to isochronous IN and OUT endpoints only. - Writing to this field sets the even and odd (micro)frame (EO_FrNum) field to odd (micro)frame. Reset: 1'b0"]
    #[inline(always)]
    pub fn setd1pid(&self) -> SETD1PID_R {
        SETD1PID_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint Disable (EPDis) Applies to IN and OUT endpoints. The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint Disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint Disabled interrupt. The application must set this bit only if Endpoint Enable is already set for this endpoint."]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable (EPEna) Applies to IN and OUT endpoints. When Scatter/Gather DMA mode is enabled, - For IN endpoints this bit indicates that the descriptor structure and data buffer with data ready to transmit is setup. - For OUT endpoint it indicates that the descriptor structure and data buffer to receive data is setup. When Scatter/Gather DMA mode is enabled such as for buffer-pointer based DMA mode: - For IN endpoints, this bit indicates that data is ready to be transmitted on the endpoint. - For OUT endpoints, this bit indicates that the application has allocated the memory to start receiving data from the USB. The core clears this bit before setting any of the following interrupts on this endpoint: - SETUP Phase Done - Endpoint Disabled - Transfer Completed Note: For control endpoints in DMA mode, this bit must be set for the controller to transfer SETUP data packets to the memory. This bit is not cleared on Transfer Completed interrupt of the SETUP packet."]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL1")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("dpid", &self.dpid())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("setd0pid", &self.setd0pid())
            .field("setd1pid", &self.setd1pid())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size (MPS) The application must program this field with the maximum packet size for the current logical endpoint. This value is in bytes."]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W<'_, DOEPCTL1_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bit 15 - USB Active Endpoint (USBActEP) Indicates whether this endpoint is active in the current configuration and interface. The core clears this bit for all endpoints (other than EP 0) after detecting a USB reset. After receiving the SetConfiguration and SetInterface commands, the application must program endpoint registers accordingly and set this bit."]
    #[inline(always)]
    pub fn usbactep(&mut self) -> USBACTEP_W<'_, DOEPCTL1_SPEC> {
        USBACTEP_W::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint Type (EPType) This is the transfer type supported by this logical endpoint. - 2'b00: Control - 2'b01: Isochronous - 2'b10: Bulk - 2'b11: Interrupt"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W<'_, DOEPCTL1_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bit 20 - RESERVED"]
    #[inline(always)]
    pub fn snp(&mut self) -> SNP_W<'_, DOEPCTL1_SPEC> {
        SNP_W::new(self, 20)
    }
    #[doc = "Bit 21 - STALL Handshake (Stall) Applies to non-control, non-isochronous IN and OUT endpoints only. The application sets this bit to stall all tokens from the USB host to this endpoint. If a NAK bit, Global Non-periodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Only the application can clear this bit, never the core. Applies to control endpoints only. The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global Non-periodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, DOEPCTL1_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<'_, DOEPCTL1_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for an endpoint after a SETUP packet is received on that endpoint."]
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<'_, DOEPCTL1_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set DATA0 PID (SetD0PID) - Applies to interrupt/bulk IN and OUT endpoints only. - Writing to this field sets the Endpoint Data PID (DPID) field in this register to DATA0. - This field is applicable both for Scatter/Gather DMA mode and non-Scatter/Gather DMA mode. Reset: 1'b0 In non-Scatter/Gather DMA mode: Set Even (micro)frame (SetEvenFr) - Applies to isochronous IN and OUT endpoints only. - Writing to this field sets the Even/Odd (micro)frame (EO_FrNum) field to even (micro)frame. - When Scatter/Gather DMA mode is enabled, this field is reserved. The frame number in which to send data is in the transmit descriptor structure. The frame in which to receive data is updated in receive descriptor structure. Reset: 1'b0"]
    #[inline(always)]
    pub fn setd0pid(&mut self) -> SETD0PID_W<'_, DOEPCTL1_SPEC> {
        SETD0PID_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set DATA1 PID (SetD1PID) - Applies to interrupt and bulk IN and OUT endpoints only. - Writing to this field sets the Endpoint Data PID (DPID) field in this register to DATA1. - This field is applicable both for scatter-gather DMA mode and non scatter-gather DMA mode. Reset: 1'b0 Set Odd (micro)frame (SetOddFr) - Applies to isochronous IN and OUT endpoints only. - Writing to this field sets the even and odd (micro)frame (EO_FrNum) field to odd (micro)frame. Reset: 1'b0"]
    #[inline(always)]
    pub fn setd1pid(&mut self) -> SETD1PID_W<'_, DOEPCTL1_SPEC> {
        SETD1PID_W::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint Disable (EPDis) Applies to IN and OUT endpoints. The application sets this bit to stop transmitting/receiving data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint Disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint Disabled interrupt. The application must set this bit only if Endpoint Enable is already set for this endpoint."]
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<'_, DOEPCTL1_SPEC> {
        EPDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint Enable (EPEna) Applies to IN and OUT endpoints. When Scatter/Gather DMA mode is enabled, - For IN endpoints this bit indicates that the descriptor structure and data buffer with data ready to transmit is setup. - For OUT endpoint it indicates that the descriptor structure and data buffer to receive data is setup. When Scatter/Gather DMA mode is enabled such as for buffer-pointer based DMA mode: - For IN endpoints, this bit indicates that data is ready to be transmitted on the endpoint. - For OUT endpoints, this bit indicates that the application has allocated the memory to start receiving data from the USB. The core clears this bit before setting any of the following interrupts on this endpoint: - SETUP Phase Done - Endpoint Disabled - Transfer Completed Note: For control endpoints in DMA mode, this bit must be set for the controller to transfer SETUP data packets to the memory. This bit is not cleared on Transfer Completed interrupt of the SETUP packet."]
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<'_, DOEPCTL1_SPEC> {
        EPENA_W::new(self, 31)
    }
}
#[doc = "This register is used to control the characteristics of OUT Endpoint 1 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPCTL1_SPEC;
impl crate::RegisterSpec for DOEPCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl1::R`](R) reader structure"]
impl crate::Readable for DOEPCTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepctl1::W`](W) writer structure"]
impl crate::Writable for DOEPCTL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPCTL1 to value 0"]
impl crate::Resettable for DOEPCTL1_SPEC {}
