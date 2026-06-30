#[doc = "Register `GHWCFG3` reader"]
pub type R = crate::R<GHWCFG3_SPEC>;
#[doc = "Field `XFERSIZEWIDTH` reader - Width of Transfer Size Counters (XferSizeWidth) - 4'b0000: 11 bits - 4'b0001: 12 bits ... - 4'b1000: 19 bits - Others: Reserved Note: This field is configured using the OTG_PACKET_COUNT_WIDTH parameter."]
pub type XFERSIZEWIDTH_R = crate::FieldReader;
#[doc = "Field `PKTSIZEWIDTH` reader - Width of Packet Size Counters (PktSizeWidth) - 3'b000: 4 bits - 3'b001: 5 bits - 3'b010: 6 bits - 3'b011: 7 bits - 3'b100: 8 bits - 3'b101: 9 bits - 3'b110: 10 bits - Others: Reserved Note: This field is configured using the OTG_PACKET_COUNT_WIDTH parameter."]
pub type PKTSIZEWIDTH_R = crate::FieldReader;
#[doc = "Field `OTGEN` reader - OTG Function Enabled (OtgEn) The application uses this bit to indicate the OTG capabilities of the controller . - 1'b0: Not OTG capable - 1'b1: OTG Capable Note: This field is configured using the OTG_MODE parameter."]
pub type OTGEN_R = crate::BitReader;
#[doc = "Field `I2CINTSEL` reader - I2C Selection (I2CIntSel) - 1'b0: I2C Interface is not available on the controller. - 1'b1: I2C Interface is available on the controller. Note: This field is configured using the OTG_I2C_INTERFACE parameter."]
pub type I2CINTSEL_R = crate::BitReader;
#[doc = "Field `VNDCTLSUPT` reader - Vendor Control Interface Support (VndctlSupt) - 1'b0: Vendor Control Interface is not available on the core. - 1'b1: Vendor Control Interface is available. Note: This field is configured using the OTG_VENDOR_CTL_INTERFACE parameter."]
pub type VNDCTLSUPT_R = crate::BitReader;
#[doc = "Field `OPTFEATURE` reader - Optional Features Removed (OptFeature) Indicates whether the User ID register, GPIO interface ports, and SOF toggle and counter ports were removed for gate count optimization by enabling Remove Optional Features. - 1'b0: No - 1'b1: Yes Note: This field is configured using the OTG_RM_OPT_FEATURES parameter."]
pub type OPTFEATURE_R = crate::BitReader;
#[doc = "Field `RSTTYPE` reader - Reset Style for Clocked always Blocks in RTL (RstType) - 1'b0: Asynchronous reset is used in the controller - 1'b1: Synchronous reset is used in the controller Note: This field is configured using the OTG_SYNC_RESET_TYPE parameter."]
pub type RSTTYPE_R = crate::BitReader;
#[doc = "Field `ADPSUPPORT` reader - This bit indicates whether ADP logic is present within or external to the controller - 0: No ADP logic present with the controller - 1: ADP logic is present along with the controller."]
pub type ADPSUPPORT_R = crate::BitReader;
#[doc = "Field `HSICMODE` reader - HSIC mode specified for Mode of Operation Value Range: 0 - 1 - 1: HSIC-capable with shared UTMI PHY interface - 0: Non-HSIC-capable"]
pub type HSICMODE_R = crate::BitReader;
#[doc = "Field `BCSUPPORT` reader - This bit indicates the controller support for Battery Charger. - 0 - No Battery Charger Support - 1 - Battery Charger support present"]
pub type BCSUPPORT_R = crate::BitReader;
#[doc = "Field `LPMMODE` reader - LPM mode specified for Mode of Operation."]
pub type LPMMODE_R = crate::BitReader;
#[doc = "Field `DFIFODEPTH` reader - DFIFO Depth (DfifoDepth - EP_LOC_CNT) This value is in terms of 32-bit words. - Minimum value is 32 - Maximum value is 32,768 Note: This field is configured using the OTG_DFIFO_DEPTH parameter. For more information on EP_LOC_CNT, see the Endpoint Information Controller (EPINFO_CTL) section."]
pub type DFIFODEPTH_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Width of Transfer Size Counters (XferSizeWidth) - 4'b0000: 11 bits - 4'b0001: 12 bits ... - 4'b1000: 19 bits - Others: Reserved Note: This field is configured using the OTG_PACKET_COUNT_WIDTH parameter."]
    #[inline(always)]
    pub fn xfersizewidth(&self) -> XFERSIZEWIDTH_R {
        XFERSIZEWIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Width of Packet Size Counters (PktSizeWidth) - 3'b000: 4 bits - 3'b001: 5 bits - 3'b010: 6 bits - 3'b011: 7 bits - 3'b100: 8 bits - 3'b101: 9 bits - 3'b110: 10 bits - Others: Reserved Note: This field is configured using the OTG_PACKET_COUNT_WIDTH parameter."]
    #[inline(always)]
    pub fn pktsizewidth(&self) -> PKTSIZEWIDTH_R {
        PKTSIZEWIDTH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OTG Function Enabled (OtgEn) The application uses this bit to indicate the OTG capabilities of the controller . - 1'b0: Not OTG capable - 1'b1: OTG Capable Note: This field is configured using the OTG_MODE parameter."]
    #[inline(always)]
    pub fn otgen(&self) -> OTGEN_R {
        OTGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Selection (I2CIntSel) - 1'b0: I2C Interface is not available on the controller. - 1'b1: I2C Interface is available on the controller. Note: This field is configured using the OTG_I2C_INTERFACE parameter."]
    #[inline(always)]
    pub fn i2cintsel(&self) -> I2CINTSEL_R {
        I2CINTSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Vendor Control Interface Support (VndctlSupt) - 1'b0: Vendor Control Interface is not available on the core. - 1'b1: Vendor Control Interface is available. Note: This field is configured using the OTG_VENDOR_CTL_INTERFACE parameter."]
    #[inline(always)]
    pub fn vndctlsupt(&self) -> VNDCTLSUPT_R {
        VNDCTLSUPT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Optional Features Removed (OptFeature) Indicates whether the User ID register, GPIO interface ports, and SOF toggle and counter ports were removed for gate count optimization by enabling Remove Optional Features. - 1'b0: No - 1'b1: Yes Note: This field is configured using the OTG_RM_OPT_FEATURES parameter."]
    #[inline(always)]
    pub fn optfeature(&self) -> OPTFEATURE_R {
        OPTFEATURE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset Style for Clocked always Blocks in RTL (RstType) - 1'b0: Asynchronous reset is used in the controller - 1'b1: Synchronous reset is used in the controller Note: This field is configured using the OTG_SYNC_RESET_TYPE parameter."]
    #[inline(always)]
    pub fn rsttype(&self) -> RSTTYPE_R {
        RSTTYPE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit indicates whether ADP logic is present within or external to the controller - 0: No ADP logic present with the controller - 1: ADP logic is present along with the controller."]
    #[inline(always)]
    pub fn adpsupport(&self) -> ADPSUPPORT_R {
        ADPSUPPORT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HSIC mode specified for Mode of Operation Value Range: 0 - 1 - 1: HSIC-capable with shared UTMI PHY interface - 0: Non-HSIC-capable"]
    #[inline(always)]
    pub fn hsicmode(&self) -> HSICMODE_R {
        HSICMODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit indicates the controller support for Battery Charger. - 0 - No Battery Charger Support - 1 - Battery Charger support present"]
    #[inline(always)]
    pub fn bcsupport(&self) -> BCSUPPORT_R {
        BCSUPPORT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPM mode specified for Mode of Operation."]
    #[inline(always)]
    pub fn lpmmode(&self) -> LPMMODE_R {
        LPMMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - DFIFO Depth (DfifoDepth - EP_LOC_CNT) This value is in terms of 32-bit words. - Minimum value is 32 - Maximum value is 32,768 Note: This field is configured using the OTG_DFIFO_DEPTH parameter. For more information on EP_LOC_CNT, see the Endpoint Information Controller (EPINFO_CTL) section."]
    #[inline(always)]
    pub fn dfifodepth(&self) -> DFIFODEPTH_R {
        DFIFODEPTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GHWCFG3")
            .field("xfersizewidth", &self.xfersizewidth())
            .field("pktsizewidth", &self.pktsizewidth())
            .field("otgen", &self.otgen())
            .field("i2cintsel", &self.i2cintsel())
            .field("vndctlsupt", &self.vndctlsupt())
            .field("optfeature", &self.optfeature())
            .field("rsttype", &self.rsttype())
            .field("adpsupport", &self.adpsupport())
            .field("hsicmode", &self.hsicmode())
            .field("bcsupport", &self.bcsupport())
            .field("lpmmode", &self.lpmmode())
            .field("dfifodepth", &self.dfifodepth())
            .finish()
    }
}
#[doc = "This register contains configuration options selected using coreConsultant.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GHWCFG3_SPEC;
impl crate::RegisterSpec for GHWCFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwcfg3::R`](R) reader structure"]
impl crate::Readable for GHWCFG3_SPEC {}
#[doc = "`reset()` method sets GHWCFG3 to value 0x0380_46e8"]
impl crate::Resettable for GHWCFG3_SPEC {
    const RESET_VALUE: u32 = 0x0380_46e8;
}
