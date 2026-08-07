#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GUSBCFG_SPEC>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GUSBCFG_SPEC>;
#[doc = "Field `TOUTCAL` reader - Mode: Host and Device HS/FS Timeout Calibration (TOutCal) The number of PHY clocks that the application programs in this field is added to the high-speed/full-speed interpacket timeout duration in the core to account for any additional delays introduced by the PHY. This can be required, because the delay introduced by the PHY in generating the linestate condition can vary from one PHY to another. The USB standard timeout value for high-speed operation is 736 to 816 (inclusive) bit times. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of enumeration. The number of bit times added per PHY clock are as follows: High-speed operation: - One 30-MHz PHY clock = 16 bit times - One 60-MHz PHY clock = 8 bit times Full-speed operation: - One 30-MHz PHY clock = 0.4 bit times - One 60-MHz PHY clock = 0.2 bit times - One 48-MHz PHY clock = 0.25 bit times"]
pub type TOUTCAL_R = crate::FieldReader;
#[doc = "Field `TOUTCAL` writer - Mode: Host and Device HS/FS Timeout Calibration (TOutCal) The number of PHY clocks that the application programs in this field is added to the high-speed/full-speed interpacket timeout duration in the core to account for any additional delays introduced by the PHY. This can be required, because the delay introduced by the PHY in generating the linestate condition can vary from one PHY to another. The USB standard timeout value for high-speed operation is 736 to 816 (inclusive) bit times. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of enumeration. The number of bit times added per PHY clock are as follows: High-speed operation: - One 30-MHz PHY clock = 16 bit times - One 60-MHz PHY clock = 8 bit times Full-speed operation: - One 30-MHz PHY clock = 0.4 bit times - One 60-MHz PHY clock = 0.2 bit times - One 48-MHz PHY clock = 0.25 bit times"]
pub type TOUTCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHYIF` reader - Mode: Host and Device PHY Interface (PHYIf) The application uses this bit to configure the core to support a UTMI+ PHY with an 8- or 16-bit interface. When a ULPI PHY is chosen, this must be Set to 8-bit mode. - 1'b0: 8 bits - 1'b1: 16 bits This bit is writable only If UTMI+ and ULPI were selected. Otherwise, this bit returns the value for the power-on interface selected during configuration."]
pub type PHYIF_R = crate::BitReader;
#[doc = "Field `PHYIF` writer - Mode: Host and Device PHY Interface (PHYIf) The application uses this bit to configure the core to support a UTMI+ PHY with an 8- or 16-bit interface. When a ULPI PHY is chosen, this must be Set to 8-bit mode. - 1'b0: 8 bits - 1'b1: 16 bits This bit is writable only If UTMI+ and ULPI were selected. Otherwise, this bit returns the value for the power-on interface selected during configuration."]
pub type PHYIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSINTF` reader - Mode: Host and Device Full-Speed Serial Interface Select (FSIntf) The application uses this bit to select either a unidirectional or bidirectional USB 1.1 full-speed serial transceiver interface. - 1'b0: 6-pin unidirectional full-speed serial interface - 1'b1: 3-pin bidirectional full-speed serial interface If a USB 1.1 Full-Speed Serial Transceiver interface was not selected, this bit is always 0, with Write Only access. If a USB 1.1 FS interface was selected, Then the application can Set this bit to select between the 3- and 6-pin interfaces, and access is Read and Write. Note: For supporting the new 4-pin bi-directional interface, you need to select 6-pin unidirectional FS serial mode, and add an external control to convert it to a 4-pin interface."]
pub type FSINTF_R = crate::BitReader;
#[doc = "Field `FSINTF` writer - Mode: Host and Device Full-Speed Serial Interface Select (FSIntf) The application uses this bit to select either a unidirectional or bidirectional USB 1.1 full-speed serial transceiver interface. - 1'b0: 6-pin unidirectional full-speed serial interface - 1'b1: 3-pin bidirectional full-speed serial interface If a USB 1.1 Full-Speed Serial Transceiver interface was not selected, this bit is always 0, with Write Only access. If a USB 1.1 FS interface was selected, Then the application can Set this bit to select between the 3- and 6-pin interfaces, and access is Read and Write. Note: For supporting the new 4-pin bi-directional interface, you need to select 6-pin unidirectional FS serial mode, and add an external control to convert it to a 4-pin interface."]
pub type FSINTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSEL` reader - PHYSel Mode: Host and Device USB 2.0 High-Speed PHY or USB 1.1 Full-Speed Serial Transceiver Select (PHYSel) The application uses this bit to select either a high-speed UTMI+ or ULPI PHY, or a full-speed transceiver. - 1'b0: USB 2.0 high-speed UTMI+ or ULPI PHY - 1'b1: USB 1.1 full-speed serial transceiver If a USB 1.1 Full-Speed Serial Transceiver interface was not selected in, this bit is always 0, with Write Only access. If a high-speed PHY interface was not selected in, this bit is always 1, with Write Only access. If both interface types were selected (parameters have non-zero values), the application uses this bit to select which interface is active, and access is Read and Write."]
pub type PHYSEL_R = crate::BitReader;
#[doc = "Field `SRPCAP` reader - Mode: Host and Device SRP-Capable (SRPCap) The application uses this bit to control the controller's SRP capabilities. If the core operates as a non-SRP-capable B-device, it cannot request the connected A-device (host) to activate VBUS and start a session. - 1'b0: SRP capability is not enabled. - 1'b1: SRP capability is enabled. If SRP functionality is disabled by the software, the OTG signals on the PHY domain must be tied to the appropriate values."]
pub type SRPCAP_R = crate::BitReader;
#[doc = "Field `SRPCAP` writer - Mode: Host and Device SRP-Capable (SRPCap) The application uses this bit to control the controller's SRP capabilities. If the core operates as a non-SRP-capable B-device, it cannot request the connected A-device (host) to activate VBUS and start a session. - 1'b0: SRP capability is not enabled. - 1'b1: SRP capability is enabled. If SRP functionality is disabled by the software, the OTG signals on the PHY domain must be tied to the appropriate values."]
pub type SRPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPCAP` reader - Mode: Host and Device HNP-Capable (HNPCap) The application uses this bit to control the controller's HNP capabilities. - 1'b0: HNP capability is not enabled. - 1'b1: HNP capability is enabled. If HNP functionality is disabled by the software, the OTG signals on the PHY domain must be tied to the appropriate values."]
pub type HNPCAP_R = crate::BitReader;
#[doc = "Field `HNPCAP` writer - Mode: Host and Device HNP-Capable (HNPCap) The application uses this bit to control the controller's HNP capabilities. - 1'b0: HNP capability is not enabled. - 1'b1: HNP capability is enabled. If HNP functionality is disabled by the software, the OTG signals on the PHY domain must be tied to the appropriate values."]
pub type HNPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBTRDTIM` reader - Mode: Device only USB Turnaround Time (USBTrdTim) Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). This must be programmed to - 4'h5: When the MAC interface is 16-bit UTMI+ . - 4'h9: When the MAC interface is 8-bit UTMI+ . Note: The previous values are calculated for the minimum AHB frequency of 30 MHz. USB turnaround time is critical for certification where long cables and 5-Hubs are used. If you need the AHB to run at less than 30 MHz, and if USB turnaround time is not critical, these bits can be programmed to a larger value."]
pub type USBTRDTIM_R = crate::FieldReader;
#[doc = "Field `USBTRDTIM` writer - Mode: Device only USB Turnaround Time (USBTrdTim) Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). This must be programmed to - 4'h5: When the MAC interface is 16-bit UTMI+ . - 4'h9: When the MAC interface is 8-bit UTMI+ . Note: The previous values are calculated for the minimum AHB frequency of 30 MHz. USB turnaround time is critical for certification where long cables and 5-Hubs are used. If you need the AHB to run at less than 30 MHz, and if USB turnaround time is not critical, these bits can be programmed to a larger value."]
pub type USBTRDTIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TERMSELDLPULSE` reader - Mode: Device only TermSel DLine Pulsing Selection (TermSelDLPulse) This bit selects utmi_termselect to drive data line pulse during SRP. - 1'b0: Data line pulsing using utmi_txvalid (Default). - 1'b1: Data line pulsing using utmi_termsel."]
pub type TERMSELDLPULSE_R = crate::BitReader;
#[doc = "Field `IC_USBCAP` reader - Mode: Host and Device IC_USB-Capable (IC_USBCap) The application uses this bit to control the core's IC_USB capabilities. - 1'b0: IC_USB PHY Interface is not selected. - 1'b1: IC_USB PHY Interface is selected. This bit is writable only if OTG_ENABLE_IC_USB=1 and OTG_FSPHY_INTERFACE!=0. The reset value depends on the configuration parameter OTG_SELECT_IC_USB when OTG_ENABLE_IC_USB = 1. In all other cases, this bit is set to 1'b0 and the bit is read only."]
pub type IC_USBCAP_R = crate::BitReader;
#[doc = "Field `TXENDDELAY` reader - Mode: Device only Tx End Delay (TxEndDelay) Writing 1'b1 to this bit enables the controller to follow the TxEndDelay timings as per UTMI+ specification 1.05 section 4.1.5 for opmode signal during remote wakeup. - 1'b0 : Normal Mode. - 1'b1 : Tx End delay."]
pub type TXENDDELAY_R = crate::BitReader;
#[doc = "Field `TXENDDELAY` writer - Mode: Device only Tx End Delay (TxEndDelay) Writing 1'b1 to this bit enables the controller to follow the TxEndDelay timings as per UTMI+ specification 1.05 section 4.1.5 for opmode signal during remote wakeup. - 1'b0 : Normal Mode. - 1'b1 : Tx End delay."]
pub type TXENDDELAY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEHSTMODE` reader - Mode: Host and device Force Host Mode (ForceHstMode) Writing a 1 to this bit forces the core to host mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Host Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
pub type FORCEHSTMODE_R = crate::BitReader;
#[doc = "Field `FORCEHSTMODE` writer - Mode: Host and device Force Host Mode (ForceHstMode) Writing a 1 to this bit forces the core to host mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Host Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
pub type FORCEHSTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEDEVMODE` reader - Mode:Host and device Force Device Mode (ForceDevMode) Writing a 1 to this bit forces the controller to device mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Device Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
pub type FORCEDEVMODE_R = crate::BitReader;
#[doc = "Field `FORCEDEVMODE` writer - Mode:Host and device Force Device Mode (ForceDevMode) Writing a 1 to this bit forces the controller to device mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Device Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
pub type FORCEDEVMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORRUPTTXPKT` reader - Mode: Host and device Corrupt Tx packet (CorruptTxPkt) This bit is for debug purposes only. Never Set this bit to 1. The application should always write 1'b0 to this bit."]
pub type CORRUPTTXPKT_R = crate::BitReader;
#[doc = "Field `CORRUPTTXPKT` writer - Mode: Host and device Corrupt Tx packet (CorruptTxPkt) This bit is for debug purposes only. Never Set this bit to 1. The application should always write 1'b0 to this bit."]
pub type CORRUPTTXPKT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Mode: Host and Device HS/FS Timeout Calibration (TOutCal) The number of PHY clocks that the application programs in this field is added to the high-speed/full-speed interpacket timeout duration in the core to account for any additional delays introduced by the PHY. This can be required, because the delay introduced by the PHY in generating the linestate condition can vary from one PHY to another. The USB standard timeout value for high-speed operation is 736 to 816 (inclusive) bit times. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of enumeration. The number of bit times added per PHY clock are as follows: High-speed operation: - One 30-MHz PHY clock = 16 bit times - One 60-MHz PHY clock = 8 bit times Full-speed operation: - One 30-MHz PHY clock = 0.4 bit times - One 60-MHz PHY clock = 0.2 bit times - One 48-MHz PHY clock = 0.25 bit times"]
    #[inline(always)]
    pub fn toutcal(&self) -> TOUTCAL_R {
        TOUTCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Mode: Host and Device PHY Interface (PHYIf) The application uses this bit to configure the core to support a UTMI+ PHY with an 8- or 16-bit interface. When a ULPI PHY is chosen, this must be Set to 8-bit mode. - 1'b0: 8 bits - 1'b1: 16 bits This bit is writable only If UTMI+ and ULPI were selected. Otherwise, this bit returns the value for the power-on interface selected during configuration."]
    #[inline(always)]
    pub fn phyif(&self) -> PHYIF_R {
        PHYIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode: Host and Device Full-Speed Serial Interface Select (FSIntf) The application uses this bit to select either a unidirectional or bidirectional USB 1.1 full-speed serial transceiver interface. - 1'b0: 6-pin unidirectional full-speed serial interface - 1'b1: 3-pin bidirectional full-speed serial interface If a USB 1.1 Full-Speed Serial Transceiver interface was not selected, this bit is always 0, with Write Only access. If a USB 1.1 FS interface was selected, Then the application can Set this bit to select between the 3- and 6-pin interfaces, and access is Read and Write. Note: For supporting the new 4-pin bi-directional interface, you need to select 6-pin unidirectional FS serial mode, and add an external control to convert it to a 4-pin interface."]
    #[inline(always)]
    pub fn fsintf(&self) -> FSINTF_R {
        FSINTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PHYSel Mode: Host and Device USB 2.0 High-Speed PHY or USB 1.1 Full-Speed Serial Transceiver Select (PHYSel) The application uses this bit to select either a high-speed UTMI+ or ULPI PHY, or a full-speed transceiver. - 1'b0: USB 2.0 high-speed UTMI+ or ULPI PHY - 1'b1: USB 1.1 full-speed serial transceiver If a USB 1.1 Full-Speed Serial Transceiver interface was not selected in, this bit is always 0, with Write Only access. If a high-speed PHY interface was not selected in, this bit is always 1, with Write Only access. If both interface types were selected (parameters have non-zero values), the application uses this bit to select which interface is active, and access is Read and Write."]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Mode: Host and Device SRP-Capable (SRPCap) The application uses this bit to control the controller's SRP capabilities. If the core operates as a non-SRP-capable B-device, it cannot request the connected A-device (host) to activate VBUS and start a session. - 1'b0: SRP capability is not enabled. - 1'b1: SRP capability is enabled. If SRP functionality is disabled by the software, the OTG signals on the PHY domain must be tied to the appropriate values."]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mode: Host and Device HNP-Capable (HNPCap) The application uses this bit to control the controller's HNP capabilities. - 1'b0: HNP capability is not enabled. - 1'b1: HNP capability is enabled. If HNP functionality is disabled by the software, the OTG signals on the PHY domain must be tied to the appropriate values."]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Mode: Device only USB Turnaround Time (USBTrdTim) Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). This must be programmed to - 4'h5: When the MAC interface is 16-bit UTMI+ . - 4'h9: When the MAC interface is 8-bit UTMI+ . Note: The previous values are calculated for the minimum AHB frequency of 30 MHz. USB turnaround time is critical for certification where long cables and 5-Hubs are used. If you need the AHB to run at less than 30 MHz, and if USB turnaround time is not critical, these bits can be programmed to a larger value."]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Mode: Device only TermSel DLine Pulsing Selection (TermSelDLPulse) This bit selects utmi_termselect to drive data line pulse during SRP. - 1'b0: Data line pulsing using utmi_txvalid (Default). - 1'b1: Data line pulsing using utmi_termsel."]
    #[inline(always)]
    pub fn termseldlpulse(&self) -> TERMSELDLPULSE_R {
        TERMSELDLPULSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - Mode: Host and Device IC_USB-Capable (IC_USBCap) The application uses this bit to control the core's IC_USB capabilities. - 1'b0: IC_USB PHY Interface is not selected. - 1'b1: IC_USB PHY Interface is selected. This bit is writable only if OTG_ENABLE_IC_USB=1 and OTG_FSPHY_INTERFACE!=0. The reset value depends on the configuration parameter OTG_SELECT_IC_USB when OTG_ENABLE_IC_USB = 1. In all other cases, this bit is set to 1'b0 and the bit is read only."]
    #[inline(always)]
    pub fn ic_usbcap(&self) -> IC_USBCAP_R {
        IC_USBCAP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Mode: Device only Tx End Delay (TxEndDelay) Writing 1'b1 to this bit enables the controller to follow the TxEndDelay timings as per UTMI+ specification 1.05 section 4.1.5 for opmode signal during remote wakeup. - 1'b0 : Normal Mode. - 1'b1 : Tx End delay."]
    #[inline(always)]
    pub fn txenddelay(&self) -> TXENDDELAY_R {
        TXENDDELAY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Mode: Host and device Force Host Mode (ForceHstMode) Writing a 1 to this bit forces the core to host mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Host Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
    #[inline(always)]
    pub fn forcehstmode(&self) -> FORCEHSTMODE_R {
        FORCEHSTMODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Mode:Host and device Force Device Mode (ForceDevMode) Writing a 1 to this bit forces the controller to device mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Device Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
    #[inline(always)]
    pub fn forcedevmode(&self) -> FORCEDEVMODE_R {
        FORCEDEVMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mode: Host and device Corrupt Tx packet (CorruptTxPkt) This bit is for debug purposes only. Never Set this bit to 1. The application should always write 1'b0 to this bit."]
    #[inline(always)]
    pub fn corrupttxpkt(&self) -> CORRUPTTXPKT_R {
        CORRUPTTXPKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GUSBCFG")
            .field("toutcal", &self.toutcal())
            .field("phyif", &self.phyif())
            .field("fsintf", &self.fsintf())
            .field("physel", &self.physel())
            .field("srpcap", &self.srpcap())
            .field("hnpcap", &self.hnpcap())
            .field("usbtrdtim", &self.usbtrdtim())
            .field("termseldlpulse", &self.termseldlpulse())
            .field("ic_usbcap", &self.ic_usbcap())
            .field("txenddelay", &self.txenddelay())
            .field("forcehstmode", &self.forcehstmode())
            .field("forcedevmode", &self.forcedevmode())
            .field("corrupttxpkt", &self.corrupttxpkt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode: Host and Device HS/FS Timeout Calibration (TOutCal) The number of PHY clocks that the application programs in this field is added to the high-speed/full-speed interpacket timeout duration in the core to account for any additional delays introduced by the PHY. This can be required, because the delay introduced by the PHY in generating the linestate condition can vary from one PHY to another. The USB standard timeout value for high-speed operation is 736 to 816 (inclusive) bit times. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of enumeration. The number of bit times added per PHY clock are as follows: High-speed operation: - One 30-MHz PHY clock = 16 bit times - One 60-MHz PHY clock = 8 bit times Full-speed operation: - One 30-MHz PHY clock = 0.4 bit times - One 60-MHz PHY clock = 0.2 bit times - One 48-MHz PHY clock = 0.25 bit times"]
    #[inline(always)]
    pub fn toutcal(&mut self) -> TOUTCAL_W<'_, GUSBCFG_SPEC> {
        TOUTCAL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Mode: Host and Device PHY Interface (PHYIf) The application uses this bit to configure the core to support a UTMI+ PHY with an 8- or 16-bit interface. When a ULPI PHY is chosen, this must be Set to 8-bit mode. - 1'b0: 8 bits - 1'b1: 16 bits This bit is writable only If UTMI+ and ULPI were selected. Otherwise, this bit returns the value for the power-on interface selected during configuration."]
    #[inline(always)]
    pub fn phyif(&mut self) -> PHYIF_W<'_, GUSBCFG_SPEC> {
        PHYIF_W::new(self, 3)
    }
    #[doc = "Bit 5 - Mode: Host and Device Full-Speed Serial Interface Select (FSIntf) The application uses this bit to select either a unidirectional or bidirectional USB 1.1 full-speed serial transceiver interface. - 1'b0: 6-pin unidirectional full-speed serial interface - 1'b1: 3-pin bidirectional full-speed serial interface If a USB 1.1 Full-Speed Serial Transceiver interface was not selected, this bit is always 0, with Write Only access. If a USB 1.1 FS interface was selected, Then the application can Set this bit to select between the 3- and 6-pin interfaces, and access is Read and Write. Note: For supporting the new 4-pin bi-directional interface, you need to select 6-pin unidirectional FS serial mode, and add an external control to convert it to a 4-pin interface."]
    #[inline(always)]
    pub fn fsintf(&mut self) -> FSINTF_W<'_, GUSBCFG_SPEC> {
        FSINTF_W::new(self, 5)
    }
    #[doc = "Bit 8 - Mode: Host and Device SRP-Capable (SRPCap) The application uses this bit to control the controller's SRP capabilities. If the core operates as a non-SRP-capable B-device, it cannot request the connected A-device (host) to activate VBUS and start a session. - 1'b0: SRP capability is not enabled. - 1'b1: SRP capability is enabled. If SRP functionality is disabled by the software, the OTG signals on the PHY domain must be tied to the appropriate values."]
    #[inline(always)]
    pub fn srpcap(&mut self) -> SRPCAP_W<'_, GUSBCFG_SPEC> {
        SRPCAP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Mode: Host and Device HNP-Capable (HNPCap) The application uses this bit to control the controller's HNP capabilities. - 1'b0: HNP capability is not enabled. - 1'b1: HNP capability is enabled. If HNP functionality is disabled by the software, the OTG signals on the PHY domain must be tied to the appropriate values."]
    #[inline(always)]
    pub fn hnpcap(&mut self) -> HNPCAP_W<'_, GUSBCFG_SPEC> {
        HNPCAP_W::new(self, 9)
    }
    #[doc = "Bits 10:13 - Mode: Device only USB Turnaround Time (USBTrdTim) Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). This must be programmed to - 4'h5: When the MAC interface is 16-bit UTMI+ . - 4'h9: When the MAC interface is 8-bit UTMI+ . Note: The previous values are calculated for the minimum AHB frequency of 30 MHz. USB turnaround time is critical for certification where long cables and 5-Hubs are used. If you need the AHB to run at less than 30 MHz, and if USB turnaround time is not critical, these bits can be programmed to a larger value."]
    #[inline(always)]
    pub fn usbtrdtim(&mut self) -> USBTRDTIM_W<'_, GUSBCFG_SPEC> {
        USBTRDTIM_W::new(self, 10)
    }
    #[doc = "Bit 28 - Mode: Device only Tx End Delay (TxEndDelay) Writing 1'b1 to this bit enables the controller to follow the TxEndDelay timings as per UTMI+ specification 1.05 section 4.1.5 for opmode signal during remote wakeup. - 1'b0 : Normal Mode. - 1'b1 : Tx End delay."]
    #[inline(always)]
    pub fn txenddelay(&mut self) -> TXENDDELAY_W<'_, GUSBCFG_SPEC> {
        TXENDDELAY_W::new(self, 28)
    }
    #[doc = "Bit 29 - Mode: Host and device Force Host Mode (ForceHstMode) Writing a 1 to this bit forces the core to host mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Host Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
    #[inline(always)]
    pub fn forcehstmode(&mut self) -> FORCEHSTMODE_W<'_, GUSBCFG_SPEC> {
        FORCEHSTMODE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Mode:Host and device Force Device Mode (ForceDevMode) Writing a 1 to this bit forces the controller to device mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Device Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
    #[inline(always)]
    pub fn forcedevmode(&mut self) -> FORCEDEVMODE_W<'_, GUSBCFG_SPEC> {
        FORCEDEVMODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Mode: Host and device Corrupt Tx packet (CorruptTxPkt) This bit is for debug purposes only. Never Set this bit to 1. The application should always write 1'b0 to this bit."]
    #[inline(always)]
    pub fn corrupttxpkt(&mut self) -> CORRUPTTXPKT_W<'_, GUSBCFG_SPEC> {
        CORRUPTTXPKT_W::new(self, 31)
    }
}
#[doc = "USB Configuration Register This register can be used to configure the core after power-on or a changing to Host mode or Device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1440"]
impl crate::Resettable for GUSBCFG_SPEC {
    const RESET_VALUE: u32 = 0x1440;
}
