#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    register0_macconfigurationregister: REGISTER0_MACCONFIGURATIONREGISTER,
    register1_macframefilter: REGISTER1_MACFRAMEFILTER,
    register2_hashtablehighregister: REGISTER2_HASHTABLEHIGHREGISTER,
    register3_hashtablelowregister: REGISTER3_HASHTABLELOWREGISTER,
    register4_gmiiaddressregister: REGISTER4_GMIIADDRESSREGISTER,
    register5_gmiidataregister: REGISTER5_GMIIDATAREGISTER,
    register6_flowcontrolregister: REGISTER6_FLOWCONTROLREGISTER,
    register7_vlantagregister: REGISTER7_VLANTAGREGISTER,
    register8_versionregister: REGISTER8_VERSIONREGISTER,
    register9_debugregister: REGISTER9_DEBUGREGISTER,
    register10_remotewakeupframefilterregister: REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER,
    register11_pmtcontrolandstatusregister: REGISTER11_PMTCONTROLANDSTATUSREGISTER,
    register12_lpicontrolandstatusregister: REGISTER12_LPICONTROLANDSTATUSREGISTER,
    register13_lpitimerscontrolregister: REGISTER13_LPITIMERSCONTROLREGISTER,
    register14_interruptstatusregister: REGISTER14_INTERRUPTSTATUSREGISTER,
    register15_interruptmaskregister: REGISTER15_INTERRUPTMASKREGISTER,
    register16_macaddress0highregister: REGISTER16_MACADDRESS0HIGHREGISTER,
    register17_macaddress0lowregister: REGISTER17_MACADDRESS0LOWREGISTER,
    register18_macaddress1highregister: REGISTER18_MACADDRESS1HIGHREGISTER,
    register19_macaddress1lowregister: REGISTER19_MACADDRESS1LOWREGISTER,
    macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress:
        MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS,
    macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress:
        MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS,
    macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress:
        MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS,
    macaddress3lowregistercontainsthelower32bitsofthefourthmacaddress:
        MACADDRESS3LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFOURTHMACADDRESS,
    macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress:
        MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS,
    macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress:
        MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS,
    macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress:
        MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS,
    macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress:
        MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS,
    macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress:
        MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS,
    macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress:
        MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS,
    macaddress7highregistercontainsthehigher16bitsoftheeighthmacaddress:
        MACADDRESS7HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEEIGHTHMACADDRESS,
    macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress:
        MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS,
    _reserved32: [u8; 0x40],
    register48_ancontrolregister: REGISTER48_ANCONTROLREGISTER,
    register49_anstatusregister: REGISTER49_ANSTATUSREGISTER,
    register50_autonegotiationadvertisementregister:
        REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER,
    register51_autonegotiationlinkpartnerabilityregister:
        REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER,
    register52_autonegotiationexpansionregister: REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER,
    register53_tbiextendedstatusregister: REGISTER53_TBIEXTENDEDSTATUSREGISTER,
    register54_sgmii_rgmii_smiicontrolandstatusregister:
        REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER,
    register55_watchdogtimeoutregister: REGISTER55_WATCHDOGTIMEOUTREGISTER,
    register56_generalpurposeioregister: REGISTER56_GENERALPURPOSEIOREGISTER,
    _reserved41: [u8; 0x031c],
    register256_layer3andlayer4controlregister0: REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0,
    register257_layer4addressregister0: REGISTER257_LAYER4ADDRESSREGISTER0,
    _reserved43: [u8; 0x08],
    register260_layer3address0register0: REGISTER260_LAYER3ADDRESS0REGISTER0,
    register261_layer3address1register0: REGISTER261_LAYER3ADDRESS1REGISTER0,
    register262_layer3address2register0: REGISTER262_LAYER3ADDRESS2REGISTER0,
    register263_layer3address3register0: REGISTER263_LAYER3ADDRESS3REGISTER0,
    _reserved47: [u8; 0xe0],
    register320_hashtableregister0: REGISTER320_HASHTABLEREGISTER0,
    _reserved48: [u8; 0x80],
    register353_vlantaginclusionorreplacementregister:
        REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER,
    register354_vlanhashtableregister: REGISTER354_VLANHASHTABLEREGISTER,
    _reserved50: [u8; 0x0174],
    register448_timestampcontrolregister: REGISTER448_TIMESTAMPCONTROLREGISTER,
    register449_subsecondincrementregister: REGISTER449_SUBSECONDINCREMENTREGISTER,
    register450_systemtimesecondsregister: REGISTER450_SYSTEMTIMESECONDSREGISTER,
    register451_systemtimenanosecondsregister: REGISTER451_SYSTEMTIMENANOSECONDSREGISTER,
    register452_systemtimesecondsupdateregister: REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER,
    register453_systemtimenanosecondsupdateregister:
        REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER,
    register454_timestampaddendregister: REGISTER454_TIMESTAMPADDENDREGISTER,
    register455_targettimesecondsregister: REGISTER455_TARGETTIMESECONDSREGISTER,
    register456_targettimenanosecondsregister: REGISTER456_TARGETTIMENANOSECONDSREGISTER,
    register457_systemtimehigherwordsecondsregister:
        REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER,
    register458_timestampstatusregister: REGISTER458_TIMESTAMPSTATUSREGISTER,
    register459_ppscontrolregister: REGISTER459_PPSCONTROLREGISTER,
    register460_auxiliarytimestampnanosecondsregister:
        REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER,
    register461_auxiliarytimestampsecondsregister: REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER,
    register462_avmaccontrolregister: REGISTER462_AVMACCONTROLREGISTER,
    _reserved65: [u8; 0x24],
    register472_pps0intervalregister: REGISTER472_PPS0INTERVALREGISTER,
    register473_pps0widthregister: REGISTER473_PPS0WIDTHREGISTER,
    _reserved67: [u8; 0x18],
    register480_pps1targettimesecondsregister: REGISTER480_PPS1TARGETTIMESECONDSREGISTER,
    register481_pps1targettimenanosecondsregister: REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER,
    _reserved69: [u8; 0xf8],
    register544_macaddress32highregister: REGISTER544_MACADDRESS32HIGHREGISTER,
    _reserved70: [u8; 0x077c],
    register0_busmoderegister: REGISTER0_BUSMODEREGISTER,
    register1_transmitpolldemandregister: REGISTER1_TRANSMITPOLLDEMANDREGISTER,
    register2_receivepolldemandregister: REGISTER2_RECEIVEPOLLDEMANDREGISTER,
    register3_receivedescriptorlistaddressregister: REGISTER3_RECEIVEDESCRIPTORLISTADDRESSREGISTER,
    register4_transmitdescriptorlistaddressregister:
        REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER,
    register5_statusregister: REGISTER5_STATUSREGISTER,
    register6_operationmoderegister: REGISTER6_OPERATIONMODEREGISTER,
    register7_interruptenableregister: REGISTER7_INTERRUPTENABLEREGISTER,
    register8_missedframeandbufferoverflowcounterregister:
        REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER,
    register9_receiveinterruptwatchdogtimerregister:
        REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER,
    register10_axibusmoderegister: REGISTER10_AXIBUSMODEREGISTER,
    register11_ahboraxistatusregister: REGISTER11_AHBORAXISTATUSREGISTER,
    _reserved82: [u8; 0x18],
    register18_currenthosttransmitdescriptorregister:
        REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER,
    register19_currenthostreceivedescriptorregister:
        REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER,
    register20_currenthosttransmitbufferaddressregister:
        REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER,
    register21_currenthostreceivebufferaddressregister:
        REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER,
    register22_hwfeatureregister: REGISTER22_HWFEATUREREGISTER,
    _reserved87: [u8; 0xa4],
    register64_channel1busmoderegister: REGISTER64_CHANNEL1BUSMODEREGISTER,
    register65_channel1transmitpolldemandregister: REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER,
    register66_channel1receivepolldemandregister: REGISTER66_CHANNEL1RECEIVEPOLLDEMANDREGISTER,
    register67_channel1receivedescriptorlistaddressregister:
        REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER,
    register68_channel1transmitdescriptorlistaddressregister:
        REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER,
    register69_channel1statusregister: REGISTER69_CHANNEL1STATUSREGISTER,
    register70_channel1operationmoderegister: REGISTER70_CHANNEL1OPERATIONMODEREGISTER,
    register71_channel1interruptenableregister: REGISTER71_CHANNEL1INTERRUPTENABLEREGISTER,
    register72_channel1missedframeandbufferoverflowcounterregister:
        REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER,
    register73_channel1receiveinterruptwatchdogtimerregister:
        REGISTER73_CHANNEL1RECEIVEINTERRUPTWATCHDOGTIMERREGISTER,
    _reserved97: [u8; 0x08],
    register76_channel1slotfunctioncontrolandstatusregister:
        REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER,
    _reserved98: [u8; 0x14],
    register82_channel1currenthosttransmitdescriptorregister:
        REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER,
    register83_channel1currenthostreceivedescriptorregister:
        REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER,
    register84_channel1currenthosttransmitbufferaddressregister:
        REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER,
    register85_channel1currenthostreceivebufferaddressregister:
        REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER,
    _reserved102: [u8; 0x08],
    register88_channel1cbscontrolregister: REGISTER88_CHANNEL1CBSCONTROLREGISTER,
    register89_channel1cbsstatusregister: REGISTER89_CHANNEL1CBSSTATUSREGISTER,
    register90_channel1idleslopecreditregister: REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER,
    register91_channel1sendslopecreditregister: REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER,
    register92_channel1hicreditregister: REGISTER92_CHANNEL1HICREDITREGISTER,
    register93_channel1locreditregister: REGISTER93_CHANNEL1LOCREDITREGISTER,
    _reserved108: [u8; 0x88],
    register128_channel2busmoderegister: REGISTER128_CHANNEL2BUSMODEREGISTER,
    register129_channel2transmitpolldemandregister: REGISTER129_CHANNEL2TRANSMITPOLLDEMANDREGISTER,
    register130_channel2receivepolldemandregister: REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER,
    register131_channel2receivedescriptorlistaddressregister:
        REGISTER131_CHANNEL2RECEIVEDESCRIPTORLISTADDRESSREGISTER,
    register132_channel2transmitdescriptorlistaddressregister:
        REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER,
    register133_channel2statusregister: REGISTER133_CHANNEL2STATUSREGISTER,
    register134_channel2operationmoderegister: REGISTER134_CHANNEL2OPERATIONMODEREGISTER,
    register135_channel2interruptenableregister: REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER,
    register136_channel2missedframeandbufferoverflowcounterregister:
        REGISTER136_CHANNEL2MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER,
    register137_channel2receiveinterruptwatchdogtimerregister:
        REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER,
    _reserved118: [u8; 0x20],
    register146_channel2currenthosttransmitdescriptorregister:
        REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER,
    register147_channel2currenthostreceivedescriptorregister:
        REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER,
    register148_channel2currenthosttransmitbufferaddressregister:
        REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER,
    register149_channel2currenthostreceivebufferaddressregister:
        REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER,
    _reserved122: [u8; 0x08],
    register152_channel2cbscontrolregister: REGISTER152_CHANNEL2CBSCONTROLREGISTER,
    register153_channel2cbsstatusregister: REGISTER153_CHANNEL2CBSSTATUSREGISTER,
    register154_channel2idleslopecreditregister: REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER,
    register155_channel2sendslopecreditregister: REGISTER155_CHANNEL2SENDSLOPECREDITREGISTER,
    register156_channel2hicreditregister: REGISTER156_CHANNEL2HICREDITREGISTER,
    register157_channel2locreditregister: REGISTER157_CHANNEL2LOCREDITREGISTER,
}
impl RegisterBlock {
    #[doc = "0x00 - This is the operation mode register for the MAC"]
    #[inline(always)]
    pub const fn register0_macconfigurationregister(&self) -> &REGISTER0_MACCONFIGURATIONREGISTER {
        &self.register0_macconfigurationregister
    }
    #[doc = "0x04 - Contains the frame filtering controls"]
    #[inline(always)]
    pub const fn register1_macframefilter(&self) -> &REGISTER1_MACFRAMEFILTER {
        &self.register1_macframefilter
    }
    #[doc = "0x08 - Contains the higher 32 bits of the Multicast Hash table This register is present only when you select the 64bit Hash filter function in coreConsultant _See Table 79_"]
    #[inline(always)]
    pub const fn register2_hashtablehighregister(&self) -> &REGISTER2_HASHTABLEHIGHREGISTER {
        &self.register2_hashtablehighregister
    }
    #[doc = "0x0c - Contains the lower 32 bits of the Multicast Hash table This register is present only when you select the Hash filter function in coreConsultant _See Table 79_"]
    #[inline(always)]
    pub const fn register3_hashtablelowregister(&self) -> &REGISTER3_HASHTABLELOWREGISTER {
        &self.register3_hashtablelowregister
    }
    #[doc = "0x10 - Controls the management cycles to an external PHY This register is present only when you select the Station Management _MDIO_ feature in coreConsultant _See Table 726_"]
    #[inline(always)]
    pub const fn register4_gmiiaddressregister(&self) -> &REGISTER4_GMIIADDRESSREGISTER {
        &self.register4_gmiiaddressregister
    }
    #[doc = "0x14 - Contains the data to be written to or read from the PHY register This register is present only when you select the Station Management _MDIO_ feature in coreConsultant _See Table 726_"]
    #[inline(always)]
    pub const fn register5_gmiidataregister(&self) -> &REGISTER5_GMIIDATAREGISTER {
        &self.register5_gmiidataregister
    }
    #[doc = "0x18 - Controls the generation of control frames"]
    #[inline(always)]
    pub const fn register6_flowcontrolregister(&self) -> &REGISTER6_FLOWCONTROLREGISTER {
        &self.register6_flowcontrolregister
    }
    #[doc = "0x1c - Identifies IEEE 8021Q VLAN type frames"]
    #[inline(always)]
    pub const fn register7_vlantagregister(&self) -> &REGISTER7_VLANTAGREGISTER {
        &self.register7_vlantagregister
    }
    #[doc = "0x20 - Identifies the version of the Core"]
    #[inline(always)]
    pub const fn register8_versionregister(&self) -> &REGISTER8_VERSIONREGISTER {
        &self.register8_versionregister
    }
    #[doc = "0x24 - Gives the status of various internal blocks for debugging"]
    #[inline(always)]
    pub const fn register9_debugregister(&self) -> &REGISTER9_DEBUGREGISTER {
        &self.register9_debugregister
    }
    #[doc = "0x28 - Remote Wake-Up Frame Filter Register"]
    #[inline(always)]
    pub const fn register10_remotewakeupframefilterregister(
        &self,
    ) -> &REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER {
        &self.register10_remotewakeupframefilterregister
    }
    #[doc = "0x2c - PMT Control and Status Register. This register is present only when you select the PMT module in coreConsultant."]
    #[inline(always)]
    pub const fn register11_pmtcontrolandstatusregister(
        &self,
    ) -> &REGISTER11_PMTCONTROLANDSTATUSREGISTER {
        &self.register11_pmtcontrolandstatusregister
    }
    #[doc = "0x30 - Controls the Low Power Idle _LPI_ operations and provides the LPI status of the core This register is present only when you select the Energy Efficient Ethernet feature in coreConsultant"]
    #[inline(always)]
    pub const fn register12_lpicontrolandstatusregister(
        &self,
    ) -> &REGISTER12_LPICONTROLANDSTATUSREGISTER {
        &self.register12_lpicontrolandstatusregister
    }
    #[doc = "0x34 - Controls the timeout values in LPI states This register is present only when you select the Energy Efficient Ethernet feature in coreConsultant"]
    #[inline(always)]
    pub const fn register13_lpitimerscontrolregister(
        &self,
    ) -> &REGISTER13_LPITIMERSCONTROLREGISTER {
        &self.register13_lpitimerscontrolregister
    }
    #[doc = "0x38 - Contains the interrupt status"]
    #[inline(always)]
    pub const fn register14_interruptstatusregister(&self) -> &REGISTER14_INTERRUPTSTATUSREGISTER {
        &self.register14_interruptstatusregister
    }
    #[doc = "0x3c - Contains the masks for generating the interrupts"]
    #[inline(always)]
    pub const fn register15_interruptmaskregister(&self) -> &REGISTER15_INTERRUPTMASKREGISTER {
        &self.register15_interruptmaskregister
    }
    #[doc = "0x40 - Contains the higher 16 bits of the first MAC address"]
    #[inline(always)]
    pub const fn register16_macaddress0highregister(&self) -> &REGISTER16_MACADDRESS0HIGHREGISTER {
        &self.register16_macaddress0highregister
    }
    #[doc = "0x44 - Contains the lower 32 bits of the first MAC address"]
    #[inline(always)]
    pub const fn register17_macaddress0lowregister(&self) -> &REGISTER17_MACADDRESS0LOWREGISTER {
        &self.register17_macaddress0lowregister
    }
    #[doc = "0x48 - Contains the higher 16 bits of the second MAC address This register is present only when Enable MAC Address1 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub const fn register18_macaddress1highregister(&self) -> &REGISTER18_MACADDRESS1HIGHREGISTER {
        &self.register18_macaddress1highregister
    }
    #[doc = "0x4c - Contains the lower 32 bits of the second MAC address This register is present only when Enable MAC Address1 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub const fn register19_macaddress1lowregister(&self) -> &REGISTER19_MACADDRESS1LOWREGISTER {
        &self.register19_macaddress1lowregister
    }
    #[doc = "0x50 - Reserved"]
    #[inline(always)]
    pub const fn macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress(
        &self,
    ) -> &MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS {
        &self.macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress
    }
    #[doc = "0x54 - Reserved"]
    #[inline(always)]
    pub const fn macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress(
        &self,
    ) -> &MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS {
        &self.macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress
    }
    #[doc = "0x58 - Reserved"]
    #[inline(always)]
    pub const fn macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress(
        &self,
    ) -> &MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS {
        &self.macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress
    }
    #[doc = "0x5c - Reserved"]
    #[inline(always)]
    pub const fn macaddress3lowregistercontainsthelower32bitsofthefourthmacaddress(
        &self,
    ) -> &MACADDRESS3LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFOURTHMACADDRESS {
        &self.macaddress3lowregistercontainsthelower32bitsofthefourthmacaddress
    }
    #[doc = "0x60 - Reserved"]
    #[inline(always)]
    pub const fn macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress(
        &self,
    ) -> &MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS {
        &self.macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress
    }
    #[doc = "0x64 - Reserved"]
    #[inline(always)]
    pub const fn macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress(
        &self,
    ) -> &MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS {
        &self.macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress
    }
    #[doc = "0x68 - Reserved"]
    #[inline(always)]
    pub const fn macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress(
        &self,
    ) -> &MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS {
        &self.macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress
    }
    #[doc = "0x6c - Reserved"]
    #[inline(always)]
    pub const fn macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress(
        &self,
    ) -> &MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS {
        &self.macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress
    }
    #[doc = "0x70 - Reserved"]
    #[inline(always)]
    pub const fn macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress(
        &self,
    ) -> &MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS {
        &self.macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress
    }
    #[doc = "0x74 - Reserved"]
    #[inline(always)]
    pub const fn macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress(
        &self,
    ) -> &MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS {
        &self.macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress
    }
    #[doc = "0x78 - Reserved"]
    #[inline(always)]
    pub const fn macaddress7highregistercontainsthehigher16bitsoftheeighthmacaddress(
        &self,
    ) -> &MACADDRESS7HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEEIGHTHMACADDRESS {
        &self.macaddress7highregistercontainsthehigher16bitsoftheeighthmacaddress
    }
    #[doc = "0x7c - Reserved"]
    #[inline(always)]
    pub const fn macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress(
        &self,
    ) -> &MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS {
        &self.macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress
    }
    #[doc = "0xc0 - Enables and/or restarts autonegotiation This register also enables the Physical Coding Sublayer _PCS_ loopback This register is present only when you select the TBI, RTBI, or SGMII interface in coreConsultant"]
    #[inline(always)]
    pub const fn register48_ancontrolregister(&self) -> &REGISTER48_ANCONTROLREGISTER {
        &self.register48_ancontrolregister
    }
    #[doc = "0xc4 - Indicates the link and autonegotiation status This register is present only when you select the TBI, RTBI, or SGMII interface in coreConsultant"]
    #[inline(always)]
    pub const fn register49_anstatusregister(&self) -> &REGISTER49_ANSTATUSREGISTER {
        &self.register49_anstatusregister
    }
    #[doc = "0xc8 - This register is configured before autonegotiation begins It contains the advertised ability of the MAC This register is present only when you select the TBI or RTBI interface in coreConsultant"]
    #[inline(always)]
    pub const fn register50_autonegotiationadvertisementregister(
        &self,
    ) -> &REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER {
        &self.register50_autonegotiationadvertisementregister
    }
    #[doc = "0xcc - Contains the advertised ability of the link partner Its value is valid after successful completion of autonegotiation or when a new base page has been received _indicated in the AutoNegotiation Expansion Register_ This register is present only when you select the TBI or RTBI interface in coreConsultant"]
    #[inline(always)]
    pub const fn register51_autonegotiationlinkpartnerabilityregister(
        &self,
    ) -> &REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER {
        &self.register51_autonegotiationlinkpartnerabilityregister
    }
    #[doc = "0xd0 - Indicates whether a new base page has been received from the link partner This register is present only when you select the TBI or RTBI interface in coreConsultant"]
    #[inline(always)]
    pub const fn register52_autonegotiationexpansionregister(
        &self,
    ) -> &REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER {
        &self.register52_autonegotiationexpansionregister
    }
    #[doc = "0xd4 - Indicates all modes of operation of the MAC This register is present only when you select the TBI or RTBI interface in coreConsultant"]
    #[inline(always)]
    pub const fn register53_tbiextendedstatusregister(
        &self,
    ) -> &REGISTER53_TBIEXTENDEDSTATUSREGISTER {
        &self.register53_tbiextendedstatusregister
    }
    #[doc = "0xd8 - Indicates the status signals received from the PHY through the SGMII, RGMII, or SMII interface This register is present only when you select the SGMII, RGMII, or SMII interface in coreConsultant"]
    #[inline(always)]
    pub const fn register54_sgmii_rgmii_smiicontrolandstatusregister(
        &self,
    ) -> &REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER {
        &self.register54_sgmii_rgmii_smiicontrolandstatusregister
    }
    #[doc = "0xdc - Controls the watchdog timeout for received frames"]
    #[inline(always)]
    pub const fn register55_watchdogtimeoutregister(&self) -> &REGISTER55_WATCHDOGTIMEOUTREGISTER {
        &self.register55_watchdogtimeoutregister
    }
    #[doc = "0xe0 - Provides the control to drive up to 4 bits of output ports _GPO_ and also provides the status of up to 4 input ports _GPIS_"]
    #[inline(always)]
    pub const fn register56_generalpurposeioregister(
        &self,
    ) -> &REGISTER56_GENERALPURPOSEIOREGISTER {
        &self.register56_generalpurposeioregister
    }
    #[doc = "0x400 - Controls the operations of the Layer 3 and Layer 4 frame filtering"]
    #[inline(always)]
    pub const fn register256_layer3andlayer4controlregister0(
        &self,
    ) -> &REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0 {
        &self.register256_layer3andlayer4controlregister0
    }
    #[doc = "0x404 - Layer 4 Port number field It contains the 16bit Source and Destination Port numbers of the TCP or UDP frame"]
    #[inline(always)]
    pub const fn register257_layer4addressregister0(&self) -> &REGISTER257_LAYER4ADDRESSREGISTER0 {
        &self.register257_layer4addressregister0
    }
    #[doc = "0x410 - Layer 3 Address field For IPv4 frames, it contains the 32bit IP Source Address field For IPv6 frames, it contains Bits \\[31:0\\] of the 128bit IP Source Address or Destination Address field"]
    #[inline(always)]
    pub const fn register260_layer3address0register0(
        &self,
    ) -> &REGISTER260_LAYER3ADDRESS0REGISTER0 {
        &self.register260_layer3address0register0
    }
    #[doc = "0x414 - Layer 3 Address 1 field For IPv4 frames, it contains the 32bit IP Destination Address field For IPv6 frames, it contains Bits \\[63:32\\] of the 128bit IP Source Address or Destination Address field"]
    #[inline(always)]
    pub const fn register261_layer3address1register0(
        &self,
    ) -> &REGISTER261_LAYER3ADDRESS1REGISTER0 {
        &self.register261_layer3address1register0
    }
    #[doc = "0x418 - Layer 3 Address 2 field This register is reserved for IPv4 frames For IPv6 frames, it contains Bits \\[95:64\\] of the 128bit IP Source Address or Destination Address field"]
    #[inline(always)]
    pub const fn register262_layer3address2register0(
        &self,
    ) -> &REGISTER262_LAYER3ADDRESS2REGISTER0 {
        &self.register262_layer3address2register0
    }
    #[doc = "0x41c - Layer 3 Address 3 field This register is reserved for IPv4 frames For IPv6 frames, it contains Bits \\[127:96\\] of the 128bit IP Source Address or Destination Address field"]
    #[inline(always)]
    pub const fn register263_layer3address3register0(
        &self,
    ) -> &REGISTER263_LAYER3ADDRESS3REGISTER0 {
        &self.register263_layer3address3register0
    }
    #[doc = "0x500 - This register contains the first 32 bits of the hash table when the width of the Hash table is 128 bits or 256 bits"]
    #[inline(always)]
    pub const fn register320_hashtableregister0(&self) -> &REGISTER320_HASHTABLEREGISTER0 {
        &self.register320_hashtableregister0
    }
    #[doc = "0x584 - This register contains the VLAN tag for insertion into or replacement in the transmit frames"]
    #[inline(always)]
    pub const fn register353_vlantaginclusionorreplacementregister(
        &self,
    ) -> &REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER {
        &self.register353_vlantaginclusionorreplacementregister
    }
    #[doc = "0x588 - This register contains the VLAN hash table"]
    #[inline(always)]
    pub const fn register354_vlanhashtableregister(&self) -> &REGISTER354_VLANHASHTABLEREGISTER {
        &self.register354_vlanhashtableregister
    }
    #[doc = "0x700 - Controls the timestamp generation and update logic This register is present only when IEEE1588 timestamping is enabled during coreConsultant configuration"]
    #[inline(always)]
    pub const fn register448_timestampcontrolregister(
        &self,
    ) -> &REGISTER448_TIMESTAMPCONTROLREGISTER {
        &self.register448_timestampcontrolregister
    }
    #[doc = "0x704 - Contains the 8bit value by which the SubSecond register is incremented This register is present only when IEEE1588 timestamping is enabled without an external timestamp input"]
    #[inline(always)]
    pub const fn register449_subsecondincrementregister(
        &self,
    ) -> &REGISTER449_SUBSECONDINCREMENTREGISTER {
        &self.register449_subsecondincrementregister
    }
    #[doc = "0x708 - Contains the lower 32 bits of the seconds field of the system time This register is present only when IEEE1588 timestamping is enabled without an external timestamp input"]
    #[inline(always)]
    pub const fn register450_systemtimesecondsregister(
        &self,
    ) -> &REGISTER450_SYSTEMTIMESECONDSREGISTER {
        &self.register450_systemtimesecondsregister
    }
    #[doc = "0x70c - Contains 32 bits of the nanoseconds field of the system time This register is only present when IEEE1588 timestamping is enabled without an external timestamp input"]
    #[inline(always)]
    pub const fn register451_systemtimenanosecondsregister(
        &self,
    ) -> &REGISTER451_SYSTEMTIMENANOSECONDSREGISTER {
        &self.register451_systemtimenanosecondsregister
    }
    #[doc = "0x710 - Contains the lower 32 bits of the seconds field to be written to, added to, or subtracted from the System Time value This register is only present when IEEE1588 timestamping is enabled without an external timestamp input"]
    #[inline(always)]
    pub const fn register452_systemtimesecondsupdateregister(
        &self,
    ) -> &REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER {
        &self.register452_systemtimesecondsupdateregister
    }
    #[doc = "0x714 - Contains 32 bits of the nanoseconds field to be written to, added to, or subtracted from the System Time value This register is only present when IEEE1588 timestamping is enabled without an external timestamp input"]
    #[inline(always)]
    pub const fn register453_systemtimenanosecondsupdateregister(
        &self,
    ) -> &REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER {
        &self.register453_systemtimenanosecondsupdateregister
    }
    #[doc = "0x718 - This register is used by the software to readjust the clock frequency linearly to match the master clock frequency This register is only present when IEEE1588 timestamping is enabled without an external timestamp input"]
    #[inline(always)]
    pub const fn register454_timestampaddendregister(
        &self,
    ) -> &REGISTER454_TIMESTAMPADDENDREGISTER {
        &self.register454_timestampaddendregister
    }
    #[doc = "0x71c - Contains the higher 32 bits of time to be compared with the system time for interrupt event generation or to start the PPS signal output generation This register is present only when IEEE1588 timestamping is enabled without an external timestamp input"]
    #[inline(always)]
    pub const fn register455_targettimesecondsregister(
        &self,
    ) -> &REGISTER455_TARGETTIMESECONDSREGISTER {
        &self.register455_targettimesecondsregister
    }
    #[doc = "0x720 - Contains the lower 32 bits of time to be compared with the system time for interrupt event generation or to start the PPS signal output generation This register is present only when IEEE1588 timestamping is enabled without an external timestamp input"]
    #[inline(always)]
    pub const fn register456_targettimenanosecondsregister(
        &self,
    ) -> &REGISTER456_TARGETTIMENANOSECONDSREGISTER {
        &self.register456_targettimenanosecondsregister
    }
    #[doc = "0x724 - Contains the most significant 16bits of the timestamp seconds value This register is optional and can be selected using the parameter mentioned in “IEEE 1588 Timestamp Block” on page 492"]
    #[inline(always)]
    pub const fn register457_systemtimehigherwordsecondsregister(
        &self,
    ) -> &REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER {
        &self.register457_systemtimehigherwordsecondsregister
    }
    #[doc = "0x728 - Contains the PTP status This register is available only when the advanced IEEE 1588 timestamp feature is selected"]
    #[inline(always)]
    pub const fn register458_timestampstatusregister(
        &self,
    ) -> &REGISTER458_TIMESTAMPSTATUSREGISTER {
        &self.register458_timestampstatusregister
    }
    #[doc = "0x72c - This register is used to control the interval of the PPS signal output This register is available only when the advanced IEEE 1588 timestamp feature is selected"]
    #[inline(always)]
    pub const fn register459_ppscontrolregister(&self) -> &REGISTER459_PPSCONTROLREGISTER {
        &self.register459_ppscontrolregister
    }
    #[doc = "0x730 - Contains the lower 32 bits _nanoseconds field_ of the auxiliary timestamp register"]
    #[inline(always)]
    pub const fn register460_auxiliarytimestampnanosecondsregister(
        &self,
    ) -> &REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER {
        &self.register460_auxiliarytimestampnanosecondsregister
    }
    #[doc = "0x734 - Contains the lower 32 bits of the Seconds field of the auxiliary timestamp register"]
    #[inline(always)]
    pub const fn register461_auxiliarytimestampsecondsregister(
        &self,
    ) -> &REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER {
        &self.register461_auxiliarytimestampsecondsregister
    }
    #[doc = "0x738 - Controls the AV traffic and queue management in the MAC Receiver This register is present only when you select the AV feature in coreConsultant"]
    #[inline(always)]
    pub const fn register462_avmaccontrolregister(&self) -> &REGISTER462_AVMACCONTROLREGISTER {
        &self.register462_avmaccontrolregister
    }
    #[doc = "0x760 - Contains the number of units of subsecond increment value between the rising edges of PPS0 signal output This register is available only when the flexible PPS feature is selected"]
    #[inline(always)]
    pub const fn register472_pps0intervalregister(&self) -> &REGISTER472_PPS0INTERVALREGISTER {
        &self.register472_pps0intervalregister
    }
    #[doc = "0x764 - Contains the number of units of subsecond increment value between the rising and corresponding falling edges of PPS0 signal output This register is available only when the flexible PPS feature is selected"]
    #[inline(always)]
    pub const fn register473_pps0widthregister(&self) -> &REGISTER473_PPS0WIDTHREGISTER {
        &self.register473_pps0widthregister
    }
    #[doc = "0x780 - Contains the higher 32 bits of time to be compared with the system time to generate the interrupt event or to start generating the PPS1 output signal This register is present only when IEEE1588 timestamping is enabled without an external timestamp input and at least one additional PPS output is selected"]
    #[inline(always)]
    pub const fn register480_pps1targettimesecondsregister(
        &self,
    ) -> &REGISTER480_PPS1TARGETTIMESECONDSREGISTER {
        &self.register480_pps1targettimesecondsregister
    }
    #[doc = "0x784 - Contains the lower 32 bits of time to be compared with the system time to generate the interrupt event or to start generating the PPS1 output signal This register is present only when IEEE1588 timestamping is enabled without an external timestamp input and at least one additional PPS output is selected"]
    #[inline(always)]
    pub const fn register481_pps1targettimenanosecondsregister(
        &self,
    ) -> &REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER {
        &self.register481_pps1targettimenanosecondsregister
    }
    #[doc = "0x880 - Contains the higher 16 bits of the 33rd MAC address This register is present only when Enable Additional 32 MAC Address Registers is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub const fn register544_macaddress32highregister(
        &self,
    ) -> &REGISTER544_MACADDRESS32HIGHREGISTER {
        &self.register544_macaddress32highregister
    }
    #[doc = "0x1000 - Controls the Host Interface Mode"]
    #[inline(always)]
    pub const fn register0_busmoderegister(&self) -> &REGISTER0_BUSMODEREGISTER {
        &self.register0_busmoderegister
    }
    #[doc = "0x1004 - Used by the host to instruct the DMA to poll the Transmit Descriptor list"]
    #[inline(always)]
    pub const fn register1_transmitpolldemandregister(
        &self,
    ) -> &REGISTER1_TRANSMITPOLLDEMANDREGISTER {
        &self.register1_transmitpolldemandregister
    }
    #[doc = "0x1008 - Used by the host to instruct the DMA to poll the Receive Descriptor list"]
    #[inline(always)]
    pub const fn register2_receivepolldemandregister(
        &self,
    ) -> &REGISTER2_RECEIVEPOLLDEMANDREGISTER {
        &self.register2_receivepolldemandregister
    }
    #[doc = "0x100c - Points the DMA to the start of the Receive Descriptor list"]
    #[inline(always)]
    pub const fn register3_receivedescriptorlistaddressregister(
        &self,
    ) -> &REGISTER3_RECEIVEDESCRIPTORLISTADDRESSREGISTER {
        &self.register3_receivedescriptorlistaddressregister
    }
    #[doc = "0x1010 - Points the DMA to the start of the Transmit Descriptor list"]
    #[inline(always)]
    pub const fn register4_transmitdescriptorlistaddressregister(
        &self,
    ) -> &REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER {
        &self.register4_transmitdescriptorlistaddressregister
    }
    #[doc = "0x1014 - The Software driver _application_ reads this register during interrupt service routine or polling to determine the status of the DMA"]
    #[inline(always)]
    pub const fn register5_statusregister(&self) -> &REGISTER5_STATUSREGISTER {
        &self.register5_statusregister
    }
    #[doc = "0x1018 - Establishes the Receive and Transmit operating modes and command Note: This register is valid and present in the GMACMTL configuration"]
    #[inline(always)]
    pub const fn register6_operationmoderegister(&self) -> &REGISTER6_OPERATIONMODEREGISTER {
        &self.register6_operationmoderegister
    }
    #[doc = "0x101c - Enables the interrupts reported by the Status Register"]
    #[inline(always)]
    pub const fn register7_interruptenableregister(&self) -> &REGISTER7_INTERRUPTENABLEREGISTER {
        &self.register7_interruptenableregister
    }
    #[doc = "0x1020 - Contains the counters for discarded frames because no host Receive Descriptor was available or because of Receive FIFO Overflow"]
    #[inline(always)]
    pub const fn register8_missedframeandbufferoverflowcounterregister(
        &self,
    ) -> &REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER {
        &self.register8_missedframeandbufferoverflowcounterregister
    }
    #[doc = "0x1024 - Watchdog timeout for Receive Interrupt _RI_ from DMA"]
    #[inline(always)]
    pub const fn register9_receiveinterruptwatchdogtimerregister(
        &self,
    ) -> &REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER {
        &self.register9_receiveinterruptwatchdogtimerregister
    }
    #[doc = "0x1028 - Controls AXI master behavior _mainly controls burst splitting and number of outstanding requests_"]
    #[inline(always)]
    pub const fn register10_axibusmoderegister(&self) -> &REGISTER10_AXIBUSMODEREGISTER {
        &self.register10_axibusmoderegister
    }
    #[doc = "0x102c - Gives the idle status of the AHB master interface in the GMACAHB configuration Gives the idle status of the AXI master's read or write channel in the GMACAXI configuration"]
    #[inline(always)]
    pub const fn register11_ahboraxistatusregister(&self) -> &REGISTER11_AHBORAXISTATUSREGISTER {
        &self.register11_ahboraxistatusregister
    }
    #[doc = "0x1048 - Points to the start of current Transmit Descriptor read by the DMA"]
    #[inline(always)]
    pub const fn register18_currenthosttransmitdescriptorregister(
        &self,
    ) -> &REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER {
        &self.register18_currenthosttransmitdescriptorregister
    }
    #[doc = "0x104c - Points to the start of current Receive Descriptor read by the DMA"]
    #[inline(always)]
    pub const fn register19_currenthostreceivedescriptorregister(
        &self,
    ) -> &REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER {
        &self.register19_currenthostreceivedescriptorregister
    }
    #[doc = "0x1050 - Points to the current Transmit Buffer address read by the DMA"]
    #[inline(always)]
    pub const fn register20_currenthosttransmitbufferaddressregister(
        &self,
    ) -> &REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER {
        &self.register20_currenthosttransmitbufferaddressregister
    }
    #[doc = "0x1054 - Points to the current Receive Buffer address read by the DMA"]
    #[inline(always)]
    pub const fn register21_currenthostreceivebufferaddressregister(
        &self,
    ) -> &REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER {
        &self.register21_currenthostreceivebufferaddressregister
    }
    #[doc = "0x1058 - Indicates the presence of the optional features of the core"]
    #[inline(always)]
    pub const fn register22_hwfeatureregister(&self) -> &REGISTER22_HWFEATUREREGISTER {
        &self.register22_hwfeatureregister
    }
    #[doc = "0x1100 - Controls the Host Interface mode for Channel 1"]
    #[inline(always)]
    pub const fn register64_channel1busmoderegister(&self) -> &REGISTER64_CHANNEL1BUSMODEREGISTER {
        &self.register64_channel1busmoderegister
    }
    #[doc = "0x1104 - Used by the host to instruct the DMA to poll the Transmit Descriptor list"]
    #[inline(always)]
    pub const fn register65_channel1transmitpolldemandregister(
        &self,
    ) -> &REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER {
        &self.register65_channel1transmitpolldemandregister
    }
    #[doc = "0x1108 - Used by the Host to instruct the DMA to poll the Receive Descriptor list"]
    #[inline(always)]
    pub const fn register66_channel1receivepolldemandregister(
        &self,
    ) -> &REGISTER66_CHANNEL1RECEIVEPOLLDEMANDREGISTER {
        &self.register66_channel1receivepolldemandregister
    }
    #[doc = "0x110c - Points the DMA to the start of the Receive Descriptor list"]
    #[inline(always)]
    pub const fn register67_channel1receivedescriptorlistaddressregister(
        &self,
    ) -> &REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER {
        &self.register67_channel1receivedescriptorlistaddressregister
    }
    #[doc = "0x1110 - Points the DMA to the start of the Transmit Descriptor list"]
    #[inline(always)]
    pub const fn register68_channel1transmitdescriptorlistaddressregister(
        &self,
    ) -> &REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER {
        &self.register68_channel1transmitdescriptorlistaddressregister
    }
    #[doc = "0x1114 - The Software driver _application_ reads this register during interrupt service routine or polling to determine the status of the DMA Bits 29:26 are reserved for the Channel 1 Status Register"]
    #[inline(always)]
    pub const fn register69_channel1statusregister(&self) -> &REGISTER69_CHANNEL1STATUSREGISTER {
        &self.register69_channel1statusregister
    }
    #[doc = "0x1118 - Establishes the Receive and Transmit operating modes and command"]
    #[inline(always)]
    pub const fn register70_channel1operationmoderegister(
        &self,
    ) -> &REGISTER70_CHANNEL1OPERATIONMODEREGISTER {
        &self.register70_channel1operationmoderegister
    }
    #[doc = "0x111c - Enables the interrupts reported by the Status Register"]
    #[inline(always)]
    pub const fn register71_channel1interruptenableregister(
        &self,
    ) -> &REGISTER71_CHANNEL1INTERRUPTENABLEREGISTER {
        &self.register71_channel1interruptenableregister
    }
    #[doc = "0x1120 - Contains the counters for discarded frames because no host Receive Descriptor was available, and discarded frames because of Receive FIFO Overflow"]
    #[inline(always)]
    pub const fn register72_channel1missedframeandbufferoverflowcounterregister(
        &self,
    ) -> &REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER {
        &self.register72_channel1missedframeandbufferoverflowcounterregister
    }
    #[doc = "0x1124 - Watchdog timeout for Receive Interrupt _RI_ from DMA"]
    #[inline(always)]
    pub const fn register73_channel1receiveinterruptwatchdogtimerregister(
        &self,
    ) -> &REGISTER73_CHANNEL1RECEIVEINTERRUPTWATCHDOGTIMERREGISTER {
        &self.register73_channel1receiveinterruptwatchdogtimerregister
    }
    #[doc = "0x1130 - Contains the control bits for slot function and its status for Channel 1 transmit path"]
    #[inline(always)]
    pub const fn register76_channel1slotfunctioncontrolandstatusregister(
        &self,
    ) -> &REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER {
        &self.register76_channel1slotfunctioncontrolandstatusregister
    }
    #[doc = "0x1148 - Points to the start of current Transmit Descriptor read by the DMA"]
    #[inline(always)]
    pub const fn register82_channel1currenthosttransmitdescriptorregister(
        &self,
    ) -> &REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER {
        &self.register82_channel1currenthosttransmitdescriptorregister
    }
    #[doc = "0x114c - Points to the start of current Receive Descriptor read by the DMA"]
    #[inline(always)]
    pub const fn register83_channel1currenthostreceivedescriptorregister(
        &self,
    ) -> &REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER {
        &self.register83_channel1currenthostreceivedescriptorregister
    }
    #[doc = "0x1150 - Points to the current Transmit Buffer address read by the DMA"]
    #[inline(always)]
    pub const fn register84_channel1currenthosttransmitbufferaddressregister(
        &self,
    ) -> &REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER {
        &self.register84_channel1currenthosttransmitbufferaddressregister
    }
    #[doc = "0x1154 - Points to the current Receive Buffer address read by the DMA"]
    #[inline(always)]
    pub const fn register85_channel1currenthostreceivebufferaddressregister(
        &self,
    ) -> &REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER {
        &self.register85_channel1currenthostreceivebufferaddressregister
    }
    #[doc = "0x1160 - Controls the Channel 1 credit shaping operation on the transmit path"]
    #[inline(always)]
    pub const fn register88_channel1cbscontrolregister(
        &self,
    ) -> &REGISTER88_CHANNEL1CBSCONTROLREGISTER {
        &self.register88_channel1cbscontrolregister
    }
    #[doc = "0x1164 - Provides the average traffic transmitted in Channel 1"]
    #[inline(always)]
    pub const fn register89_channel1cbsstatusregister(
        &self,
    ) -> &REGISTER89_CHANNEL1CBSSTATUSREGISTER {
        &self.register89_channel1cbsstatusregister
    }
    #[doc = "0x1168 - Contains the idleSlope credit value required for the creditbased shaper algorithm for Channel 1"]
    #[inline(always)]
    pub const fn register90_channel1idleslopecreditregister(
        &self,
    ) -> &REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER {
        &self.register90_channel1idleslopecreditregister
    }
    #[doc = "0x116c - Contains the sendSlope credit value required for the creditbased shaper algorithm for Channel 1"]
    #[inline(always)]
    pub const fn register91_channel1sendslopecreditregister(
        &self,
    ) -> &REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER {
        &self.register91_channel1sendslopecreditregister
    }
    #[doc = "0x1170 - Contains the hiCredit value required for the creditbased shaper algorithm for Channel 1"]
    #[inline(always)]
    pub const fn register92_channel1hicreditregister(
        &self,
    ) -> &REGISTER92_CHANNEL1HICREDITREGISTER {
        &self.register92_channel1hicreditregister
    }
    #[doc = "0x1174 - Contains the loCredit value required for the creditbased shaper algorithm for Channel 1"]
    #[inline(always)]
    pub const fn register93_channel1locreditregister(
        &self,
    ) -> &REGISTER93_CHANNEL1LOCREDITREGISTER {
        &self.register93_channel1locreditregister
    }
    #[doc = "0x1200 - Controls the Host Interface mode for Channel 2"]
    #[inline(always)]
    pub const fn register128_channel2busmoderegister(
        &self,
    ) -> &REGISTER128_CHANNEL2BUSMODEREGISTER {
        &self.register128_channel2busmoderegister
    }
    #[doc = "0x1204 - Used by the host to instruct the DMA to poll the Transmit Descriptor list"]
    #[inline(always)]
    pub const fn register129_channel2transmitpolldemandregister(
        &self,
    ) -> &REGISTER129_CHANNEL2TRANSMITPOLLDEMANDREGISTER {
        &self.register129_channel2transmitpolldemandregister
    }
    #[doc = "0x1208 - Used by the Host to instruct the DMA to poll the Receive Descriptor list"]
    #[inline(always)]
    pub const fn register130_channel2receivepolldemandregister(
        &self,
    ) -> &REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER {
        &self.register130_channel2receivepolldemandregister
    }
    #[doc = "0x120c - Points the DMA to the start of the Receive Descriptor list"]
    #[inline(always)]
    pub const fn register131_channel2receivedescriptorlistaddressregister(
        &self,
    ) -> &REGISTER131_CHANNEL2RECEIVEDESCRIPTORLISTADDRESSREGISTER {
        &self.register131_channel2receivedescriptorlistaddressregister
    }
    #[doc = "0x1210 - Points the DMA to the start of the Transmit Descriptor List"]
    #[inline(always)]
    pub const fn register132_channel2transmitdescriptorlistaddressregister(
        &self,
    ) -> &REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER {
        &self.register132_channel2transmitdescriptorlistaddressregister
    }
    #[doc = "0x1214 - The software driver _application_ reads this register during interrupt service routine or polling to determine the status of the DMA Bits \\[29:26\\] are reserved for the Channel 2 Status Register"]
    #[inline(always)]
    pub const fn register133_channel2statusregister(&self) -> &REGISTER133_CHANNEL2STATUSREGISTER {
        &self.register133_channel2statusregister
    }
    #[doc = "0x1218 - Establishes the Receive and Transmit operating modes and command"]
    #[inline(always)]
    pub const fn register134_channel2operationmoderegister(
        &self,
    ) -> &REGISTER134_CHANNEL2OPERATIONMODEREGISTER {
        &self.register134_channel2operationmoderegister
    }
    #[doc = "0x121c - Enables the interrupts reported by the Status Register"]
    #[inline(always)]
    pub const fn register135_channel2interruptenableregister(
        &self,
    ) -> &REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER {
        &self.register135_channel2interruptenableregister
    }
    #[doc = "0x1220 - Contains the counters for discarded frames because no host Receive Descriptor was available or because of Receive FIFO Overflow"]
    #[inline(always)]
    pub const fn register136_channel2missedframeandbufferoverflowcounterregister(
        &self,
    ) -> &REGISTER136_CHANNEL2MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER {
        &self.register136_channel2missedframeandbufferoverflowcounterregister
    }
    #[doc = "0x1224 - Watchdog timeout for Receive Interrupt _RI_ from DMA"]
    #[inline(always)]
    pub const fn register137_channel2receiveinterruptwatchdogtimerregister(
        &self,
    ) -> &REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER {
        &self.register137_channel2receiveinterruptwatchdogtimerregister
    }
    #[doc = "0x1248 - Points to the start of current Transmit Descriptor read by the DMA"]
    #[inline(always)]
    pub const fn register146_channel2currenthosttransmitdescriptorregister(
        &self,
    ) -> &REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER {
        &self.register146_channel2currenthosttransmitdescriptorregister
    }
    #[doc = "0x124c - Points to the start of current Receive Descriptor read by the DMA"]
    #[inline(always)]
    pub const fn register147_channel2currenthostreceivedescriptorregister(
        &self,
    ) -> &REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER {
        &self.register147_channel2currenthostreceivedescriptorregister
    }
    #[doc = "0x1250 - Points to the current Transmit Buffer address read by the DMA"]
    #[inline(always)]
    pub const fn register148_channel2currenthosttransmitbufferaddressregister(
        &self,
    ) -> &REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER {
        &self.register148_channel2currenthosttransmitbufferaddressregister
    }
    #[doc = "0x1254 - Points to the current Receive Buffer address read by the DMA"]
    #[inline(always)]
    pub const fn register149_channel2currenthostreceivebufferaddressregister(
        &self,
    ) -> &REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER {
        &self.register149_channel2currenthostreceivebufferaddressregister
    }
    #[doc = "0x1260 - Controls the Channel 2 credit shaping operation on the transmit path"]
    #[inline(always)]
    pub const fn register152_channel2cbscontrolregister(
        &self,
    ) -> &REGISTER152_CHANNEL2CBSCONTROLREGISTER {
        &self.register152_channel2cbscontrolregister
    }
    #[doc = "0x1264 - Provides the average traffic transmitted in Channel 2"]
    #[inline(always)]
    pub const fn register153_channel2cbsstatusregister(
        &self,
    ) -> &REGISTER153_CHANNEL2CBSSTATUSREGISTER {
        &self.register153_channel2cbsstatusregister
    }
    #[doc = "0x1268 - Contains the idleSlope credit value required for the creditbased shaper algorithm for Channel 2"]
    #[inline(always)]
    pub const fn register154_channel2idleslopecreditregister(
        &self,
    ) -> &REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER {
        &self.register154_channel2idleslopecreditregister
    }
    #[doc = "0x126c - Contains the sendSlope credit value required for the creditbased shaper algorithm for Channel 2"]
    #[inline(always)]
    pub const fn register155_channel2sendslopecreditregister(
        &self,
    ) -> &REGISTER155_CHANNEL2SENDSLOPECREDITREGISTER {
        &self.register155_channel2sendslopecreditregister
    }
    #[doc = "0x1270 - Contains the hiCredit value required for the creditbased shaper algorithm for Channel 2"]
    #[inline(always)]
    pub const fn register156_channel2hicreditregister(
        &self,
    ) -> &REGISTER156_CHANNEL2HICREDITREGISTER {
        &self.register156_channel2hicreditregister
    }
    #[doc = "0x1274 - Contains the loCredit value required for the creditbased shaper algorithm for Channel 2"]
    #[inline(always)]
    pub const fn register157_channel2locreditregister(
        &self,
    ) -> &REGISTER157_CHANNEL2LOCREDITREGISTER {
        &self.register157_channel2locreditregister
    }
}
#[doc = "REGISTER0_MACCONFIGURATIONREGISTER (rw) register accessor: This is the operation mode register for the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`register0_macconfigurationregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register0_macconfigurationregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register0_macconfigurationregister`] module"]
pub type REGISTER0_MACCONFIGURATIONREGISTER =
    crate::Reg<register0_macconfigurationregister::REGISTER0_MACCONFIGURATIONREGISTER_SPEC>;
#[doc = "This is the operation mode register for the MAC"]
pub mod register0_macconfigurationregister;
#[doc = "REGISTER1_MACFRAMEFILTER (rw) register accessor: Contains the frame filtering controls\n\nYou can [`read`](crate::Reg::read) this register and get [`register1_macframefilter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register1_macframefilter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register1_macframefilter`] module"]
pub type REGISTER1_MACFRAMEFILTER =
    crate::Reg<register1_macframefilter::REGISTER1_MACFRAMEFILTER_SPEC>;
#[doc = "Contains the frame filtering controls"]
pub mod register1_macframefilter;
#[doc = "REGISTER2_HASHTABLEHIGHREGISTER (rw) register accessor: Contains the higher 32 bits of the Multicast Hash table This register is present only when you select the 64bit Hash filter function in coreConsultant _See Table 79_\n\nYou can [`read`](crate::Reg::read) this register and get [`register2_hashtablehighregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register2_hashtablehighregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register2_hashtablehighregister`] module"]
pub type REGISTER2_HASHTABLEHIGHREGISTER =
    crate::Reg<register2_hashtablehighregister::REGISTER2_HASHTABLEHIGHREGISTER_SPEC>;
#[doc = "Contains the higher 32 bits of the Multicast Hash table This register is present only when you select the 64bit Hash filter function in coreConsultant _See Table 79_"]
pub mod register2_hashtablehighregister;
#[doc = "REGISTER3_HASHTABLELOWREGISTER (rw) register accessor: Contains the lower 32 bits of the Multicast Hash table This register is present only when you select the Hash filter function in coreConsultant _See Table 79_\n\nYou can [`read`](crate::Reg::read) this register and get [`register3_hashtablelowregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register3_hashtablelowregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register3_hashtablelowregister`] module"]
pub type REGISTER3_HASHTABLELOWREGISTER =
    crate::Reg<register3_hashtablelowregister::REGISTER3_HASHTABLELOWREGISTER_SPEC>;
#[doc = "Contains the lower 32 bits of the Multicast Hash table This register is present only when you select the Hash filter function in coreConsultant _See Table 79_"]
pub mod register3_hashtablelowregister;
#[doc = "REGISTER4_GMIIADDRESSREGISTER (rw) register accessor: Controls the management cycles to an external PHY This register is present only when you select the Station Management _MDIO_ feature in coreConsultant _See Table 726_\n\nYou can [`read`](crate::Reg::read) this register and get [`register4_gmiiaddressregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register4_gmiiaddressregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register4_gmiiaddressregister`] module"]
pub type REGISTER4_GMIIADDRESSREGISTER =
    crate::Reg<register4_gmiiaddressregister::REGISTER4_GMIIADDRESSREGISTER_SPEC>;
#[doc = "Controls the management cycles to an external PHY This register is present only when you select the Station Management _MDIO_ feature in coreConsultant _See Table 726_"]
pub mod register4_gmiiaddressregister;
#[doc = "REGISTER5_GMIIDATAREGISTER (rw) register accessor: Contains the data to be written to or read from the PHY register This register is present only when you select the Station Management _MDIO_ feature in coreConsultant _See Table 726_\n\nYou can [`read`](crate::Reg::read) this register and get [`register5_gmiidataregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register5_gmiidataregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register5_gmiidataregister`] module"]
pub type REGISTER5_GMIIDATAREGISTER =
    crate::Reg<register5_gmiidataregister::REGISTER5_GMIIDATAREGISTER_SPEC>;
#[doc = "Contains the data to be written to or read from the PHY register This register is present only when you select the Station Management _MDIO_ feature in coreConsultant _See Table 726_"]
pub mod register5_gmiidataregister;
#[doc = "REGISTER6_FLOWCONTROLREGISTER (rw) register accessor: Controls the generation of control frames\n\nYou can [`read`](crate::Reg::read) this register and get [`register6_flowcontrolregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register6_flowcontrolregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register6_flowcontrolregister`] module"]
pub type REGISTER6_FLOWCONTROLREGISTER =
    crate::Reg<register6_flowcontrolregister::REGISTER6_FLOWCONTROLREGISTER_SPEC>;
#[doc = "Controls the generation of control frames"]
pub mod register6_flowcontrolregister;
#[doc = "REGISTER7_VLANTAGREGISTER (rw) register accessor: Identifies IEEE 8021Q VLAN type frames\n\nYou can [`read`](crate::Reg::read) this register and get [`register7_vlantagregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register7_vlantagregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register7_vlantagregister`] module"]
pub type REGISTER7_VLANTAGREGISTER =
    crate::Reg<register7_vlantagregister::REGISTER7_VLANTAGREGISTER_SPEC>;
#[doc = "Identifies IEEE 8021Q VLAN type frames"]
pub mod register7_vlantagregister;
#[doc = "REGISTER8_VERSIONREGISTER (r) register accessor: Identifies the version of the Core\n\nYou can [`read`](crate::Reg::read) this register and get [`register8_versionregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register8_versionregister`] module"]
pub type REGISTER8_VERSIONREGISTER =
    crate::Reg<register8_versionregister::REGISTER8_VERSIONREGISTER_SPEC>;
#[doc = "Identifies the version of the Core"]
pub mod register8_versionregister;
#[doc = "REGISTER9_DEBUGREGISTER (r) register accessor: Gives the status of various internal blocks for debugging\n\nYou can [`read`](crate::Reg::read) this register and get [`register9_debugregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register9_debugregister`] module"]
pub type REGISTER9_DEBUGREGISTER =
    crate::Reg<register9_debugregister::REGISTER9_DEBUGREGISTER_SPEC>;
#[doc = "Gives the status of various internal blocks for debugging"]
pub mod register9_debugregister;
#[doc = "REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER (rw) register accessor: Remote Wake-Up Frame Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register10_remotewakeupframefilterregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register10_remotewakeupframefilterregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register10_remotewakeupframefilterregister`] module"]
pub type REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER = crate::Reg<
    register10_remotewakeupframefilterregister::REGISTER10_REMOTEWAKEUPFRAMEFILTERREGISTER_SPEC,
>;
#[doc = "Remote Wake-Up Frame Filter Register"]
pub mod register10_remotewakeupframefilterregister;
#[doc = "REGISTER11_PMTCONTROLANDSTATUSREGISTER (rw) register accessor: PMT Control and Status Register. This register is present only when you select the PMT module in coreConsultant.\n\nYou can [`read`](crate::Reg::read) this register and get [`register11_pmtcontrolandstatusregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register11_pmtcontrolandstatusregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register11_pmtcontrolandstatusregister`] module"]
pub type REGISTER11_PMTCONTROLANDSTATUSREGISTER =
    crate::Reg<register11_pmtcontrolandstatusregister::REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC>;
#[doc = "PMT Control and Status Register. This register is present only when you select the PMT module in coreConsultant."]
pub mod register11_pmtcontrolandstatusregister;
#[doc = "REGISTER12_LPICONTROLANDSTATUSREGISTER (rw) register accessor: Controls the Low Power Idle _LPI_ operations and provides the LPI status of the core This register is present only when you select the Energy Efficient Ethernet feature in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register12_lpicontrolandstatusregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register12_lpicontrolandstatusregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register12_lpicontrolandstatusregister`] module"]
pub type REGISTER12_LPICONTROLANDSTATUSREGISTER =
    crate::Reg<register12_lpicontrolandstatusregister::REGISTER12_LPICONTROLANDSTATUSREGISTER_SPEC>;
#[doc = "Controls the Low Power Idle _LPI_ operations and provides the LPI status of the core This register is present only when you select the Energy Efficient Ethernet feature in coreConsultant"]
pub mod register12_lpicontrolandstatusregister;
#[doc = "REGISTER13_LPITIMERSCONTROLREGISTER (rw) register accessor: Controls the timeout values in LPI states This register is present only when you select the Energy Efficient Ethernet feature in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register13_lpitimerscontrolregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register13_lpitimerscontrolregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register13_lpitimerscontrolregister`] module"]
pub type REGISTER13_LPITIMERSCONTROLREGISTER =
    crate::Reg<register13_lpitimerscontrolregister::REGISTER13_LPITIMERSCONTROLREGISTER_SPEC>;
#[doc = "Controls the timeout values in LPI states This register is present only when you select the Energy Efficient Ethernet feature in coreConsultant"]
pub mod register13_lpitimerscontrolregister;
#[doc = "REGISTER14_INTERRUPTSTATUSREGISTER (r) register accessor: Contains the interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`register14_interruptstatusregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register14_interruptstatusregister`] module"]
pub type REGISTER14_INTERRUPTSTATUSREGISTER =
    crate::Reg<register14_interruptstatusregister::REGISTER14_INTERRUPTSTATUSREGISTER_SPEC>;
#[doc = "Contains the interrupt status"]
pub mod register14_interruptstatusregister;
#[doc = "REGISTER15_INTERRUPTMASKREGISTER (rw) register accessor: Contains the masks for generating the interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`register15_interruptmaskregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register15_interruptmaskregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register15_interruptmaskregister`] module"]
pub type REGISTER15_INTERRUPTMASKREGISTER =
    crate::Reg<register15_interruptmaskregister::REGISTER15_INTERRUPTMASKREGISTER_SPEC>;
#[doc = "Contains the masks for generating the interrupts"]
pub mod register15_interruptmaskregister;
#[doc = "REGISTER16_MACADDRESS0HIGHREGISTER (rw) register accessor: Contains the higher 16 bits of the first MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`register16_macaddress0highregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register16_macaddress0highregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register16_macaddress0highregister`] module"]
pub type REGISTER16_MACADDRESS0HIGHREGISTER =
    crate::Reg<register16_macaddress0highregister::REGISTER16_MACADDRESS0HIGHREGISTER_SPEC>;
#[doc = "Contains the higher 16 bits of the first MAC address"]
pub mod register16_macaddress0highregister;
#[doc = "REGISTER17_MACADDRESS0LOWREGISTER (rw) register accessor: Contains the lower 32 bits of the first MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`register17_macaddress0lowregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register17_macaddress0lowregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register17_macaddress0lowregister`] module"]
pub type REGISTER17_MACADDRESS0LOWREGISTER =
    crate::Reg<register17_macaddress0lowregister::REGISTER17_MACADDRESS0LOWREGISTER_SPEC>;
#[doc = "Contains the lower 32 bits of the first MAC address"]
pub mod register17_macaddress0lowregister;
#[doc = "REGISTER18_MACADDRESS1HIGHREGISTER (rw) register accessor: Contains the higher 16 bits of the second MAC address This register is present only when Enable MAC Address1 is selected in coreConsultant _See Table 78_\n\nYou can [`read`](crate::Reg::read) this register and get [`register18_macaddress1highregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register18_macaddress1highregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register18_macaddress1highregister`] module"]
pub type REGISTER18_MACADDRESS1HIGHREGISTER =
    crate::Reg<register18_macaddress1highregister::REGISTER18_MACADDRESS1HIGHREGISTER_SPEC>;
#[doc = "Contains the higher 16 bits of the second MAC address This register is present only when Enable MAC Address1 is selected in coreConsultant _See Table 78_"]
pub mod register18_macaddress1highregister;
#[doc = "REGISTER19_MACADDRESS1LOWREGISTER (rw) register accessor: Contains the lower 32 bits of the second MAC address This register is present only when Enable MAC Address1 is selected in coreConsultant _See Table 78_\n\nYou can [`read`](crate::Reg::read) this register and get [`register19_macaddress1lowregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register19_macaddress1lowregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register19_macaddress1lowregister`] module"]
pub type REGISTER19_MACADDRESS1LOWREGISTER =
    crate::Reg<register19_macaddress1lowregister::REGISTER19_MACADDRESS1LOWREGISTER_SPEC>;
#[doc = "Contains the lower 32 bits of the second MAC address This register is present only when Enable MAC Address1 is selected in coreConsultant _See Table 78_"]
pub mod register19_macaddress1lowregister;
#[doc = "MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress`] module"]
pub type MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS = crate :: Reg < macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress :: MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress;
#[doc = "MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress`] module"]
pub type MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS = crate :: Reg < macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress :: MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress;
#[doc = "MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress`] module"]
pub type MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS = crate :: Reg < macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress :: MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress;
#[doc = "MACADDRESS3LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFOURTHMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress3lowregistercontainsthelower32bitsofthefourthmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress3lowregistercontainsthelower32bitsofthefourthmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress3lowregistercontainsthelower32bitsofthefourthmacaddress`] module"]
pub type MACADDRESS3LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFOURTHMACADDRESS = crate :: Reg < macaddress3lowregistercontainsthelower32bitsofthefourthmacaddress :: MACADDRESS3LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFOURTHMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress3lowregistercontainsthelower32bitsofthefourthmacaddress;
#[doc = "MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress`] module"]
pub type MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS = crate :: Reg < macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress :: MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress;
#[doc = "MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress`] module"]
pub type MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS = crate :: Reg < macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress :: MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress;
#[doc = "MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress`] module"]
pub type MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS = crate :: Reg < macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress :: MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress;
#[doc = "MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress`] module"]
pub type MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS = crate :: Reg < macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress :: MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress;
#[doc = "MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress`] module"]
pub type MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS = crate :: Reg < macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress :: MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress;
#[doc = "MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress`] module"]
pub type MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS = crate :: Reg < macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress :: MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress;
#[doc = "MACADDRESS7HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEEIGHTHMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress7highregistercontainsthehigher16bitsoftheeighthmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress7highregistercontainsthehigher16bitsoftheeighthmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress7highregistercontainsthehigher16bitsoftheeighthmacaddress`] module"]
pub type MACADDRESS7HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEEIGHTHMACADDRESS = crate :: Reg < macaddress7highregistercontainsthehigher16bitsoftheeighthmacaddress :: MACADDRESS7HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEEIGHTHMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress7highregistercontainsthehigher16bitsoftheeighthmacaddress;
#[doc = "MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS (rw) register accessor: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress`] module"]
pub type MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS = crate :: Reg < macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress :: MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS_SPEC > ;
#[doc = "Reserved"]
pub mod macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress;
#[doc = "REGISTER48_ANCONTROLREGISTER (rw) register accessor: Enables and/or restarts autonegotiation This register also enables the Physical Coding Sublayer _PCS_ loopback This register is present only when you select the TBI, RTBI, or SGMII interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register48_ancontrolregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register48_ancontrolregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register48_ancontrolregister`] module"]
pub type REGISTER48_ANCONTROLREGISTER =
    crate::Reg<register48_ancontrolregister::REGISTER48_ANCONTROLREGISTER_SPEC>;
#[doc = "Enables and/or restarts autonegotiation This register also enables the Physical Coding Sublayer _PCS_ loopback This register is present only when you select the TBI, RTBI, or SGMII interface in coreConsultant"]
pub mod register48_ancontrolregister;
#[doc = "REGISTER49_ANSTATUSREGISTER (r) register accessor: Indicates the link and autonegotiation status This register is present only when you select the TBI, RTBI, or SGMII interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register49_anstatusregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register49_anstatusregister`] module"]
pub type REGISTER49_ANSTATUSREGISTER =
    crate::Reg<register49_anstatusregister::REGISTER49_ANSTATUSREGISTER_SPEC>;
#[doc = "Indicates the link and autonegotiation status This register is present only when you select the TBI, RTBI, or SGMII interface in coreConsultant"]
pub mod register49_anstatusregister;
#[doc = "REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER (rw) register accessor: This register is configured before autonegotiation begins It contains the advertised ability of the MAC This register is present only when you select the TBI or RTBI interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register50_autonegotiationadvertisementregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register50_autonegotiationadvertisementregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register50_autonegotiationadvertisementregister`] module"]
pub type REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER = crate :: Reg < register50_autonegotiationadvertisementregister :: REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC > ;
#[doc = "This register is configured before autonegotiation begins It contains the advertised ability of the MAC This register is present only when you select the TBI or RTBI interface in coreConsultant"]
pub mod register50_autonegotiationadvertisementregister;
#[doc = "REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER (r) register accessor: Contains the advertised ability of the link partner Its value is valid after successful completion of autonegotiation or when a new base page has been received _indicated in the AutoNegotiation Expansion Register_ This register is present only when you select the TBI or RTBI interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register51_autonegotiationlinkpartnerabilityregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register51_autonegotiationlinkpartnerabilityregister`] module"]
pub type REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER = crate :: Reg < register51_autonegotiationlinkpartnerabilityregister :: REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER_SPEC > ;
#[doc = "Contains the advertised ability of the link partner Its value is valid after successful completion of autonegotiation or when a new base page has been received _indicated in the AutoNegotiation Expansion Register_ This register is present only when you select the TBI or RTBI interface in coreConsultant"]
pub mod register51_autonegotiationlinkpartnerabilityregister;
#[doc = "REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER (r) register accessor: Indicates whether a new base page has been received from the link partner This register is present only when you select the TBI or RTBI interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register52_autonegotiationexpansionregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register52_autonegotiationexpansionregister`] module"]
pub type REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER = crate::Reg<
    register52_autonegotiationexpansionregister::REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER_SPEC,
>;
#[doc = "Indicates whether a new base page has been received from the link partner This register is present only when you select the TBI or RTBI interface in coreConsultant"]
pub mod register52_autonegotiationexpansionregister;
#[doc = "REGISTER53_TBIEXTENDEDSTATUSREGISTER (r) register accessor: Indicates all modes of operation of the MAC This register is present only when you select the TBI or RTBI interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register53_tbiextendedstatusregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register53_tbiextendedstatusregister`] module"]
pub type REGISTER53_TBIEXTENDEDSTATUSREGISTER =
    crate::Reg<register53_tbiextendedstatusregister::REGISTER53_TBIEXTENDEDSTATUSREGISTER_SPEC>;
#[doc = "Indicates all modes of operation of the MAC This register is present only when you select the TBI or RTBI interface in coreConsultant"]
pub mod register53_tbiextendedstatusregister;
#[doc = "REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER (rw) register accessor: Indicates the status signals received from the PHY through the SGMII, RGMII, or SMII interface This register is present only when you select the SGMII, RGMII, or SMII interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register54_sgmii_rgmii_smiicontrolandstatusregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register54_sgmii_rgmii_smiicontrolandstatusregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register54_sgmii_rgmii_smiicontrolandstatusregister`] module"]
pub type REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER = crate :: Reg < register54_sgmii_rgmii_smiicontrolandstatusregister :: REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER_SPEC > ;
#[doc = "Indicates the status signals received from the PHY through the SGMII, RGMII, or SMII interface This register is present only when you select the SGMII, RGMII, or SMII interface in coreConsultant"]
pub mod register54_sgmii_rgmii_smiicontrolandstatusregister;
#[doc = "REGISTER55_WATCHDOGTIMEOUTREGISTER (rw) register accessor: Controls the watchdog timeout for received frames\n\nYou can [`read`](crate::Reg::read) this register and get [`register55_watchdogtimeoutregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register55_watchdogtimeoutregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register55_watchdogtimeoutregister`] module"]
pub type REGISTER55_WATCHDOGTIMEOUTREGISTER =
    crate::Reg<register55_watchdogtimeoutregister::REGISTER55_WATCHDOGTIMEOUTREGISTER_SPEC>;
#[doc = "Controls the watchdog timeout for received frames"]
pub mod register55_watchdogtimeoutregister;
#[doc = "REGISTER56_GENERALPURPOSEIOREGISTER (rw) register accessor: Provides the control to drive up to 4 bits of output ports _GPO_ and also provides the status of up to 4 input ports _GPIS_\n\nYou can [`read`](crate::Reg::read) this register and get [`register56_generalpurposeioregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register56_generalpurposeioregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register56_generalpurposeioregister`] module"]
pub type REGISTER56_GENERALPURPOSEIOREGISTER =
    crate::Reg<register56_generalpurposeioregister::REGISTER56_GENERALPURPOSEIOREGISTER_SPEC>;
#[doc = "Provides the control to drive up to 4 bits of output ports _GPO_ and also provides the status of up to 4 input ports _GPIS_"]
pub mod register56_generalpurposeioregister;
#[doc = "REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0 (rw) register accessor: Controls the operations of the Layer 3 and Layer 4 frame filtering\n\nYou can [`read`](crate::Reg::read) this register and get [`register256_layer3andlayer4controlregister0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register256_layer3andlayer4controlregister0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register256_layer3andlayer4controlregister0`] module"]
pub type REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0 = crate::Reg<
    register256_layer3andlayer4controlregister0::REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC,
>;
#[doc = "Controls the operations of the Layer 3 and Layer 4 frame filtering"]
pub mod register256_layer3andlayer4controlregister0;
#[doc = "REGISTER257_LAYER4ADDRESSREGISTER0 (rw) register accessor: Layer 4 Port number field It contains the 16bit Source and Destination Port numbers of the TCP or UDP frame\n\nYou can [`read`](crate::Reg::read) this register and get [`register257_layer4addressregister0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register257_layer4addressregister0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register257_layer4addressregister0`] module"]
pub type REGISTER257_LAYER4ADDRESSREGISTER0 =
    crate::Reg<register257_layer4addressregister0::REGISTER257_LAYER4ADDRESSREGISTER0_SPEC>;
#[doc = "Layer 4 Port number field It contains the 16bit Source and Destination Port numbers of the TCP or UDP frame"]
pub mod register257_layer4addressregister0;
#[doc = "REGISTER260_LAYER3ADDRESS0REGISTER0 (rw) register accessor: Layer 3 Address field For IPv4 frames, it contains the 32bit IP Source Address field For IPv6 frames, it contains Bits \\[31:0\\] of the 128bit IP Source Address or Destination Address field\n\nYou can [`read`](crate::Reg::read) this register and get [`register260_layer3address0register0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register260_layer3address0register0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register260_layer3address0register0`] module"]
pub type REGISTER260_LAYER3ADDRESS0REGISTER0 =
    crate::Reg<register260_layer3address0register0::REGISTER260_LAYER3ADDRESS0REGISTER0_SPEC>;
#[doc = "Layer 3 Address field For IPv4 frames, it contains the 32bit IP Source Address field For IPv6 frames, it contains Bits \\[31:0\\] of the 128bit IP Source Address or Destination Address field"]
pub mod register260_layer3address0register0;
#[doc = "REGISTER261_LAYER3ADDRESS1REGISTER0 (rw) register accessor: Layer 3 Address 1 field For IPv4 frames, it contains the 32bit IP Destination Address field For IPv6 frames, it contains Bits \\[63:32\\] of the 128bit IP Source Address or Destination Address field\n\nYou can [`read`](crate::Reg::read) this register and get [`register261_layer3address1register0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register261_layer3address1register0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register261_layer3address1register0`] module"]
pub type REGISTER261_LAYER3ADDRESS1REGISTER0 =
    crate::Reg<register261_layer3address1register0::REGISTER261_LAYER3ADDRESS1REGISTER0_SPEC>;
#[doc = "Layer 3 Address 1 field For IPv4 frames, it contains the 32bit IP Destination Address field For IPv6 frames, it contains Bits \\[63:32\\] of the 128bit IP Source Address or Destination Address field"]
pub mod register261_layer3address1register0;
#[doc = "REGISTER262_LAYER3ADDRESS2REGISTER0 (rw) register accessor: Layer 3 Address 2 field This register is reserved for IPv4 frames For IPv6 frames, it contains Bits \\[95:64\\] of the 128bit IP Source Address or Destination Address field\n\nYou can [`read`](crate::Reg::read) this register and get [`register262_layer3address2register0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register262_layer3address2register0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register262_layer3address2register0`] module"]
pub type REGISTER262_LAYER3ADDRESS2REGISTER0 =
    crate::Reg<register262_layer3address2register0::REGISTER262_LAYER3ADDRESS2REGISTER0_SPEC>;
#[doc = "Layer 3 Address 2 field This register is reserved for IPv4 frames For IPv6 frames, it contains Bits \\[95:64\\] of the 128bit IP Source Address or Destination Address field"]
pub mod register262_layer3address2register0;
#[doc = "REGISTER263_LAYER3ADDRESS3REGISTER0 (rw) register accessor: Layer 3 Address 3 field This register is reserved for IPv4 frames For IPv6 frames, it contains Bits \\[127:96\\] of the 128bit IP Source Address or Destination Address field\n\nYou can [`read`](crate::Reg::read) this register and get [`register263_layer3address3register0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register263_layer3address3register0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register263_layer3address3register0`] module"]
pub type REGISTER263_LAYER3ADDRESS3REGISTER0 =
    crate::Reg<register263_layer3address3register0::REGISTER263_LAYER3ADDRESS3REGISTER0_SPEC>;
#[doc = "Layer 3 Address 3 field This register is reserved for IPv4 frames For IPv6 frames, it contains Bits \\[127:96\\] of the 128bit IP Source Address or Destination Address field"]
pub mod register263_layer3address3register0;
#[doc = "REGISTER320_HASHTABLEREGISTER0 (rw) register accessor: This register contains the first 32 bits of the hash table when the width of the Hash table is 128 bits or 256 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`register320_hashtableregister0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register320_hashtableregister0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register320_hashtableregister0`] module"]
pub type REGISTER320_HASHTABLEREGISTER0 =
    crate::Reg<register320_hashtableregister0::REGISTER320_HASHTABLEREGISTER0_SPEC>;
#[doc = "This register contains the first 32 bits of the hash table when the width of the Hash table is 128 bits or 256 bits"]
pub mod register320_hashtableregister0;
#[doc = "REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER (rw) register accessor: This register contains the VLAN tag for insertion into or replacement in the transmit frames\n\nYou can [`read`](crate::Reg::read) this register and get [`register353_vlantaginclusionorreplacementregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register353_vlantaginclusionorreplacementregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register353_vlantaginclusionorreplacementregister`] module"]
pub type REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER = crate :: Reg < register353_vlantaginclusionorreplacementregister :: REGISTER353_VLANTAGINCLUSIONORREPLACEMENTREGISTER_SPEC > ;
#[doc = "This register contains the VLAN tag for insertion into or replacement in the transmit frames"]
pub mod register353_vlantaginclusionorreplacementregister;
#[doc = "REGISTER354_VLANHASHTABLEREGISTER (rw) register accessor: This register contains the VLAN hash table\n\nYou can [`read`](crate::Reg::read) this register and get [`register354_vlanhashtableregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register354_vlanhashtableregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register354_vlanhashtableregister`] module"]
pub type REGISTER354_VLANHASHTABLEREGISTER =
    crate::Reg<register354_vlanhashtableregister::REGISTER354_VLANHASHTABLEREGISTER_SPEC>;
#[doc = "This register contains the VLAN hash table"]
pub mod register354_vlanhashtableregister;
#[doc = "REGISTER448_TIMESTAMPCONTROLREGISTER (rw) register accessor: Controls the timestamp generation and update logic This register is present only when IEEE1588 timestamping is enabled during coreConsultant configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`register448_timestampcontrolregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register448_timestampcontrolregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register448_timestampcontrolregister`] module"]
pub type REGISTER448_TIMESTAMPCONTROLREGISTER =
    crate::Reg<register448_timestampcontrolregister::REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC>;
#[doc = "Controls the timestamp generation and update logic This register is present only when IEEE1588 timestamping is enabled during coreConsultant configuration"]
pub mod register448_timestampcontrolregister;
#[doc = "REGISTER449_SUBSECONDINCREMENTREGISTER (rw) register accessor: Contains the 8bit value by which the SubSecond register is incremented This register is present only when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register449_subsecondincrementregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register449_subsecondincrementregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register449_subsecondincrementregister`] module"]
pub type REGISTER449_SUBSECONDINCREMENTREGISTER =
    crate::Reg<register449_subsecondincrementregister::REGISTER449_SUBSECONDINCREMENTREGISTER_SPEC>;
#[doc = "Contains the 8bit value by which the SubSecond register is incremented This register is present only when IEEE1588 timestamping is enabled without an external timestamp input"]
pub mod register449_subsecondincrementregister;
#[doc = "REGISTER450_SYSTEMTIMESECONDSREGISTER (r) register accessor: Contains the lower 32 bits of the seconds field of the system time This register is present only when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register450_systemtimesecondsregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register450_systemtimesecondsregister`] module"]
pub type REGISTER450_SYSTEMTIMESECONDSREGISTER =
    crate::Reg<register450_systemtimesecondsregister::REGISTER450_SYSTEMTIMESECONDSREGISTER_SPEC>;
#[doc = "Contains the lower 32 bits of the seconds field of the system time This register is present only when IEEE1588 timestamping is enabled without an external timestamp input"]
pub mod register450_systemtimesecondsregister;
#[doc = "REGISTER451_SYSTEMTIMENANOSECONDSREGISTER (r) register accessor: Contains 32 bits of the nanoseconds field of the system time This register is only present when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register451_systemtimenanosecondsregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register451_systemtimenanosecondsregister`] module"]
pub type REGISTER451_SYSTEMTIMENANOSECONDSREGISTER = crate::Reg<
    register451_systemtimenanosecondsregister::REGISTER451_SYSTEMTIMENANOSECONDSREGISTER_SPEC,
>;
#[doc = "Contains 32 bits of the nanoseconds field of the system time This register is only present when IEEE1588 timestamping is enabled without an external timestamp input"]
pub mod register451_systemtimenanosecondsregister;
#[doc = "REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER (rw) register accessor: Contains the lower 32 bits of the seconds field to be written to, added to, or subtracted from the System Time value This register is only present when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register452_systemtimesecondsupdateregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register452_systemtimesecondsupdateregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register452_systemtimesecondsupdateregister`] module"]
pub type REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER = crate::Reg<
    register452_systemtimesecondsupdateregister::REGISTER452_SYSTEMTIMESECONDSUPDATEREGISTER_SPEC,
>;
#[doc = "Contains the lower 32 bits of the seconds field to be written to, added to, or subtracted from the System Time value This register is only present when IEEE1588 timestamping is enabled without an external timestamp input"]
pub mod register452_systemtimesecondsupdateregister;
#[doc = "REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER (rw) register accessor: Contains 32 bits of the nanoseconds field to be written to, added to, or subtracted from the System Time value This register is only present when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register453_systemtimenanosecondsupdateregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register453_systemtimenanosecondsupdateregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register453_systemtimenanosecondsupdateregister`] module"]
pub type REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER = crate :: Reg < register453_systemtimenanosecondsupdateregister :: REGISTER453_SYSTEMTIMENANOSECONDSUPDATEREGISTER_SPEC > ;
#[doc = "Contains 32 bits of the nanoseconds field to be written to, added to, or subtracted from the System Time value This register is only present when IEEE1588 timestamping is enabled without an external timestamp input"]
pub mod register453_systemtimenanosecondsupdateregister;
#[doc = "REGISTER454_TIMESTAMPADDENDREGISTER (rw) register accessor: This register is used by the software to readjust the clock frequency linearly to match the master clock frequency This register is only present when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register454_timestampaddendregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register454_timestampaddendregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register454_timestampaddendregister`] module"]
pub type REGISTER454_TIMESTAMPADDENDREGISTER =
    crate::Reg<register454_timestampaddendregister::REGISTER454_TIMESTAMPADDENDREGISTER_SPEC>;
#[doc = "This register is used by the software to readjust the clock frequency linearly to match the master clock frequency This register is only present when IEEE1588 timestamping is enabled without an external timestamp input"]
pub mod register454_timestampaddendregister;
#[doc = "REGISTER455_TARGETTIMESECONDSREGISTER (rw) register accessor: Contains the higher 32 bits of time to be compared with the system time for interrupt event generation or to start the PPS signal output generation This register is present only when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register455_targettimesecondsregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register455_targettimesecondsregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register455_targettimesecondsregister`] module"]
pub type REGISTER455_TARGETTIMESECONDSREGISTER =
    crate::Reg<register455_targettimesecondsregister::REGISTER455_TARGETTIMESECONDSREGISTER_SPEC>;
#[doc = "Contains the higher 32 bits of time to be compared with the system time for interrupt event generation or to start the PPS signal output generation This register is present only when IEEE1588 timestamping is enabled without an external timestamp input"]
pub mod register455_targettimesecondsregister;
#[doc = "REGISTER456_TARGETTIMENANOSECONDSREGISTER (rw) register accessor: Contains the lower 32 bits of time to be compared with the system time for interrupt event generation or to start the PPS signal output generation This register is present only when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register456_targettimenanosecondsregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register456_targettimenanosecondsregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register456_targettimenanosecondsregister`] module"]
pub type REGISTER456_TARGETTIMENANOSECONDSREGISTER = crate::Reg<
    register456_targettimenanosecondsregister::REGISTER456_TARGETTIMENANOSECONDSREGISTER_SPEC,
>;
#[doc = "Contains the lower 32 bits of time to be compared with the system time for interrupt event generation or to start the PPS signal output generation This register is present only when IEEE1588 timestamping is enabled without an external timestamp input"]
pub mod register456_targettimenanosecondsregister;
#[doc = "REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER (rw) register accessor: Contains the most significant 16bits of the timestamp seconds value This register is optional and can be selected using the parameter mentioned in “IEEE 1588 Timestamp Block” on page 492\n\nYou can [`read`](crate::Reg::read) this register and get [`register457_systemtimehigherwordsecondsregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register457_systemtimehigherwordsecondsregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register457_systemtimehigherwordsecondsregister`] module"]
pub type REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER = crate :: Reg < register457_systemtimehigherwordsecondsregister :: REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER_SPEC > ;
#[doc = "Contains the most significant 16bits of the timestamp seconds value This register is optional and can be selected using the parameter mentioned in “IEEE 1588 Timestamp Block” on page 492"]
pub mod register457_systemtimehigherwordsecondsregister;
#[doc = "REGISTER458_TIMESTAMPSTATUSREGISTER (rw) register accessor: Contains the PTP status This register is available only when the advanced IEEE 1588 timestamp feature is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register458_timestampstatusregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register458_timestampstatusregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register458_timestampstatusregister`] module"]
pub type REGISTER458_TIMESTAMPSTATUSREGISTER =
    crate::Reg<register458_timestampstatusregister::REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC>;
#[doc = "Contains the PTP status This register is available only when the advanced IEEE 1588 timestamp feature is selected"]
pub mod register458_timestampstatusregister;
#[doc = "REGISTER459_PPSCONTROLREGISTER (rw) register accessor: This register is used to control the interval of the PPS signal output This register is available only when the advanced IEEE 1588 timestamp feature is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register459_ppscontrolregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register459_ppscontrolregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register459_ppscontrolregister`] module"]
pub type REGISTER459_PPSCONTROLREGISTER =
    crate::Reg<register459_ppscontrolregister::REGISTER459_PPSCONTROLREGISTER_SPEC>;
#[doc = "This register is used to control the interval of the PPS signal output This register is available only when the advanced IEEE 1588 timestamp feature is selected"]
pub mod register459_ppscontrolregister;
#[doc = "REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER (r) register accessor: Contains the lower 32 bits _nanoseconds field_ of the auxiliary timestamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`register460_auxiliarytimestampnanosecondsregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register460_auxiliarytimestampnanosecondsregister`] module"]
pub type REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER = crate :: Reg < register460_auxiliarytimestampnanosecondsregister :: REGISTER460_AUXILIARYTIMESTAMPNANOSECONDSREGISTER_SPEC > ;
#[doc = "Contains the lower 32 bits _nanoseconds field_ of the auxiliary timestamp register"]
pub mod register460_auxiliarytimestampnanosecondsregister;
#[doc = "REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER (r) register accessor: Contains the lower 32 bits of the Seconds field of the auxiliary timestamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`register461_auxiliarytimestampsecondsregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register461_auxiliarytimestampsecondsregister`] module"]
pub type REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER = crate :: Reg < register461_auxiliarytimestampsecondsregister :: REGISTER461_AUXILIARYTIMESTAMPSECONDSREGISTER_SPEC > ;
#[doc = "Contains the lower 32 bits of the Seconds field of the auxiliary timestamp register"]
pub mod register461_auxiliarytimestampsecondsregister;
#[doc = "REGISTER462_AVMACCONTROLREGISTER (rw) register accessor: Controls the AV traffic and queue management in the MAC Receiver This register is present only when you select the AV feature in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register462_avmaccontrolregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register462_avmaccontrolregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register462_avmaccontrolregister`] module"]
pub type REGISTER462_AVMACCONTROLREGISTER =
    crate::Reg<register462_avmaccontrolregister::REGISTER462_AVMACCONTROLREGISTER_SPEC>;
#[doc = "Controls the AV traffic and queue management in the MAC Receiver This register is present only when you select the AV feature in coreConsultant"]
pub mod register462_avmaccontrolregister;
#[doc = "REGISTER472_PPS0INTERVALREGISTER (rw) register accessor: Contains the number of units of subsecond increment value between the rising edges of PPS0 signal output This register is available only when the flexible PPS feature is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register472_pps0intervalregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register472_pps0intervalregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register472_pps0intervalregister`] module"]
pub type REGISTER472_PPS0INTERVALREGISTER =
    crate::Reg<register472_pps0intervalregister::REGISTER472_PPS0INTERVALREGISTER_SPEC>;
#[doc = "Contains the number of units of subsecond increment value between the rising edges of PPS0 signal output This register is available only when the flexible PPS feature is selected"]
pub mod register472_pps0intervalregister;
#[doc = "REGISTER473_PPS0WIDTHREGISTER (rw) register accessor: Contains the number of units of subsecond increment value between the rising and corresponding falling edges of PPS0 signal output This register is available only when the flexible PPS feature is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register473_pps0widthregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register473_pps0widthregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register473_pps0widthregister`] module"]
pub type REGISTER473_PPS0WIDTHREGISTER =
    crate::Reg<register473_pps0widthregister::REGISTER473_PPS0WIDTHREGISTER_SPEC>;
#[doc = "Contains the number of units of subsecond increment value between the rising and corresponding falling edges of PPS0 signal output This register is available only when the flexible PPS feature is selected"]
pub mod register473_pps0widthregister;
#[doc = "REGISTER480_PPS1TARGETTIMESECONDSREGISTER (rw) register accessor: Contains the higher 32 bits of time to be compared with the system time to generate the interrupt event or to start generating the PPS1 output signal This register is present only when IEEE1588 timestamping is enabled without an external timestamp input and at least one additional PPS output is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register480_pps1targettimesecondsregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register480_pps1targettimesecondsregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register480_pps1targettimesecondsregister`] module"]
pub type REGISTER480_PPS1TARGETTIMESECONDSREGISTER = crate::Reg<
    register480_pps1targettimesecondsregister::REGISTER480_PPS1TARGETTIMESECONDSREGISTER_SPEC,
>;
#[doc = "Contains the higher 32 bits of time to be compared with the system time to generate the interrupt event or to start generating the PPS1 output signal This register is present only when IEEE1588 timestamping is enabled without an external timestamp input and at least one additional PPS output is selected"]
pub mod register480_pps1targettimesecondsregister;
#[doc = "REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER (rw) register accessor: Contains the lower 32 bits of time to be compared with the system time to generate the interrupt event or to start generating the PPS1 output signal This register is present only when IEEE1588 timestamping is enabled without an external timestamp input and at least one additional PPS output is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register481_pps1targettimenanosecondsregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register481_pps1targettimenanosecondsregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register481_pps1targettimenanosecondsregister`] module"]
pub type REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER = crate :: Reg < register481_pps1targettimenanosecondsregister :: REGISTER481_PPS1TARGETTIMENANOSECONDSREGISTER_SPEC > ;
#[doc = "Contains the lower 32 bits of time to be compared with the system time to generate the interrupt event or to start generating the PPS1 output signal This register is present only when IEEE1588 timestamping is enabled without an external timestamp input and at least one additional PPS output is selected"]
pub mod register481_pps1targettimenanosecondsregister;
#[doc = "REGISTER544_MACADDRESS32HIGHREGISTER (rw) register accessor: Contains the higher 16 bits of the 33rd MAC address This register is present only when Enable Additional 32 MAC Address Registers is selected in coreConsultant _See Table 78_\n\nYou can [`read`](crate::Reg::read) this register and get [`register544_macaddress32highregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register544_macaddress32highregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register544_macaddress32highregister`] module"]
pub type REGISTER544_MACADDRESS32HIGHREGISTER =
    crate::Reg<register544_macaddress32highregister::REGISTER544_MACADDRESS32HIGHREGISTER_SPEC>;
#[doc = "Contains the higher 16 bits of the 33rd MAC address This register is present only when Enable Additional 32 MAC Address Registers is selected in coreConsultant _See Table 78_"]
pub mod register544_macaddress32highregister;
#[doc = "REGISTER0_BUSMODEREGISTER (rw) register accessor: Controls the Host Interface Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`register0_busmoderegister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register0_busmoderegister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register0_busmoderegister`] module"]
pub type REGISTER0_BUSMODEREGISTER =
    crate::Reg<register0_busmoderegister::REGISTER0_BUSMODEREGISTER_SPEC>;
#[doc = "Controls the Host Interface Mode"]
pub mod register0_busmoderegister;
#[doc = "REGISTER1_TRANSMITPOLLDEMANDREGISTER (r) register accessor: Used by the host to instruct the DMA to poll the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register1_transmitpolldemandregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register1_transmitpolldemandregister`] module"]
pub type REGISTER1_TRANSMITPOLLDEMANDREGISTER =
    crate::Reg<register1_transmitpolldemandregister::REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC>;
#[doc = "Used by the host to instruct the DMA to poll the Transmit Descriptor list"]
pub mod register1_transmitpolldemandregister;
#[doc = "REGISTER2_RECEIVEPOLLDEMANDREGISTER (r) register accessor: Used by the host to instruct the DMA to poll the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register2_receivepolldemandregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register2_receivepolldemandregister`] module"]
pub type REGISTER2_RECEIVEPOLLDEMANDREGISTER =
    crate::Reg<register2_receivepolldemandregister::REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC>;
#[doc = "Used by the host to instruct the DMA to poll the Receive Descriptor list"]
pub mod register2_receivepolldemandregister;
#[doc = "REGISTER3_RECEIVEDESCRIPTORLISTADDRESSREGISTER (rw) register accessor: Points the DMA to the start of the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register3_receivedescriptorlistaddressregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register3_receivedescriptorlistaddressregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register3_receivedescriptorlistaddressregister`] module"]
pub type REGISTER3_RECEIVEDESCRIPTORLISTADDRESSREGISTER = crate :: Reg < register3_receivedescriptorlistaddressregister :: REGISTER3_RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC > ;
#[doc = "Points the DMA to the start of the Receive Descriptor list"]
pub mod register3_receivedescriptorlistaddressregister;
#[doc = "REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER (rw) register accessor: Points the DMA to the start of the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register4_transmitdescriptorlistaddressregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register4_transmitdescriptorlistaddressregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register4_transmitdescriptorlistaddressregister`] module"]
pub type REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER = crate :: Reg < register4_transmitdescriptorlistaddressregister :: REGISTER4_TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC > ;
#[doc = "Points the DMA to the start of the Transmit Descriptor list"]
pub mod register4_transmitdescriptorlistaddressregister;
#[doc = "REGISTER5_STATUSREGISTER (rw) register accessor: The Software driver _application_ reads this register during interrupt service routine or polling to determine the status of the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register5_statusregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register5_statusregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register5_statusregister`] module"]
pub type REGISTER5_STATUSREGISTER =
    crate::Reg<register5_statusregister::REGISTER5_STATUSREGISTER_SPEC>;
#[doc = "The Software driver _application_ reads this register during interrupt service routine or polling to determine the status of the DMA"]
pub mod register5_statusregister;
#[doc = "REGISTER6_OPERATIONMODEREGISTER (rw) register accessor: Establishes the Receive and Transmit operating modes and command Note: This register is valid and present in the GMACMTL configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`register6_operationmoderegister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register6_operationmoderegister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register6_operationmoderegister`] module"]
pub type REGISTER6_OPERATIONMODEREGISTER =
    crate::Reg<register6_operationmoderegister::REGISTER6_OPERATIONMODEREGISTER_SPEC>;
#[doc = "Establishes the Receive and Transmit operating modes and command Note: This register is valid and present in the GMACMTL configuration"]
pub mod register6_operationmoderegister;
#[doc = "REGISTER7_INTERRUPTENABLEREGISTER (rw) register accessor: Enables the interrupts reported by the Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register7_interruptenableregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register7_interruptenableregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register7_interruptenableregister`] module"]
pub type REGISTER7_INTERRUPTENABLEREGISTER =
    crate::Reg<register7_interruptenableregister::REGISTER7_INTERRUPTENABLEREGISTER_SPEC>;
#[doc = "Enables the interrupts reported by the Status Register"]
pub mod register7_interruptenableregister;
#[doc = "REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER (rw) register accessor: Contains the counters for discarded frames because no host Receive Descriptor was available or because of Receive FIFO Overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`register8_missedframeandbufferoverflowcounterregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register8_missedframeandbufferoverflowcounterregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register8_missedframeandbufferoverflowcounterregister`] module"]
pub type REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER = crate :: Reg < register8_missedframeandbufferoverflowcounterregister :: REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC > ;
#[doc = "Contains the counters for discarded frames because no host Receive Descriptor was available or because of Receive FIFO Overflow"]
pub mod register8_missedframeandbufferoverflowcounterregister;
#[doc = "REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER (rw) register accessor: Watchdog timeout for Receive Interrupt _RI_ from DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register9_receiveinterruptwatchdogtimerregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register9_receiveinterruptwatchdogtimerregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register9_receiveinterruptwatchdogtimerregister`] module"]
pub type REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER = crate :: Reg < register9_receiveinterruptwatchdogtimerregister :: REGISTER9_RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC > ;
#[doc = "Watchdog timeout for Receive Interrupt _RI_ from DMA"]
pub mod register9_receiveinterruptwatchdogtimerregister;
#[doc = "REGISTER10_AXIBUSMODEREGISTER (rw) register accessor: Controls AXI master behavior _mainly controls burst splitting and number of outstanding requests_\n\nYou can [`read`](crate::Reg::read) this register and get [`register10_axibusmoderegister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register10_axibusmoderegister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register10_axibusmoderegister`] module"]
pub type REGISTER10_AXIBUSMODEREGISTER =
    crate::Reg<register10_axibusmoderegister::REGISTER10_AXIBUSMODEREGISTER_SPEC>;
#[doc = "Controls AXI master behavior _mainly controls burst splitting and number of outstanding requests_"]
pub mod register10_axibusmoderegister;
#[doc = "REGISTER11_AHBORAXISTATUSREGISTER (r) register accessor: Gives the idle status of the AHB master interface in the GMACAHB configuration Gives the idle status of the AXI master's read or write channel in the GMACAXI configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`register11_ahboraxistatusregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register11_ahboraxistatusregister`] module"]
pub type REGISTER11_AHBORAXISTATUSREGISTER =
    crate::Reg<register11_ahboraxistatusregister::REGISTER11_AHBORAXISTATUSREGISTER_SPEC>;
#[doc = "Gives the idle status of the AHB master interface in the GMACAHB configuration Gives the idle status of the AXI master's read or write channel in the GMACAXI configuration"]
pub mod register11_ahboraxistatusregister;
#[doc = "REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER (r) register accessor: Points to the start of current Transmit Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register18_currenthosttransmitdescriptorregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register18_currenthosttransmitdescriptorregister`] module"]
pub type REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER = crate :: Reg < register18_currenthosttransmitdescriptorregister :: REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC > ;
#[doc = "Points to the start of current Transmit Descriptor read by the DMA"]
pub mod register18_currenthosttransmitdescriptorregister;
#[doc = "REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER (r) register accessor: Points to the start of current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register19_currenthostreceivedescriptorregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register19_currenthostreceivedescriptorregister`] module"]
pub type REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER = crate :: Reg < register19_currenthostreceivedescriptorregister :: REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC > ;
#[doc = "Points to the start of current Receive Descriptor read by the DMA"]
pub mod register19_currenthostreceivedescriptorregister;
#[doc = "REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER (r) register accessor: Points to the current Transmit Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register20_currenthosttransmitbufferaddressregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register20_currenthosttransmitbufferaddressregister`] module"]
pub type REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER = crate :: Reg < register20_currenthosttransmitbufferaddressregister :: REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC > ;
#[doc = "Points to the current Transmit Buffer address read by the DMA"]
pub mod register20_currenthosttransmitbufferaddressregister;
#[doc = "REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER (r) register accessor: Points to the current Receive Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register21_currenthostreceivebufferaddressregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register21_currenthostreceivebufferaddressregister`] module"]
pub type REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER = crate :: Reg < register21_currenthostreceivebufferaddressregister :: REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC > ;
#[doc = "Points to the current Receive Buffer address read by the DMA"]
pub mod register21_currenthostreceivebufferaddressregister;
#[doc = "REGISTER22_HWFEATUREREGISTER (r) register accessor: Indicates the presence of the optional features of the core\n\nYou can [`read`](crate::Reg::read) this register and get [`register22_hwfeatureregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register22_hwfeatureregister`] module"]
pub type REGISTER22_HWFEATUREREGISTER =
    crate::Reg<register22_hwfeatureregister::REGISTER22_HWFEATUREREGISTER_SPEC>;
#[doc = "Indicates the presence of the optional features of the core"]
pub mod register22_hwfeatureregister;
#[doc = "REGISTER64_CHANNEL1BUSMODEREGISTER (rw) register accessor: Controls the Host Interface mode for Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register64_channel1busmoderegister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register64_channel1busmoderegister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register64_channel1busmoderegister`] module"]
pub type REGISTER64_CHANNEL1BUSMODEREGISTER =
    crate::Reg<register64_channel1busmoderegister::REGISTER64_CHANNEL1BUSMODEREGISTER_SPEC>;
#[doc = "Controls the Host Interface mode for Channel 1"]
pub mod register64_channel1busmoderegister;
#[doc = "REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER (r) register accessor: Used by the host to instruct the DMA to poll the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register65_channel1transmitpolldemandregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register65_channel1transmitpolldemandregister`] module"]
pub type REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER = crate :: Reg < register65_channel1transmitpolldemandregister :: REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER_SPEC > ;
#[doc = "Used by the host to instruct the DMA to poll the Transmit Descriptor list"]
pub mod register65_channel1transmitpolldemandregister;
#[doc = "REGISTER66_CHANNEL1RECEIVEPOLLDEMANDREGISTER (r) register accessor: Used by the Host to instruct the DMA to poll the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register66_channel1receivepolldemandregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register66_channel1receivepolldemandregister`] module"]
pub type REGISTER66_CHANNEL1RECEIVEPOLLDEMANDREGISTER = crate::Reg<
    register66_channel1receivepolldemandregister::REGISTER66_CHANNEL1RECEIVEPOLLDEMANDREGISTER_SPEC,
>;
#[doc = "Used by the Host to instruct the DMA to poll the Receive Descriptor list"]
pub mod register66_channel1receivepolldemandregister;
#[doc = "REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER (rw) register accessor: Points the DMA to the start of the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register67_channel1receivedescriptorlistaddressregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register67_channel1receivedescriptorlistaddressregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register67_channel1receivedescriptorlistaddressregister`] module"]
pub type REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER = crate :: Reg < register67_channel1receivedescriptorlistaddressregister :: REGISTER67_CHANNEL1RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC > ;
#[doc = "Points the DMA to the start of the Receive Descriptor list"]
pub mod register67_channel1receivedescriptorlistaddressregister;
#[doc = "REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER (rw) register accessor: Points the DMA to the start of the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register68_channel1transmitdescriptorlistaddressregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register68_channel1transmitdescriptorlistaddressregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register68_channel1transmitdescriptorlistaddressregister`] module"]
pub type REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER = crate :: Reg < register68_channel1transmitdescriptorlistaddressregister :: REGISTER68_CHANNEL1TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC > ;
#[doc = "Points the DMA to the start of the Transmit Descriptor list"]
pub mod register68_channel1transmitdescriptorlistaddressregister;
#[doc = "REGISTER69_CHANNEL1STATUSREGISTER (rw) register accessor: The Software driver _application_ reads this register during interrupt service routine or polling to determine the status of the DMA Bits 29:26 are reserved for the Channel 1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register69_channel1statusregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register69_channel1statusregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register69_channel1statusregister`] module"]
pub type REGISTER69_CHANNEL1STATUSREGISTER =
    crate::Reg<register69_channel1statusregister::REGISTER69_CHANNEL1STATUSREGISTER_SPEC>;
#[doc = "The Software driver _application_ reads this register during interrupt service routine or polling to determine the status of the DMA Bits 29:26 are reserved for the Channel 1 Status Register"]
pub mod register69_channel1statusregister;
#[doc = "REGISTER70_CHANNEL1OPERATIONMODEREGISTER (rw) register accessor: Establishes the Receive and Transmit operating modes and command\n\nYou can [`read`](crate::Reg::read) this register and get [`register70_channel1operationmoderegister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register70_channel1operationmoderegister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register70_channel1operationmoderegister`] module"]
pub type REGISTER70_CHANNEL1OPERATIONMODEREGISTER = crate::Reg<
    register70_channel1operationmoderegister::REGISTER70_CHANNEL1OPERATIONMODEREGISTER_SPEC,
>;
#[doc = "Establishes the Receive and Transmit operating modes and command"]
pub mod register70_channel1operationmoderegister;
#[doc = "REGISTER71_CHANNEL1INTERRUPTENABLEREGISTER (rw) register accessor: Enables the interrupts reported by the Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register71_channel1interruptenableregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register71_channel1interruptenableregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register71_channel1interruptenableregister`] module"]
pub type REGISTER71_CHANNEL1INTERRUPTENABLEREGISTER = crate::Reg<
    register71_channel1interruptenableregister::REGISTER71_CHANNEL1INTERRUPTENABLEREGISTER_SPEC,
>;
#[doc = "Enables the interrupts reported by the Status Register"]
pub mod register71_channel1interruptenableregister;
#[doc = "REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER (rw) register accessor: Contains the counters for discarded frames because no host Receive Descriptor was available, and discarded frames because of Receive FIFO Overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`register72_channel1missedframeandbufferoverflowcounterregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register72_channel1missedframeandbufferoverflowcounterregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register72_channel1missedframeandbufferoverflowcounterregister`] module"]
pub type REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER = crate :: Reg < register72_channel1missedframeandbufferoverflowcounterregister :: REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC > ;
#[doc = "Contains the counters for discarded frames because no host Receive Descriptor was available, and discarded frames because of Receive FIFO Overflow"]
pub mod register72_channel1missedframeandbufferoverflowcounterregister;
#[doc = "REGISTER73_CHANNEL1RECEIVEINTERRUPTWATCHDOGTIMERREGISTER (rw) register accessor: Watchdog timeout for Receive Interrupt _RI_ from DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register73_channel1receiveinterruptwatchdogtimerregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register73_channel1receiveinterruptwatchdogtimerregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register73_channel1receiveinterruptwatchdogtimerregister`] module"]
pub type REGISTER73_CHANNEL1RECEIVEINTERRUPTWATCHDOGTIMERREGISTER = crate :: Reg < register73_channel1receiveinterruptwatchdogtimerregister :: REGISTER73_CHANNEL1RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC > ;
#[doc = "Watchdog timeout for Receive Interrupt _RI_ from DMA"]
pub mod register73_channel1receiveinterruptwatchdogtimerregister;
#[doc = "REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER (rw) register accessor: Contains the control bits for slot function and its status for Channel 1 transmit path\n\nYou can [`read`](crate::Reg::read) this register and get [`register76_channel1slotfunctioncontrolandstatusregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register76_channel1slotfunctioncontrolandstatusregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register76_channel1slotfunctioncontrolandstatusregister`] module"]
pub type REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER = crate :: Reg < register76_channel1slotfunctioncontrolandstatusregister :: REGISTER76_CHANNEL1SLOTFUNCTIONCONTROLANDSTATUSREGISTER_SPEC > ;
#[doc = "Contains the control bits for slot function and its status for Channel 1 transmit path"]
pub mod register76_channel1slotfunctioncontrolandstatusregister;
#[doc = "REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER (r) register accessor: Points to the start of current Transmit Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register82_channel1currenthosttransmitdescriptorregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register82_channel1currenthosttransmitdescriptorregister`] module"]
pub type REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER = crate :: Reg < register82_channel1currenthosttransmitdescriptorregister :: REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC > ;
#[doc = "Points to the start of current Transmit Descriptor read by the DMA"]
pub mod register82_channel1currenthosttransmitdescriptorregister;
#[doc = "REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER (r) register accessor: Points to the start of current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register83_channel1currenthostreceivedescriptorregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register83_channel1currenthostreceivedescriptorregister`] module"]
pub type REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER = crate :: Reg < register83_channel1currenthostreceivedescriptorregister :: REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC > ;
#[doc = "Points to the start of current Receive Descriptor read by the DMA"]
pub mod register83_channel1currenthostreceivedescriptorregister;
#[doc = "REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER (r) register accessor: Points to the current Transmit Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register84_channel1currenthosttransmitbufferaddressregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register84_channel1currenthosttransmitbufferaddressregister`] module"]
pub type REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER = crate :: Reg < register84_channel1currenthosttransmitbufferaddressregister :: REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC > ;
#[doc = "Points to the current Transmit Buffer address read by the DMA"]
pub mod register84_channel1currenthosttransmitbufferaddressregister;
#[doc = "REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER (r) register accessor: Points to the current Receive Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register85_channel1currenthostreceivebufferaddressregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register85_channel1currenthostreceivebufferaddressregister`] module"]
pub type REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER = crate :: Reg < register85_channel1currenthostreceivebufferaddressregister :: REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC > ;
#[doc = "Points to the current Receive Buffer address read by the DMA"]
pub mod register85_channel1currenthostreceivebufferaddressregister;
#[doc = "REGISTER88_CHANNEL1CBSCONTROLREGISTER (rw) register accessor: Controls the Channel 1 credit shaping operation on the transmit path\n\nYou can [`read`](crate::Reg::read) this register and get [`register88_channel1cbscontrolregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register88_channel1cbscontrolregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register88_channel1cbscontrolregister`] module"]
pub type REGISTER88_CHANNEL1CBSCONTROLREGISTER =
    crate::Reg<register88_channel1cbscontrolregister::REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC>;
#[doc = "Controls the Channel 1 credit shaping operation on the transmit path"]
pub mod register88_channel1cbscontrolregister;
#[doc = "REGISTER89_CHANNEL1CBSSTATUSREGISTER (r) register accessor: Provides the average traffic transmitted in Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register89_channel1cbsstatusregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register89_channel1cbsstatusregister`] module"]
pub type REGISTER89_CHANNEL1CBSSTATUSREGISTER =
    crate::Reg<register89_channel1cbsstatusregister::REGISTER89_CHANNEL1CBSSTATUSREGISTER_SPEC>;
#[doc = "Provides the average traffic transmitted in Channel 1"]
pub mod register89_channel1cbsstatusregister;
#[doc = "REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER (rw) register accessor: Contains the idleSlope credit value required for the creditbased shaper algorithm for Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register90_channel1idleslopecreditregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register90_channel1idleslopecreditregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register90_channel1idleslopecreditregister`] module"]
pub type REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER = crate::Reg<
    register90_channel1idleslopecreditregister::REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER_SPEC,
>;
#[doc = "Contains the idleSlope credit value required for the creditbased shaper algorithm for Channel 1"]
pub mod register90_channel1idleslopecreditregister;
#[doc = "REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER (rw) register accessor: Contains the sendSlope credit value required for the creditbased shaper algorithm for Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register91_channel1sendslopecreditregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register91_channel1sendslopecreditregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register91_channel1sendslopecreditregister`] module"]
pub type REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER = crate::Reg<
    register91_channel1sendslopecreditregister::REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER_SPEC,
>;
#[doc = "Contains the sendSlope credit value required for the creditbased shaper algorithm for Channel 1"]
pub mod register91_channel1sendslopecreditregister;
#[doc = "REGISTER92_CHANNEL1HICREDITREGISTER (rw) register accessor: Contains the hiCredit value required for the creditbased shaper algorithm for Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register92_channel1hicreditregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register92_channel1hicreditregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register92_channel1hicreditregister`] module"]
pub type REGISTER92_CHANNEL1HICREDITREGISTER =
    crate::Reg<register92_channel1hicreditregister::REGISTER92_CHANNEL1HICREDITREGISTER_SPEC>;
#[doc = "Contains the hiCredit value required for the creditbased shaper algorithm for Channel 1"]
pub mod register92_channel1hicreditregister;
#[doc = "REGISTER93_CHANNEL1LOCREDITREGISTER (rw) register accessor: Contains the loCredit value required for the creditbased shaper algorithm for Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register93_channel1locreditregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register93_channel1locreditregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register93_channel1locreditregister`] module"]
pub type REGISTER93_CHANNEL1LOCREDITREGISTER =
    crate::Reg<register93_channel1locreditregister::REGISTER93_CHANNEL1LOCREDITREGISTER_SPEC>;
#[doc = "Contains the loCredit value required for the creditbased shaper algorithm for Channel 1"]
pub mod register93_channel1locreditregister;
#[doc = "REGISTER128_CHANNEL2BUSMODEREGISTER (rw) register accessor: Controls the Host Interface mode for Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register128_channel2busmoderegister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register128_channel2busmoderegister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register128_channel2busmoderegister`] module"]
pub type REGISTER128_CHANNEL2BUSMODEREGISTER =
    crate::Reg<register128_channel2busmoderegister::REGISTER128_CHANNEL2BUSMODEREGISTER_SPEC>;
#[doc = "Controls the Host Interface mode for Channel 2"]
pub mod register128_channel2busmoderegister;
#[doc = "REGISTER129_CHANNEL2TRANSMITPOLLDEMANDREGISTER (r) register accessor: Used by the host to instruct the DMA to poll the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register129_channel2transmitpolldemandregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register129_channel2transmitpolldemandregister`] module"]
pub type REGISTER129_CHANNEL2TRANSMITPOLLDEMANDREGISTER = crate :: Reg < register129_channel2transmitpolldemandregister :: REGISTER129_CHANNEL2TRANSMITPOLLDEMANDREGISTER_SPEC > ;
#[doc = "Used by the host to instruct the DMA to poll the Transmit Descriptor list"]
pub mod register129_channel2transmitpolldemandregister;
#[doc = "REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER (r) register accessor: Used by the Host to instruct the DMA to poll the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register130_channel2receivepolldemandregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register130_channel2receivepolldemandregister`] module"]
pub type REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER = crate :: Reg < register130_channel2receivepolldemandregister :: REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER_SPEC > ;
#[doc = "Used by the Host to instruct the DMA to poll the Receive Descriptor list"]
pub mod register130_channel2receivepolldemandregister;
#[doc = "REGISTER131_CHANNEL2RECEIVEDESCRIPTORLISTADDRESSREGISTER (rw) register accessor: Points the DMA to the start of the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register131_channel2receivedescriptorlistaddressregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register131_channel2receivedescriptorlistaddressregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register131_channel2receivedescriptorlistaddressregister`] module"]
pub type REGISTER131_CHANNEL2RECEIVEDESCRIPTORLISTADDRESSREGISTER = crate :: Reg < register131_channel2receivedescriptorlistaddressregister :: REGISTER131_CHANNEL2RECEIVEDESCRIPTORLISTADDRESSREGISTER_SPEC > ;
#[doc = "Points the DMA to the start of the Receive Descriptor list"]
pub mod register131_channel2receivedescriptorlistaddressregister;
#[doc = "REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER (rw) register accessor: Points the DMA to the start of the Transmit Descriptor List\n\nYou can [`read`](crate::Reg::read) this register and get [`register132_channel2transmitdescriptorlistaddressregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register132_channel2transmitdescriptorlistaddressregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register132_channel2transmitdescriptorlistaddressregister`] module"]
pub type REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER = crate :: Reg < register132_channel2transmitdescriptorlistaddressregister :: REGISTER132_CHANNEL2TRANSMITDESCRIPTORLISTADDRESSREGISTER_SPEC > ;
#[doc = "Points the DMA to the start of the Transmit Descriptor List"]
pub mod register132_channel2transmitdescriptorlistaddressregister;
#[doc = "REGISTER133_CHANNEL2STATUSREGISTER (rw) register accessor: The software driver _application_ reads this register during interrupt service routine or polling to determine the status of the DMA Bits \\[29:26\\] are reserved for the Channel 2 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register133_channel2statusregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register133_channel2statusregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register133_channel2statusregister`] module"]
pub type REGISTER133_CHANNEL2STATUSREGISTER =
    crate::Reg<register133_channel2statusregister::REGISTER133_CHANNEL2STATUSREGISTER_SPEC>;
#[doc = "The software driver _application_ reads this register during interrupt service routine or polling to determine the status of the DMA Bits \\[29:26\\] are reserved for the Channel 2 Status Register"]
pub mod register133_channel2statusregister;
#[doc = "REGISTER134_CHANNEL2OPERATIONMODEREGISTER (rw) register accessor: Establishes the Receive and Transmit operating modes and command\n\nYou can [`read`](crate::Reg::read) this register and get [`register134_channel2operationmoderegister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register134_channel2operationmoderegister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register134_channel2operationmoderegister`] module"]
pub type REGISTER134_CHANNEL2OPERATIONMODEREGISTER = crate::Reg<
    register134_channel2operationmoderegister::REGISTER134_CHANNEL2OPERATIONMODEREGISTER_SPEC,
>;
#[doc = "Establishes the Receive and Transmit operating modes and command"]
pub mod register134_channel2operationmoderegister;
#[doc = "REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER (rw) register accessor: Enables the interrupts reported by the Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register135_channel2interruptenableregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register135_channel2interruptenableregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register135_channel2interruptenableregister`] module"]
pub type REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER = crate::Reg<
    register135_channel2interruptenableregister::REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC,
>;
#[doc = "Enables the interrupts reported by the Status Register"]
pub mod register135_channel2interruptenableregister;
#[doc = "REGISTER136_CHANNEL2MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER (rw) register accessor: Contains the counters for discarded frames because no host Receive Descriptor was available or because of Receive FIFO Overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`register136_channel2missedframeandbufferoverflowcounterregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register136_channel2missedframeandbufferoverflowcounterregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register136_channel2missedframeandbufferoverflowcounterregister`] module"]
pub type REGISTER136_CHANNEL2MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER = crate :: Reg < register136_channel2missedframeandbufferoverflowcounterregister :: REGISTER136_CHANNEL2MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC > ;
#[doc = "Contains the counters for discarded frames because no host Receive Descriptor was available or because of Receive FIFO Overflow"]
pub mod register136_channel2missedframeandbufferoverflowcounterregister;
#[doc = "REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER (rw) register accessor: Watchdog timeout for Receive Interrupt _RI_ from DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register137_channel2receiveinterruptwatchdogtimerregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register137_channel2receiveinterruptwatchdogtimerregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register137_channel2receiveinterruptwatchdogtimerregister`] module"]
pub type REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER = crate :: Reg < register137_channel2receiveinterruptwatchdogtimerregister :: REGISTER137_CHANNEL2RECEIVEINTERRUPTWATCHDOGTIMERREGISTER_SPEC > ;
#[doc = "Watchdog timeout for Receive Interrupt _RI_ from DMA"]
pub mod register137_channel2receiveinterruptwatchdogtimerregister;
#[doc = "REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER (r) register accessor: Points to the start of current Transmit Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register146_channel2currenthosttransmitdescriptorregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register146_channel2currenthosttransmitdescriptorregister`] module"]
pub type REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER = crate :: Reg < register146_channel2currenthosttransmitdescriptorregister :: REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC > ;
#[doc = "Points to the start of current Transmit Descriptor read by the DMA"]
pub mod register146_channel2currenthosttransmitdescriptorregister;
#[doc = "REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER (r) register accessor: Points to the start of current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register147_channel2currenthostreceivedescriptorregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register147_channel2currenthostreceivedescriptorregister`] module"]
pub type REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER = crate :: Reg < register147_channel2currenthostreceivedescriptorregister :: REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC > ;
#[doc = "Points to the start of current Receive Descriptor read by the DMA"]
pub mod register147_channel2currenthostreceivedescriptorregister;
#[doc = "REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER (r) register accessor: Points to the current Transmit Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register148_channel2currenthosttransmitbufferaddressregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register148_channel2currenthosttransmitbufferaddressregister`] module"]
pub type REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER = crate :: Reg < register148_channel2currenthosttransmitbufferaddressregister :: REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC > ;
#[doc = "Points to the current Transmit Buffer address read by the DMA"]
pub mod register148_channel2currenthosttransmitbufferaddressregister;
#[doc = "REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER (r) register accessor: Points to the current Receive Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register149_channel2currenthostreceivebufferaddressregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register149_channel2currenthostreceivebufferaddressregister`] module"]
pub type REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER = crate :: Reg < register149_channel2currenthostreceivebufferaddressregister :: REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC > ;
#[doc = "Points to the current Receive Buffer address read by the DMA"]
pub mod register149_channel2currenthostreceivebufferaddressregister;
#[doc = "REGISTER152_CHANNEL2CBSCONTROLREGISTER (rw) register accessor: Controls the Channel 2 credit shaping operation on the transmit path\n\nYou can [`read`](crate::Reg::read) this register and get [`register152_channel2cbscontrolregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register152_channel2cbscontrolregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register152_channel2cbscontrolregister`] module"]
pub type REGISTER152_CHANNEL2CBSCONTROLREGISTER =
    crate::Reg<register152_channel2cbscontrolregister::REGISTER152_CHANNEL2CBSCONTROLREGISTER_SPEC>;
#[doc = "Controls the Channel 2 credit shaping operation on the transmit path"]
pub mod register152_channel2cbscontrolregister;
#[doc = "REGISTER153_CHANNEL2CBSSTATUSREGISTER (r) register accessor: Provides the average traffic transmitted in Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register153_channel2cbsstatusregister::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register153_channel2cbsstatusregister`] module"]
pub type REGISTER153_CHANNEL2CBSSTATUSREGISTER =
    crate::Reg<register153_channel2cbsstatusregister::REGISTER153_CHANNEL2CBSSTATUSREGISTER_SPEC>;
#[doc = "Provides the average traffic transmitted in Channel 2"]
pub mod register153_channel2cbsstatusregister;
#[doc = "REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER (rw) register accessor: Contains the idleSlope credit value required for the creditbased shaper algorithm for Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register154_channel2idleslopecreditregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register154_channel2idleslopecreditregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register154_channel2idleslopecreditregister`] module"]
pub type REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER = crate::Reg<
    register154_channel2idleslopecreditregister::REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER_SPEC,
>;
#[doc = "Contains the idleSlope credit value required for the creditbased shaper algorithm for Channel 2"]
pub mod register154_channel2idleslopecreditregister;
#[doc = "REGISTER155_CHANNEL2SENDSLOPECREDITREGISTER (rw) register accessor: Contains the sendSlope credit value required for the creditbased shaper algorithm for Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register155_channel2sendslopecreditregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register155_channel2sendslopecreditregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register155_channel2sendslopecreditregister`] module"]
pub type REGISTER155_CHANNEL2SENDSLOPECREDITREGISTER = crate::Reg<
    register155_channel2sendslopecreditregister::REGISTER155_CHANNEL2SENDSLOPECREDITREGISTER_SPEC,
>;
#[doc = "Contains the sendSlope credit value required for the creditbased shaper algorithm for Channel 2"]
pub mod register155_channel2sendslopecreditregister;
#[doc = "REGISTER156_CHANNEL2HICREDITREGISTER (rw) register accessor: Contains the hiCredit value required for the creditbased shaper algorithm for Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register156_channel2hicreditregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register156_channel2hicreditregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register156_channel2hicreditregister`] module"]
pub type REGISTER156_CHANNEL2HICREDITREGISTER =
    crate::Reg<register156_channel2hicreditregister::REGISTER156_CHANNEL2HICREDITREGISTER_SPEC>;
#[doc = "Contains the hiCredit value required for the creditbased shaper algorithm for Channel 2"]
pub mod register156_channel2hicreditregister;
#[doc = "REGISTER157_CHANNEL2LOCREDITREGISTER (rw) register accessor: Contains the loCredit value required for the creditbased shaper algorithm for Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register157_channel2locreditregister::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register157_channel2locreditregister::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@register157_channel2locreditregister`] module"]
pub type REGISTER157_CHANNEL2LOCREDITREGISTER =
    crate::Reg<register157_channel2locreditregister::REGISTER157_CHANNEL2LOCREDITREGISTER_SPEC>;
#[doc = "Contains the loCredit value required for the creditbased shaper algorithm for Channel 2"]
pub mod register157_channel2locreditregister;
