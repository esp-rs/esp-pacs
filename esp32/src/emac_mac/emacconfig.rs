#[doc = "Register `EMACCONFIG` reader"]
pub type R = crate::R<EMACCONFIG_SPEC>;
#[doc = "Register `EMACCONFIG` writer"]
pub type W = crate::W<EMACCONFIG_SPEC>;
#[doc = "Field `PLTF` reader - These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.2'b00: 7 bytes of preamble. 2'b01: 5 bytes of preamble. 2'b10: 3 bytes of preamble."]
pub type PLTF_R = crate::FieldReader;
#[doc = "Field `PLTF` writer - These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.2'b00: 7 bytes of preamble. 2'b01: 5 bytes of preamble. 2'b10: 3 bytes of preamble."]
pub type PLTF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `RX` reader - When this bit is set the receiver state machine of the MAC is enabled for receiving frames from the MII. When this bit is reset the MAC receive state machine is disabled after the completion of the reception of the current frame and does not receive any further frames from the MII."]
pub type RX_R = crate::BitReader;
#[doc = "Field `RX` writer - When this bit is set the receiver state machine of the MAC is enabled for receiving frames from the MII. When this bit is reset the MAC receive state machine is disabled after the completion of the reception of the current frame and does not receive any further frames from the MII."]
pub type RX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX` reader - When this bit is set the transmit state machine of the MAC is enabled for transmission on the MII. When this bit is reset the MAC transmit state machine is disabled after the completion of the transmission of the current frame and does not transmit any further frames."]
pub type TX_R = crate::BitReader;
#[doc = "Field `TX` writer - When this bit is set the transmit state machine of the MAC is enabled for transmission on the MII. When this bit is reset the MAC transmit state machine is disabled after the completion of the transmission of the current frame and does not transmit any further frames."]
pub type TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEFERRALCHECK` reader - Deferral Check."]
pub type DEFERRALCHECK_R = crate::BitReader;
#[doc = "Field `DEFERRALCHECK` writer - Deferral Check."]
pub type DEFERRALCHECK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BACKOFFLIMIT` reader - The Back-Off limit determines the random integer number (r) of slot time delays (512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode. 00: k= min (n 10). 01: k = min (n 8). 10: k = min (n 4). 11: k = min (n 1) n = retransmission attempt. The random integer r takes the value in the Range 0 ~ 2000."]
pub type BACKOFFLIMIT_R = crate::FieldReader;
#[doc = "Field `BACKOFFLIMIT` writer - The Back-Off limit determines the random integer number (r) of slot time delays (512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode. 00: k= min (n 10). 01: k = min (n 8). 10: k = min (n 4). 11: k = min (n 1) n = retransmission attempt. The random integer r takes the value in the Range 0 ~ 2000."]
pub type BACKOFFLIMIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PADCRCSTRIP` reader - When this bit is set the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1 536 bytes. All received frames with length field greater than or equal to 1 536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset the MAC passes all incoming frames without modifying them to the Host."]
pub type PADCRCSTRIP_R = crate::BitReader;
#[doc = "Field `PADCRCSTRIP` writer - When this bit is set the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1 536 bytes. All received frames with length field greater than or equal to 1 536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset the MAC passes all incoming frames without modifying them to the Host."]
pub type PADCRCSTRIP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RETRY` reader - When this bit is set the MAC attempts only one transmission. When a collision occurs on the MII interface the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\]). This bit is applicable only in the half-duplex Mode."]
pub type RETRY_R = crate::BitReader;
#[doc = "Field `RETRY` writer - When this bit is set the MAC attempts only one transmission. When a collision occurs on the MII interface the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\]). This bit is applicable only in the half-duplex Mode."]
pub type RETRY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXIPCOFFLOAD` reader - When this bit is set the MAC calculates the 16-bit one's complement of the one's complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25/26 or 29/30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset this function is disabled."]
pub type RXIPCOFFLOAD_R = crate::BitReader;
#[doc = "Field `RXIPCOFFLOAD` writer - When this bit is set the MAC calculates the 16-bit one's complement of the one's complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25/26 or 29/30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset this function is disabled."]
pub type RXIPCOFFLOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUPLEX` reader - When this bit is set the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is read only with default value of 1'b1 in the full-duplex-mode."]
pub type DUPLEX_R = crate::BitReader;
#[doc = "Field `DUPLEX` writer - When this bit is set the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is read only with default value of 1'b1 in the full-duplex-mode."]
pub type DUPLEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOOPBACK` reader - When this bit is set the MAC operates in the loopback mode MII. The MII Receive clock input (CLK_RX) is required for the loopback to work properly because the transmit clock is not looped-back internally."]
pub type LOOPBACK_R = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - When this bit is set the MAC operates in the loopback mode MII. The MII Receive clock input (CLK_RX) is required for the loopback to work properly because the transmit clock is not looped-back internally."]
pub type LOOPBACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOWN` reader - When this bit is set the MAC disables the reception of frames when the TX_EN is asserted in the half-duplex mode. When this bit is reset the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full duplex mode."]
pub type RXOWN_R = crate::BitReader;
#[doc = "Field `RXOWN` writer - When this bit is set the MAC disables the reception of frames when the TX_EN is asserted in the half-duplex mode. When this bit is reset the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full duplex mode."]
pub type RXOWN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FESPEED` reader - This bit selects the speed in the MII RMII interface. 0: 10 Mbps. 1: 100 Mbps."]
pub type FESPEED_R = crate::BitReader;
#[doc = "Field `FESPEED` writer - This bit selects the speed in the MII RMII interface. 0: 10 Mbps. 1: 100 Mbps."]
pub type FESPEED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MII` reader - This bit selects the Ethernet line speed. It should be set to 1 for 10 or 100 Mbps operations.In 10 or 100 Mbps operations this bit along with FES(EMACFESPEED) bit it selects the exact linespeed. In the 10/100 Mbps-only operations the bit is always 1."]
pub type MII_R = crate::BitReader;
#[doc = "Field `MII` writer - This bit selects the Ethernet line speed. It should be set to 1 for 10 or 100 Mbps operations.In 10 or 100 Mbps operations this bit along with FES(EMACFESPEED) bit it selects the exact linespeed. In the 10/100 Mbps-only operations the bit is always 1."]
pub type MII_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISABLECRS` reader - When set high this bit makes the MAC transmitter ignore the MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
pub type DISABLECRS_R = crate::BitReader;
#[doc = "Field `DISABLECRS` writer - When set high this bit makes the MAC transmitter ignore the MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
pub type DISABLECRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTERFRAMEGAP` reader - These bits control the minimum IFG between frames during transmission. 3'b000: 96 bit times. 3'b001: 88 bit times. 3'b010: 80 bit times. 3'b111: 40 bit times. In the half-duplex mode the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered."]
pub type INTERFRAMEGAP_R = crate::FieldReader;
#[doc = "Field `INTERFRAMEGAP` writer - These bits control the minimum IFG between frames during transmission. 3'b000: 96 bit times. 3'b001: 88 bit times. 3'b010: 80 bit times. 3'b111: 40 bit times. In the half-duplex mode the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered."]
pub type INTERFRAMEGAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `JUMBOFRAME` reader - When this bit is set the MAC allows Jumbo frames of 9 018 bytes (9 022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
pub type JUMBOFRAME_R = crate::BitReader;
#[doc = "Field `JUMBOFRAME` writer - When this bit is set the MAC allows Jumbo frames of 9 018 bytes (9 022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
pub type JUMBOFRAME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JABBER` reader - When this bit is set the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16 383 bytes. When this bit is reset the MAC cuts off the transmitter if the application sends out more than 2 048 bytes of data (10 240 if JE is set high) during Transmission."]
pub type JABBER_R = crate::BitReader;
#[doc = "Field `JABBER` writer - When this bit is set the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16 383 bytes. When this bit is reset the MAC cuts off the transmitter if the application sends out more than 2 048 bytes of data (10 240 if JE is set high) during Transmission."]
pub type JABBER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WATCHDOG` reader - When this bit is set the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16 383 bytes. When this bit is reset the MAC does not allow a receive frame which more than 2 048 bytes (10 240 if JE is set high) or the value programmed in Register (Watchdog Timeout Register). The MAC cuts off any bytes received after the watchdog limit number of bytes."]
pub type WATCHDOG_R = crate::BitReader;
#[doc = "Field `WATCHDOG` writer - When this bit is set the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16 383 bytes. When this bit is reset the MAC does not allow a receive frame which more than 2 048 bytes (10 240 if JE is set high) or the value programmed in Register (Watchdog Timeout Register). The MAC cuts off any bytes received after the watchdog limit number of bytes."]
pub type WATCHDOG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASS2KP` reader - When set the MAC considers all frames with up to 2 000 bytes length as normal packets.When Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 1 518 bytes (1 522 bytes for tagged) as Giant frames. When Bit\\[20\\] is set setting this bit has no effect on Giant Frame status."]
pub type ASS2KP_R = crate::BitReader;
#[doc = "Field `ASS2KP` writer - When set the MAC considers all frames with up to 2 000 bytes length as normal packets.When Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 1 518 bytes (1 522 bytes for tagged) as Giant frames. When Bit\\[20\\] is set setting this bit has no effect on Giant Frame status."]
pub type ASS2KP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAIRC` reader - This field controls the source address insertion or replacement for all transmitted frames.Bit\\[30\\] specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. 2'b10: If Bit\\[30\\] is set to 0 the MAC inserts the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC inserts the content of the MAC Address 1 registers in the SA field of all transmitted frames. 2'b11: If Bit\\[30\\] is set to 0 the MAC replaces the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC replaces the content of the MAC Address 1 registers in the SA field of all transmitted frames."]
pub type SAIRC_R = crate::FieldReader;
#[doc = "Field `SAIRC` writer - This field controls the source address insertion or replacement for all transmitted frames.Bit\\[30\\] specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. 2'b10: If Bit\\[30\\] is set to 0 the MAC inserts the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC inserts the content of the MAC Address 1 registers in the SA field of all transmitted frames. 2'b11: If Bit\\[30\\] is set to 0 the MAC replaces the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC replaces the content of the MAC Address 1 registers in the SA field of all transmitted frames."]
pub type SAIRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:1 - These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.2'b00: 7 bytes of preamble. 2'b01: 5 bytes of preamble. 2'b10: 3 bytes of preamble."]
    #[inline(always)]
    pub fn pltf(&self) -> PLTF_R {
        PLTF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - When this bit is set the receiver state machine of the MAC is enabled for receiving frames from the MII. When this bit is reset the MAC receive state machine is disabled after the completion of the reception of the current frame and does not receive any further frames from the MII."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When this bit is set the transmit state machine of the MAC is enabled for transmission on the MII. When this bit is reset the MAC transmit state machine is disabled after the completion of the transmission of the current frame and does not transmit any further frames."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral Check."]
    #[inline(always)]
    pub fn deferralcheck(&self) -> DEFERRALCHECK_R {
        DEFERRALCHECK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - The Back-Off limit determines the random integer number (r) of slot time delays (512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode. 00: k= min (n 10). 01: k = min (n 8). 10: k = min (n 4). 11: k = min (n 1) n = retransmission attempt. The random integer r takes the value in the Range 0 ~ 2000."]
    #[inline(always)]
    pub fn backofflimit(&self) -> BACKOFFLIMIT_R {
        BACKOFFLIMIT_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - When this bit is set the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1 536 bytes. All received frames with length field greater than or equal to 1 536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset the MAC passes all incoming frames without modifying them to the Host."]
    #[inline(always)]
    pub fn padcrcstrip(&self) -> PADCRCSTRIP_R {
        PADCRCSTRIP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - When this bit is set the MAC attempts only one transmission. When a collision occurs on the MII interface the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\]). This bit is applicable only in the half-duplex Mode."]
    #[inline(always)]
    pub fn retry(&self) -> RETRY_R {
        RETRY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When this bit is set the MAC calculates the 16-bit one's complement of the one's complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25/26 or 29/30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset this function is disabled."]
    #[inline(always)]
    pub fn rxipcoffload(&self) -> RXIPCOFFLOAD_R {
        RXIPCOFFLOAD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When this bit is set the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is read only with default value of 1'b1 in the full-duplex-mode."]
    #[inline(always)]
    pub fn duplex(&self) -> DUPLEX_R {
        DUPLEX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When this bit is set the MAC operates in the loopback mode MII. The MII Receive clock input (CLK_RX) is required for the loopback to work properly because the transmit clock is not looped-back internally."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When this bit is set the MAC disables the reception of frames when the TX_EN is asserted in the half-duplex mode. When this bit is reset the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full duplex mode."]
    #[inline(always)]
    pub fn rxown(&self) -> RXOWN_R {
        RXOWN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit selects the speed in the MII RMII interface. 0: 10 Mbps. 1: 100 Mbps."]
    #[inline(always)]
    pub fn fespeed(&self) -> FESPEED_R {
        FESPEED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit selects the Ethernet line speed. It should be set to 1 for 10 or 100 Mbps operations.In 10 or 100 Mbps operations this bit along with FES(EMACFESPEED) bit it selects the exact linespeed. In the 10/100 Mbps-only operations the bit is always 1."]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When set high this bit makes the MAC transmitter ignore the MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
    #[inline(always)]
    pub fn disablecrs(&self) -> DISABLECRS_R {
        DISABLECRS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - These bits control the minimum IFG between frames during transmission. 3'b000: 96 bit times. 3'b001: 88 bit times. 3'b010: 80 bit times. 3'b111: 40 bit times. In the half-duplex mode the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered."]
    #[inline(always)]
    pub fn interframegap(&self) -> INTERFRAMEGAP_R {
        INTERFRAMEGAP_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - When this bit is set the MAC allows Jumbo frames of 9 018 bytes (9 022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn jumboframe(&self) -> JUMBOFRAME_R {
        JUMBOFRAME_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - When this bit is set the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16 383 bytes. When this bit is reset the MAC cuts off the transmitter if the application sends out more than 2 048 bytes of data (10 240 if JE is set high) during Transmission."]
    #[inline(always)]
    pub fn jabber(&self) -> JABBER_R {
        JABBER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When this bit is set the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16 383 bytes. When this bit is reset the MAC does not allow a receive frame which more than 2 048 bytes (10 240 if JE is set high) or the value programmed in Register (Watchdog Timeout Register). The MAC cuts off any bytes received after the watchdog limit number of bytes."]
    #[inline(always)]
    pub fn watchdog(&self) -> WATCHDOG_R {
        WATCHDOG_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - When set the MAC considers all frames with up to 2 000 bytes length as normal packets.When Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 1 518 bytes (1 522 bytes for tagged) as Giant frames. When Bit\\[20\\] is set setting this bit has no effect on Giant Frame status."]
    #[inline(always)]
    pub fn ass2kp(&self) -> ASS2KP_R {
        ASS2KP_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - This field controls the source address insertion or replacement for all transmitted frames.Bit\\[30\\] specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. 2'b10: If Bit\\[30\\] is set to 0 the MAC inserts the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC inserts the content of the MAC Address 1 registers in the SA field of all transmitted frames. 2'b11: If Bit\\[30\\] is set to 0 the MAC replaces the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC replaces the content of the MAC Address 1 registers in the SA field of all transmitted frames."]
    #[inline(always)]
    pub fn sairc(&self) -> SAIRC_R {
        SAIRC_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACCONFIG")
            .field("pltf", &format_args!("{}", self.pltf().bits()))
            .field("rx", &format_args!("{}", self.rx().bit()))
            .field("tx", &format_args!("{}", self.tx().bit()))
            .field(
                "deferralcheck",
                &format_args!("{}", self.deferralcheck().bit()),
            )
            .field(
                "backofflimit",
                &format_args!("{}", self.backofflimit().bits()),
            )
            .field("padcrcstrip", &format_args!("{}", self.padcrcstrip().bit()))
            .field("retry", &format_args!("{}", self.retry().bit()))
            .field(
                "rxipcoffload",
                &format_args!("{}", self.rxipcoffload().bit()),
            )
            .field("duplex", &format_args!("{}", self.duplex().bit()))
            .field("loopback", &format_args!("{}", self.loopback().bit()))
            .field("rxown", &format_args!("{}", self.rxown().bit()))
            .field("fespeed", &format_args!("{}", self.fespeed().bit()))
            .field("mii", &format_args!("{}", self.mii().bit()))
            .field("disablecrs", &format_args!("{}", self.disablecrs().bit()))
            .field(
                "interframegap",
                &format_args!("{}", self.interframegap().bits()),
            )
            .field("jumboframe", &format_args!("{}", self.jumboframe().bit()))
            .field("jabber", &format_args!("{}", self.jabber().bit()))
            .field("watchdog", &format_args!("{}", self.watchdog().bit()))
            .field("ass2kp", &format_args!("{}", self.ass2kp().bit()))
            .field("sairc", &format_args!("{}", self.sairc().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACCONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode.2'b00: 7 bytes of preamble. 2'b01: 5 bytes of preamble. 2'b10: 3 bytes of preamble."]
    #[inline(always)]
    #[must_use]
    pub fn pltf(&mut self) -> PLTF_W<EMACCONFIG_SPEC, 0> {
        PLTF_W::new(self)
    }
    #[doc = "Bit 2 - When this bit is set the receiver state machine of the MAC is enabled for receiving frames from the MII. When this bit is reset the MAC receive state machine is disabled after the completion of the reception of the current frame and does not receive any further frames from the MII."]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<EMACCONFIG_SPEC, 2> {
        RX_W::new(self)
    }
    #[doc = "Bit 3 - When this bit is set the transmit state machine of the MAC is enabled for transmission on the MII. When this bit is reset the MAC transmit state machine is disabled after the completion of the transmission of the current frame and does not transmit any further frames."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<EMACCONFIG_SPEC, 3> {
        TX_W::new(self)
    }
    #[doc = "Bit 4 - Deferral Check."]
    #[inline(always)]
    #[must_use]
    pub fn deferralcheck(&mut self) -> DEFERRALCHECK_W<EMACCONFIG_SPEC, 4> {
        DEFERRALCHECK_W::new(self)
    }
    #[doc = "Bits 5:6 - The Back-Off limit determines the random integer number (r) of slot time delays (512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode. 00: k= min (n 10). 01: k = min (n 8). 10: k = min (n 4). 11: k = min (n 1) n = retransmission attempt. The random integer r takes the value in the Range 0 ~ 2000."]
    #[inline(always)]
    #[must_use]
    pub fn backofflimit(&mut self) -> BACKOFFLIMIT_W<EMACCONFIG_SPEC, 5> {
        BACKOFFLIMIT_W::new(self)
    }
    #[doc = "Bit 7 - When this bit is set the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1 536 bytes. All received frames with length field greater than or equal to 1 536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset the MAC passes all incoming frames without modifying them to the Host."]
    #[inline(always)]
    #[must_use]
    pub fn padcrcstrip(&mut self) -> PADCRCSTRIP_W<EMACCONFIG_SPEC, 7> {
        PADCRCSTRIP_W::new(self)
    }
    #[doc = "Bit 9 - When this bit is set the MAC attempts only one transmission. When a collision occurs on the MII interface the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\]). This bit is applicable only in the half-duplex Mode."]
    #[inline(always)]
    #[must_use]
    pub fn retry(&mut self) -> RETRY_W<EMACCONFIG_SPEC, 9> {
        RETRY_W::new(self)
    }
    #[doc = "Bit 10 - When this bit is set the MAC calculates the 16-bit one's complement of the one's complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25/26 or 29/30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset this function is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxipcoffload(&mut self) -> RXIPCOFFLOAD_W<EMACCONFIG_SPEC, 10> {
        RXIPCOFFLOAD_W::new(self)
    }
    #[doc = "Bit 11 - When this bit is set the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is read only with default value of 1'b1 in the full-duplex-mode."]
    #[inline(always)]
    #[must_use]
    pub fn duplex(&mut self) -> DUPLEX_W<EMACCONFIG_SPEC, 11> {
        DUPLEX_W::new(self)
    }
    #[doc = "Bit 12 - When this bit is set the MAC operates in the loopback mode MII. The MII Receive clock input (CLK_RX) is required for the loopback to work properly because the transmit clock is not looped-back internally."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LOOPBACK_W<EMACCONFIG_SPEC, 12> {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bit 13 - When this bit is set the MAC disables the reception of frames when the TX_EN is asserted in the half-duplex mode. When this bit is reset the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn rxown(&mut self) -> RXOWN_W<EMACCONFIG_SPEC, 13> {
        RXOWN_W::new(self)
    }
    #[doc = "Bit 14 - This bit selects the speed in the MII RMII interface. 0: 10 Mbps. 1: 100 Mbps."]
    #[inline(always)]
    #[must_use]
    pub fn fespeed(&mut self) -> FESPEED_W<EMACCONFIG_SPEC, 14> {
        FESPEED_W::new(self)
    }
    #[doc = "Bit 15 - This bit selects the Ethernet line speed. It should be set to 1 for 10 or 100 Mbps operations.In 10 or 100 Mbps operations this bit along with FES(EMACFESPEED) bit it selects the exact linespeed. In the 10/100 Mbps-only operations the bit is always 1."]
    #[inline(always)]
    #[must_use]
    pub fn mii(&mut self) -> MII_W<EMACCONFIG_SPEC, 15> {
        MII_W::new(self)
    }
    #[doc = "Bit 16 - When set high this bit makes the MAC transmitter ignore the MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
    #[inline(always)]
    #[must_use]
    pub fn disablecrs(&mut self) -> DISABLECRS_W<EMACCONFIG_SPEC, 16> {
        DISABLECRS_W::new(self)
    }
    #[doc = "Bits 17:19 - These bits control the minimum IFG between frames during transmission. 3'b000: 96 bit times. 3'b001: 88 bit times. 3'b010: 80 bit times. 3'b111: 40 bit times. In the half-duplex mode the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered."]
    #[inline(always)]
    #[must_use]
    pub fn interframegap(&mut self) -> INTERFRAMEGAP_W<EMACCONFIG_SPEC, 17> {
        INTERFRAMEGAP_W::new(self)
    }
    #[doc = "Bit 20 - When this bit is set the MAC allows Jumbo frames of 9 018 bytes (9 022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    #[must_use]
    pub fn jumboframe(&mut self) -> JUMBOFRAME_W<EMACCONFIG_SPEC, 20> {
        JUMBOFRAME_W::new(self)
    }
    #[doc = "Bit 22 - When this bit is set the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16 383 bytes. When this bit is reset the MAC cuts off the transmitter if the application sends out more than 2 048 bytes of data (10 240 if JE is set high) during Transmission."]
    #[inline(always)]
    #[must_use]
    pub fn jabber(&mut self) -> JABBER_W<EMACCONFIG_SPEC, 22> {
        JABBER_W::new(self)
    }
    #[doc = "Bit 23 - When this bit is set the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16 383 bytes. When this bit is reset the MAC does not allow a receive frame which more than 2 048 bytes (10 240 if JE is set high) or the value programmed in Register (Watchdog Timeout Register). The MAC cuts off any bytes received after the watchdog limit number of bytes."]
    #[inline(always)]
    #[must_use]
    pub fn watchdog(&mut self) -> WATCHDOG_W<EMACCONFIG_SPEC, 23> {
        WATCHDOG_W::new(self)
    }
    #[doc = "Bit 27 - When set the MAC considers all frames with up to 2 000 bytes length as normal packets.When Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit\\[20\\] (JE) is not set the MAC considers all received frames of size more than 1 518 bytes (1 522 bytes for tagged) as Giant frames. When Bit\\[20\\] is set setting this bit has no effect on Giant Frame status."]
    #[inline(always)]
    #[must_use]
    pub fn ass2kp(&mut self) -> ASS2KP_W<EMACCONFIG_SPEC, 27> {
        ASS2KP_W::new(self)
    }
    #[doc = "Bits 28:30 - This field controls the source address insertion or replacement for all transmitted frames.Bit\\[30\\] specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. 2'b10: If Bit\\[30\\] is set to 0 the MAC inserts the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC inserts the content of the MAC Address 1 registers in the SA field of all transmitted frames. 2'b11: If Bit\\[30\\] is set to 0 the MAC replaces the content of the MAC Address 0 registers in the SA field of all transmitted frames. If Bit\\[30\\] is set to 1 the MAC replaces the content of the MAC Address 1 registers in the SA field of all transmitted frames."]
    #[inline(always)]
    #[must_use]
    pub fn sairc(&mut self) -> SAIRC_W<EMACCONFIG_SPEC, 28> {
        SAIRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MAC configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACCONFIG_SPEC;
impl crate::RegisterSpec for EMACCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacconfig::R`](R) reader structure"]
impl crate::Readable for EMACCONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacconfig::W`](W) writer structure"]
impl crate::Writable for EMACCONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMACCONFIG to value 0"]
impl crate::Resettable for EMACCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
