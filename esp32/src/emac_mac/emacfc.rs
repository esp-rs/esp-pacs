#[doc = "Register `EMACFC` reader"]
pub type R = crate::R<EMACFC_SPEC>;
#[doc = "Register `EMACFC` writer"]
pub type W = crate::W<EMACFC_SPEC>;
#[doc = "Field `FCBBA` reader - This bit initiates a Pause frame in the full-duplex mode and activates the backpressure function in the half-duplex mode if the TFCE bit is set. In the full-duplex mode this bit should be read as 1'b0 before writing to the Flow Control register. To initiate a Pause frame the Application must set this bit to 1'b1. During a transfer of the Control Frame this bit continues to be set to signify that a frame transmission is in progress. After the completion of Pause frame transmission the MAC resets this bit to 1'b0. The Flow Control register should not be written to until this bit is cleared. In the half-duplex mode when this bit is set (and TFCE is set) then backpressure is asserted by the MAC. During backpressure when the MAC receives a new frame the transmitter starts sending a JAM pattern resulting in a collision. When the MAC is configured for the full-duplex mode the BPA(backpressure activate) is automatically disabled."]
pub type FCBBA_R = crate::BitReader;
#[doc = "Field `FCBBA` writer - This bit initiates a Pause frame in the full-duplex mode and activates the backpressure function in the half-duplex mode if the TFCE bit is set. In the full-duplex mode this bit should be read as 1'b0 before writing to the Flow Control register. To initiate a Pause frame the Application must set this bit to 1'b1. During a transfer of the Control Frame this bit continues to be set to signify that a frame transmission is in progress. After the completion of Pause frame transmission the MAC resets this bit to 1'b0. The Flow Control register should not be written to until this bit is cleared. In the half-duplex mode when this bit is set (and TFCE is set) then backpressure is asserted by the MAC. During backpressure when the MAC receives a new frame the transmitter starts sending a JAM pattern resulting in a collision. When the MAC is configured for the full-duplex mode the BPA(backpressure activate) is automatically disabled."]
pub type FCBBA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFCE` reader - In the full-duplex mode when this bit is set the MAC enables the flow control operation to transmit Pause frames. When this bit is reset the flow control operation in the MAC is disabled and the MAC does not transmit any Pause frames. In the half-duplex mode when this bit is set the MAC enables the backpressure operation. When this bit is reset the backpressure feature is Disabled."]
pub type TFCE_R = crate::BitReader;
#[doc = "Field `TFCE` writer - In the full-duplex mode when this bit is set the MAC enables the flow control operation to transmit Pause frames. When this bit is reset the flow control operation in the MAC is disabled and the MAC does not transmit any Pause frames. In the half-duplex mode when this bit is set the MAC enables the backpressure operation. When this bit is reset the backpressure feature is Disabled."]
pub type TFCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFCE` reader - When this bit is set the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause) time. When this bit is reset the decode function of the Pause frame is disabled."]
pub type RFCE_R = crate::BitReader;
#[doc = "Field `RFCE` writer - When this bit is set the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause) time. When this bit is reset the decode function of the Pause frame is disabled."]
pub type RFCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UPFD` reader - A pause frame is processed when it has the unique multicast address specified in the IEEE Std 802.3. When this bit is set the MAC can also detect Pause frames with unicast address of the station. This unicast address should be as specified in the EMACADDR0 High Register and EMACADDR0 Low Register. When this bit is reset the MAC only detects Pause frames with unique multicast address."]
pub type UPFD_R = crate::BitReader;
#[doc = "Field `UPFD` writer - A pause frame is processed when it has the unique multicast address specified in the IEEE Std 802.3. When this bit is set the MAC can also detect Pause frames with unicast address of the station. This unicast address should be as specified in the EMACADDR0 High Register and EMACADDR0 Low Register. When this bit is reset the MAC only detects Pause frames with unique multicast address."]
pub type UPFD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLT` reader - This field configures the threshold of the Pause timer automatic retransmission of the Pause frame.The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example if PT = 100H (256 slot-times) and PLT = 01 then a second Pause frame is automatically transmitted at 228 (256-28) slot times after the first Pause frame is transmitted. The following list provides the threshold values for different values: 2'b00: The threshold is Pause time minus 4 slot times (PT-4 slot times). 2'b01: The threshold is Pause time minus 28 slot times (PT-28 slot times). 2'b10: The threshold is Pause time minus 144 slot times (PT-144 slot times). 2'b11: The threshold is Pause time minus 256 slot times (PT-256 slot times). The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the MII interface."]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - This field configures the threshold of the Pause timer automatic retransmission of the Pause frame.The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example if PT = 100H (256 slot-times) and PLT = 01 then a second Pause frame is automatically transmitted at 228 (256-28) slot times after the first Pause frame is transmitted. The following list provides the threshold values for different values: 2'b00: The threshold is Pause time minus 4 slot times (PT-4 slot times). 2'b01: The threshold is Pause time minus 28 slot times (PT-28 slot times). 2'b10: The threshold is Pause time minus 144 slot times (PT-144 slot times). 2'b11: The threshold is Pause time minus 256 slot times (PT-256 slot times). The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the MII interface."]
pub type PLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DZPQ` reader - When this bit is set it disables the automatic generation of the Zero-Quanta Pause frames on the de-assertion of the flow-control signal from the FIFO layer. When this bit is reset normal operation with automatic Zero-Quanta Pause frame generation is enabled."]
pub type DZPQ_R = crate::BitReader;
#[doc = "Field `DZPQ` writer - When this bit is set it disables the automatic generation of the Zero-Quanta Pause frames on the de-assertion of the flow-control signal from the FIFO layer. When this bit is reset normal operation with automatic Zero-Quanta Pause frame generation is enabled."]
pub type DZPQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAUSE_TIME` reader - This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the MII clock domain then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain."]
pub type PAUSE_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `PAUSE_TIME` writer - This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the MII clock domain then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain."]
pub type PAUSE_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - This bit initiates a Pause frame in the full-duplex mode and activates the backpressure function in the half-duplex mode if the TFCE bit is set. In the full-duplex mode this bit should be read as 1'b0 before writing to the Flow Control register. To initiate a Pause frame the Application must set this bit to 1'b1. During a transfer of the Control Frame this bit continues to be set to signify that a frame transmission is in progress. After the completion of Pause frame transmission the MAC resets this bit to 1'b0. The Flow Control register should not be written to until this bit is cleared. In the half-duplex mode when this bit is set (and TFCE is set) then backpressure is asserted by the MAC. During backpressure when the MAC receives a new frame the transmitter starts sending a JAM pattern resulting in a collision. When the MAC is configured for the full-duplex mode the BPA(backpressure activate) is automatically disabled."]
    #[inline(always)]
    pub fn fcbba(&self) -> FCBBA_R {
        FCBBA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - In the full-duplex mode when this bit is set the MAC enables the flow control operation to transmit Pause frames. When this bit is reset the flow control operation in the MAC is disabled and the MAC does not transmit any Pause frames. In the half-duplex mode when this bit is set the MAC enables the backpressure operation. When this bit is reset the backpressure feature is Disabled."]
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is set the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause) time. When this bit is reset the decode function of the Pause frame is disabled."]
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A pause frame is processed when it has the unique multicast address specified in the IEEE Std 802.3. When this bit is set the MAC can also detect Pause frames with unicast address of the station. This unicast address should be as specified in the EMACADDR0 High Register and EMACADDR0 Low Register. When this bit is reset the MAC only detects Pause frames with unique multicast address."]
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - This field configures the threshold of the Pause timer automatic retransmission of the Pause frame.The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example if PT = 100H (256 slot-times) and PLT = 01 then a second Pause frame is automatically transmitted at 228 (256-28) slot times after the first Pause frame is transmitted. The following list provides the threshold values for different values: 2'b00: The threshold is Pause time minus 4 slot times (PT-4 slot times). 2'b01: The threshold is Pause time minus 28 slot times (PT-28 slot times). 2'b10: The threshold is Pause time minus 144 slot times (PT-144 slot times). 2'b11: The threshold is Pause time minus 256 slot times (PT-256 slot times). The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the MII interface."]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - When this bit is set it disables the automatic generation of the Zero-Quanta Pause frames on the de-assertion of the flow-control signal from the FIFO layer. When this bit is reset normal operation with automatic Zero-Quanta Pause frame generation is enabled."]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the MII clock domain then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain."]
    #[inline(always)]
    pub fn pause_time(&self) -> PAUSE_TIME_R {
        PAUSE_TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACFC")
            .field("fcbba", &format_args!("{}", self.fcbba().bit()))
            .field("tfce", &format_args!("{}", self.tfce().bit()))
            .field("rfce", &format_args!("{}", self.rfce().bit()))
            .field("upfd", &format_args!("{}", self.upfd().bit()))
            .field("plt", &format_args!("{}", self.plt().bits()))
            .field("dzpq", &format_args!("{}", self.dzpq().bit()))
            .field("pause_time", &format_args!("{}", self.pause_time().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACFC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit initiates a Pause frame in the full-duplex mode and activates the backpressure function in the half-duplex mode if the TFCE bit is set. In the full-duplex mode this bit should be read as 1'b0 before writing to the Flow Control register. To initiate a Pause frame the Application must set this bit to 1'b1. During a transfer of the Control Frame this bit continues to be set to signify that a frame transmission is in progress. After the completion of Pause frame transmission the MAC resets this bit to 1'b0. The Flow Control register should not be written to until this bit is cleared. In the half-duplex mode when this bit is set (and TFCE is set) then backpressure is asserted by the MAC. During backpressure when the MAC receives a new frame the transmitter starts sending a JAM pattern resulting in a collision. When the MAC is configured for the full-duplex mode the BPA(backpressure activate) is automatically disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fcbba(&mut self) -> FCBBA_W<EMACFC_SPEC, 0> {
        FCBBA_W::new(self)
    }
    #[doc = "Bit 1 - In the full-duplex mode when this bit is set the MAC enables the flow control operation to transmit Pause frames. When this bit is reset the flow control operation in the MAC is disabled and the MAC does not transmit any Pause frames. In the half-duplex mode when this bit is set the MAC enables the backpressure operation. When this bit is reset the backpressure feature is Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tfce(&mut self) -> TFCE_W<EMACFC_SPEC, 1> {
        TFCE_W::new(self)
    }
    #[doc = "Bit 2 - When this bit is set the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause) time. When this bit is reset the decode function of the Pause frame is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rfce(&mut self) -> RFCE_W<EMACFC_SPEC, 2> {
        RFCE_W::new(self)
    }
    #[doc = "Bit 3 - A pause frame is processed when it has the unique multicast address specified in the IEEE Std 802.3. When this bit is set the MAC can also detect Pause frames with unicast address of the station. This unicast address should be as specified in the EMACADDR0 High Register and EMACADDR0 Low Register. When this bit is reset the MAC only detects Pause frames with unique multicast address."]
    #[inline(always)]
    #[must_use]
    pub fn upfd(&mut self) -> UPFD_W<EMACFC_SPEC, 3> {
        UPFD_W::new(self)
    }
    #[doc = "Bits 4:5 - This field configures the threshold of the Pause timer automatic retransmission of the Pause frame.The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example if PT = 100H (256 slot-times) and PLT = 01 then a second Pause frame is automatically transmitted at 228 (256-28) slot times after the first Pause frame is transmitted. The following list provides the threshold values for different values: 2'b00: The threshold is Pause time minus 4 slot times (PT-4 slot times). 2'b01: The threshold is Pause time minus 28 slot times (PT-28 slot times). 2'b10: The threshold is Pause time minus 144 slot times (PT-144 slot times). 2'b11: The threshold is Pause time minus 256 slot times (PT-256 slot times). The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the MII interface."]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<EMACFC_SPEC, 4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 7 - When this bit is set it disables the automatic generation of the Zero-Quanta Pause frames on the de-assertion of the flow-control signal from the FIFO layer. When this bit is reset normal operation with automatic Zero-Quanta Pause frame generation is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<EMACFC_SPEC, 7> {
        DZPQ_W::new(self)
    }
    #[doc = "Bits 16:31 - This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the MII clock domain then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn pause_time(&mut self) -> PAUSE_TIME_W<EMACFC_SPEC, 16> {
        PAUSE_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Frame flow control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACFC_SPEC;
impl crate::RegisterSpec for EMACFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacfc::R`](R) reader structure"]
impl crate::Readable for EMACFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacfc::W`](W) writer structure"]
impl crate::Writable for EMACFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMACFC to value 0"]
impl crate::Resettable for EMACFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
