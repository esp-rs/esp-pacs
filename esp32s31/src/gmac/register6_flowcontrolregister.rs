#[doc = "Register `REGISTER6_FLOWCONTROLREGISTER` reader"]
pub type R = crate::R<REGISTER6_FLOWCONTROLREGISTER_SPEC>;
#[doc = "Register `REGISTER6_FLOWCONTROLREGISTER` writer"]
pub type W = crate::W<REGISTER6_FLOWCONTROLREGISTER_SPEC>;
#[doc = "Field `FCB_BPA` reader - Flow Control Busy or Backpressure Activate This bit initiates a Pause frame in the fullduplex mode and activates the backpressure function in the halfduplex mode if the TFE bit is set In the fullduplex mode, this bit should be read as 1'b0 before writing to the Flow Control register To initiate a Pause frame, the Application must set this bit to 1'b1 During a transfer of the Control Frame, this bit continues to be set to signify that a frame transmission is in progress After the completion of Pause frame transmission, the MAC resets this bit to 1'b0 The Flow Control register should not be written to until this bit is cleared In the halfduplex mode, when this bit is set _and TFE is set_, then backpressure is asserted by the MAC During backpressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision This control register bit is logically ORed with the mti_flowctrl_i input signal for the backpressure function When the MAC is configured for the fullduplex mode, the BPA is automatically disabled"]
pub type FCB_BPA_R = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - Flow Control Busy or Backpressure Activate This bit initiates a Pause frame in the fullduplex mode and activates the backpressure function in the halfduplex mode if the TFE bit is set In the fullduplex mode, this bit should be read as 1'b0 before writing to the Flow Control register To initiate a Pause frame, the Application must set this bit to 1'b1 During a transfer of the Control Frame, this bit continues to be set to signify that a frame transmission is in progress After the completion of Pause frame transmission, the MAC resets this bit to 1'b0 The Flow Control register should not be written to until this bit is cleared In the halfduplex mode, when this bit is set _and TFE is set_, then backpressure is asserted by the MAC During backpressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision This control register bit is logically ORed with the mti_flowctrl_i input signal for the backpressure function When the MAC is configured for the fullduplex mode, the BPA is automatically disabled"]
pub type FCB_BPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable In the fullduplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames In the halfduplex mode, when this bit is set, the MAC enables the backpressure operation When this bit is reset, the backpressure feature is disabled"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable In the fullduplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames In the halfduplex mode, when this bit is set, the MAC enables the backpressure operation When this bit is reset, the backpressure feature is disabled"]
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_FLOW_CTRL_E` reader - Receive Flow Control Enable When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified _Pause_ time When this bit is reset, the decode function of the Pause frame is disabled"]
pub type RECEIVE_FLOW_CTRL_E_R = crate::BitReader;
#[doc = "Field `RECEIVE_FLOW_CTRL_E` writer - Receive Flow Control Enable When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified _Pause_ time When this bit is reset, the decode function of the Pause frame is disabled"]
pub type RECEIVE_FLOW_CTRL_E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP` reader - Unicast Pause Frame Detect A pause frame is processed when it has the unique multicast address specified in the IEEE Std 8023 When this bit is set, the MAC can also detect Pause frames with unicast address of the station This unicast address should be as specified in the MAC Address0 High Register and MAC Address0 Low Register When this bit is reset, the MAC only detects Pause frames with unique multicast address Note: The MAC does not process a Pause frame if the multicast address of received frame is different from the unique multicast address"]
pub type UP_R = crate::BitReader;
#[doc = "Field `UP` writer - Unicast Pause Frame Detect A pause frame is processed when it has the unique multicast address specified in the IEEE Std 8023 When this bit is set, the MAC can also detect Pause frames with unicast address of the station This unicast address should be as specified in the MAC Address0 High Register and MAC Address0 Low Register When this bit is reset, the MAC only detects Pause frames with unique multicast address Note: The MAC does not process a Pause frame if the multicast address of received frame is different from the unique multicast address"]
pub type UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow control signal mti_flowctrl_i _or sbd_flowctrl_i_ is checked for automatic retransmission of the Pause frame The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\] For example, if PT = 100H _256 slottimes_, and PLT = 01, then a second Pause frame is automatically transmitted if the mti_flowctrl_i signal is asserted at 228 _256 28_ slot times after the first Pause frame is transmitted The following list provides the threshold values for different values: 00: The threshold is Pause time minus 4 slot times _PT 4 slot times_ 01: The threshold is Pause time minus 28 slot times _PT 28 slot times_ 10: The threshold is Pause time minus 144 slot times _PT 144 slot times_ 11: The threshold is Pause time minus 256 slot times _PT 256 slot times_ The slot time is defined as the time taken to transmit 512 bits _64 bytes_ on the GMII or MII interface"]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow control signal mti_flowctrl_i _or sbd_flowctrl_i_ is checked for automatic retransmission of the Pause frame The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\] For example, if PT = 100H _256 slottimes_, and PLT = 01, then a second Pause frame is automatically transmitted if the mti_flowctrl_i signal is asserted at 228 _256 28_ slot times after the first Pause frame is transmitted The following list provides the threshold values for different values: 00: The threshold is Pause time minus 4 slot times _PT 4 slot times_ 01: The threshold is Pause time minus 28 slot times _PT 28 slot times_ 10: The threshold is Pause time minus 144 slot times _PT 144 slot times_ 11: The threshold is Pause time minus 256 slot times _PT 256 slot times_ The slot time is defined as the time taken to transmit 512 bits _64 bytes_ on the GMII or MII interface"]
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DZPQ` reader - Disable ZeroQuanta Pause When this bit is set, it disables the automatic generation of the ZeroQuanta Pause frames on the deassertion of the flowcontrol signal from the FIFO layer _MTL or external sideband flow control signal sbd_flowctrl_i/mti_flowctrl_i_ When this bit is reset, normal operation with automatic ZeroQuanta Pause frame generation is enabled"]
pub type DZPQ_R = crate::BitReader;
#[doc = "Field `DZPQ` writer - Disable ZeroQuanta Pause When this bit is set, it disables the automatic generation of the ZeroQuanta Pause frames on the deassertion of the flowcontrol signal from the FIFO layer _MTL or external sideband flow control signal sbd_flowctrl_i/mti_flowctrl_i_ When this bit is reset, normal operation with automatic ZeroQuanta Pause frame generation is enabled"]
pub type DZPQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - Pause Time This field holds the value to be used in the Pause Time field in the transmit control frame If the Pause Time bits is configured to be doublesynchronized to the _G_MII clock domain, then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pause Time This field holds the value to be used in the Pause Time field in the transmit control frame If the Pause Time bits is configured to be doublesynchronized to the _G_MII clock domain, then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain"]
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate This bit initiates a Pause frame in the fullduplex mode and activates the backpressure function in the halfduplex mode if the TFE bit is set In the fullduplex mode, this bit should be read as 1'b0 before writing to the Flow Control register To initiate a Pause frame, the Application must set this bit to 1'b1 During a transfer of the Control Frame, this bit continues to be set to signify that a frame transmission is in progress After the completion of Pause frame transmission, the MAC resets this bit to 1'b0 The Flow Control register should not be written to until this bit is cleared In the halfduplex mode, when this bit is set _and TFE is set_, then backpressure is asserted by the MAC During backpressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision This control register bit is logically ORed with the mti_flowctrl_i input signal for the backpressure function When the MAC is configured for the fullduplex mode, the BPA is automatically disabled"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable In the fullduplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames In the halfduplex mode, when this bit is set, the MAC enables the backpressure operation When this bit is reset, the backpressure feature is disabled"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified _Pause_ time When this bit is reset, the decode function of the Pause frame is disabled"]
    #[inline(always)]
    pub fn receive_flow_ctrl_e(&self) -> RECEIVE_FLOW_CTRL_E_R {
        RECEIVE_FLOW_CTRL_E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect A pause frame is processed when it has the unique multicast address specified in the IEEE Std 8023 When this bit is set, the MAC can also detect Pause frames with unicast address of the station This unicast address should be as specified in the MAC Address0 High Register and MAC Address0 Low Register When this bit is reset, the MAC only detects Pause frames with unique multicast address Note: The MAC does not process a Pause frame if the multicast address of received frame is different from the unique multicast address"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow control signal mti_flowctrl_i _or sbd_flowctrl_i_ is checked for automatic retransmission of the Pause frame The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\] For example, if PT = 100H _256 slottimes_, and PLT = 01, then a second Pause frame is automatically transmitted if the mti_flowctrl_i signal is asserted at 228 _256 28_ slot times after the first Pause frame is transmitted The following list provides the threshold values for different values: 00: The threshold is Pause time minus 4 slot times _PT 4 slot times_ 01: The threshold is Pause time minus 28 slot times _PT 28 slot times_ 10: The threshold is Pause time minus 144 slot times _PT 144 slot times_ 11: The threshold is Pause time minus 256 slot times _PT 256 slot times_ The slot time is defined as the time taken to transmit 512 bits _64 bytes_ on the GMII or MII interface"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable ZeroQuanta Pause When this bit is set, it disables the automatic generation of the ZeroQuanta Pause frames on the deassertion of the flowcontrol signal from the FIFO layer _MTL or external sideband flow control signal sbd_flowctrl_i/mti_flowctrl_i_ When this bit is reset, normal operation with automatic ZeroQuanta Pause frame generation is enabled"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time This field holds the value to be used in the Pause Time field in the transmit control frame If the Pause Time bits is configured to be doublesynchronized to the _G_MII clock domain, then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER6_FLOWCONTROLREGISTER")
            .field("fcb_bpa", &self.fcb_bpa())
            .field("tfe", &self.tfe())
            .field("receive_flow_ctrl_e", &self.receive_flow_ctrl_e())
            .field("up", &self.up())
            .field("plt", &self.plt())
            .field("dzpq", &self.dzpq())
            .field("pt", &self.pt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate This bit initiates a Pause frame in the fullduplex mode and activates the backpressure function in the halfduplex mode if the TFE bit is set In the fullduplex mode, this bit should be read as 1'b0 before writing to the Flow Control register To initiate a Pause frame, the Application must set this bit to 1'b1 During a transfer of the Control Frame, this bit continues to be set to signify that a frame transmission is in progress After the completion of Pause frame transmission, the MAC resets this bit to 1'b0 The Flow Control register should not be written to until this bit is cleared In the halfduplex mode, when this bit is set _and TFE is set_, then backpressure is asserted by the MAC During backpressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision This control register bit is logically ORed with the mti_flowctrl_i input signal for the backpressure function When the MAC is configured for the fullduplex mode, the BPA is automatically disabled"]
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<'_, REGISTER6_FLOWCONTROLREGISTER_SPEC> {
        FCB_BPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable In the fullduplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames In the halfduplex mode, when this bit is set, the MAC enables the backpressure operation When this bit is reset, the backpressure feature is disabled"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<'_, REGISTER6_FLOWCONTROLREGISTER_SPEC> {
        TFE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified _Pause_ time When this bit is reset, the decode function of the Pause frame is disabled"]
    #[inline(always)]
    pub fn receive_flow_ctrl_e(
        &mut self,
    ) -> RECEIVE_FLOW_CTRL_E_W<'_, REGISTER6_FLOWCONTROLREGISTER_SPEC> {
        RECEIVE_FLOW_CTRL_E_W::new(self, 2)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect A pause frame is processed when it has the unique multicast address specified in the IEEE Std 8023 When this bit is set, the MAC can also detect Pause frames with unicast address of the station This unicast address should be as specified in the MAC Address0 High Register and MAC Address0 Low Register When this bit is reset, the MAC only detects Pause frames with unique multicast address Note: The MAC does not process a Pause frame if the multicast address of received frame is different from the unique multicast address"]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W<'_, REGISTER6_FLOWCONTROLREGISTER_SPEC> {
        UP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow control signal mti_flowctrl_i _or sbd_flowctrl_i_ is checked for automatic retransmission of the Pause frame The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\] For example, if PT = 100H _256 slottimes_, and PLT = 01, then a second Pause frame is automatically transmitted if the mti_flowctrl_i signal is asserted at 228 _256 28_ slot times after the first Pause frame is transmitted The following list provides the threshold values for different values: 00: The threshold is Pause time minus 4 slot times _PT 4 slot times_ 01: The threshold is Pause time minus 28 slot times _PT 28 slot times_ 10: The threshold is Pause time minus 144 slot times _PT 144 slot times_ 11: The threshold is Pause time minus 256 slot times _PT 256 slot times_ The slot time is defined as the time taken to transmit 512 bits _64 bytes_ on the GMII or MII interface"]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<'_, REGISTER6_FLOWCONTROLREGISTER_SPEC> {
        PLT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Disable ZeroQuanta Pause When this bit is set, it disables the automatic generation of the ZeroQuanta Pause frames on the deassertion of the flowcontrol signal from the FIFO layer _MTL or external sideband flow control signal sbd_flowctrl_i/mti_flowctrl_i_ When this bit is reset, normal operation with automatic ZeroQuanta Pause frame generation is enabled"]
    #[inline(always)]
    pub fn dzpq(&mut self) -> DZPQ_W<'_, REGISTER6_FLOWCONTROLREGISTER_SPEC> {
        DZPQ_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pause Time This field holds the value to be used in the Pause Time field in the transmit control frame If the Pause Time bits is configured to be doublesynchronized to the _G_MII clock domain, then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<'_, REGISTER6_FLOWCONTROLREGISTER_SPEC> {
        PT_W::new(self, 16)
    }
}
#[doc = "Controls the generation of control frames\n\nYou can [`read`](crate::Reg::read) this register and get [`register6_flowcontrolregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register6_flowcontrolregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER6_FLOWCONTROLREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER6_FLOWCONTROLREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register6_flowcontrolregister::R`](R) reader structure"]
impl crate::Readable for REGISTER6_FLOWCONTROLREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register6_flowcontrolregister::W`](W) writer structure"]
impl crate::Writable for REGISTER6_FLOWCONTROLREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER6_FLOWCONTROLREGISTER to value 0"]
impl crate::Resettable for REGISTER6_FLOWCONTROLREGISTER_SPEC {}
