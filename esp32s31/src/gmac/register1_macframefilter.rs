#[doc = "Register `REGISTER1_MACFRAMEFILTER` reader"]
pub type R = crate::R<REGISTER1_MACFRAMEFILTER_SPEC>;
#[doc = "Register `REGISTER1_MACFRAMEFILTER` writer"]
pub type W = crate::W<REGISTER1_MACFRAMEFILTER_SPEC>;
#[doc = "Field `PROMISCUOUS_MODE` reader - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames irrespective of the destination or source address The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR is set"]
pub type PROMISCUOUS_MODE_R = crate::BitReader;
#[doc = "Field `PROMISCUOUS_MODE` writer - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames irrespective of the destination or source address The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR is set"]
pub type PROMISCUOUS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUC` reader - Hash Unicast When set, the MAC performs destination address filtering of unicast frames according to the hash table When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers If Hash Filter is not selected during core configuration, this bit is reserved _and RO_"]
pub type HUC_R = crate::BitReader;
#[doc = "Field `HUC` writer - Hash Unicast When set, the MAC performs destination address filtering of unicast frames according to the hash table When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers If Hash Filter is not selected during core configuration, this bit is reserved _and RO_"]
pub type HUC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMC` reader - Hash Multicast When set, the MAC performs destination address filtering of received multicast frames according to the hash table When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers If Hash Filter is not selected during core configuration, this bit is reserved _and RO_"]
pub type HMC_R = crate::BitReader;
#[doc = "Field `HMC` writer - Hash Multicast When set, the MAC performs destination address filtering of received multicast frames according to the hash table When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers If Hash Filter is not selected during core configuration, this bit is reserved _and RO_"]
pub type HMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames When reset, normal filtering of frames is performed"]
pub type DAIF_R = crate::BitReader;
#[doc = "Field `DAIF` writer - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames When reset, normal filtering of frames is performed"]
pub type DAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address _first bit in the destination address field is '1'_ are passed When reset, filtering of multicast frame depends on HMC bit"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address _first bit in the destination address field is '1'_ are passed When reset, filtering of multicast frame depends on HMC bit"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBF` reader - Disable Broadcast Frames When this bit is set, the AFM module blocks all incoming broadcast frames In addition, it overrides all other filter settings When this bit is reset, the AFM module passes all received broadcast frames"]
pub type DBF_R = crate::BitReader;
#[doc = "Field `DBF` writer - Disable Broadcast Frames When this bit is set, the AFM module blocks all incoming broadcast frames In addition, it overrides all other filter settings When this bit is reset, the AFM module passes all received broadcast frames"]
pub type DBF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - Pass Control Frames These bits control the forwarding of all control frames _including unicast and multicast Pause frames_ 00: MAC filters all control frames from reaching the application 01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter 10: MAC forwards all control frames to application even if they fail the Address Filter 11: MAC forwards control frames that pass the Address Filter The following conditions should be true for the Pause frames processing: Condition 1 : The MAC is in the fullduplex mode and flow control is enabled by setting Bit 2 _RFE_ of Register 6 _Flow Control Register_ to 1 Condition 2 : The destination address _DA_ of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 _UP_ of the Register 6 _Flow Control Register_ is set Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001 Note: This field should be set to 01 only when the Condition 1 is true, that is, the MAC is programmed to operate in the fullduplex mode and the RFE bit is enabled Otherwise, the Pause frame filtering may be inconsistent When Condition 1 is false, the Pause frames are considered as generic control frames Therefore, to pass all control frames _including Pause frames_ when the fullduplex mode and flow control is not enabled, you should set the PCF field to 10 or 11 _as required by the application_"]
pub type PCF_R = crate::FieldReader;
#[doc = "Field `PCF` writer - Pass Control Frames These bits control the forwarding of all control frames _including unicast and multicast Pause frames_ 00: MAC filters all control frames from reaching the application 01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter 10: MAC forwards all control frames to application even if they fail the Address Filter 11: MAC forwards control frames that pass the Address Filter The following conditions should be true for the Pause frames processing: Condition 1 : The MAC is in the fullduplex mode and flow control is enabled by setting Bit 2 _RFE_ of Register 6 _Flow Control Register_ to 1 Condition 2 : The destination address _DA_ of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 _UP_ of the Register 6 _Flow Control Register_ is set Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001 Note: This field should be set to 01 only when the Condition 1 is true, that is, the MAC is programmed to operate in the fullduplex mode and the RFE bit is enabled Otherwise, the Pause frame filtering may be inconsistent When Condition 1 is false, the Pause frames are considered as generic control frames Therefore, to pass all control frames _including Pause frames_ when the fullduplex mode and flow control is not enabled, you should set the PCF field to 10 or 11 _as required by the application_"]
pub type PCF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAIF` reader - SA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the SA address comparison The frames whose SA matches the SA registers are marked as failing the SA Address filter When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA Address filter"]
pub type SAIF_R = crate::BitReader;
#[doc = "Field `SAIF` writer - SA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the SA address comparison The frames whose SA matches the SA registers are marked as failing the SA Address filter When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA Address filter"]
pub type SAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers If the comparison fails, the MAC drops the frame When this bit is reset, the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison Note: According to the IEEE specification, Bit 47 of the SA is reserved and set to 0 However, in DWC_gmac, the MAC compares all 48 bits The software driver should take this into consideration while programming the MAC address registers for SA"]
pub type SAF_R = crate::BitReader;
#[doc = "Field `SAF` writer - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers If the comparison fails, the MAC drops the frame When this bit is reset, the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison Note: According to the IEEE specification, Bit 47 of the SA is reserved and set to 0 However, in DWC_gmac, the MAC compares all 48 bits The software driver should take this into consideration while programming the MAC address registers for SA"]
pub type SAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPF` reader - Hash or Perfect Filter When this bit is set, it configures the address filter to pass a frame if it matches either the perfect filtering or the hash filtering as set by the HMC or HUC bits When this bit is low and the HUC or HMC bit is set, the frame is passed only if it matches the Hash filter This bit is reserved _and RO_ if the Hash filter is not selected during core configuration"]
pub type HPF_R = crate::BitReader;
#[doc = "Field `HPF` writer - Hash or Perfect Filter When this bit is set, it configures the address filter to pass a frame if it matches either the perfect filtering or the hash filtering as set by the HMC or HUC bits When this bit is low and the HUC or HMC bit is set, the frame is passed only if it matches the Hash filter This bit is reserved _and RO_ if the Hash filter is not selected during core configuration"]
pub type HPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTFE` reader - VLAN Tag Filter Enable When set, this bit enables the MAC to drop VLAN tagged frames that do not match the VLAN Tag comparison When reset, the MAC forwards all frames irrespective of the match status of the VLAN Tag"]
pub type VTFE_R = crate::BitReader;
#[doc = "Field `VTFE` writer - VLAN Tag Filter Enable When set, this bit enables the MAC to drop VLAN tagged frames that do not match the VLAN Tag comparison When reset, the MAC forwards all frames irrespective of the match status of the VLAN Tag"]
pub type VTFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPFE` reader - Layer 3 and Layer 4 Filter Enable When set, this bit enables the MAC to drop frames that do not match the enabled Layer 3 and Layer 4 filters If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect When reset, the MAC forwards all frames irrespective of the match status of the Layer 3 and Layer 4 fields If the Layer 3 and Layer 4 Filtering feature is not selected during core configuration, this bit is reserved _RO with default value_"]
pub type IPFE_R = crate::BitReader;
#[doc = "Field `IPFE` writer - Layer 3 and Layer 4 Filter Enable When set, this bit enables the MAC to drop frames that do not match the enabled Layer 3 and Layer 4 filters If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect When reset, the MAC forwards all frames irrespective of the match status of the Layer 3 and Layer 4 fields If the Layer 3 and Layer 4 Filtering feature is not selected during core configuration, this bit is reserved _RO with default value_"]
pub type IPFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNTU` reader - Drop nonTCP/UDP over IP Frames When set, this bit enables the MAC to drop the nonTCP or UDP over IP frames The MAC forward only those frames that are processed by the Layer 4 filter When reset, this bit enables the MAC to forward all nonTCP or UDP over IP frames If the Layer 3 and Layer 4 Filtering feature is not selected during core configuration, this bit is reserved _RO with default value_"]
pub type DNTU_R = crate::BitReader;
#[doc = "Field `DNTU` writer - Drop nonTCP/UDP over IP Frames When set, this bit enables the MAC to drop the nonTCP or UDP over IP frames The MAC forward only those frames that are processed by the Layer 4 filter When reset, this bit enables the MAC to forward all nonTCP or UDP over IP frames If the Layer 3 and Layer 4 Filtering feature is not selected during core configuration, this bit is reserved _RO with default value_"]
pub type DNTU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RA` reader - Receive All When this bit is set, the MAC Receiver module passes all received frames, irrespective of whether they pass the address filter or not, to the Application The result of the SA or DA filtering is updated _pass or fail_ in the corresponding bits in the Receive Status Word When this bit is reset, the Receiver module passes only those frames to the Application that pass the SA or DA address filter"]
pub type RA_R = crate::BitReader;
#[doc = "Field `RA` writer - Receive All When this bit is set, the MAC Receiver module passes all received frames, irrespective of whether they pass the address filter or not, to the Application The result of the SA or DA filtering is updated _pass or fail_ in the corresponding bits in the Receive Status Word When this bit is reset, the Receiver module passes only those frames to the Application that pass the SA or DA address filter"]
pub type RA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames irrespective of the destination or source address The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR is set"]
    #[inline(always)]
    pub fn promiscuous_mode(&self) -> PROMISCUOUS_MODE_R {
        PROMISCUOUS_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast When set, the MAC performs destination address filtering of unicast frames according to the hash table When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers If Hash Filter is not selected during core configuration, this bit is reserved _and RO_"]
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast When set, the MAC performs destination address filtering of received multicast frames according to the hash table When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers If Hash Filter is not selected during core configuration, this bit is reserved _and RO_"]
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames When reset, normal filtering of frames is performed"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address _first bit in the destination address field is '1'_ are passed When reset, filtering of multicast frame depends on HMC bit"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames When this bit is set, the AFM module blocks all incoming broadcast frames In addition, it overrides all other filter settings When this bit is reset, the AFM module passes all received broadcast frames"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames These bits control the forwarding of all control frames _including unicast and multicast Pause frames_ 00: MAC filters all control frames from reaching the application 01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter 10: MAC forwards all control frames to application even if they fail the Address Filter 11: MAC forwards control frames that pass the Address Filter The following conditions should be true for the Pause frames processing: Condition 1 : The MAC is in the fullduplex mode and flow control is enabled by setting Bit 2 _RFE_ of Register 6 _Flow Control Register_ to 1 Condition 2 : The destination address _DA_ of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 _UP_ of the Register 6 _Flow Control Register_ is set Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001 Note: This field should be set to 01 only when the Condition 1 is true, that is, the MAC is programmed to operate in the fullduplex mode and the RFE bit is enabled Otherwise, the Pause frame filtering may be inconsistent When Condition 1 is false, the Pause frames are considered as generic control frames Therefore, to pass all control frames _including Pause frames_ when the fullduplex mode and flow control is not enabled, you should set the PCF field to 10 or 11 _as required by the application_"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the SA address comparison The frames whose SA matches the SA registers are marked as failing the SA Address filter When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA Address filter"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers If the comparison fails, the MAC drops the frame When this bit is reset, the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison Note: According to the IEEE specification, Bit 47 of the SA is reserved and set to 0 However, in DWC_gmac, the MAC compares all 48 bits The software driver should take this into consideration while programming the MAC address registers for SA"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter When this bit is set, it configures the address filter to pass a frame if it matches either the perfect filtering or the hash filtering as set by the HMC or HUC bits When this bit is low and the HUC or HMC bit is set, the frame is passed only if it matches the Hash filter This bit is reserved _and RO_ if the Hash filter is not selected during core configuration"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable When set, this bit enables the MAC to drop VLAN tagged frames that do not match the VLAN Tag comparison When reset, the MAC forwards all frames irrespective of the match status of the VLAN Tag"]
    #[inline(always)]
    pub fn vtfe(&self) -> VTFE_R {
        VTFE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Layer 3 and Layer 4 Filter Enable When set, this bit enables the MAC to drop frames that do not match the enabled Layer 3 and Layer 4 filters If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect When reset, the MAC forwards all frames irrespective of the match status of the Layer 3 and Layer 4 fields If the Layer 3 and Layer 4 Filtering feature is not selected during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn ipfe(&self) -> IPFE_R {
        IPFE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Drop nonTCP/UDP over IP Frames When set, this bit enables the MAC to drop the nonTCP or UDP over IP frames The MAC forward only those frames that are processed by the Layer 4 filter When reset, this bit enables the MAC to forward all nonTCP or UDP over IP frames If the Layer 3 and Layer 4 Filtering feature is not selected during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive All When this bit is set, the MAC Receiver module passes all received frames, irrespective of whether they pass the address filter or not, to the Application The result of the SA or DA filtering is updated _pass or fail_ in the corresponding bits in the Receive Status Word When this bit is reset, the Receiver module passes only those frames to the Application that pass the SA or DA address filter"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER1_MACFRAMEFILTER")
            .field("promiscuous_mode", &self.promiscuous_mode())
            .field("huc", &self.huc())
            .field("hmc", &self.hmc())
            .field("daif", &self.daif())
            .field("pm", &self.pm())
            .field("dbf", &self.dbf())
            .field("pcf", &self.pcf())
            .field("saif", &self.saif())
            .field("saf", &self.saf())
            .field("hpf", &self.hpf())
            .field("vtfe", &self.vtfe())
            .field("ipfe", &self.ipfe())
            .field("dntu", &self.dntu())
            .field("ra", &self.ra())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames irrespective of the destination or source address The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR is set"]
    #[inline(always)]
    pub fn promiscuous_mode(&mut self) -> PROMISCUOUS_MODE_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        PROMISCUOUS_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Hash Unicast When set, the MAC performs destination address filtering of unicast frames according to the hash table When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers If Hash Filter is not selected during core configuration, this bit is reserved _and RO_"]
    #[inline(always)]
    pub fn huc(&mut self) -> HUC_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        HUC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Hash Multicast When set, the MAC performs destination address filtering of received multicast frames according to the hash table When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers If Hash Filter is not selected during core configuration, this bit is reserved _and RO_"]
    #[inline(always)]
    pub fn hmc(&mut self) -> HMC_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        HMC_W::new(self, 2)
    }
    #[doc = "Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames When reset, normal filtering of frames is performed"]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        DAIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address _first bit in the destination address field is '1'_ are passed When reset, filtering of multicast frame depends on HMC bit"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        PM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames When this bit is set, the AFM module blocks all incoming broadcast frames In addition, it overrides all other filter settings When this bit is reset, the AFM module passes all received broadcast frames"]
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        DBF_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Pass Control Frames These bits control the forwarding of all control frames _including unicast and multicast Pause frames_ 00: MAC filters all control frames from reaching the application 01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter 10: MAC forwards all control frames to application even if they fail the Address Filter 11: MAC forwards control frames that pass the Address Filter The following conditions should be true for the Pause frames processing: Condition 1 : The MAC is in the fullduplex mode and flow control is enabled by setting Bit 2 _RFE_ of Register 6 _Flow Control Register_ to 1 Condition 2 : The destination address _DA_ of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 _UP_ of the Register 6 _Flow Control Register_ is set Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001 Note: This field should be set to 01 only when the Condition 1 is true, that is, the MAC is programmed to operate in the fullduplex mode and the RFE bit is enabled Otherwise, the Pause frame filtering may be inconsistent When Condition 1 is false, the Pause frames are considered as generic control frames Therefore, to pass all control frames _including Pause frames_ when the fullduplex mode and flow control is not enabled, you should set the PCF field to 10 or 11 _as required by the application_"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        PCF_W::new(self, 6)
    }
    #[doc = "Bit 8 - SA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the SA address comparison The frames whose SA matches the SA registers are marked as failing the SA Address filter When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA Address filter"]
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        SAIF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers If the comparison fails, the MAC drops the frame When this bit is reset, the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison Note: According to the IEEE specification, Bit 47 of the SA is reserved and set to 0 However, in DWC_gmac, the MAC compares all 48 bits The software driver should take this into consideration while programming the MAC address registers for SA"]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        SAF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter When this bit is set, it configures the address filter to pass a frame if it matches either the perfect filtering or the hash filtering as set by the HMC or HUC bits When this bit is low and the HUC or HMC bit is set, the frame is passed only if it matches the Hash filter This bit is reserved _and RO_ if the Hash filter is not selected during core configuration"]
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        HPF_W::new(self, 10)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable When set, this bit enables the MAC to drop VLAN tagged frames that do not match the VLAN Tag comparison When reset, the MAC forwards all frames irrespective of the match status of the VLAN Tag"]
    #[inline(always)]
    pub fn vtfe(&mut self) -> VTFE_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        VTFE_W::new(self, 16)
    }
    #[doc = "Bit 20 - Layer 3 and Layer 4 Filter Enable When set, this bit enables the MAC to drop frames that do not match the enabled Layer 3 and Layer 4 filters If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect When reset, the MAC forwards all frames irrespective of the match status of the Layer 3 and Layer 4 fields If the Layer 3 and Layer 4 Filtering feature is not selected during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn ipfe(&mut self) -> IPFE_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        IPFE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Drop nonTCP/UDP over IP Frames When set, this bit enables the MAC to drop the nonTCP or UDP over IP frames The MAC forward only those frames that are processed by the Layer 4 filter When reset, this bit enables the MAC to forward all nonTCP or UDP over IP frames If the Layer 3 and Layer 4 Filtering feature is not selected during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn dntu(&mut self) -> DNTU_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        DNTU_W::new(self, 21)
    }
    #[doc = "Bit 31 - Receive All When this bit is set, the MAC Receiver module passes all received frames, irrespective of whether they pass the address filter or not, to the Application The result of the SA or DA filtering is updated _pass or fail_ in the corresponding bits in the Receive Status Word When this bit is reset, the Receiver module passes only those frames to the Application that pass the SA or DA address filter"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<'_, REGISTER1_MACFRAMEFILTER_SPEC> {
        RA_W::new(self, 31)
    }
}
#[doc = "Contains the frame filtering controls\n\nYou can [`read`](crate::Reg::read) this register and get [`register1_macframefilter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register1_macframefilter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER1_MACFRAMEFILTER_SPEC;
impl crate::RegisterSpec for REGISTER1_MACFRAMEFILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register1_macframefilter::R`](R) reader structure"]
impl crate::Readable for REGISTER1_MACFRAMEFILTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register1_macframefilter::W`](W) writer structure"]
impl crate::Writable for REGISTER1_MACFRAMEFILTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER1_MACFRAMEFILTER to value 0"]
impl crate::Resettable for REGISTER1_MACFRAMEFILTER_SPEC {}
