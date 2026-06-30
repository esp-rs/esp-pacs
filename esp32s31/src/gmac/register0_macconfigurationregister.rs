#[doc = "Register `REGISTER0_MACCONFIGURATIONREGISTER` reader"]
pub type R = crate::R<REGISTER0_MACCONFIGURATIONREGISTER_SPEC>;
#[doc = "Register `REGISTER0_MACCONFIGURATIONREGISTER` writer"]
pub type W = crate::W<REGISTER0_MACCONFIGURATIONREGISTER_SPEC>;
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame The preamble reduction occurs only when the MAC is operating in the fullduplex mode 2'b00: 7 bytes of preamble 2'b01: 5 bytes of preamble 2'b10: 3 bytes of preamble 2'b11: Reserved"]
pub type PRELEN_R = crate::FieldReader;
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame The preamble reduction occurs only when the MAC is operating in the fullduplex mode 2'b00: 7 bytes of preamble 2'b01: 5 bytes of preamble 2'b10: 3 bytes of preamble 2'b11: Reserved"]
pub type PRELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RE` reader - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DC` reader - Deferral Check When this bit is set, the deferral check function is enabled in the MAC The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal _CRS_ on GMII or MII The defer time is not cumulative For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens Because of collision, the transmitter needs to back off and then defer again after back off completion In such a scenario, the deferral timer is reset to 0 and it is restarted When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration"]
pub type DC_R = crate::BitReader;
#[doc = "Field `DC` writer - Deferral Check When this bit is set, the deferral check function is enabled in the MAC The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal _CRS_ on GMII or MII The defer time is not cumulative For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens Because of collision, the transmitter needs to back off and then defer again after back off completion In such a scenario, the deferral timer is reset to 0 and it is restarted When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration"]
pub type DC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - BackOff Limit The BackOff limit determines the random integer number _r_ of slot time delays _4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps_ for which the MAC waits before rescheduling a transmission attempt during retries after a collision This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration 00: k= min _n, 10_ 01: k = min _n, 8_ 10: k = min _n, 4_ 11: k = min _n, 1_ where n = retransmission attempt The random integer r takes the value in the range 0 ≤ r < 2k"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - BackOff Limit The BackOff limit determines the random integer number _r_ of slot time delays _4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps_ for which the MAC waits before rescheduling a transmission attempt during retries after a collision This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration 00: k= min _n, 10_ 01: k = min _n, 8_ 10: k = min _n, 4_ 11: k = min _n, 1_ where n = retransmission attempt The random integer r takes the value in the range 0 ≤ r < 2k"]
pub type BL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACS` reader - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host Note: For information about how the settings of Bit 25 _CST_ and this bit impact the frame length, see Table 632"]
pub type ACS_R = crate::BitReader;
#[doc = "Field `ACS` writer - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host Note: For information about how the settings of Bit 25 _CST_ and this bit impact the frame length, see Table 632"]
pub type ACS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUD` reader - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: 0: Link Down 1: Link Up This bit is reserved _RO with default value_ and is enabled when the RGMII, SGMII, or SMII interface is enabled during core configuration"]
pub type LUD_R = crate::BitReader;
#[doc = "Field `LUD` writer - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: 0: Link Down 1: Link Up This bit is reserved _RO with default value_ and is enabled when the RGMII, SGMII, or SMII interface is enabled during core configuration"]
pub type LUD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR` reader - Disable Retry When this bit is set, the MAC attempts only one transmission When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status When this bit is reset, the MAC attempts retries based on the settings of the BL field _Bits \\[6:5\\]_ This bit is applicable only in the halfduplex mode and is reserved _RO with default value_ in the fullduplexonly configuration"]
pub type DR_R = crate::BitReader;
#[doc = "Field `DR` writer - Disable Retry When this bit is set, the MAC attempts only one transmission When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status When this bit is reset, the MAC attempts retries based on the settings of the BL field _Bits \\[6:5\\]_ This bit is applicable only in the halfduplex mode and is reserved _RO with default value_ in the fullduplexonly configuration"]
pub type DR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPC` reader - Checksum Offload When this bit is set, the MAC calculates the 16bit one’s complement of the one’s complement sum of all received Ethernet frame payloads It also checks whether the IPv4 Header checksum _assumed to be bytes 2526 or 2930 _VLAN tagged_ of the received Ethernet frame_ is correct for the received frame and gives the status in the receive status word The MAC also appends the 16bit checksum calculated for the IP header datagram payload _bytes after the IPv4 header_ and appends it to the Ethernet frame transferred to the application _when Type 2 COE is deselected_ When this bit is reset, this function is disabled When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking When this bit is reset, the COE function in the receiver is disabled and the corresponding PCE and IP HCE status bits _see Table 310 on page 138_ are always cleared If the IP Checksum Offload feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
pub type IPC_R = crate::BitReader;
#[doc = "Field `IPC` writer - Checksum Offload When this bit is set, the MAC calculates the 16bit one’s complement of the one’s complement sum of all received Ethernet frame payloads It also checks whether the IPv4 Header checksum _assumed to be bytes 2526 or 2930 _VLAN tagged_ of the received Ethernet frame_ is correct for the received frame and gives the status in the receive status word The MAC also appends the 16bit checksum calculated for the IP header datagram payload _bytes after the IPv4 header_ and appends it to the Ethernet frame transferred to the application _when Type 2 COE is deselected_ When this bit is reset, this function is disabled When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking When this bit is reset, the COE function in the receiver is disabled and the corresponding PCE and IP HCE status bits _see Table 310 on page 138_ are always cleared If the IP Checksum Offload feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
pub type IPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - Duplex Mode When this bit is set, the MAC operates in the fullduplex mode where it can transmit and receive simultaneously This bit is RO with default value of 1'b1 in the fullduplexonly configuration"]
pub type DM_R = crate::BitReader;
#[doc = "Field `DM` writer - Duplex Mode When this bit is set, the MAC operates in the fullduplex mode where it can transmit and receive simultaneously This bit is RO with default value of 1'b1 in the fullduplexonly configuration"]
pub type DM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII The _G_MII Receive clock input _clk_rx_i_ is required for the loopback to work properly, because the Transmit clock is not loopedback internally"]
pub type LM_R = crate::BitReader;
#[doc = "Field `LM` writer - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII The _G_MII Receive clock input _clk_rx_i_ is required for the loopback to work properly, because the Transmit clock is not loopedback internally"]
pub type LM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO` reader - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the halfduplex mode When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting This bit is not applicable if the MAC is operating in the fullduplex mode This bit is reserved _RO with default value_ if the MAC is configured for the fullduplexonly operation"]
pub type DO_R = crate::BitReader;
#[doc = "Field `DO` writer - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the halfduplex mode When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting This bit is not applicable if the MAC is operating in the fullduplex mode This bit is reserved _RO with default value_ if the MAC is configured for the fullduplexonly operation"]
pub type DO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: 0: 10 Mbps 1: 100 Mbps This bit is reserved _RO_ by default and is enabled only when the parameter SPEED_SELECT = Enabled This bit generates link speed encoding when Bit 24 _TC_ is set in the RGMII, SMII, or SGMII mode This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal _mac_speed_o\\[0\\]_ to reflect the value of this bit in the mac_speed_o signal In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal _mac_speed_o\\[0\\]_ to reflect its value in the mac_speed_o signal"]
pub type FES_R = crate::BitReader;
#[doc = "Field `FES` writer - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: 0: 10 Mbps 1: 100 Mbps This bit is reserved _RO_ by default and is enabled only when the parameter SPEED_SELECT = Enabled This bit generates link speed encoding when Bit 24 _TC_ is set in the RGMII, SMII, or SGMII mode This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal _mac_speed_o\\[0\\]_ to reflect the value of this bit in the mac_speed_o signal In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal _mac_speed_o\\[0\\]_ to reflect its value in the mac_speed_o signal"]
pub type FES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Port Select This bit selects the Ethernet line speed 0: For 1000 Mbps operations 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed In the 10/100 Mbpsonly _always 1_ or 1000 Mbpsonly _always 0_ configurations, this bit is readonly with the appropriate value In default 10/100/1000 Mbps configuration, this bit is R_W The mac_portselect_o or mac_speed_o\\[1\\] signal reflects the value of this bit"]
pub type PS_R = crate::BitReader;
#[doc = "Field `PS` writer - Port Select This bit selects the Ethernet line speed 0: For 1000 Mbps operations 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed In the 10/100 Mbpsonly _always 1_ or 1000 Mbpsonly _always 0_ configurations, this bit is readonly with the appropriate value In default 10/100/1000 Mbps configuration, this bit is R_W The mac_portselect_o or mac_speed_o\\[1\\] signal reflects the value of this bit"]
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the _G_MII CRS signal during frame transmission in the halfduplex mode This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions This bit is reserved _and RO_ in the fullduplexonly configurations"]
pub type DCRS_R = crate::BitReader;
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the _G_MII CRS signal during frame transmission in the halfduplex mode This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions This bit is reserved _and RO_ in the fullduplexonly configurations"]
pub type DCRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFG` reader - InterFrame Gap These bits control the minimum IFG between frames during transmission 000: 96 bit times 001: 88 bit times 010: 80 bit times 111: 40 bit times In the halfduplex mode, the minimum IFG can be configured only for 64 bit times _IFG = 100_ Lower values are not considered In the 1000Mbps mode, the minimum IFG supported is 64 bit times _and above_ in the GMACCORE configuration and 80 bit times _and above_ in other configurations When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG"]
pub type IFG_R = crate::FieldReader;
#[doc = "Field `IFG` writer - InterFrame Gap These bits control the minimum IFG between frames during transmission 000: 96 bit times 001: 88 bit times 010: 80 bit times 111: 40 bit times In the halfduplex mode, the minimum IFG can be configured only for 64 bit times _IFG = 100_ Lower values are not considered In the 1000Mbps mode, the minimum IFG supported is 64 bit times _and above_ in the GMACCORE configuration and 80 bit times _and above_ in other configurations When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG"]
pub type IFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JE` reader - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes _9,022 bytes for VLAN tagged frames_ without reporting a giant frame error in the receive frame status"]
pub type JE_R = crate::BitReader;
#[doc = "Field `JE` writer - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes _9,022 bytes for VLAN tagged frames_ without reporting a giant frame error in the receive frame status"]
pub type JE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII halfduplex mode This bit is reserved _and RO_ in the 10/100 Mbps only or fullduplexonly configurations"]
pub type BE_R = crate::BitReader;
#[doc = "Field `BE` writer - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII halfduplex mode This bit is reserved _and RO_ in the 10/100 Mbps only or fullduplexonly configurations"]
pub type BE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JD` reader - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter The MAC can transfer frames of up to 16,383 bytes When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data _10,240 if JE is set high_ during transmission"]
pub type JD_R = crate::BitReader;
#[doc = "Field `JD` writer - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter The MAC can transfer frames of up to 16,383 bytes When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data _10,240 if JE is set high_ during transmission"]
pub type JD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver The MAC can receive frames of up to 16,383 bytes When this bit is reset, the MAC does not allow a receive frame which more than 2,048 bytes _10,240 if JE is set high_ or the value programmed in Register 55 _Watchdog Timeout Register_ The MAC cuts off any bytes received after the watchdog limit number of bytes"]
pub type WD_R = crate::BitReader;
#[doc = "Field `WD` writer - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver The MAC can receive frames of up to 16,383 bytes When this bit is reset, the MAC does not allow a receive frame which more than 2,048 bytes _10,240 if JE is set high_ or the value programmed in Register 55 _Watchdog Timeout Register_ The MAC cuts off any bytes received after the watchdog limit number of bytes"]
pub type WD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port When this bit is reset, no such information is driven to the PHY This bit is reserved _and RO_ if the RGMII, SMII, or SGMII PHY port is not selected during core configuration The details of this feature are explained in the following sections: “Reduced Gigabit Media Independent Interface” on page 249 “Serial Media Independent Interface” on page 245 “Serial Gigabit Media Independent Interface” on page 257"]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port When this bit is reset, no such information is driven to the PHY This bit is reserved _and RO_ if the RGMII, SMII, or SGMII PHY port is not selected during core configuration The details of this feature are explained in the following sections: “Reduced Gigabit Media Independent Interface” on page 249 “Serial Media Independent Interface” on page 245 “Serial Gigabit Media Independent Interface” on page 257"]
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CST` reader - CRC Stripping for Type Frames When this bit is set, the last 4 bytes _FCS_ of all frames of Ether type _Length/Type field greater than or equal to 1,536_ are stripped and dropped before forwarding the frame to the application This function is not valid when the IP Checksum Engine _Type 1_ is enabled in the MAC receiver This function is valid when Type 2 Checksum Offload Engine is enabled Note: For information about how the settings of Bit 7 _ACS_ and this bit impact the frame length, see Table 632"]
pub type CST_R = crate::BitReader;
#[doc = "Field `CST` writer - CRC Stripping for Type Frames When this bit is set, the last 4 bytes _FCS_ of all frames of Ether type _Length/Type field greater than or equal to 1,536_ are stripped and dropped before forwarding the frame to the application This function is not valid when the IP Checksum Engine _Type 1_ is enabled in the MAC receiver This function is valid when Type 2 Checksum Offload Engine is enabled Note: For information about how the settings of Bit 7 _ACS_ and this bit impact the frame length, see Table 632"]
pub type CST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTERR` reader - SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted This bit is reserved if the SMII PHY port is not selected during core configuration"]
pub type SFTERR_R = crate::BitReader;
#[doc = "Field `SFTERR` writer - SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted This bit is reserved if the SMII PHY port is not selected during core configuration"]
pub type SFTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWOKPE` reader - IEEE 8023as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets When Bit 20 _JE_ is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames When this bit is reset and Bit 20 _JE_ is not set, the MAC considers all received frames of size more than 1,518 bytes _1,522 bytes for tagged_ as Giant frames When Bit 20 is set, setting this bit has no effect on Giant Frame status For more information about how the setting of this bit and Bit 20 impact the Giant frame status, see Table 631"]
pub type TWOKPE_R = crate::BitReader;
#[doc = "Field `TWOKPE` writer - IEEE 8023as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets When Bit 20 _JE_ is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames When this bit is reset and Bit 20 _JE_ is not set, the MAC considers all received frames of size more than 1,518 bytes _1,522 bytes for tagged_ as Giant frames When Bit 20 is set, setting this bit has no effect on Giant Frame status For more information about how the setting of this bit and Bit 20 impact the Giant frame status, see Table 631"]
pub type TWOKPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARC` reader - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames Bit 30 specifies which MAC Address register _0 or 1_ is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation 2'b10: If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers _registers 16 and 17_ in the SA field of all transmitted frames If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers _registers 18 and 19_ in the SA field of all transmitted frames 2'b11: If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers _registers 16 and 17_ in the SA field of all transmitted frames If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers _registers 18 and 19_ in the SA field of all transmitted frames Note: Changes to this field take effect only on the start of a frame If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration"]
pub type SARC_R = crate::FieldReader;
#[doc = "Field `SARC` writer - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames Bit 30 specifies which MAC Address register _0 or 1_ is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation 2'b10: If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers _registers 16 and 17_ in the SA field of all transmitted frames If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers _registers 18 and 19_ in the SA field of all transmitted frames 2'b11: If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers _registers 16 and 17_ in the SA field of all transmitted frames If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers _registers 18 and 19_ in the SA field of all transmitted frames Note: Changes to this field take effect only on the start of a frame If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration"]
pub type SARC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame The preamble reduction occurs only when the MAC is operating in the fullduplex mode 2'b00: 7 bytes of preamble 2'b01: 5 bytes of preamble 2'b10: 3 bytes of preamble 2'b11: Reserved"]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal _CRS_ on GMII or MII The defer time is not cumulative For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens Because of collision, the transmitter needs to back off and then defer again after back off completion In such a scenario, the deferral timer is reset to 0 and it is restarted When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - BackOff Limit The BackOff limit determines the random integer number _r_ of slot time delays _4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps_ for which the MAC waits before rescheduling a transmission attempt during retries after a collision This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration 00: k= min _n, 10_ 01: k = min _n, 8_ 10: k = min _n, 4_ 11: k = min _n, 1_ where n = retransmission attempt The random integer r takes the value in the range 0 ≤ r < 2k"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host Note: For information about how the settings of Bit 25 _CST_ and this bit impact the frame length, see Table 632"]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: 0: Link Down 1: Link Up This bit is reserved _RO with default value_ and is enabled when the RGMII, SGMII, or SMII interface is enabled during core configuration"]
    #[inline(always)]
    pub fn lud(&self) -> LUD_R {
        LUD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Retry When this bit is set, the MAC attempts only one transmission When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status When this bit is reset, the MAC attempts retries based on the settings of the BL field _Bits \\[6:5\\]_ This bit is applicable only in the halfduplex mode and is reserved _RO with default value_ in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload When this bit is set, the MAC calculates the 16bit one’s complement of the one’s complement sum of all received Ethernet frame payloads It also checks whether the IPv4 Header checksum _assumed to be bytes 2526 or 2930 _VLAN tagged_ of the received Ethernet frame_ is correct for the received frame and gives the status in the receive status word The MAC also appends the 16bit checksum calculated for the IP header datagram payload _bytes after the IPv4 header_ and appends it to the Ethernet frame transferred to the application _when Type 2 COE is deselected_ When this bit is reset, this function is disabled When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking When this bit is reset, the COE function in the receiver is disabled and the corresponding PCE and IP HCE status bits _see Table 310 on page 138_ are always cleared If the IP Checksum Offload feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode When this bit is set, the MAC operates in the fullduplex mode where it can transmit and receive simultaneously This bit is RO with default value of 1'b1 in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII The _G_MII Receive clock input _clk_rx_i_ is required for the loopback to work properly, because the Transmit clock is not loopedback internally"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the halfduplex mode When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting This bit is not applicable if the MAC is operating in the fullduplex mode This bit is reserved _RO with default value_ if the MAC is configured for the fullduplexonly operation"]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: 0: 10 Mbps 1: 100 Mbps This bit is reserved _RO_ by default and is enabled only when the parameter SPEED_SELECT = Enabled This bit generates link speed encoding when Bit 24 _TC_ is set in the RGMII, SMII, or SGMII mode This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal _mac_speed_o\\[0\\]_ to reflect the value of this bit in the mac_speed_o signal In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal _mac_speed_o\\[0\\]_ to reflect its value in the mac_speed_o signal"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Select This bit selects the Ethernet line speed 0: For 1000 Mbps operations 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed In the 10/100 Mbpsonly _always 1_ or 1000 Mbpsonly _always 0_ configurations, this bit is readonly with the appropriate value In default 10/100/1000 Mbps configuration, this bit is R_W The mac_portselect_o or mac_speed_o\\[1\\] signal reflects the value of this bit"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the _G_MII CRS signal during frame transmission in the halfduplex mode This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions This bit is reserved _and RO_ in the fullduplexonly configurations"]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - InterFrame Gap These bits control the minimum IFG between frames during transmission 000: 96 bit times 001: 88 bit times 010: 80 bit times 111: 40 bit times In the halfduplex mode, the minimum IFG can be configured only for 64 bit times _IFG = 100_ Lower values are not considered In the 1000Mbps mode, the minimum IFG supported is 64 bit times _and above_ in the GMACCORE configuration and 80 bit times _and above_ in other configurations When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes _9,022 bytes for VLAN tagged frames_ without reporting a giant frame error in the receive frame status"]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII halfduplex mode This bit is reserved _and RO_ in the 10/100 Mbps only or fullduplexonly configurations"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter The MAC can transfer frames of up to 16,383 bytes When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data _10,240 if JE is set high_ during transmission"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver The MAC can receive frames of up to 16,383 bytes When this bit is reset, the MAC does not allow a receive frame which more than 2,048 bytes _10,240 if JE is set high_ or the value programmed in Register 55 _Watchdog Timeout Register_ The MAC cuts off any bytes received after the watchdog limit number of bytes"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port When this bit is reset, no such information is driven to the PHY This bit is reserved _and RO_ if the RGMII, SMII, or SGMII PHY port is not selected during core configuration The details of this feature are explained in the following sections: “Reduced Gigabit Media Independent Interface” on page 249 “Serial Media Independent Interface” on page 245 “Serial Gigabit Media Independent Interface” on page 257"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames When this bit is set, the last 4 bytes _FCS_ of all frames of Ether type _Length/Type field greater than or equal to 1,536_ are stripped and dropped before forwarding the frame to the application This function is not valid when the IP Checksum Engine _Type 1_ is enabled in the MAC receiver This function is valid when Type 2 Checksum Offload Engine is enabled Note: For information about how the settings of Bit 7 _ACS_ and this bit impact the frame length, see Table 632"]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted This bit is reserved if the SMII PHY port is not selected during core configuration"]
    #[inline(always)]
    pub fn sfterr(&self) -> SFTERR_R {
        SFTERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IEEE 8023as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets When Bit 20 _JE_ is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames When this bit is reset and Bit 20 _JE_ is not set, the MAC considers all received frames of size more than 1,518 bytes _1,522 bytes for tagged_ as Giant frames When Bit 20 is set, setting this bit has no effect on Giant Frame status For more information about how the setting of this bit and Bit 20 impact the Giant frame status, see Table 631"]
    #[inline(always)]
    pub fn twokpe(&self) -> TWOKPE_R {
        TWOKPE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames Bit 30 specifies which MAC Address register _0 or 1_ is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation 2'b10: If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers _registers 16 and 17_ in the SA field of all transmitted frames If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers _registers 18 and 19_ in the SA field of all transmitted frames 2'b11: If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers _registers 16 and 17_ in the SA field of all transmitted frames If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers _registers 18 and 19_ in the SA field of all transmitted frames Note: Changes to this field take effect only on the start of a frame If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration"]
    #[inline(always)]
    pub fn sarc(&self) -> SARC_R {
        SARC_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER0_MACCONFIGURATIONREGISTER")
            .field("prelen", &self.prelen())
            .field("re", &self.re())
            .field("te", &self.te())
            .field("dc", &self.dc())
            .field("bl", &self.bl())
            .field("acs", &self.acs())
            .field("lud", &self.lud())
            .field("dr", &self.dr())
            .field("ipc", &self.ipc())
            .field("dm", &self.dm())
            .field("lm", &self.lm())
            .field("do_", &self.do_())
            .field("fes", &self.fes())
            .field("ps", &self.ps())
            .field("dcrs", &self.dcrs())
            .field("ifg", &self.ifg())
            .field("je", &self.je())
            .field("be", &self.be())
            .field("jd", &self.jd())
            .field("wd", &self.wd())
            .field("tc", &self.tc())
            .field("cst", &self.cst())
            .field("sfterr", &self.sfterr())
            .field("twokpe", &self.twokpe())
            .field("sarc", &self.sarc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame The preamble reduction occurs only when the MAC is operating in the fullduplex mode 2'b00: 7 bytes of preamble 2'b01: 5 bytes of preamble 2'b10: 3 bytes of preamble 2'b11: Reserved"]
    #[inline(always)]
    pub fn prelen(&mut self) -> PRELEN_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        PRELEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal _CRS_ on GMII or MII The defer time is not cumulative For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens Because of collision, the transmitter needs to back off and then defer again after back off completion In such a scenario, the deferral timer is reset to 0 and it is restarted When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        DC_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - BackOff Limit The BackOff limit determines the random integer number _r_ of slot time delays _4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps_ for which the MAC waits before rescheduling a transmission attempt during retries after a collision This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration 00: k= min _n, 10_ 01: k = min _n, 8_ 10: k = min _n, 4_ 11: k = min _n, 1_ where n = retransmission attempt The random integer r takes the value in the range 0 ≤ r < 2k"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        BL_W::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host Note: For information about how the settings of Bit 25 _CST_ and this bit impact the frame length, see Table 632"]
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        ACS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: 0: Link Down 1: Link Up This bit is reserved _RO with default value_ and is enabled when the RGMII, SGMII, or SMII interface is enabled during core configuration"]
    #[inline(always)]
    pub fn lud(&mut self) -> LUD_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        LUD_W::new(self, 8)
    }
    #[doc = "Bit 9 - Disable Retry When this bit is set, the MAC attempts only one transmission When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status When this bit is reset, the MAC attempts retries based on the settings of the BL field _Bits \\[6:5\\]_ This bit is applicable only in the halfduplex mode and is reserved _RO with default value_ in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        DR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Checksum Offload When this bit is set, the MAC calculates the 16bit one’s complement of the one’s complement sum of all received Ethernet frame payloads It also checks whether the IPv4 Header checksum _assumed to be bytes 2526 or 2930 _VLAN tagged_ of the received Ethernet frame_ is correct for the received frame and gives the status in the receive status word The MAC also appends the 16bit checksum calculated for the IP header datagram payload _bytes after the IPv4 header_ and appends it to the Ethernet frame transferred to the application _when Type 2 COE is deselected_ When this bit is reset, this function is disabled When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking When this bit is reset, the COE function in the receiver is disabled and the corresponding PCE and IP HCE status bits _see Table 310 on page 138_ are always cleared If the IP Checksum Offload feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        IPC_W::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex Mode When this bit is set, the MAC operates in the fullduplex mode where it can transmit and receive simultaneously This bit is RO with default value of 1'b1 in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        DM_W::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII The _G_MII Receive clock input _clk_rx_i_ is required for the loopback to work properly, because the Transmit clock is not loopedback internally"]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        LM_W::new(self, 12)
    }
    #[doc = "Bit 13 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the halfduplex mode When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting This bit is not applicable if the MAC is operating in the fullduplex mode This bit is reserved _RO with default value_ if the MAC is configured for the fullduplexonly operation"]
    #[inline(always)]
    pub fn do_(&mut self) -> DO_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        DO_W::new(self, 13)
    }
    #[doc = "Bit 14 - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: 0: 10 Mbps 1: 100 Mbps This bit is reserved _RO_ by default and is enabled only when the parameter SPEED_SELECT = Enabled This bit generates link speed encoding when Bit 24 _TC_ is set in the RGMII, SMII, or SGMII mode This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal _mac_speed_o\\[0\\]_ to reflect the value of this bit in the mac_speed_o signal In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal _mac_speed_o\\[0\\]_ to reflect its value in the mac_speed_o signal"]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        FES_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port Select This bit selects the Ethernet line speed 0: For 1000 Mbps operations 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed In the 10/100 Mbpsonly _always 1_ or 1000 Mbpsonly _always 0_ configurations, this bit is readonly with the appropriate value In default 10/100/1000 Mbps configuration, this bit is R_W The mac_portselect_o or mac_speed_o\\[1\\] signal reflects the value of this bit"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        PS_W::new(self, 15)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the _G_MII CRS signal during frame transmission in the halfduplex mode This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions This bit is reserved _and RO_ in the fullduplexonly configurations"]
    #[inline(always)]
    pub fn dcrs(&mut self) -> DCRS_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        DCRS_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - InterFrame Gap These bits control the minimum IFG between frames during transmission 000: 96 bit times 001: 88 bit times 010: 80 bit times 111: 40 bit times In the halfduplex mode, the minimum IFG can be configured only for 64 bit times _IFG = 100_ Lower values are not considered In the 1000Mbps mode, the minimum IFG supported is 64 bit times _and above_ in the GMACCORE configuration and 80 bit times _and above_ in other configurations When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        IFG_W::new(self, 17)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes _9,022 bytes for VLAN tagged frames_ without reporting a giant frame error in the receive frame status"]
    #[inline(always)]
    pub fn je(&mut self) -> JE_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        JE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII halfduplex mode This bit is reserved _and RO_ in the 10/100 Mbps only or fullduplexonly configurations"]
    #[inline(always)]
    pub fn be(&mut self) -> BE_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        BE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter The MAC can transfer frames of up to 16,383 bytes When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data _10,240 if JE is set high_ during transmission"]
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        JD_W::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver The MAC can receive frames of up to 16,383 bytes When this bit is reset, the MAC does not allow a receive frame which more than 2,048 bytes _10,240 if JE is set high_ or the value programmed in Register 55 _Watchdog Timeout Register_ The MAC cuts off any bytes received after the watchdog limit number of bytes"]
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        WD_W::new(self, 23)
    }
    #[doc = "Bit 24 - Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port When this bit is reset, no such information is driven to the PHY This bit is reserved _and RO_ if the RGMII, SMII, or SGMII PHY port is not selected during core configuration The details of this feature are explained in the following sections: “Reduced Gigabit Media Independent Interface” on page 249 “Serial Media Independent Interface” on page 245 “Serial Gigabit Media Independent Interface” on page 257"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        TC_W::new(self, 24)
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames When this bit is set, the last 4 bytes _FCS_ of all frames of Ether type _Length/Type field greater than or equal to 1,536_ are stripped and dropped before forwarding the frame to the application This function is not valid when the IP Checksum Engine _Type 1_ is enabled in the MAC receiver This function is valid when Type 2 Checksum Offload Engine is enabled Note: For information about how the settings of Bit 7 _ACS_ and this bit impact the frame length, see Table 632"]
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        CST_W::new(self, 25)
    }
    #[doc = "Bit 26 - SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted This bit is reserved if the SMII PHY port is not selected during core configuration"]
    #[inline(always)]
    pub fn sfterr(&mut self) -> SFTERR_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        SFTERR_W::new(self, 26)
    }
    #[doc = "Bit 27 - IEEE 8023as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets When Bit 20 _JE_ is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames When this bit is reset and Bit 20 _JE_ is not set, the MAC considers all received frames of size more than 1,518 bytes _1,522 bytes for tagged_ as Giant frames When Bit 20 is set, setting this bit has no effect on Giant Frame status For more information about how the setting of this bit and Bit 20 impact the Giant frame status, see Table 631"]
    #[inline(always)]
    pub fn twokpe(&mut self) -> TWOKPE_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        TWOKPE_W::new(self, 27)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames Bit 30 specifies which MAC Address register _0 or 1_ is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation 2'b10: If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers _registers 16 and 17_ in the SA field of all transmitted frames If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers _registers 18 and 19_ in the SA field of all transmitted frames 2'b11: If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers _registers 16 and 17_ in the SA field of all transmitted frames If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers _registers 18 and 19_ in the SA field of all transmitted frames Note: Changes to this field take effect only on the start of a frame If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration"]
    #[inline(always)]
    pub fn sarc(&mut self) -> SARC_W<'_, REGISTER0_MACCONFIGURATIONREGISTER_SPEC> {
        SARC_W::new(self, 28)
    }
}
#[doc = "This is the operation mode register for the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`register0_macconfigurationregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register0_macconfigurationregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER0_MACCONFIGURATIONREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER0_MACCONFIGURATIONREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register0_macconfigurationregister::R`](R) reader structure"]
impl crate::Readable for REGISTER0_MACCONFIGURATIONREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register0_macconfigurationregister::W`](W) writer structure"]
impl crate::Writable for REGISTER0_MACCONFIGURATIONREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER0_MACCONFIGURATIONREGISTER to value 0"]
impl crate::Resettable for REGISTER0_MACCONFIGURATIONREGISTER_SPEC {}
