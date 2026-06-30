#[doc = "Register `DIEPCTL0` reader"]
pub type R = crate::R<DIEPCTL0_SPEC>;
#[doc = "Register `DIEPCTL0` writer"]
pub type W = crate::W<DIEPCTL0_SPEC>;
#[doc = "Field `MPS` reader - Maximum Packet Size (MPS) Applies to IN and OUT endpoints. The application must program this field with the maximum packet size for the current logical endpoint. - 2'b00: 64 bytes - 2'b01: 32 bytes - 2'b10: 16 bytes - 2'b11: 8 bytes"]
pub type MPS_R = crate::FieldReader;
#[doc = "Field `MPS` writer - Maximum Packet Size (MPS) Applies to IN and OUT endpoints. The application must program this field with the maximum packet size for the current logical endpoint. - 2'b00: 64 bytes - 2'b01: 32 bytes - 2'b10: 16 bytes - 2'b11: 8 bytes"]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBACTEP` reader - USB Active Endpoint (USBActEP) This bit is always SET to 1, indicating that control endpoint 0 is always active in all configurations and interfaces."]
pub type USBACTEP_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status (NAKSts) Indicates the following: - 1'b0: The core is transmitting non-NAK handshakes based on the FIFO status - 1'b1: The core is transmitting NAK handshakes on this endpoint. When this bit is set, either by the application or core, the core stops transmitting data, even If there is data available in the TxFIFO. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type (EPType) Hardcoded to 00 for control."]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `STALL` reader - STALL Handshake (Stall) The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global Nonperiodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority."]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL Handshake (Stall) The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global Nonperiodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority."]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO Number (TxFNum) - For Shared FIFO operation, this value is always set to 0, indicating that control IN endpoint 0 data is always written in the Non-Periodic Transmit FIFO. - For Dedicated FIFO operation, this value is set to the FIFO number that is assigned to IN Endpoint."]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO Number (TxFNum) - For Shared FIFO operation, this value is always set to 0, indicating that control IN endpoint 0 data is always written in the Non-Periodic Transmit FIFO. - For Dedicated FIFO operation, this value is set to the FIFO number that is assigned to IN Endpoint."]
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` reader - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
pub type CNAK_R = crate::BitReader;
#[doc = "Field `CNAK` writer - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` reader - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for an endpoint after a SETUP packet is received on that endpoint."]
pub type SNAK_R = crate::BitReader;
#[doc = "Field `SNAK` writer - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for an endpoint after a SETUP packet is received on that endpoint."]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable (EPDis) The application sets this bit to stop transmitting data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint Disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint Disabled Interrupt. The application must Set this bit only if Endpoint Enable is already set for this endpoint."]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint Disable (EPDis) The application sets this bit to stop transmitting data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint Disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint Disabled Interrupt. The application must Set this bit only if Endpoint Enable is already set for this endpoint."]
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - Endpoint Enable (EPEna) When Scatter/Gather DMA mode is enabled for IN endpoints, this bit indicates that the descriptor structure and data buffer with data ready to transmit is setup. When Scatter/Gather DMA mode is disabled (such as in buffer pointer based DMA mode) this bit indicates that data is ready to be transmitted on the endpoint. The core clears this bit before setting the following interrupts on this endpoint: - Endpoint Disabled - Transfer Completed"]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint Enable (EPEna) When Scatter/Gather DMA mode is enabled for IN endpoints, this bit indicates that the descriptor structure and data buffer with data ready to transmit is setup. When Scatter/Gather DMA mode is disabled (such as in buffer pointer based DMA mode) this bit indicates that data is ready to be transmitted on the endpoint. The core clears this bit before setting the following interrupts on this endpoint: - Endpoint Disabled - Transfer Completed"]
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Maximum Packet Size (MPS) Applies to IN and OUT endpoints. The application must program this field with the maximum packet size for the current logical endpoint. - 2'b00: 64 bytes - 2'b01: 32 bytes - 2'b10: 16 bytes - 2'b11: 8 bytes"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB Active Endpoint (USBActEP) This bit is always SET to 1, indicating that control endpoint 0 is always active in all configurations and interfaces."]
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status (NAKSts) Indicates the following: - 1'b0: The core is transmitting non-NAK handshakes based on the FIFO status - 1'b1: The core is transmitting NAK handshakes on this endpoint. When this bit is set, either by the application or core, the core stops transmitting data, even If there is data available in the TxFIFO. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type (EPType) Hardcoded to 00 for control."]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - STALL Handshake (Stall) The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global Nonperiodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO Number (TxFNum) - For Shared FIFO operation, this value is always set to 0, indicating that control IN endpoint 0 data is always written in the Non-Periodic Transmit FIFO. - For Dedicated FIFO operation, this value is set to the FIFO number that is assigned to IN Endpoint."]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
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
    #[doc = "Bit 30 - Endpoint Disable (EPDis) The application sets this bit to stop transmitting data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint Disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint Disabled Interrupt. The application must Set this bit only if Endpoint Enable is already set for this endpoint."]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable (EPEna) When Scatter/Gather DMA mode is enabled for IN endpoints, this bit indicates that the descriptor structure and data buffer with data ready to transmit is setup. When Scatter/Gather DMA mode is disabled (such as in buffer pointer based DMA mode) this bit indicates that data is ready to be transmitted on the endpoint. The core clears this bit before setting the following interrupts on this endpoint: - Endpoint Disabled - Transfer Completed"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL0")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Maximum Packet Size (MPS) Applies to IN and OUT endpoints. The application must program this field with the maximum packet size for the current logical endpoint. - 2'b00: 64 bytes - 2'b01: 32 bytes - 2'b10: 16 bytes - 2'b11: 8 bytes"]
    #[inline(always)]
    pub fn mps(&mut self) -> MPS_W<'_, DIEPCTL0_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bit 21 - STALL Handshake (Stall) The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit, Global Nonperiodic IN NAK, or Global OUT NAK is set along with this bit, the STALL bit takes priority."]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, DIEPCTL0_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO Number (TxFNum) - For Shared FIFO operation, this value is always set to 0, indicating that control IN endpoint 0 data is always written in the Non-Periodic Transmit FIFO. - For Dedicated FIFO operation, this value is set to the FIFO number that is assigned to IN Endpoint."]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<'_, DIEPCTL0_SPEC> {
        TXFNUM_W::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<'_, DIEPCTL0_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set this bit for an endpoint after a SETUP packet is received on that endpoint."]
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<'_, DIEPCTL0_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 30 - Endpoint Disable (EPDis) The application sets this bit to stop transmitting data on an endpoint, even before the transfer for that endpoint is complete. The application must wait for the Endpoint Disabled interrupt before treating the endpoint as disabled. The core clears this bit before setting the Endpoint Disabled Interrupt. The application must Set this bit only if Endpoint Enable is already set for this endpoint."]
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<'_, DIEPCTL0_SPEC> {
        EPDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint Enable (EPEna) When Scatter/Gather DMA mode is enabled for IN endpoints, this bit indicates that the descriptor structure and data buffer with data ready to transmit is setup. When Scatter/Gather DMA mode is disabled (such as in buffer pointer based DMA mode) this bit indicates that data is ready to be transmitted on the endpoint. The core clears this bit before setting the following interrupts on this endpoint: - Endpoint Disabled - Transfer Completed"]
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<'_, DIEPCTL0_SPEC> {
        EPENA_W::new(self, 31)
    }
}
#[doc = "This register is used to control the characteristics of the IN Endpoint 0 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPCTL0_SPEC;
impl crate::RegisterSpec for DIEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl0::R`](R) reader structure"]
impl crate::Readable for DIEPCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepctl0::W`](W) writer structure"]
impl crate::Writable for DIEPCTL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPCTL0 to value 0x8000"]
impl crate::Resettable for DIEPCTL0_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
