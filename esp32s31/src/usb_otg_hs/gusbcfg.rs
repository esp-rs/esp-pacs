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
#[doc = "Field `ULPI_UTMI_SEL` reader - Mode: Host and Device ULPI or UTMI+ Select (ULPI_UTMI_Sel) The application uses this bit to select either a UTMI+ interface or ULPI Interface. - 1'b0: UTMI+ Interface - 1'b1: ULPI Interface"]
pub type ULPI_UTMI_SEL_R = crate::BitReader;
#[doc = "Field `ULPI_UTMI_SEL` writer - Mode: Host and Device ULPI or UTMI+ Select (ULPI_UTMI_Sel) The application uses this bit to select either a UTMI+ interface or ULPI Interface. - 1'b0: UTMI+ Interface - 1'b1: ULPI Interface"]
pub type ULPI_UTMI_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSINTF` reader - Mode: Host and Device Full-Speed Serial Interface Select (FSIntf) The application uses this bit to select either a unidirectional or bidirectional USB 1.1 full-speed serial transceiver interface. - 1'b0: 6-pin unidirectional full-speed serial interface - 1'b1: 3-pin bidirectional full-speed serial interface If a USB 1.1 Full-Speed Serial Transceiver interface was not selected, this bit is always 0, with Write Only access. If a USB 1.1 FS interface was selected, Then the application can Set this bit to select between the 3- and 6-pin interfaces, and access is Read and Write. Note: For supporting the new 4-pin bi-directional interface, you need to select 6-pin unidirectional FS serial mode, and add an external control to convert it to a 4-pin interface."]
pub type FSINTF_R = crate::BitReader;
#[doc = "Field `FSINTF` writer - Mode: Host and Device Full-Speed Serial Interface Select (FSIntf) The application uses this bit to select either a unidirectional or bidirectional USB 1.1 full-speed serial transceiver interface. - 1'b0: 6-pin unidirectional full-speed serial interface - 1'b1: 3-pin bidirectional full-speed serial interface If a USB 1.1 Full-Speed Serial Transceiver interface was not selected, this bit is always 0, with Write Only access. If a USB 1.1 FS interface was selected, Then the application can Set this bit to select between the 3- and 6-pin interfaces, and access is Read and Write. Note: For supporting the new 4-pin bi-directional interface, you need to select 6-pin unidirectional FS serial mode, and add an external control to convert it to a 4-pin interface."]
pub type FSINTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSEL` reader - PHYSel Mode: Host and Device USB 2.0 High-Speed PHY or USB 1.1 Full-Speed Serial Transceiver Select (PHYSel) The application uses this bit to select either a high-speed UTMI+ or ULPI PHY, or a full-speed transceiver. - 1'b0: USB 2.0 high-speed UTMI+ or ULPI PHY - 1'b1: USB 1.1 full-speed serial transceiver If a USB 1.1 Full-Speed Serial Transceiver interface was not selected in, this bit is always 0, with Write Only access. If a high-speed PHY interface was not selected in, this bit is always 1, with Write Only access. If both interface types were selected (parameters have non-zero values), the application uses this bit to select which interface is active, and access is Read and Write."]
pub type PHYSEL_R = crate::BitReader;
#[doc = "Field `PHYSEL` writer - PHYSel Mode: Host and Device USB 2.0 High-Speed PHY or USB 1.1 Full-Speed Serial Transceiver Select (PHYSel) The application uses this bit to select either a high-speed UTMI+ or ULPI PHY, or a full-speed transceiver. - 1'b0: USB 2.0 high-speed UTMI+ or ULPI PHY - 1'b1: USB 1.1 full-speed serial transceiver If a USB 1.1 Full-Speed Serial Transceiver interface was not selected in, this bit is always 0, with Write Only access. If a high-speed PHY interface was not selected in, this bit is always 1, with Write Only access. If both interface types were selected (parameters have non-zero values), the application uses this bit to select which interface is active, and access is Read and Write."]
pub type PHYSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRSEL` reader - Mode: Host and Device ULPI DDR Select (DDRSel) The application uses this bit to select a Single Data Rate (SDR) or Double Data Rate (DDR) or ULPI interface. - 1'b0: Single Data Rate ULPI Interface, with 8-bit-wide data bus - 1'b1: Double Data Rate ULPI Interface, with 4-bit-wide data bus"]
pub type DDRSEL_R = crate::BitReader;
#[doc = "Field `DDRSEL` writer - Mode: Host and Device ULPI DDR Select (DDRSel) The application uses this bit to select a Single Data Rate (SDR) or Double Data Rate (DDR) or ULPI interface. - 1'b0: Single Data Rate ULPI Interface, with 8-bit-wide data bus - 1'b1: Double Data Rate ULPI Interface, with 4-bit-wide data bus"]
pub type DDRSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBTRDTIM` reader - Mode: Device only USB Turnaround Time (USBTrdTim) Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). This must be programmed to - 4'h5 : When the MAC interface is 16-bit UTMI+ - 4'h9 : When the MAC interface is 8-bit UTMI+ Note:Refer to Programming Guide Section Choosing the Value of GUSBCFG.USBTrdTim for the turnaround time calculation. During the AHB Clock Frequency selection, it is recommended to consider mis-sampling into account for USBTrdTim. USB turnaround time is critical for certification where long cables and 5-Hubs are used. If you need the AHB to run at less than 30 MHz (or less than 42MHz when operating in 16-bit UTMI mode in HS speed), and if USB turnaround time is not critical, these bits can be programmed to a larger value."]
pub type USBTRDTIM_R = crate::FieldReader;
#[doc = "Field `USBTRDTIM` writer - Mode: Device only USB Turnaround Time (USBTrdTim) Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). This must be programmed to - 4'h5 : When the MAC interface is 16-bit UTMI+ - 4'h9 : When the MAC interface is 8-bit UTMI+ Note:Refer to Programming Guide Section Choosing the Value of GUSBCFG.USBTrdTim for the turnaround time calculation. During the AHB Clock Frequency selection, it is recommended to consider mis-sampling into account for USBTrdTim. USB turnaround time is critical for certification where long cables and 5-Hubs are used. If you need the AHB to run at less than 30 MHz (or less than 42MHz when operating in 16-bit UTMI mode in HS speed), and if USB turnaround time is not critical, these bits can be programmed to a larger value."]
pub type USBTRDTIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHYLPWRCLKSEL` reader - PHY Low-Power Clock Select (PhyLPwrClkSel) Mode: Host and Device Selects either 480-MHz or 48-MHz (low-power) PHY mode. In FS and LS modes, the PHY can usually operate on a 48-MHz clock to save power. - 1'b0: 480-MHz Internal PLL clock - 1'b1: 48-MHz External Clock In 480 MHz mode, the UTMI interface operates at either 60 or 30-MHz, depending upon whether 8- or 16-bit data width is selected. In 48-MHz mode, the UTMI interface operates at 48 MHz in FS mode and at either 48 or 6 MHz in LS mode (depending on the PHY vendor). This bit drives the utmi_fsls_low_power core output signal, and is valid only for UTMI+ PHYs."]
pub type PHYLPWRCLKSEL_R = crate::BitReader;
#[doc = "Field `PHYLPWRCLKSEL` writer - PHY Low-Power Clock Select (PhyLPwrClkSel) Mode: Host and Device Selects either 480-MHz or 48-MHz (low-power) PHY mode. In FS and LS modes, the PHY can usually operate on a 48-MHz clock to save power. - 1'b0: 480-MHz Internal PLL clock - 1'b1: 48-MHz External Clock In 480 MHz mode, the UTMI interface operates at either 60 or 30-MHz, depending upon whether 8- or 16-bit data width is selected. In 48-MHz mode, the UTMI interface operates at 48 MHz in FS mode and at either 48 or 6 MHz in LS mode (depending on the PHY vendor). This bit drives the utmi_fsls_low_power core output signal, and is valid only for UTMI+ PHYs."]
pub type PHYLPWRCLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIFSLS` reader - Mode: Host and Device ULPI FS/LS Select (ULPIFsLs) The application uses this bit to select the FS/LS serial interface for the ULPI PHY. This bit is valid only when the FS serial transceiver is selected on the ULPI PHY. - 1'b0: ULPI interface - 1'b1: ULPI FS/LS serial interface Note: Before setting this bit, the application needs to ensure that GUSBCFG.ULPI_UTMI_SEL = 1'b1."]
pub type ULPIFSLS_R = crate::BitReader;
#[doc = "Field `ULPIFSLS` writer - Mode: Host and Device ULPI FS/LS Select (ULPIFsLs) The application uses this bit to select the FS/LS serial interface for the ULPI PHY. This bit is valid only when the FS serial transceiver is selected on the ULPI PHY. - 1'b0: ULPI interface - 1'b1: ULPI FS/LS serial interface Note: Before setting this bit, the application needs to ensure that GUSBCFG.ULPI_UTMI_SEL = 1'b1."]
pub type ULPIFSLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIAUTORES` reader - Mode: Host and Device ULPI Auto Resume (ULPIAutoRes) This bit sets the AutoResume bit in the Interface Control register on the ULPI PHY. - 1'b0: PHY does not use AutoResume feature. - 1'b1: PHY uses AutoResume feature."]
pub type ULPIAUTORES_R = crate::BitReader;
#[doc = "Field `ULPIAUTORES` writer - Mode: Host and Device ULPI Auto Resume (ULPIAutoRes) This bit sets the AutoResume bit in the Interface Control register on the ULPI PHY. - 1'b0: PHY does not use AutoResume feature. - 1'b1: PHY uses AutoResume feature."]
pub type ULPIAUTORES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPICLKSUSM` reader - Mode: Host and Device ULPI Clock SuspendM (ULPIClkSusM) This bit sets the ClockSuspendM bit in the Interface Control register on the ULPI PHY. This bit applies only in serial or carkit modes. - 1'b0: PHY powers down internal clock during suspend. - 1'b1: PHY does not power down internal clock."]
pub type ULPICLKSUSM_R = crate::BitReader;
#[doc = "Field `ULPICLKSUSM` writer - Mode: Host and Device ULPI Clock SuspendM (ULPIClkSusM) This bit sets the ClockSuspendM bit in the Interface Control register on the ULPI PHY. This bit applies only in serial or carkit modes. - 1'b0: PHY powers down internal clock during suspend. - 1'b1: PHY does not power down internal clock."]
pub type ULPICLKSUSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIEXTVBUSDRV` reader - Mode: Host only ULPI External VBUS Drive (ULPIExtVbusDrv) This bit selects between internal or external supply to drive 5V on VBUS, in ULPI PHY. - 1'b0: PHY drives VBUS using internal charge pump (Default). - 1'b1: PHY drives VBUS using external supply."]
pub type ULPIEXTVBUSDRV_R = crate::BitReader;
#[doc = "Field `ULPIEXTVBUSDRV` writer - Mode: Host only ULPI External VBUS Drive (ULPIExtVbusDrv) This bit selects between internal or external supply to drive 5V on VBUS, in ULPI PHY. - 1'b0: PHY drives VBUS using internal charge pump (Default). - 1'b1: PHY drives VBUS using external supply."]
pub type ULPIEXTVBUSDRV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPIEXTVBUSINDICATOR` reader - Mode: Host only ULPI External VBUS Indicator (ULPIExtVbusIndicator) This bit indicates to the ULPI PHY to use an external VBUS overcurrent indicator. - 1'b0: PHY uses internal VBUS valid comparator. - 1'b1: PHY uses external VBUS valid comparator."]
pub type ULPIEXTVBUSINDICATOR_R = crate::BitReader;
#[doc = "Field `ULPIEXTVBUSINDICATOR` writer - Mode: Host only ULPI External VBUS Indicator (ULPIExtVbusIndicator) This bit indicates to the ULPI PHY to use an external VBUS overcurrent indicator. - 1'b0: PHY uses internal VBUS valid comparator. - 1'b1: PHY uses external VBUS valid comparator."]
pub type ULPIEXTVBUSINDICATOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERMSELDLPULSE` reader - Mode: Device only TermSel DLine Pulsing Selection (TermSelDLPulse) This bit selects utmi_termselect to drive data line pulse during SRP. - 1'b0: Data line pulsing using utmi_txvalid (Default). - 1'b1: Data line pulsing using utmi_termsel."]
pub type TERMSELDLPULSE_R = crate::BitReader;
#[doc = "Field `TERMSELDLPULSE` writer - Mode: Device only TermSel DLine Pulsing Selection (TermSelDLPulse) This bit selects utmi_termselect to drive data line pulse during SRP. - 1'b0: Data line pulsing using utmi_txvalid (Default). - 1'b1: Data line pulsing using utmi_termsel."]
pub type TERMSELDLPULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPLEMENT` reader - Mode: Host only Indicator Complement Controls the PHY to invert the ExternalVbusIndicator input signal, generating the Complement Output. For more information, refer to the ULPI Specification. - 1'b0: PHY does not invert ExternalVbusIndicator signal - 1'b1: PHY does invert ExternalVbusIndicator signal This bit is reserved and read-only when OTG_HSPHY_INTERFACE is set to 0 or 1."]
pub type COMPLEMENT_R = crate::BitReader;
#[doc = "Field `COMPLEMENT` writer - Mode: Host only Indicator Complement Controls the PHY to invert the ExternalVbusIndicator input signal, generating the Complement Output. For more information, refer to the ULPI Specification. - 1'b0: PHY does not invert ExternalVbusIndicator signal - 1'b1: PHY does invert ExternalVbusIndicator signal This bit is reserved and read-only when OTG_HSPHY_INTERFACE is set to 0 or 1."]
pub type COMPLEMENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDICATOR` reader - Mode: Host only Indicator Pass Through Controls wether the Complement Output is qualified with the Internal Vbus Valid comparator before being used in the Vbus State in the RX CMD. For more information, refer to the ULPI Specification. - 1'b0: Complement Output signal is qualified with the Internal VbusValid comparator. - 1'b1: Complement Output signal is not qualified with the Internal VbusValid comparator. This bit is reserved and read-only when OTG_HSPHY_INTERFACE is set to 0 or 1."]
pub type INDICATOR_R = crate::BitReader;
#[doc = "Field `INDICATOR` writer - Mode: Host only Indicator Pass Through Controls wether the Complement Output is qualified with the Internal Vbus Valid comparator before being used in the Vbus State in the RX CMD. For more information, refer to the ULPI Specification. - 1'b0: Complement Output signal is qualified with the Internal VbusValid comparator. - 1'b1: Complement Output signal is not qualified with the Internal VbusValid comparator. This bit is reserved and read-only when OTG_HSPHY_INTERFACE is set to 0 or 1."]
pub type INDICATOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPI` reader - Mode: Host only ULPI Interface Protect Disable Controls circuitry built into the PHY for protecting the ULPI interface when the link tri-states STP and data. Any pull-ups or pull-downs employed by this feature can be disabled. For more information, refer to the ULPI Specification. - 1'b0: Enables the interface protect circuit - 1'b1: Disables the interface protect circuit"]
pub type ULPI_R = crate::BitReader;
#[doc = "Field `ULPI` writer - Mode: Host only ULPI Interface Protect Disable Controls circuitry built into the PHY for protecting the ULPI interface when the link tri-states STP and data. Any pull-ups or pull-downs employed by this feature can be disabled. For more information, refer to the ULPI Specification. - 1'b0: Enables the interface protect circuit - 1'b1: Disables the interface protect circuit"]
pub type ULPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_USBCAP` reader - Mode: Host and Device IC_USB-Capable (IC_USBCap) The application uses this bit to control the core's IC_USB capabilities. - 1'b0: IC_USB PHY Interface is not selected. - 1'b1: IC_USB PHY Interface is selected. This bit is writable only if OTG_ENABLE_IC_USB=1 and OTG_FSPHY_INTERFACE!=0. The reset value depends on the configuration parameter OTG_SELECT_IC_USB when OTG_ENABLE_IC_USB = 1. In all other cases, this bit is set to 1'b0 and the bit is read only."]
pub type IC_USBCAP_R = crate::BitReader;
#[doc = "Field `TXENDDELAY` reader - Mode: Device only Tx End Delay (TxEndDelay) Setting this bit to 1'b1 enables the controller to follow the TxEndDelay timings as per UTMI+ specification 1.05 section 4.1.5 for opmode signal during remote wakeup. - 1'b0 : Normal Mode. - 1'b1 : Tx End delay. The default value of this field is 1'b1 and it is not recommended to program it to 1'b0. The option to set it to 1'b0 (Normal Mode) is present only for debug purpose."]
pub type TXENDDELAY_R = crate::BitReader;
#[doc = "Field `TXENDDELAY` writer - Mode: Device only Tx End Delay (TxEndDelay) Setting this bit to 1'b1 enables the controller to follow the TxEndDelay timings as per UTMI+ specification 1.05 section 4.1.5 for opmode signal during remote wakeup. - 1'b0 : Normal Mode. - 1'b1 : Tx End delay. The default value of this field is 1'b1 and it is not recommended to program it to 1'b0. The option to set it to 1'b0 (Normal Mode) is present only for debug purpose."]
pub type TXENDDELAY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEHSTMODE` reader - Mode: Host and device Force Host Mode (ForceHstMode) Writing a 1 to this bit forces the core to host mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Host Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
pub type FORCEHSTMODE_R = crate::BitReader;
#[doc = "Field `FORCEHSTMODE` writer - Mode: Host and device Force Host Mode (ForceHstMode) Writing a 1 to this bit forces the core to host mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Host Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
pub type FORCEHSTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCEDEVMODE` reader - Mode:Host and device Force Device Mode (ForceDevMode) Writing a 1 to this bit forces the controller to device mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Device Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
pub type FORCEDEVMODE_R = crate::BitReader;
#[doc = "Field `FORCEDEVMODE` writer - Mode:Host and device Force Device Mode (ForceDevMode) Writing a 1 to this bit forces the controller to device mode irrespective of utmiotg_iddig input pin. - 1'b0 : Normal Mode. - 1'b1 : Force Device Mode. After setting the force bit, the application must wait at least 25 ms before the change to take effect. When the simulation is in scale down mode, waiting for 500 micro sec is sufficient. This bit is valid only when OTG_MODE = 0, 1 or 2. In all other cases, this bit reads 0."]
pub type FORCEDEVMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORRUPTTXPKT` reader - Mode: Host and device Corrupt Tx packet (CorruptTxPkt) This bit is for debug purposes only. Never Set this bit to 1. The application must always write 1'b0 to this bit."]
pub type CORRUPTTXPKT_R = crate::BitReader;
#[doc = "Field `CORRUPTTXPKT` writer - Mode: Host and device Corrupt Tx packet (CorruptTxPkt) This bit is for debug purposes only. Never Set this bit to 1. The application must always write 1'b0 to this bit."]
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
    #[doc = "Bit 4 - Mode: Host and Device ULPI or UTMI+ Select (ULPI_UTMI_Sel) The application uses this bit to select either a UTMI+ interface or ULPI Interface. - 1'b0: UTMI+ Interface - 1'b1: ULPI Interface"]
    #[inline(always)]
    pub fn ulpi_utmi_sel(&self) -> ULPI_UTMI_SEL_R {
        ULPI_UTMI_SEL_R::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bit 7 - Mode: Host and Device ULPI DDR Select (DDRSel) The application uses this bit to select a Single Data Rate (SDR) or Double Data Rate (DDR) or ULPI interface. - 1'b0: Single Data Rate ULPI Interface, with 8-bit-wide data bus - 1'b1: Double Data Rate ULPI Interface, with 4-bit-wide data bus"]
    #[inline(always)]
    pub fn ddrsel(&self) -> DDRSEL_R {
        DDRSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Mode: Device only USB Turnaround Time (USBTrdTim) Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). This must be programmed to - 4'h5 : When the MAC interface is 16-bit UTMI+ - 4'h9 : When the MAC interface is 8-bit UTMI+ Note:Refer to Programming Guide Section Choosing the Value of GUSBCFG.USBTrdTim for the turnaround time calculation. During the AHB Clock Frequency selection, it is recommended to consider mis-sampling into account for USBTrdTim. USB turnaround time is critical for certification where long cables and 5-Hubs are used. If you need the AHB to run at less than 30 MHz (or less than 42MHz when operating in 16-bit UTMI mode in HS speed), and if USB turnaround time is not critical, these bits can be programmed to a larger value."]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PHY Low-Power Clock Select (PhyLPwrClkSel) Mode: Host and Device Selects either 480-MHz or 48-MHz (low-power) PHY mode. In FS and LS modes, the PHY can usually operate on a 48-MHz clock to save power. - 1'b0: 480-MHz Internal PLL clock - 1'b1: 48-MHz External Clock In 480 MHz mode, the UTMI interface operates at either 60 or 30-MHz, depending upon whether 8- or 16-bit data width is selected. In 48-MHz mode, the UTMI interface operates at 48 MHz in FS mode and at either 48 or 6 MHz in LS mode (depending on the PHY vendor). This bit drives the utmi_fsls_low_power core output signal, and is valid only for UTMI+ PHYs."]
    #[inline(always)]
    pub fn phylpwrclksel(&self) -> PHYLPWRCLKSEL_R {
        PHYLPWRCLKSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Mode: Host and Device ULPI FS/LS Select (ULPIFsLs) The application uses this bit to select the FS/LS serial interface for the ULPI PHY. This bit is valid only when the FS serial transceiver is selected on the ULPI PHY. - 1'b0: ULPI interface - 1'b1: ULPI FS/LS serial interface Note: Before setting this bit, the application needs to ensure that GUSBCFG.ULPI_UTMI_SEL = 1'b1."]
    #[inline(always)]
    pub fn ulpifsls(&self) -> ULPIFSLS_R {
        ULPIFSLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mode: Host and Device ULPI Auto Resume (ULPIAutoRes) This bit sets the AutoResume bit in the Interface Control register on the ULPI PHY. - 1'b0: PHY does not use AutoResume feature. - 1'b1: PHY uses AutoResume feature."]
    #[inline(always)]
    pub fn ulpiautores(&self) -> ULPIAUTORES_R {
        ULPIAUTORES_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mode: Host and Device ULPI Clock SuspendM (ULPIClkSusM) This bit sets the ClockSuspendM bit in the Interface Control register on the ULPI PHY. This bit applies only in serial or carkit modes. - 1'b0: PHY powers down internal clock during suspend. - 1'b1: PHY does not power down internal clock."]
    #[inline(always)]
    pub fn ulpiclksusm(&self) -> ULPICLKSUSM_R {
        ULPICLKSUSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Mode: Host only ULPI External VBUS Drive (ULPIExtVbusDrv) This bit selects between internal or external supply to drive 5V on VBUS, in ULPI PHY. - 1'b0: PHY drives VBUS using internal charge pump (Default). - 1'b1: PHY drives VBUS using external supply."]
    #[inline(always)]
    pub fn ulpiextvbusdrv(&self) -> ULPIEXTVBUSDRV_R {
        ULPIEXTVBUSDRV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Mode: Host only ULPI External VBUS Indicator (ULPIExtVbusIndicator) This bit indicates to the ULPI PHY to use an external VBUS overcurrent indicator. - 1'b0: PHY uses internal VBUS valid comparator. - 1'b1: PHY uses external VBUS valid comparator."]
    #[inline(always)]
    pub fn ulpiextvbusindicator(&self) -> ULPIEXTVBUSINDICATOR_R {
        ULPIEXTVBUSINDICATOR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mode: Device only TermSel DLine Pulsing Selection (TermSelDLPulse) This bit selects utmi_termselect to drive data line pulse during SRP. - 1'b0: Data line pulsing using utmi_txvalid (Default). - 1'b1: Data line pulsing using utmi_termsel."]
    #[inline(always)]
    pub fn termseldlpulse(&self) -> TERMSELDLPULSE_R {
        TERMSELDLPULSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mode: Host only Indicator Complement Controls the PHY to invert the ExternalVbusIndicator input signal, generating the Complement Output. For more information, refer to the ULPI Specification. - 1'b0: PHY does not invert ExternalVbusIndicator signal - 1'b1: PHY does invert ExternalVbusIndicator signal This bit is reserved and read-only when OTG_HSPHY_INTERFACE is set to 0 or 1."]
    #[inline(always)]
    pub fn complement(&self) -> COMPLEMENT_R {
        COMPLEMENT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mode: Host only Indicator Pass Through Controls wether the Complement Output is qualified with the Internal Vbus Valid comparator before being used in the Vbus State in the RX CMD. For more information, refer to the ULPI Specification. - 1'b0: Complement Output signal is qualified with the Internal VbusValid comparator. - 1'b1: Complement Output signal is not qualified with the Internal VbusValid comparator. This bit is reserved and read-only when OTG_HSPHY_INTERFACE is set to 0 or 1."]
    #[inline(always)]
    pub fn indicator(&self) -> INDICATOR_R {
        INDICATOR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Mode: Host only ULPI Interface Protect Disable Controls circuitry built into the PHY for protecting the ULPI interface when the link tri-states STP and data. Any pull-ups or pull-downs employed by this feature can be disabled. For more information, refer to the ULPI Specification. - 1'b0: Enables the interface protect circuit - 1'b1: Disables the interface protect circuit"]
    #[inline(always)]
    pub fn ulpi(&self) -> ULPI_R {
        ULPI_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mode: Host and Device IC_USB-Capable (IC_USBCap) The application uses this bit to control the core's IC_USB capabilities. - 1'b0: IC_USB PHY Interface is not selected. - 1'b1: IC_USB PHY Interface is selected. This bit is writable only if OTG_ENABLE_IC_USB=1 and OTG_FSPHY_INTERFACE!=0. The reset value depends on the configuration parameter OTG_SELECT_IC_USB when OTG_ENABLE_IC_USB = 1. In all other cases, this bit is set to 1'b0 and the bit is read only."]
    #[inline(always)]
    pub fn ic_usbcap(&self) -> IC_USBCAP_R {
        IC_USBCAP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Mode: Device only Tx End Delay (TxEndDelay) Setting this bit to 1'b1 enables the controller to follow the TxEndDelay timings as per UTMI+ specification 1.05 section 4.1.5 for opmode signal during remote wakeup. - 1'b0 : Normal Mode. - 1'b1 : Tx End delay. The default value of this field is 1'b1 and it is not recommended to program it to 1'b0. The option to set it to 1'b0 (Normal Mode) is present only for debug purpose."]
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
    #[doc = "Bit 31 - Mode: Host and device Corrupt Tx packet (CorruptTxPkt) This bit is for debug purposes only. Never Set this bit to 1. The application must always write 1'b0 to this bit."]
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
            .field("ulpi_utmi_sel", &self.ulpi_utmi_sel())
            .field("fsintf", &self.fsintf())
            .field("physel", &self.physel())
            .field("ddrsel", &self.ddrsel())
            .field("usbtrdtim", &self.usbtrdtim())
            .field("phylpwrclksel", &self.phylpwrclksel())
            .field("ulpifsls", &self.ulpifsls())
            .field("ulpiautores", &self.ulpiautores())
            .field("ulpiclksusm", &self.ulpiclksusm())
            .field("ulpiextvbusdrv", &self.ulpiextvbusdrv())
            .field("ulpiextvbusindicator", &self.ulpiextvbusindicator())
            .field("termseldlpulse", &self.termseldlpulse())
            .field("complement", &self.complement())
            .field("indicator", &self.indicator())
            .field("ulpi", &self.ulpi())
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
    #[doc = "Bit 4 - Mode: Host and Device ULPI or UTMI+ Select (ULPI_UTMI_Sel) The application uses this bit to select either a UTMI+ interface or ULPI Interface. - 1'b0: UTMI+ Interface - 1'b1: ULPI Interface"]
    #[inline(always)]
    pub fn ulpi_utmi_sel(&mut self) -> ULPI_UTMI_SEL_W<'_, GUSBCFG_SPEC> {
        ULPI_UTMI_SEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Mode: Host and Device Full-Speed Serial Interface Select (FSIntf) The application uses this bit to select either a unidirectional or bidirectional USB 1.1 full-speed serial transceiver interface. - 1'b0: 6-pin unidirectional full-speed serial interface - 1'b1: 3-pin bidirectional full-speed serial interface If a USB 1.1 Full-Speed Serial Transceiver interface was not selected, this bit is always 0, with Write Only access. If a USB 1.1 FS interface was selected, Then the application can Set this bit to select between the 3- and 6-pin interfaces, and access is Read and Write. Note: For supporting the new 4-pin bi-directional interface, you need to select 6-pin unidirectional FS serial mode, and add an external control to convert it to a 4-pin interface."]
    #[inline(always)]
    pub fn fsintf(&mut self) -> FSINTF_W<'_, GUSBCFG_SPEC> {
        FSINTF_W::new(self, 5)
    }
    #[doc = "Bit 6 - PHYSel Mode: Host and Device USB 2.0 High-Speed PHY or USB 1.1 Full-Speed Serial Transceiver Select (PHYSel) The application uses this bit to select either a high-speed UTMI+ or ULPI PHY, or a full-speed transceiver. - 1'b0: USB 2.0 high-speed UTMI+ or ULPI PHY - 1'b1: USB 1.1 full-speed serial transceiver If a USB 1.1 Full-Speed Serial Transceiver interface was not selected in, this bit is always 0, with Write Only access. If a high-speed PHY interface was not selected in, this bit is always 1, with Write Only access. If both interface types were selected (parameters have non-zero values), the application uses this bit to select which interface is active, and access is Read and Write."]
    #[inline(always)]
    pub fn physel(&mut self) -> PHYSEL_W<'_, GUSBCFG_SPEC> {
        PHYSEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Mode: Host and Device ULPI DDR Select (DDRSel) The application uses this bit to select a Single Data Rate (SDR) or Double Data Rate (DDR) or ULPI interface. - 1'b0: Single Data Rate ULPI Interface, with 8-bit-wide data bus - 1'b1: Double Data Rate ULPI Interface, with 4-bit-wide data bus"]
    #[inline(always)]
    pub fn ddrsel(&mut self) -> DDRSEL_W<'_, GUSBCFG_SPEC> {
        DDRSEL_W::new(self, 7)
    }
    #[doc = "Bits 10:13 - Mode: Device only USB Turnaround Time (USBTrdTim) Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). This must be programmed to - 4'h5 : When the MAC interface is 16-bit UTMI+ - 4'h9 : When the MAC interface is 8-bit UTMI+ Note:Refer to Programming Guide Section Choosing the Value of GUSBCFG.USBTrdTim for the turnaround time calculation. During the AHB Clock Frequency selection, it is recommended to consider mis-sampling into account for USBTrdTim. USB turnaround time is critical for certification where long cables and 5-Hubs are used. If you need the AHB to run at less than 30 MHz (or less than 42MHz when operating in 16-bit UTMI mode in HS speed), and if USB turnaround time is not critical, these bits can be programmed to a larger value."]
    #[inline(always)]
    pub fn usbtrdtim(&mut self) -> USBTRDTIM_W<'_, GUSBCFG_SPEC> {
        USBTRDTIM_W::new(self, 10)
    }
    #[doc = "Bit 15 - PHY Low-Power Clock Select (PhyLPwrClkSel) Mode: Host and Device Selects either 480-MHz or 48-MHz (low-power) PHY mode. In FS and LS modes, the PHY can usually operate on a 48-MHz clock to save power. - 1'b0: 480-MHz Internal PLL clock - 1'b1: 48-MHz External Clock In 480 MHz mode, the UTMI interface operates at either 60 or 30-MHz, depending upon whether 8- or 16-bit data width is selected. In 48-MHz mode, the UTMI interface operates at 48 MHz in FS mode and at either 48 or 6 MHz in LS mode (depending on the PHY vendor). This bit drives the utmi_fsls_low_power core output signal, and is valid only for UTMI+ PHYs."]
    #[inline(always)]
    pub fn phylpwrclksel(&mut self) -> PHYLPWRCLKSEL_W<'_, GUSBCFG_SPEC> {
        PHYLPWRCLKSEL_W::new(self, 15)
    }
    #[doc = "Bit 17 - Mode: Host and Device ULPI FS/LS Select (ULPIFsLs) The application uses this bit to select the FS/LS serial interface for the ULPI PHY. This bit is valid only when the FS serial transceiver is selected on the ULPI PHY. - 1'b0: ULPI interface - 1'b1: ULPI FS/LS serial interface Note: Before setting this bit, the application needs to ensure that GUSBCFG.ULPI_UTMI_SEL = 1'b1."]
    #[inline(always)]
    pub fn ulpifsls(&mut self) -> ULPIFSLS_W<'_, GUSBCFG_SPEC> {
        ULPIFSLS_W::new(self, 17)
    }
    #[doc = "Bit 18 - Mode: Host and Device ULPI Auto Resume (ULPIAutoRes) This bit sets the AutoResume bit in the Interface Control register on the ULPI PHY. - 1'b0: PHY does not use AutoResume feature. - 1'b1: PHY uses AutoResume feature."]
    #[inline(always)]
    pub fn ulpiautores(&mut self) -> ULPIAUTORES_W<'_, GUSBCFG_SPEC> {
        ULPIAUTORES_W::new(self, 18)
    }
    #[doc = "Bit 19 - Mode: Host and Device ULPI Clock SuspendM (ULPIClkSusM) This bit sets the ClockSuspendM bit in the Interface Control register on the ULPI PHY. This bit applies only in serial or carkit modes. - 1'b0: PHY powers down internal clock during suspend. - 1'b1: PHY does not power down internal clock."]
    #[inline(always)]
    pub fn ulpiclksusm(&mut self) -> ULPICLKSUSM_W<'_, GUSBCFG_SPEC> {
        ULPICLKSUSM_W::new(self, 19)
    }
    #[doc = "Bit 20 - Mode: Host only ULPI External VBUS Drive (ULPIExtVbusDrv) This bit selects between internal or external supply to drive 5V on VBUS, in ULPI PHY. - 1'b0: PHY drives VBUS using internal charge pump (Default). - 1'b1: PHY drives VBUS using external supply."]
    #[inline(always)]
    pub fn ulpiextvbusdrv(&mut self) -> ULPIEXTVBUSDRV_W<'_, GUSBCFG_SPEC> {
        ULPIEXTVBUSDRV_W::new(self, 20)
    }
    #[doc = "Bit 21 - Mode: Host only ULPI External VBUS Indicator (ULPIExtVbusIndicator) This bit indicates to the ULPI PHY to use an external VBUS overcurrent indicator. - 1'b0: PHY uses internal VBUS valid comparator. - 1'b1: PHY uses external VBUS valid comparator."]
    #[inline(always)]
    pub fn ulpiextvbusindicator(&mut self) -> ULPIEXTVBUSINDICATOR_W<'_, GUSBCFG_SPEC> {
        ULPIEXTVBUSINDICATOR_W::new(self, 21)
    }
    #[doc = "Bit 22 - Mode: Device only TermSel DLine Pulsing Selection (TermSelDLPulse) This bit selects utmi_termselect to drive data line pulse during SRP. - 1'b0: Data line pulsing using utmi_txvalid (Default). - 1'b1: Data line pulsing using utmi_termsel."]
    #[inline(always)]
    pub fn termseldlpulse(&mut self) -> TERMSELDLPULSE_W<'_, GUSBCFG_SPEC> {
        TERMSELDLPULSE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Mode: Host only Indicator Complement Controls the PHY to invert the ExternalVbusIndicator input signal, generating the Complement Output. For more information, refer to the ULPI Specification. - 1'b0: PHY does not invert ExternalVbusIndicator signal - 1'b1: PHY does invert ExternalVbusIndicator signal This bit is reserved and read-only when OTG_HSPHY_INTERFACE is set to 0 or 1."]
    #[inline(always)]
    pub fn complement(&mut self) -> COMPLEMENT_W<'_, GUSBCFG_SPEC> {
        COMPLEMENT_W::new(self, 23)
    }
    #[doc = "Bit 24 - Mode: Host only Indicator Pass Through Controls wether the Complement Output is qualified with the Internal Vbus Valid comparator before being used in the Vbus State in the RX CMD. For more information, refer to the ULPI Specification. - 1'b0: Complement Output signal is qualified with the Internal VbusValid comparator. - 1'b1: Complement Output signal is not qualified with the Internal VbusValid comparator. This bit is reserved and read-only when OTG_HSPHY_INTERFACE is set to 0 or 1."]
    #[inline(always)]
    pub fn indicator(&mut self) -> INDICATOR_W<'_, GUSBCFG_SPEC> {
        INDICATOR_W::new(self, 24)
    }
    #[doc = "Bit 25 - Mode: Host only ULPI Interface Protect Disable Controls circuitry built into the PHY for protecting the ULPI interface when the link tri-states STP and data. Any pull-ups or pull-downs employed by this feature can be disabled. For more information, refer to the ULPI Specification. - 1'b0: Enables the interface protect circuit - 1'b1: Disables the interface protect circuit"]
    #[inline(always)]
    pub fn ulpi(&mut self) -> ULPI_W<'_, GUSBCFG_SPEC> {
        ULPI_W::new(self, 25)
    }
    #[doc = "Bit 28 - Mode: Device only Tx End Delay (TxEndDelay) Setting this bit to 1'b1 enables the controller to follow the TxEndDelay timings as per UTMI+ specification 1.05 section 4.1.5 for opmode signal during remote wakeup. - 1'b0 : Normal Mode. - 1'b1 : Tx End delay. The default value of this field is 1'b1 and it is not recommended to program it to 1'b0. The option to set it to 1'b0 (Normal Mode) is present only for debug purpose."]
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
    #[doc = "Bit 31 - Mode: Host and device Corrupt Tx packet (CorruptTxPkt) This bit is for debug purposes only. Never Set this bit to 1. The application must always write 1'b0 to this bit."]
    #[inline(always)]
    pub fn corrupttxpkt(&mut self) -> CORRUPTTXPKT_W<'_, GUSBCFG_SPEC> {
        CORRUPTTXPKT_W::new(self, 31)
    }
}
#[doc = "This register can be used to configure the core after power-on or when changing to Host mode or Device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. If you are using the HSIC interface, HSIC PHY must be in reset while programming this register. Do not make changes to this register after the initial programming.\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets GUSBCFG to value 0x1000_1400"]
impl crate::Resettable for GUSBCFG_SPEC {
    const RESET_VALUE: u32 = 0x1000_1400;
}
