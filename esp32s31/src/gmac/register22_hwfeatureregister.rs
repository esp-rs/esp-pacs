#[doc = "Register `REGISTER22_HWFEATUREREGISTER` reader"]
pub type R = crate::R<REGISTER22_HWFEATUREREGISTER_SPEC>;
#[doc = "Field `MIISEL` reader - 10 or 100 Mbps support"]
pub type MIISEL_R = crate::BitReader;
#[doc = "Field `GMIISEL` reader - 1000 Mbps support"]
pub type GMIISEL_R = crate::BitReader;
#[doc = "Field `HDSEL` reader - Halfduplex support"]
pub type HDSEL_R = crate::BitReader;
#[doc = "Field `EXTHASHEN` reader - Expanded DA Hash filter"]
pub type EXTHASHEN_R = crate::BitReader;
#[doc = "Field `HASHSEL` reader - HASH filter"]
pub type HASHSEL_R = crate::BitReader;
#[doc = "Field `ADDMACADRSEL` reader - Multiple MAC Address registers"]
pub type ADDMACADRSEL_R = crate::BitReader;
#[doc = "Field `PCSSEL` reader - PCS registers _TBI, SGMII, or RTBI PHY interface_"]
pub type PCSSEL_R = crate::BitReader;
#[doc = "Field `L3L4FLTREN` reader - Layer 3 and Layer 4 feature"]
pub type L3L4FLTREN_R = crate::BitReader;
#[doc = "Field `SMASEL` reader - SMA _MDIO_ Interface"]
pub type SMASEL_R = crate::BitReader;
#[doc = "Field `RWKSEL` reader - PMT remote wakeup frame"]
pub type RWKSEL_R = crate::BitReader;
#[doc = "Field `MGKSEL` reader - PMT magic packet"]
pub type MGKSEL_R = crate::BitReader;
#[doc = "Field `MMCSEL` reader - RMON module"]
pub type MMCSEL_R = crate::BitReader;
#[doc = "Field `TSVER1SEL` reader - Only IEEE 15882002 timestamp"]
pub type TSVER1SEL_R = crate::BitReader;
#[doc = "Field `TSVER2SEL` reader - IEEE 15882008 Advanced timestamp"]
pub type TSVER2SEL_R = crate::BitReader;
#[doc = "Field `EEESEL` reader - Energy Efficient Ethernet"]
pub type EEESEL_R = crate::BitReader;
#[doc = "Field `AVSEL` reader - AV feature"]
pub type AVSEL_R = crate::BitReader;
#[doc = "Field `TXCOESEL` reader - Checksum Offload in Tx"]
pub type TXCOESEL_R = crate::BitReader;
#[doc = "Field `RXTYP1COE` reader - IP Checksum Offload _Type 1_ in Rx Note: If IPCHKSUM_EN = Enabled and IPC_FULL_OFFLOAD = Enabled, then RXTYP1COE = 0 and RXTYP2COE = 1"]
pub type RXTYP1COE_R = crate::BitReader;
#[doc = "Field `RXTYP2COE` reader - IP Checksum Offload _Type 2_ in Rx"]
pub type RXTYP2COE_R = crate::BitReader;
#[doc = "Field `RXFIFOSIZE` reader - Rx FIFO > 2,048 Bytes"]
pub type RXFIFOSIZE_R = crate::BitReader;
#[doc = "Field `RXCHCNT` reader - Number of additional Rx Channels"]
pub type RXCHCNT_R = crate::FieldReader;
#[doc = "Field `TXCHCNT` reader - Number of additional Tx Channels"]
pub type TXCHCNT_R = crate::FieldReader;
#[doc = "Field `ENHDESSEL` reader - Alternate _Enhanced Descriptor_"]
pub type ENHDESSEL_R = crate::BitReader;
#[doc = "Field `INTTSEN` reader - Timestamping with Internal System Time"]
pub type INTTSEN_R = crate::BitReader;
#[doc = "Field `FLEXIPPSEN` reader - Flexible PulsePerSecond Output"]
pub type FLEXIPPSEN_R = crate::BitReader;
#[doc = "Field `SAVLANINS` reader - Source Address or VLAN Insertion"]
pub type SAVLANINS_R = crate::BitReader;
#[doc = "Field `ACTPHYIF` reader - Active or selected PHY interface When you have multiple PHY interfaces in your configuration, this field indicates the sampled value of phy_intf_sel_i during reset deassertion 000: GMII or MII 001: RGMII 010: SGMII 011: TBI 100: RMII 101: RTBI 110: SMII 111: RevMII All Others: Reserved"]
pub type ACTPHYIF_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 10 or 100 Mbps support"]
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1000 Mbps support"]
    #[inline(always)]
    pub fn gmiisel(&self) -> GMIISEL_R {
        GMIISEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Halfduplex support"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expanded DA Hash filter"]
    #[inline(always)]
    pub fn exthashen(&self) -> EXTHASHEN_R {
        EXTHASHEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HASH filter"]
    #[inline(always)]
    pub fn hashsel(&self) -> HASHSEL_R {
        HASHSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multiple MAC Address registers"]
    #[inline(always)]
    pub fn addmacadrsel(&self) -> ADDMACADRSEL_R {
        ADDMACADRSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCS registers _TBI, SGMII, or RTBI PHY interface_"]
    #[inline(always)]
    pub fn pcssel(&self) -> PCSSEL_R {
        PCSSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Layer 3 and Layer 4 feature"]
    #[inline(always)]
    pub fn l3l4fltren(&self) -> L3L4FLTREN_R {
        L3L4FLTREN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SMA _MDIO_ Interface"]
    #[inline(always)]
    pub fn smasel(&self) -> SMASEL_R {
        SMASEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PMT remote wakeup frame"]
    #[inline(always)]
    pub fn rwksel(&self) -> RWKSEL_R {
        RWKSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PMT magic packet"]
    #[inline(always)]
    pub fn mgksel(&self) -> MGKSEL_R {
        MGKSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RMON module"]
    #[inline(always)]
    pub fn mmcsel(&self) -> MMCSEL_R {
        MMCSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Only IEEE 15882002 timestamp"]
    #[inline(always)]
    pub fn tsver1sel(&self) -> TSVER1SEL_R {
        TSVER1SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IEEE 15882008 Advanced timestamp"]
    #[inline(always)]
    pub fn tsver2sel(&self) -> TSVER2SEL_R {
        TSVER2SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Energy Efficient Ethernet"]
    #[inline(always)]
    pub fn eeesel(&self) -> EEESEL_R {
        EEESEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AV feature"]
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Checksum Offload in Tx"]
    #[inline(always)]
    pub fn txcoesel(&self) -> TXCOESEL_R {
        TXCOESEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IP Checksum Offload _Type 1_ in Rx Note: If IPCHKSUM_EN = Enabled and IPC_FULL_OFFLOAD = Enabled, then RXTYP1COE = 0 and RXTYP2COE = 1"]
    #[inline(always)]
    pub fn rxtyp1coe(&self) -> RXTYP1COE_R {
        RXTYP1COE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IP Checksum Offload _Type 2_ in Rx"]
    #[inline(always)]
    pub fn rxtyp2coe(&self) -> RXTYP2COE_R {
        RXTYP2COE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO > 2,048 Bytes"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Number of additional Rx Channels"]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Number of additional Tx Channels"]
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Alternate _Enhanced Descriptor_"]
    #[inline(always)]
    pub fn enhdessel(&self) -> ENHDESSEL_R {
        ENHDESSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timestamping with Internal System Time"]
    #[inline(always)]
    pub fn inttsen(&self) -> INTTSEN_R {
        INTTSEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Flexible PulsePerSecond Output"]
    #[inline(always)]
    pub fn flexippsen(&self) -> FLEXIPPSEN_R {
        FLEXIPPSEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Source Address or VLAN Insertion"]
    #[inline(always)]
    pub fn savlanins(&self) -> SAVLANINS_R {
        SAVLANINS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Active or selected PHY interface When you have multiple PHY interfaces in your configuration, this field indicates the sampled value of phy_intf_sel_i during reset deassertion 000: GMII or MII 001: RGMII 010: SGMII 011: TBI 100: RMII 101: RTBI 110: SMII 111: RevMII All Others: Reserved"]
    #[inline(always)]
    pub fn actphyif(&self) -> ACTPHYIF_R {
        ACTPHYIF_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER22_HWFEATUREREGISTER")
            .field("miisel", &self.miisel())
            .field("gmiisel", &self.gmiisel())
            .field("hdsel", &self.hdsel())
            .field("exthashen", &self.exthashen())
            .field("hashsel", &self.hashsel())
            .field("addmacadrsel", &self.addmacadrsel())
            .field("pcssel", &self.pcssel())
            .field("l3l4fltren", &self.l3l4fltren())
            .field("smasel", &self.smasel())
            .field("rwksel", &self.rwksel())
            .field("mgksel", &self.mgksel())
            .field("mmcsel", &self.mmcsel())
            .field("tsver1sel", &self.tsver1sel())
            .field("tsver2sel", &self.tsver2sel())
            .field("eeesel", &self.eeesel())
            .field("avsel", &self.avsel())
            .field("txcoesel", &self.txcoesel())
            .field("rxtyp1coe", &self.rxtyp1coe())
            .field("rxtyp2coe", &self.rxtyp2coe())
            .field("rxfifosize", &self.rxfifosize())
            .field("rxchcnt", &self.rxchcnt())
            .field("txchcnt", &self.txchcnt())
            .field("enhdessel", &self.enhdessel())
            .field("inttsen", &self.inttsen())
            .field("flexippsen", &self.flexippsen())
            .field("savlanins", &self.savlanins())
            .field("actphyif", &self.actphyif())
            .finish()
    }
}
#[doc = "Indicates the presence of the optional features of the core\n\nYou can [`read`](crate::Reg::read) this register and get [`register22_hwfeatureregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER22_HWFEATUREREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER22_HWFEATUREREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register22_hwfeatureregister::R`](R) reader structure"]
impl crate::Readable for REGISTER22_HWFEATUREREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER22_HWFEATUREREGISTER to value 0"]
impl crate::Resettable for REGISTER22_HWFEATUREREGISTER_SPEC {}
