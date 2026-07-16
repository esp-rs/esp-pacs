#[doc = "Register `DOEPCTL0` reader"]
pub type R = crate::R<DOEPCTL0_SPEC>;
#[doc = "Register `DOEPCTL0` writer"]
pub type W = crate::W<DOEPCTL0_SPEC>;
#[doc = "Field `MPS` reader - Maximum Packet Size (MPS) The maximum packet size for control OUT endpoint 0 is the same as what is programmed in control IN Endpoint 0. - 2'b00: 64 bytes - 2'b01: 32 bytes - 2'b10: 16 bytes - 2'b11: 8 bytes"]
pub type MPS_R = crate::FieldReader;
#[doc = "Field `USBACTEP` reader - USB Active Endpoint (USBActEP) This bit is always set to 1, indicating that a control endpoint 0 is always active in all configurations and interfaces."]
pub type USBACTEP_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK Status (NAKSts) Indicates the following: - 1'b0: The core is transmitting non-NAK handshakes based on the FIFO status. - 1'b1: The core is transmitting NAK handshakes on this endpoint. When either the application or the core sets this bit, the core stops receiving data, even If there is space in the RxFIFO to accommodate the incoming packet. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint Type (EPType) Hardcoded to 2'b00 for control."]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `SNP` reader - RESERVED"]
pub type SNP_R = crate::BitReader;
#[doc = "Field `SNP` writer - RESERVED"]
pub type SNP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL Handshake (Stall) The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit or Global OUT NAK is Set along with this bit, the STALL bit takes priority. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL Handshake (Stall) The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit or Global OUT NAK is Set along with this bit, the STALL bit takes priority. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` reader - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
pub type CNAK_R = crate::BitReader;
#[doc = "Field `CNAK` writer - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` reader - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set bit on a Transfer Completed interrupt, or after a SETUP is received on the endpoint."]
pub type SNAK_R = crate::BitReader;
#[doc = "Field `SNAK` writer - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set bit on a Transfer Completed interrupt, or after a SETUP is received on the endpoint."]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint Disable (EPDis) The application cannot disable control OUT endpoint 0."]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPENA` reader - Endpoint Enable (EPEna) - When Scatter/Gather DMA mode is enabled, for OUT endpoints this bit indicates that the descriptor structure and data buffer to receive data is setup. - When Scatter/Gather DMA mode is disabled (such as for buffer-pointer based DMA mode)this bit indicates that the application has allocated the memory to start receiving data from the USB. - The core clears this bit before setting any of the following interrupts on this endpoint: -- SETUP Phase Done -- Endpoint Disabled -- Transfer Completed Note: In DMA mode, this bit must be set for the core to transfer SETUP data packets into memory and it is not cleared on Transfer Completed interrupt of SETUP packet."]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint Enable (EPEna) - When Scatter/Gather DMA mode is enabled, for OUT endpoints this bit indicates that the descriptor structure and data buffer to receive data is setup. - When Scatter/Gather DMA mode is disabled (such as for buffer-pointer based DMA mode)this bit indicates that the application has allocated the memory to start receiving data from the USB. - The core clears this bit before setting any of the following interrupts on this endpoint: -- SETUP Phase Done -- Endpoint Disabled -- Transfer Completed Note: In DMA mode, this bit must be set for the core to transfer SETUP data packets into memory and it is not cleared on Transfer Completed interrupt of SETUP packet."]
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Maximum Packet Size (MPS) The maximum packet size for control OUT endpoint 0 is the same as what is programmed in control IN Endpoint 0. - 2'b00: 64 bytes - 2'b01: 32 bytes - 2'b10: 16 bytes - 2'b11: 8 bytes"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB Active Endpoint (USBActEP) This bit is always set to 1, indicating that a control endpoint 0 is always active in all configurations and interfaces."]
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status (NAKSts) Indicates the following: - 1'b0: The core is transmitting non-NAK handshakes based on the FIFO status. - 1'b1: The core is transmitting NAK handshakes on this endpoint. When either the application or the core sets this bit, the core stops receiving data, even If there is space in the RxFIFO to accommodate the incoming packet. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type (EPType) Hardcoded to 2'b00 for control."]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - RESERVED"]
    #[inline(always)]
    pub fn snp(&self) -> SNP_R {
        SNP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL Handshake (Stall) The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit or Global OUT NAK is Set along with this bit, the STALL bit takes priority. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
    #[inline(always)]
    pub fn cnak(&self) -> CNAK_R {
        CNAK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set bit on a Transfer Completed interrupt, or after a SETUP is received on the endpoint."]
    #[inline(always)]
    pub fn snak(&self) -> SNAK_R {
        SNAK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint Disable (EPDis) The application cannot disable control OUT endpoint 0."]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable (EPEna) - When Scatter/Gather DMA mode is enabled, for OUT endpoints this bit indicates that the descriptor structure and data buffer to receive data is setup. - When Scatter/Gather DMA mode is disabled (such as for buffer-pointer based DMA mode)this bit indicates that the application has allocated the memory to start receiving data from the USB. - The core clears this bit before setting any of the following interrupts on this endpoint: -- SETUP Phase Done -- Endpoint Disabled -- Transfer Completed Note: In DMA mode, this bit must be set for the core to transfer SETUP data packets into memory and it is not cleared on Transfer Completed interrupt of SETUP packet."]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL0")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("cnak", &self.cnak())
            .field("snak", &self.snak())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 20 - RESERVED"]
    #[inline(always)]
    pub fn snp(&mut self) -> SNP_W<'_, DOEPCTL0_SPEC> {
        SNP_W::new(self, 20)
    }
    #[doc = "Bit 21 - STALL Handshake (Stall) The application can only set this bit, and the core clears it, when a SETUP token is received for this endpoint. If a NAK bit or Global OUT NAK is Set along with this bit, the STALL bit takes priority. Irrespective of this bit's setting, the core always responds to SETUP data packets with an ACK handshake."]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, DOEPCTL0_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK (CNAK) A write to this bit clears the NAK bit for the endpoint."]
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<'_, DOEPCTL0_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK (SNAK) A write to this bit sets the NAK bit for the endpoint. Using this bit, the application can control the transmission of NAK handshakes on an endpoint. The core can also set bit on a Transfer Completed interrupt, or after a SETUP is received on the endpoint."]
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<'_, DOEPCTL0_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 31 - Endpoint Enable (EPEna) - When Scatter/Gather DMA mode is enabled, for OUT endpoints this bit indicates that the descriptor structure and data buffer to receive data is setup. - When Scatter/Gather DMA mode is disabled (such as for buffer-pointer based DMA mode)this bit indicates that the application has allocated the memory to start receiving data from the USB. - The core clears this bit before setting any of the following interrupts on this endpoint: -- SETUP Phase Done -- Endpoint Disabled -- Transfer Completed Note: In DMA mode, this bit must be set for the core to transfer SETUP data packets into memory and it is not cleared on Transfer Completed interrupt of SETUP packet."]
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<'_, DOEPCTL0_SPEC> {
        EPENA_W::new(self, 31)
    }
}
#[doc = "This register is used to control the characteristics of the OUT Endpoint 0 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPCTL0_SPEC;
impl crate::RegisterSpec for DOEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl0::R`](R) reader structure"]
impl crate::Readable for DOEPCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepctl0::W`](W) writer structure"]
impl crate::Writable for DOEPCTL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPCTL0 to value 0x8000"]
impl crate::Resettable for DOEPCTL0_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
