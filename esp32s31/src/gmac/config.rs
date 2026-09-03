#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `PLTF` reader - Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame The preamble reduction occurs only when the MAC is operating in the fullduplex mode 2'b00: 7 bytes of preamble 2'b01: 5 bytes of preamble 2'b10: 3 bytes of preamble 2'b11: Reserved"]
pub type PLTF_R = crate::FieldReader;
#[doc = "Field `PLTF` writer - Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame The preamble reduction occurs only when the MAC is operating in the fullduplex mode 2'b00: 7 bytes of preamble 2'b01: 5 bytes of preamble 2'b10: 3 bytes of preamble 2'b11: Reserved"]
pub type PLTF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX` reader - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII"]
pub type RX_R = crate::BitReader;
#[doc = "Field `RX` writer - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII"]
pub type RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames"]
pub type TX_R = crate::BitReader;
#[doc = "Field `TX` writer - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames"]
pub type TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFERRALCHECK` reader - Deferral Check When this bit is set, the deferral check function is enabled in the MAC The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal _CRS_ on GMII or MII The defer time is not cumulative For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens Because of collision, the transmitter needs to back off and then defer again after back off completion In such a scenario, the deferral timer is reset to 0 and it is restarted When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration"]
pub type DEFERRALCHECK_R = crate::BitReader;
#[doc = "Field `DEFERRALCHECK` writer - Deferral Check When this bit is set, the deferral check function is enabled in the MAC The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal _CRS_ on GMII or MII The defer time is not cumulative For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens Because of collision, the transmitter needs to back off and then defer again after back off completion In such a scenario, the deferral timer is reset to 0 and it is restarted When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration"]
pub type DEFERRALCHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACKOFFLIMIT` reader - BackOff Limit The BackOff limit determines the random integer number _r_ of slot time delays _4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps_ for which the MAC waits before rescheduling a transmission attempt during retries after a collision This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration 00: k= min _n, 10_ 01: k = min _n, 8_ 10: k = min _n, 4_ 11: k = min _n, 1_ where n = retransmission attempt The random integer r takes the value in the range 0 ≤ r < 2k"]
pub type BACKOFFLIMIT_R = crate::FieldReader;
#[doc = "Field `BACKOFFLIMIT` writer - BackOff Limit The BackOff limit determines the random integer number _r_ of slot time delays _4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps_ for which the MAC waits before rescheduling a transmission attempt during retries after a collision This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration 00: k= min _n, 10_ 01: k = min _n, 8_ 10: k = min _n, 4_ 11: k = min _n, 1_ where n = retransmission attempt The random integer r takes the value in the range 0 ≤ r < 2k"]
pub type BACKOFFLIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCRCSTRIP` reader - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host Note: For information about how the settings of Bit 25 _CST_ and this bit impact the frame length, see Table 632"]
pub type PADCRCSTRIP_R = crate::BitReader;
#[doc = "Field `PADCRCSTRIP` writer - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host Note: For information about how the settings of Bit 25 _CST_ and this bit impact the frame length, see Table 632"]
pub type PADCRCSTRIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUD` reader - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: 0: Link Down 1: Link Up This bit is reserved _RO with default value_ and is enabled when the RGMII, SGMII, or SMII interface is enabled during core configuration"]
pub type LUD_R = crate::BitReader;
#[doc = "Field `LUD` writer - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: 0: Link Down 1: Link Up This bit is reserved _RO with default value_ and is enabled when the RGMII, SGMII, or SMII interface is enabled during core configuration"]
pub type LUD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY` reader - Disable Retry When this bit is set, the MAC attempts only one transmission When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status When this bit is reset, the MAC attempts retries based on the settings of the BL field _Bits \\[6:5\\]_ This bit is applicable only in the halfduplex mode and is reserved _RO with default value_ in the fullduplexonly configuration"]
pub type RETRY_R = crate::BitReader;
#[doc = "Field `RETRY` writer - Disable Retry When this bit is set, the MAC attempts only one transmission When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status When this bit is reset, the MAC attempts retries based on the settings of the BL field _Bits \\[6:5\\]_ This bit is applicable only in the halfduplex mode and is reserved _RO with default value_ in the fullduplexonly configuration"]
pub type RETRY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIPCOFFLOAD` reader - Checksum Offload When this bit is set, the MAC calculates the 16bit one’s complement of the one’s complement sum of all received Ethernet frame payloads It also checks whether the IPv4 Header checksum _assumed to be bytes 2526 or 2930 _VLAN tagged_ of the received Ethernet frame_ is correct for the received frame and gives the status in the receive status word The MAC also appends the 16bit checksum calculated for the IP header datagram payload _bytes after the IPv4 header_ and appends it to the Ethernet frame transferred to the application _when Type 2 COE is deselected_ When this bit is reset, this function is disabled When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking When this bit is reset, the COE function in the receiver is disabled and the corresponding PCE and IP HCE status bits _see Table 310 on page 138_ are always cleared If the IP Checksum Offload feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
pub type RXIPCOFFLOAD_R = crate::BitReader;
#[doc = "Field `RXIPCOFFLOAD` writer - Checksum Offload When this bit is set, the MAC calculates the 16bit one’s complement of the one’s complement sum of all received Ethernet frame payloads It also checks whether the IPv4 Header checksum _assumed to be bytes 2526 or 2930 _VLAN tagged_ of the received Ethernet frame_ is correct for the received frame and gives the status in the receive status word The MAC also appends the 16bit checksum calculated for the IP header datagram payload _bytes after the IPv4 header_ and appends it to the Ethernet frame transferred to the application _when Type 2 COE is deselected_ When this bit is reset, this function is disabled When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking When this bit is reset, the COE function in the receiver is disabled and the corresponding PCE and IP HCE status bits _see Table 310 on page 138_ are always cleared If the IP Checksum Offload feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
pub type RXIPCOFFLOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUPLEX` reader - Duplex Mode When this bit is set, the MAC operates in the fullduplex mode where it can transmit and receive simultaneously This bit is RO with default value of 1'b1 in the fullduplexonly configuration"]
pub type DUPLEX_R = crate::BitReader;
#[doc = "Field `DUPLEX` writer - Duplex Mode When this bit is set, the MAC operates in the fullduplex mode where it can transmit and receive simultaneously This bit is RO with default value of 1'b1 in the fullduplexonly configuration"]
pub type DUPLEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII The _G_MII Receive clock input _clk_rx_i_ is required for the loopback to work properly, because the Transmit clock is not loopedback internally"]
pub type LOOPBACK_R = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII The _G_MII Receive clock input _clk_rx_i_ is required for the loopback to work properly, because the Transmit clock is not loopedback internally"]
pub type LOOPBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOWN` reader - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the halfduplex mode When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting This bit is not applicable if the MAC is operating in the fullduplex mode This bit is reserved _RO with default value_ if the MAC is configured for the fullduplexonly operation"]
pub type RXOWN_R = crate::BitReader;
#[doc = "Field `RXOWN` writer - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the halfduplex mode When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting This bit is not applicable if the MAC is operating in the fullduplex mode This bit is reserved _RO with default value_ if the MAC is configured for the fullduplexonly operation"]
pub type RXOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FESPEED` reader - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: 0: 10 Mbps 1: 100 Mbps This bit is reserved _RO_ by default and is enabled only when the parameter SPEED_SELECT = Enabled This bit generates link speed encoding when Bit 24 _TC_ is set in the RGMII, SMII, or SGMII mode This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal _mac_speed_o\\[0\\]_ to reflect the value of this bit in the mac_speed_o signal In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal _mac_speed_o\\[0\\]_ to reflect its value in the mac_speed_o signal"]
pub type FESPEED_R = crate::BitReader;
#[doc = "Field `FESPEED` writer - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: 0: 10 Mbps 1: 100 Mbps This bit is reserved _RO_ by default and is enabled only when the parameter SPEED_SELECT = Enabled This bit generates link speed encoding when Bit 24 _TC_ is set in the RGMII, SMII, or SGMII mode This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal _mac_speed_o\\[0\\]_ to reflect the value of this bit in the mac_speed_o signal In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal _mac_speed_o\\[0\\]_ to reflect its value in the mac_speed_o signal"]
pub type FESPEED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MII` reader - Port Select This bit selects the Ethernet line speed 0: For 1000 Mbps operations 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed In the 10/100 Mbpsonly _always 1_ or 1000 Mbpsonly _always 0_ configurations, this bit is readonly with the appropriate value In default 10/100/1000 Mbps configuration, this bit is R_W The mac_portselect_o or mac_speed_o\\[1\\] signal reflects the value of this bit"]
pub type MII_R = crate::BitReader;
#[doc = "Field `MII` writer - Port Select This bit selects the Ethernet line speed 0: For 1000 Mbps operations 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed In the 10/100 Mbpsonly _always 1_ or 1000 Mbpsonly _always 0_ configurations, this bit is readonly with the appropriate value In default 10/100/1000 Mbps configuration, this bit is R_W The mac_portselect_o or mac_speed_o\\[1\\] signal reflects the value of this bit"]
pub type MII_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLECRS` reader - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the _G_MII CRS signal during frame transmission in the halfduplex mode This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions This bit is reserved _and RO_ in the fullduplexonly configurations"]
pub type DISABLECRS_R = crate::BitReader;
#[doc = "Field `DISABLECRS` writer - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the _G_MII CRS signal during frame transmission in the halfduplex mode This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions This bit is reserved _and RO_ in the fullduplexonly configurations"]
pub type DISABLECRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERFRAMEGAP` reader - InterFrame Gap These bits control the minimum IFG between frames during transmission 000: 96 bit times 001: 88 bit times 010: 80 bit times 111: 40 bit times In the halfduplex mode, the minimum IFG can be configured only for 64 bit times _IFG = 100_ Lower values are not considered In the 1000Mbps mode, the minimum IFG supported is 64 bit times _and above_ in the GMACCORE configuration and 80 bit times _and above_ in other configurations When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG"]
pub type INTERFRAMEGAP_R = crate::FieldReader;
#[doc = "Field `INTERFRAMEGAP` writer - InterFrame Gap These bits control the minimum IFG between frames during transmission 000: 96 bit times 001: 88 bit times 010: 80 bit times 111: 40 bit times In the halfduplex mode, the minimum IFG can be configured only for 64 bit times _IFG = 100_ Lower values are not considered In the 1000Mbps mode, the minimum IFG supported is 64 bit times _and above_ in the GMACCORE configuration and 80 bit times _and above_ in other configurations When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG"]
pub type INTERFRAMEGAP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JUMBOFRAME` reader - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes _9,022 bytes for VLAN tagged frames_ without reporting a giant frame error in the receive frame status"]
pub type JUMBOFRAME_R = crate::BitReader;
#[doc = "Field `JUMBOFRAME` writer - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes _9,022 bytes for VLAN tagged frames_ without reporting a giant frame error in the receive frame status"]
pub type JUMBOFRAME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII halfduplex mode This bit is reserved _and RO_ in the 10/100 Mbps only or fullduplexonly configurations"]
pub type BE_R = crate::BitReader;
#[doc = "Field `BE` writer - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII halfduplex mode This bit is reserved _and RO_ in the 10/100 Mbps only or fullduplexonly configurations"]
pub type BE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JABBER` reader - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter The MAC can transfer frames of up to 16,383 bytes When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data _10,240 if JE is set high_ during transmission"]
pub type JABBER_R = crate::BitReader;
#[doc = "Field `JABBER` writer - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter The MAC can transfer frames of up to 16,383 bytes When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data _10,240 if JE is set high_ during transmission"]
pub type JABBER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATCHDOG` reader - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver The MAC can receive frames of up to 16,383 bytes When this bit is reset, the MAC does not allow a receive frame which more than 2,048 bytes _10,240 if JE is set high_ or the value programmed in Register 55 _Watchdog Timeout Register_ The MAC cuts off any bytes received after the watchdog limit number of bytes"]
pub type WATCHDOG_R = crate::BitReader;
#[doc = "Field `WATCHDOG` writer - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver The MAC can receive frames of up to 16,383 bytes When this bit is reset, the MAC does not allow a receive frame which more than 2,048 bytes _10,240 if JE is set high_ or the value programmed in Register 55 _Watchdog Timeout Register_ The MAC cuts off any bytes received after the watchdog limit number of bytes"]
pub type WATCHDOG_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn pltf(&self) -> PLTF_R {
        PLTF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal _CRS_ on GMII or MII The defer time is not cumulative For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens Because of collision, the transmitter needs to back off and then defer again after back off completion In such a scenario, the deferral timer is reset to 0 and it is restarted When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn deferralcheck(&self) -> DEFERRALCHECK_R {
        DEFERRALCHECK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - BackOff Limit The BackOff limit determines the random integer number _r_ of slot time delays _4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps_ for which the MAC waits before rescheduling a transmission attempt during retries after a collision This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration 00: k= min _n, 10_ 01: k = min _n, 8_ 10: k = min _n, 4_ 11: k = min _n, 1_ where n = retransmission attempt The random integer r takes the value in the range 0 ≤ r < 2k"]
    #[inline(always)]
    pub fn backofflimit(&self) -> BACKOFFLIMIT_R {
        BACKOFFLIMIT_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host Note: For information about how the settings of Bit 25 _CST_ and this bit impact the frame length, see Table 632"]
    #[inline(always)]
    pub fn padcrcstrip(&self) -> PADCRCSTRIP_R {
        PADCRCSTRIP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: 0: Link Down 1: Link Up This bit is reserved _RO with default value_ and is enabled when the RGMII, SGMII, or SMII interface is enabled during core configuration"]
    #[inline(always)]
    pub fn lud(&self) -> LUD_R {
        LUD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Retry When this bit is set, the MAC attempts only one transmission When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status When this bit is reset, the MAC attempts retries based on the settings of the BL field _Bits \\[6:5\\]_ This bit is applicable only in the halfduplex mode and is reserved _RO with default value_ in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn retry(&self) -> RETRY_R {
        RETRY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload When this bit is set, the MAC calculates the 16bit one’s complement of the one’s complement sum of all received Ethernet frame payloads It also checks whether the IPv4 Header checksum _assumed to be bytes 2526 or 2930 _VLAN tagged_ of the received Ethernet frame_ is correct for the received frame and gives the status in the receive status word The MAC also appends the 16bit checksum calculated for the IP header datagram payload _bytes after the IPv4 header_ and appends it to the Ethernet frame transferred to the application _when Type 2 COE is deselected_ When this bit is reset, this function is disabled When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking When this bit is reset, the COE function in the receiver is disabled and the corresponding PCE and IP HCE status bits _see Table 310 on page 138_ are always cleared If the IP Checksum Offload feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn rxipcoffload(&self) -> RXIPCOFFLOAD_R {
        RXIPCOFFLOAD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode When this bit is set, the MAC operates in the fullduplex mode where it can transmit and receive simultaneously This bit is RO with default value of 1'b1 in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn duplex(&self) -> DUPLEX_R {
        DUPLEX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII The _G_MII Receive clock input _clk_rx_i_ is required for the loopback to work properly, because the Transmit clock is not loopedback internally"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the halfduplex mode When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting This bit is not applicable if the MAC is operating in the fullduplex mode This bit is reserved _RO with default value_ if the MAC is configured for the fullduplexonly operation"]
    #[inline(always)]
    pub fn rxown(&self) -> RXOWN_R {
        RXOWN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: 0: 10 Mbps 1: 100 Mbps This bit is reserved _RO_ by default and is enabled only when the parameter SPEED_SELECT = Enabled This bit generates link speed encoding when Bit 24 _TC_ is set in the RGMII, SMII, or SGMII mode This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal _mac_speed_o\\[0\\]_ to reflect the value of this bit in the mac_speed_o signal In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal _mac_speed_o\\[0\\]_ to reflect its value in the mac_speed_o signal"]
    #[inline(always)]
    pub fn fespeed(&self) -> FESPEED_R {
        FESPEED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Select This bit selects the Ethernet line speed 0: For 1000 Mbps operations 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed In the 10/100 Mbpsonly _always 1_ or 1000 Mbpsonly _always 0_ configurations, this bit is readonly with the appropriate value In default 10/100/1000 Mbps configuration, this bit is R_W The mac_portselect_o or mac_speed_o\\[1\\] signal reflects the value of this bit"]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the _G_MII CRS signal during frame transmission in the halfduplex mode This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions This bit is reserved _and RO_ in the fullduplexonly configurations"]
    #[inline(always)]
    pub fn disablecrs(&self) -> DISABLECRS_R {
        DISABLECRS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - InterFrame Gap These bits control the minimum IFG between frames during transmission 000: 96 bit times 001: 88 bit times 010: 80 bit times 111: 40 bit times In the halfduplex mode, the minimum IFG can be configured only for 64 bit times _IFG = 100_ Lower values are not considered In the 1000Mbps mode, the minimum IFG supported is 64 bit times _and above_ in the GMACCORE configuration and 80 bit times _and above_ in other configurations When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG"]
    #[inline(always)]
    pub fn interframegap(&self) -> INTERFRAMEGAP_R {
        INTERFRAMEGAP_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes _9,022 bytes for VLAN tagged frames_ without reporting a giant frame error in the receive frame status"]
    #[inline(always)]
    pub fn jumboframe(&self) -> JUMBOFRAME_R {
        JUMBOFRAME_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII halfduplex mode This bit is reserved _and RO_ in the 10/100 Mbps only or fullduplexonly configurations"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter The MAC can transfer frames of up to 16,383 bytes When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data _10,240 if JE is set high_ during transmission"]
    #[inline(always)]
    pub fn jabber(&self) -> JABBER_R {
        JABBER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver The MAC can receive frames of up to 16,383 bytes When this bit is reset, the MAC does not allow a receive frame which more than 2,048 bytes _10,240 if JE is set high_ or the value programmed in Register 55 _Watchdog Timeout Register_ The MAC cuts off any bytes received after the watchdog limit number of bytes"]
    #[inline(always)]
    pub fn watchdog(&self) -> WATCHDOG_R {
        WATCHDOG_R::new(((self.bits >> 23) & 1) != 0)
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
        f.debug_struct("CONFIG")
            .field("pltf", &self.pltf())
            .field("rx", &self.rx())
            .field("tx", &self.tx())
            .field("deferralcheck", &self.deferralcheck())
            .field("backofflimit", &self.backofflimit())
            .field("padcrcstrip", &self.padcrcstrip())
            .field("lud", &self.lud())
            .field("retry", &self.retry())
            .field("rxipcoffload", &self.rxipcoffload())
            .field("duplex", &self.duplex())
            .field("loopback", &self.loopback())
            .field("rxown", &self.rxown())
            .field("fespeed", &self.fespeed())
            .field("mii", &self.mii())
            .field("disablecrs", &self.disablecrs())
            .field("interframegap", &self.interframegap())
            .field("jumboframe", &self.jumboframe())
            .field("be", &self.be())
            .field("jabber", &self.jabber())
            .field("watchdog", &self.watchdog())
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
    pub fn pltf(&mut self) -> PLTF_W<'_, CONFIG_SPEC> {
        PLTF_W::new(self, 0)
    }
    #[doc = "Bit 2 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W<'_, CONFIG_SPEC> {
        RX_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W<'_, CONFIG_SPEC> {
        TX_W::new(self, 3)
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal _CRS_ on GMII or MII The defer time is not cumulative For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens Because of collision, the transmitter needs to back off and then defer again after back off completion In such a scenario, the deferral timer is reset to 0 and it is restarted When this bit is reset, the deferral check function is disabled and the MAC defers until the CRS signal goes inactive This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn deferralcheck(&mut self) -> DEFERRALCHECK_W<'_, CONFIG_SPEC> {
        DEFERRALCHECK_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - BackOff Limit The BackOff limit determines the random integer number _r_ of slot time delays _4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps_ for which the MAC waits before rescheduling a transmission attempt during retries after a collision This bit is applicable only in the halfduplex mode and is reserved _RO_ in the fullduplexonly configuration 00: k= min _n, 10_ 01: k = min _n, 8_ 10: k = min _n, 4_ 11: k = min _n, 1_ where n = retransmission attempt The random integer r takes the value in the range 0 ≤ r < 2k"]
    #[inline(always)]
    pub fn backofflimit(&mut self) -> BACKOFFLIMIT_W<'_, CONFIG_SPEC> {
        BACKOFFLIMIT_W::new(self, 5)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host Note: For information about how the settings of Bit 25 _CST_ and this bit impact the frame length, see Table 632"]
    #[inline(always)]
    pub fn padcrcstrip(&mut self) -> PADCRCSTRIP_W<'_, CONFIG_SPEC> {
        PADCRCSTRIP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: 0: Link Down 1: Link Up This bit is reserved _RO with default value_ and is enabled when the RGMII, SGMII, or SMII interface is enabled during core configuration"]
    #[inline(always)]
    pub fn lud(&mut self) -> LUD_W<'_, CONFIG_SPEC> {
        LUD_W::new(self, 8)
    }
    #[doc = "Bit 9 - Disable Retry When this bit is set, the MAC attempts only one transmission When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status When this bit is reset, the MAC attempts retries based on the settings of the BL field _Bits \\[6:5\\]_ This bit is applicable only in the halfduplex mode and is reserved _RO with default value_ in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn retry(&mut self) -> RETRY_W<'_, CONFIG_SPEC> {
        RETRY_W::new(self, 9)
    }
    #[doc = "Bit 10 - Checksum Offload When this bit is set, the MAC calculates the 16bit one’s complement of the one’s complement sum of all received Ethernet frame payloads It also checks whether the IPv4 Header checksum _assumed to be bytes 2526 or 2930 _VLAN tagged_ of the received Ethernet frame_ is correct for the received frame and gives the status in the receive status word The MAC also appends the 16bit checksum calculated for the IP header datagram payload _bytes after the IPv4 header_ and appends it to the Ethernet frame transferred to the application _when Type 2 COE is deselected_ When this bit is reset, this function is disabled When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking When this bit is reset, the COE function in the receiver is disabled and the corresponding PCE and IP HCE status bits _see Table 310 on page 138_ are always cleared If the IP Checksum Offload feature is not enabled during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn rxipcoffload(&mut self) -> RXIPCOFFLOAD_W<'_, CONFIG_SPEC> {
        RXIPCOFFLOAD_W::new(self, 10)
    }
    #[doc = "Bit 11 - Duplex Mode When this bit is set, the MAC operates in the fullduplex mode where it can transmit and receive simultaneously This bit is RO with default value of 1'b1 in the fullduplexonly configuration"]
    #[inline(always)]
    pub fn duplex(&mut self) -> DUPLEX_W<'_, CONFIG_SPEC> {
        DUPLEX_W::new(self, 11)
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII The _G_MII Receive clock input _clk_rx_i_ is required for the loopback to work properly, because the Transmit clock is not loopedback internally"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W<'_, CONFIG_SPEC> {
        LOOPBACK_W::new(self, 12)
    }
    #[doc = "Bit 13 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the halfduplex mode When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting This bit is not applicable if the MAC is operating in the fullduplex mode This bit is reserved _RO with default value_ if the MAC is configured for the fullduplexonly operation"]
    #[inline(always)]
    pub fn rxown(&mut self) -> RXOWN_W<'_, CONFIG_SPEC> {
        RXOWN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: 0: 10 Mbps 1: 100 Mbps This bit is reserved _RO_ by default and is enabled only when the parameter SPEED_SELECT = Enabled This bit generates link speed encoding when Bit 24 _TC_ is set in the RGMII, SMII, or SGMII mode This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal _mac_speed_o\\[0\\]_ to reflect the value of this bit in the mac_speed_o signal In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal _mac_speed_o\\[0\\]_ to reflect its value in the mac_speed_o signal"]
    #[inline(always)]
    pub fn fespeed(&mut self) -> FESPEED_W<'_, CONFIG_SPEC> {
        FESPEED_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port Select This bit selects the Ethernet line speed 0: For 1000 Mbps operations 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed In the 10/100 Mbpsonly _always 1_ or 1000 Mbpsonly _always 0_ configurations, this bit is readonly with the appropriate value In default 10/100/1000 Mbps configuration, this bit is R_W The mac_portselect_o or mac_speed_o\\[1\\] signal reflects the value of this bit"]
    #[inline(always)]
    pub fn mii(&mut self) -> MII_W<'_, CONFIG_SPEC> {
        MII_W::new(self, 15)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the _G_MII CRS signal during frame transmission in the halfduplex mode This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions This bit is reserved _and RO_ in the fullduplexonly configurations"]
    #[inline(always)]
    pub fn disablecrs(&mut self) -> DISABLECRS_W<'_, CONFIG_SPEC> {
        DISABLECRS_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - InterFrame Gap These bits control the minimum IFG between frames during transmission 000: 96 bit times 001: 88 bit times 010: 80 bit times 111: 40 bit times In the halfduplex mode, the minimum IFG can be configured only for 64 bit times _IFG = 100_ Lower values are not considered In the 1000Mbps mode, the minimum IFG supported is 64 bit times _and above_ in the GMACCORE configuration and 80 bit times _and above_ in other configurations When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG"]
    #[inline(always)]
    pub fn interframegap(&mut self) -> INTERFRAMEGAP_W<'_, CONFIG_SPEC> {
        INTERFRAMEGAP_W::new(self, 17)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes _9,022 bytes for VLAN tagged frames_ without reporting a giant frame error in the receive frame status"]
    #[inline(always)]
    pub fn jumboframe(&mut self) -> JUMBOFRAME_W<'_, CONFIG_SPEC> {
        JUMBOFRAME_W::new(self, 20)
    }
    #[doc = "Bit 21 - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII halfduplex mode This bit is reserved _and RO_ in the 10/100 Mbps only or fullduplexonly configurations"]
    #[inline(always)]
    pub fn be(&mut self) -> BE_W<'_, CONFIG_SPEC> {
        BE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter The MAC can transfer frames of up to 16,383 bytes When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data _10,240 if JE is set high_ during transmission"]
    #[inline(always)]
    pub fn jabber(&mut self) -> JABBER_W<'_, CONFIG_SPEC> {
        JABBER_W::new(self, 22)
    }
    #[doc = "Bit 23 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver The MAC can receive frames of up to 16,383 bytes When this bit is reset, the MAC does not allow a receive frame which more than 2,048 bytes _10,240 if JE is set high_ or the value programmed in Register 55 _Watchdog Timeout Register_ The MAC cuts off any bytes received after the watchdog limit number of bytes"]
    #[inline(always)]
    pub fn watchdog(&mut self) -> WATCHDOG_W<'_, CONFIG_SPEC> {
        WATCHDOG_W::new(self, 23)
    }
    #[doc = "Bit 24 - Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port When this bit is reset, no such information is driven to the PHY This bit is reserved _and RO_ if the RGMII, SMII, or SGMII PHY port is not selected during core configuration The details of this feature are explained in the following sections: “Reduced Gigabit Media Independent Interface” on page 249 “Serial Media Independent Interface” on page 245 “Serial Gigabit Media Independent Interface” on page 257"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<'_, CONFIG_SPEC> {
        TC_W::new(self, 24)
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames When this bit is set, the last 4 bytes _FCS_ of all frames of Ether type _Length/Type field greater than or equal to 1,536_ are stripped and dropped before forwarding the frame to the application This function is not valid when the IP Checksum Engine _Type 1_ is enabled in the MAC receiver This function is valid when Type 2 Checksum Offload Engine is enabled Note: For information about how the settings of Bit 7 _ACS_ and this bit impact the frame length, see Table 632"]
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W<'_, CONFIG_SPEC> {
        CST_W::new(self, 25)
    }
    #[doc = "Bit 26 - SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted This bit is reserved if the SMII PHY port is not selected during core configuration"]
    #[inline(always)]
    pub fn sfterr(&mut self) -> SFTERR_W<'_, CONFIG_SPEC> {
        SFTERR_W::new(self, 26)
    }
    #[doc = "Bit 27 - IEEE 8023as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets When Bit 20 _JE_ is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames When this bit is reset and Bit 20 _JE_ is not set, the MAC considers all received frames of size more than 1,518 bytes _1,522 bytes for tagged_ as Giant frames When Bit 20 is set, setting this bit has no effect on Giant Frame status For more information about how the setting of this bit and Bit 20 impact the Giant frame status, see Table 631"]
    #[inline(always)]
    pub fn twokpe(&mut self) -> TWOKPE_W<'_, CONFIG_SPEC> {
        TWOKPE_W::new(self, 27)
    }
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames Bit 30 specifies which MAC Address register _0 or 1_ is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation 2'b10: If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers _registers 16 and 17_ in the SA field of all transmitted frames If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers _registers 18 and 19_ in the SA field of all transmitted frames 2'b11: If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers _registers 16 and 17_ in the SA field of all transmitted frames If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers _registers 18 and 19_ in the SA field of all transmitted frames Note: Changes to this field take effect only on the start of a frame If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration"]
    #[inline(always)]
    pub fn sarc(&mut self) -> SARC_W<'_, CONFIG_SPEC> {
        SARC_W::new(self, 28)
    }
}
#[doc = "MAC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {}
