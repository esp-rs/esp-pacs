#[doc = "Register `REGISTER14_INTERRUPTSTATUSREGISTER` reader"]
pub type R = crate::R<REGISTER14_INTERRUPTSTATUSREGISTER_SPEC>;
#[doc = "Field `RGSMIIIS` reader - RGMII or SMII Interrupt Status This bit is set because of any change in value of the Link Status of RGMII or SMII interface _Bit 3 in Register 54 _SGMII/RGMII/SMII Control and Status Register__ This bit is cleared when you perform a read operation on the SGMII/RGMII/SMII Control and Status Register This bit is valid only when you select the optional RGMII or SMII PHY interface during core configuration and operation"]
pub type RGSMIIIS_R = crate::BitReader;
#[doc = "Field `PCSLCHGIS` reader - PCS Link Status Changed This bit is set because of any change in Link Status in the TBI, RTBI, or SGMII PHY interface _Bit 2 in Register 49 _AN Status Register__ This bit is cleared when you perform a read operation on the AN Status register This bit is valid only when you select the optional TBI, RTBI, or SGMII PHY interface during core configuration and operation"]
pub type PCSLCHGIS_R = crate::BitReader;
#[doc = "Field `PCSANCIS` reader - PCS AutoNegotiation Complete This bit is set when the Autonegotiation is completed in the TBI, RTBI, or SGMII PHY interface _Bit 5 in Register 49 _AN Status Register__ This bit is cleared when you perform a read operation to the AN Status register This bit is valid only when you select the optional TBI, RTBI, or SGMII PHY interface during core configuration and operation"]
pub type PCSANCIS_R = crate::BitReader;
#[doc = "Field `PMTIS` reader - PMT Interrupt Status This bit is set when a magic packet or remote wakeup frame is received in the powerdown mode _see Bits 5 and 6 in the PMT Control and Status Register_ This bit is cleared when both Bits\\[6:5\\] are cleared because of a read operation to the PMT Control and Status register This bit is valid only when you select the optional PMT module during core configuration"]
pub type PMTIS_R = crate::BitReader;
#[doc = "Field `MMCIS` reader - MMC Interrupt Status This bit is set high when any of the Bits \\[7:5\\] is set high and cleared only when all of these bits are low This bit is valid only when you select the optional MMC module during core configuration"]
pub type MMCIS_R = crate::BitReader;
#[doc = "Field `MMCRXIS` reader - MMC Receive Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Interrupt Register This bit is cleared when all the bits in this interrupt register are cleared This bit is valid only when you select the optional MMC module during core configuration"]
pub type MMCRXIS_R = crate::BitReader;
#[doc = "Field `MMCTXIS` reader - MMC Transmit Interrupt Status This bit is set high when an interrupt is generated in the MMC Transmit Interrupt Register This bit is cleared when all the bits in this interrupt register are cleared This bit is valid only when you select the optional MMC module during core configuration"]
pub type MMCTXIS_R = crate::BitReader;
#[doc = "Field `MMCRXIPIS` reader - MMC Receive Checksum Offload Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Checksum Offload Interrupt Register This bit is cleared when all the bits in this interrupt register are cleared This bit is valid only when you select the optional MMC module and Checksum Offload Engine _Type 2_ during core configuration"]
pub type MMCRXIPIS_R = crate::BitReader;
#[doc = "Field `TSIS` reader - Timestamp Interrupt Status When the Advanced Timestamp feature is enabled, this bit is set when R_SS_RC any of the following conditions is true: The system time value equals or exceeds the value specified in the Target Time High and Low registers There is an overflow in the seconds register The Auxiliary snapshot trigger is asserted This bit is cleared on reading Bit 0 of Register 458 _Timestamp Status Register_ If default Timestamping is enabled, when set, this bit indicates that the system time value is equal to or exceeds the value specified in the Target Time registers In this mode, this bit is cleared after the completion of the read of this bit In all other modes, this bit is reserved"]
pub type TSIS_R = crate::BitReader;
#[doc = "Field `LPIIS` reader - LPI Interrupt Status When the Energy Efficient Ethernet feature is enabled, this bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver This bit is cleared on reading Bit 0 of Register 12 _LPI Control and Status Register_ In all other modes, this bit is reserved"]
pub type LPIIS_R = crate::BitReader;
#[doc = "Field `GPIIS` reader - GPI Interrupt Status When the GPIO feature is enabled, this bit is set when any active event _LL or LH_ occurs on the GPIS field _Bits \\[3:0\\]_ of Register 56 _General Purpose IO Register_ and the corresponding GPIE bit is enabled This bit is cleared on reading lane 0 _GPIS_ of Register 56 _General Purpose IO Register_ When the GPIO feature is not enabled, this bit is reserved"]
pub type GPIIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RGMII or SMII Interrupt Status This bit is set because of any change in value of the Link Status of RGMII or SMII interface _Bit 3 in Register 54 _SGMII/RGMII/SMII Control and Status Register__ This bit is cleared when you perform a read operation on the SGMII/RGMII/SMII Control and Status Register This bit is valid only when you select the optional RGMII or SMII PHY interface during core configuration and operation"]
    #[inline(always)]
    pub fn rgsmiiis(&self) -> RGSMIIIS_R {
        RGSMIIIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCS Link Status Changed This bit is set because of any change in Link Status in the TBI, RTBI, or SGMII PHY interface _Bit 2 in Register 49 _AN Status Register__ This bit is cleared when you perform a read operation on the AN Status register This bit is valid only when you select the optional TBI, RTBI, or SGMII PHY interface during core configuration and operation"]
    #[inline(always)]
    pub fn pcslchgis(&self) -> PCSLCHGIS_R {
        PCSLCHGIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCS AutoNegotiation Complete This bit is set when the Autonegotiation is completed in the TBI, RTBI, or SGMII PHY interface _Bit 5 in Register 49 _AN Status Register__ This bit is cleared when you perform a read operation to the AN Status register This bit is valid only when you select the optional TBI, RTBI, or SGMII PHY interface during core configuration and operation"]
    #[inline(always)]
    pub fn pcsancis(&self) -> PCSANCIS_R {
        PCSANCIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Status This bit is set when a magic packet or remote wakeup frame is received in the powerdown mode _see Bits 5 and 6 in the PMT Control and Status Register_ This bit is cleared when both Bits\\[6:5\\] are cleared because of a read operation to the PMT Control and Status register This bit is valid only when you select the optional PMT module during core configuration"]
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status This bit is set high when any of the Bits \\[7:5\\] is set high and cleared only when all of these bits are low This bit is valid only when you select the optional MMC module during core configuration"]
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Interrupt Register This bit is cleared when all the bits in this interrupt register are cleared This bit is valid only when you select the optional MMC module during core configuration"]
    #[inline(always)]
    pub fn mmcrxis(&self) -> MMCRXIS_R {
        MMCRXIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status This bit is set high when an interrupt is generated in the MMC Transmit Interrupt Register This bit is cleared when all the bits in this interrupt register are cleared This bit is valid only when you select the optional MMC module during core configuration"]
    #[inline(always)]
    pub fn mmctxis(&self) -> MMCTXIS_R {
        MMCTXIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Checksum Offload Interrupt Status This bit is set high when an interrupt is generated in the MMC Receive Checksum Offload Interrupt Register This bit is cleared when all the bits in this interrupt register are cleared This bit is valid only when you select the optional MMC module and Checksum Offload Engine _Type 2_ during core configuration"]
    #[inline(always)]
    pub fn mmcrxipis(&self) -> MMCRXIPIS_R {
        MMCRXIPIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Status When the Advanced Timestamp feature is enabled, this bit is set when R_SS_RC any of the following conditions is true: The system time value equals or exceeds the value specified in the Target Time High and Low registers There is an overflow in the seconds register The Auxiliary snapshot trigger is asserted This bit is cleared on reading Bit 0 of Register 458 _Timestamp Status Register_ If default Timestamping is enabled, when set, this bit indicates that the system time value is equal to or exceeds the value specified in the Target Time registers In this mode, this bit is cleared after the completion of the read of this bit In all other modes, this bit is reserved"]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPI Interrupt Status When the Energy Efficient Ethernet feature is enabled, this bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver This bit is cleared on reading Bit 0 of Register 12 _LPI Control and Status Register_ In all other modes, this bit is reserved"]
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPI Interrupt Status When the GPIO feature is enabled, this bit is set when any active event _LL or LH_ occurs on the GPIS field _Bits \\[3:0\\]_ of Register 56 _General Purpose IO Register_ and the corresponding GPIE bit is enabled This bit is cleared on reading lane 0 _GPIS_ of Register 56 _General Purpose IO Register_ When the GPIO feature is not enabled, this bit is reserved"]
    #[inline(always)]
    pub fn gpiis(&self) -> GPIIS_R {
        GPIIS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER14_INTERRUPTSTATUSREGISTER")
            .field("rgsmiiis", &self.rgsmiiis())
            .field("pcslchgis", &self.pcslchgis())
            .field("pcsancis", &self.pcsancis())
            .field("pmtis", &self.pmtis())
            .field("mmcis", &self.mmcis())
            .field("mmcrxis", &self.mmcrxis())
            .field("mmctxis", &self.mmctxis())
            .field("mmcrxipis", &self.mmcrxipis())
            .field("tsis", &self.tsis())
            .field("lpiis", &self.lpiis())
            .field("gpiis", &self.gpiis())
            .finish()
    }
}
#[doc = "Contains the interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`register14_interruptstatusregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER14_INTERRUPTSTATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER14_INTERRUPTSTATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register14_interruptstatusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER14_INTERRUPTSTATUSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER14_INTERRUPTSTATUSREGISTER to value 0"]
impl crate::Resettable for REGISTER14_INTERRUPTSTATUSREGISTER_SPEC {}
