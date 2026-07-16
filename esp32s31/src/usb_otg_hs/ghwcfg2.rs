#[doc = "Register `GHWCFG2` reader"]
pub type R = crate::R<GHWCFG2_SPEC>;
#[doc = "Field `OTGMODE` reader - Mode of Operation (OtgMode) - 3'b000: HNP- and SRP-Capable OTG (Host & Device) - 3'b001: SRP-Capable OTG (Host & Device) - 3'b010: Non-HNP and Non-SRP Capable OTG (Host and Device) - 3'b011: SRP-Capable Device - 3'b100: Non-OTG Device - 3'b101: SRP-Capable Host - 3'b110: Non-OTG Host - Others: Reserved Note: This field is configured using the OTG_MODE parameter."]
pub type OTGMODE_R = crate::FieldReader;
#[doc = "Field `OTGARCH` reader - Architecture (OtgArch) - 2'b00: Slave-Only - 2'b01: External DMA - 2'b10: Internal DMA - Others: Reserved Note: This field is configured using the OTG_ARCHITECTURE parameter."]
pub type OTGARCH_R = crate::FieldReader;
#[doc = "Field `SINGPNT` reader - Point-to-Point (SingPnt) - 1'b0: Multi-point application (hub and split support) - 1'b1: Single-point application (no hub and split support) Note: This field is configured using the OTG_SINGLE_POINT parameter."]
pub type SINGPNT_R = crate::BitReader;
#[doc = "Field `HSPHYTYPE` reader - High-Speed PHY Interface Type (HSPhyType) - 2'b00: High-Speed interface not supported - 2'b01: UTMI+ - 2'b10: ULPI - 2'b11: UTMI+ and ULPI Note: This field is configured using the OTG_HSPHY_INTERFACE parameter."]
pub type HSPHYTYPE_R = crate::FieldReader;
#[doc = "Field `FSPHYTYPE` reader - Full-Speed PHY Interface Type (FSPhyType) - 2'b00: Full-speed interface not supported - 2'b01: Dedicated full-speed interface - 2'b10: FS pins shared with UTMI+ pins - 2'b11: FS pins shared with ULPI pins Note: This field is configured using the OTG_FSPHY_INTERFACE parameter."]
pub type FSPHYTYPE_R = crate::FieldReader;
#[doc = "Field `NUMDEVEPS` reader - Number of Device Endpoints (NumDevEps) Indicates the number of device endpoints supported by the core in Device mode. The range of this field is 0-15. Note: This field is configured using the OTG_NUM_EPS parameter."]
pub type NUMDEVEPS_R = crate::FieldReader;
#[doc = "Field `NUMHSTCHNL` reader - Number of Host Channels (NumHstChnl) Indicates the number of host channels supported by the core in Host mode. The range of this field is 0-15: 0 specifies 1 channel, 15 specifies 16 channels. Note: This field is configured using the OTG_NUM_HOST_CHAN parameter."]
pub type NUMHSTCHNL_R = crate::FieldReader;
#[doc = "Field `PERIOSUPPORT` reader - Periodic OUT Channels Supported in Host Mode (PerioSupport) - 1'b0: No - 1'b1: Yes Note: This field is configured using the OTG_EN_PERIO_HOST parameter."]
pub type PERIOSUPPORT_R = crate::BitReader;
#[doc = "Field `DYNFIFOSIZING` reader - Dynamic FIFO Sizing Enabled (DynFifoSizing) - 1'b0: No - 1'b1: Yes Note: This field is configured using the OTG_DFIFO_DYNAMIC parameter."]
pub type DYNFIFOSIZING_R = crate::BitReader;
#[doc = "Field `MULTIPROCINTRPT` reader - Multi Processor Interrupt Enabled (MultiProcIntrpt) - 1'b0: No - 1'b1: Yes Note: This field is configured using the OTG_MULTI_PROC_INTRPT parameter."]
pub type MULTIPROCINTRPT_R = crate::BitReader;
#[doc = "Field `NPTXQDEPTH` reader - Non-periodic Request Queue Depth (NPTxQDepth) - 2'b00: 2 - 2'b01: 4 - 2'b10: 8 - Others: Reserved Note: This field is configured using the OTG_NPERIO_TX_QUEUE_DEPTH parameter."]
pub type NPTXQDEPTH_R = crate::FieldReader;
#[doc = "Field `PTXQDEPTH` reader - Host Mode Periodic Request Queue Depth (PTxQDepth) - 2'b00: 2 - 2'b01: 4 - 2'b10: 8 - 2'b11:16 Note: This field is configured using the OTG_PERIO_TX_QUEUE_DEPTH parameter."]
pub type PTXQDEPTH_R = crate::FieldReader;
#[doc = "Field `TKNQDEPTH` reader - Device Mode IN Token Sequence Learning Queue Depth (TknQDepth) Range: 0-30 Note: This field is configured using the OTG_TOKEN_QUEUE_DEPTH parameter."]
pub type TKNQDEPTH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Mode of Operation (OtgMode) - 3'b000: HNP- and SRP-Capable OTG (Host & Device) - 3'b001: SRP-Capable OTG (Host & Device) - 3'b010: Non-HNP and Non-SRP Capable OTG (Host and Device) - 3'b011: SRP-Capable Device - 3'b100: Non-OTG Device - 3'b101: SRP-Capable Host - 3'b110: Non-OTG Host - Others: Reserved Note: This field is configured using the OTG_MODE parameter."]
    #[inline(always)]
    pub fn otgmode(&self) -> OTGMODE_R {
        OTGMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Architecture (OtgArch) - 2'b00: Slave-Only - 2'b01: External DMA - 2'b10: Internal DMA - Others: Reserved Note: This field is configured using the OTG_ARCHITECTURE parameter."]
    #[inline(always)]
    pub fn otgarch(&self) -> OTGARCH_R {
        OTGARCH_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Point-to-Point (SingPnt) - 1'b0: Multi-point application (hub and split support) - 1'b1: Single-point application (no hub and split support) Note: This field is configured using the OTG_SINGLE_POINT parameter."]
    #[inline(always)]
    pub fn singpnt(&self) -> SINGPNT_R {
        SINGPNT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - High-Speed PHY Interface Type (HSPhyType) - 2'b00: High-Speed interface not supported - 2'b01: UTMI+ - 2'b10: ULPI - 2'b11: UTMI+ and ULPI Note: This field is configured using the OTG_HSPHY_INTERFACE parameter."]
    #[inline(always)]
    pub fn hsphytype(&self) -> HSPHYTYPE_R {
        HSPHYTYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Full-Speed PHY Interface Type (FSPhyType) - 2'b00: Full-speed interface not supported - 2'b01: Dedicated full-speed interface - 2'b10: FS pins shared with UTMI+ pins - 2'b11: FS pins shared with ULPI pins Note: This field is configured using the OTG_FSPHY_INTERFACE parameter."]
    #[inline(always)]
    pub fn fsphytype(&self) -> FSPHYTYPE_R {
        FSPHYTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Number of Device Endpoints (NumDevEps) Indicates the number of device endpoints supported by the core in Device mode. The range of this field is 0-15. Note: This field is configured using the OTG_NUM_EPS parameter."]
    #[inline(always)]
    pub fn numdeveps(&self) -> NUMDEVEPS_R {
        NUMDEVEPS_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17 - Number of Host Channels (NumHstChnl) Indicates the number of host channels supported by the core in Host mode. The range of this field is 0-15: 0 specifies 1 channel, 15 specifies 16 channels. Note: This field is configured using the OTG_NUM_HOST_CHAN parameter."]
    #[inline(always)]
    pub fn numhstchnl(&self) -> NUMHSTCHNL_R {
        NUMHSTCHNL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - Periodic OUT Channels Supported in Host Mode (PerioSupport) - 1'b0: No - 1'b1: Yes Note: This field is configured using the OTG_EN_PERIO_HOST parameter."]
    #[inline(always)]
    pub fn periosupport(&self) -> PERIOSUPPORT_R {
        PERIOSUPPORT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Dynamic FIFO Sizing Enabled (DynFifoSizing) - 1'b0: No - 1'b1: Yes Note: This field is configured using the OTG_DFIFO_DYNAMIC parameter."]
    #[inline(always)]
    pub fn dynfifosizing(&self) -> DYNFIFOSIZING_R {
        DYNFIFOSIZING_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Multi Processor Interrupt Enabled (MultiProcIntrpt) - 1'b0: No - 1'b1: Yes Note: This field is configured using the OTG_MULTI_PROC_INTRPT parameter."]
    #[inline(always)]
    pub fn multiprocintrpt(&self) -> MULTIPROCINTRPT_R {
        MULTIPROCINTRPT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Non-periodic Request Queue Depth (NPTxQDepth) - 2'b00: 2 - 2'b01: 4 - 2'b10: 8 - Others: Reserved Note: This field is configured using the OTG_NPERIO_TX_QUEUE_DEPTH parameter."]
    #[inline(always)]
    pub fn nptxqdepth(&self) -> NPTXQDEPTH_R {
        NPTXQDEPTH_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Host Mode Periodic Request Queue Depth (PTxQDepth) - 2'b00: 2 - 2'b01: 4 - 2'b10: 8 - 2'b11:16 Note: This field is configured using the OTG_PERIO_TX_QUEUE_DEPTH parameter."]
    #[inline(always)]
    pub fn ptxqdepth(&self) -> PTXQDEPTH_R {
        PTXQDEPTH_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:30 - Device Mode IN Token Sequence Learning Queue Depth (TknQDepth) Range: 0-30 Note: This field is configured using the OTG_TOKEN_QUEUE_DEPTH parameter."]
    #[inline(always)]
    pub fn tknqdepth(&self) -> TKNQDEPTH_R {
        TKNQDEPTH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GHWCFG2")
            .field("otgmode", &self.otgmode())
            .field("otgarch", &self.otgarch())
            .field("singpnt", &self.singpnt())
            .field("hsphytype", &self.hsphytype())
            .field("fsphytype", &self.fsphytype())
            .field("numdeveps", &self.numdeveps())
            .field("numhstchnl", &self.numhstchnl())
            .field("periosupport", &self.periosupport())
            .field("dynfifosizing", &self.dynfifosizing())
            .field("multiprocintrpt", &self.multiprocintrpt())
            .field("nptxqdepth", &self.nptxqdepth())
            .field("ptxqdepth", &self.ptxqdepth())
            .field("tknqdepth", &self.tknqdepth())
            .finish()
    }
}
#[doc = "This register contains configuration options selected using coreConsultant.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GHWCFG2_SPEC;
impl crate::RegisterSpec for GHWCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwcfg2::R`](R) reader structure"]
impl crate::Readable for GHWCFG2_SPEC {}
#[doc = "`reset()` method sets GHWCFG2 to value 0x239f_fed2"]
impl crate::Resettable for GHWCFG2_SPEC {
    const RESET_VALUE: u32 = 0x239f_fed2;
}
