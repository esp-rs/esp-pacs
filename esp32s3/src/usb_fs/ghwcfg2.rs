#[doc = "Register `GHWCFG2` reader"]
pub type R = crate::R<GHWCFG2_SPEC>;
#[doc = "Field `OTGMODE` reader - "]
pub type OTGMODE_R = crate::FieldReader;
#[doc = "Field `OTGARCH` reader - "]
pub type OTGARCH_R = crate::FieldReader;
#[doc = "Field `SINGPNT` reader - "]
pub type SINGPNT_R = crate::BitReader;
#[doc = "Field `HSPHYTYPE` reader - "]
pub type HSPHYTYPE_R = crate::FieldReader;
#[doc = "Field `FSPHYTYPE` reader - "]
pub type FSPHYTYPE_R = crate::FieldReader;
#[doc = "Field `NUMDEVEPS` reader - "]
pub type NUMDEVEPS_R = crate::FieldReader;
#[doc = "Field `NUMHSTCHNL` reader - "]
pub type NUMHSTCHNL_R = crate::FieldReader;
#[doc = "Field `PERIOSUPPORT` reader - "]
pub type PERIOSUPPORT_R = crate::BitReader;
#[doc = "Field `DYNFIFOSIZING` reader - "]
pub type DYNFIFOSIZING_R = crate::BitReader;
#[doc = "Field `MULTIPROCINTRPT` reader - "]
pub type MULTIPROCINTRPT_R = crate::BitReader;
#[doc = "Field `NPTXQDEPTH` reader - "]
pub type NPTXQDEPTH_R = crate::FieldReader;
#[doc = "Field `PTXQDEPTH` reader - "]
pub type PTXQDEPTH_R = crate::FieldReader;
#[doc = "Field `TKNQDEPTH` reader - "]
pub type TKNQDEPTH_R = crate::FieldReader;
#[doc = "Field `OTG_ENABLE_IC_USB` reader - "]
pub type OTG_ENABLE_IC_USB_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn otgmode(&self) -> OTGMODE_R {
        OTGMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn otgarch(&self) -> OTGARCH_R {
        OTGARCH_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn singpnt(&self) -> SINGPNT_R {
        SINGPNT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn hsphytype(&self) -> HSPHYTYPE_R {
        HSPHYTYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn fsphytype(&self) -> FSPHYTYPE_R {
        FSPHYTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn numdeveps(&self) -> NUMDEVEPS_R {
        NUMDEVEPS_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn numhstchnl(&self) -> NUMHSTCHNL_R {
        NUMHSTCHNL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn periosupport(&self) -> PERIOSUPPORT_R {
        PERIOSUPPORT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dynfifosizing(&self) -> DYNFIFOSIZING_R {
        DYNFIFOSIZING_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn multiprocintrpt(&self) -> MULTIPROCINTRPT_R {
        MULTIPROCINTRPT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn nptxqdepth(&self) -> NPTXQDEPTH_R {
        NPTXQDEPTH_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ptxqdepth(&self) -> PTXQDEPTH_R {
        PTXQDEPTH_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:30"]
    #[inline(always)]
    pub fn tknqdepth(&self) -> TKNQDEPTH_R {
        TKNQDEPTH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn otg_enable_ic_usb(&self) -> OTG_ENABLE_IC_USB_R {
        OTG_ENABLE_IC_USB_R::new(((self.bits >> 31) & 1) != 0)
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
            .field("otg_enable_ic_usb", &self.otg_enable_ic_usb())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GHWCFG2_SPEC;
impl crate::RegisterSpec for GHWCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ghwcfg2::R`](R) reader structure"]
impl crate::Readable for GHWCFG2_SPEC {}
#[doc = "`reset()` method sets GHWCFG2 to value 0x224d_d930"]
impl crate::Resettable for GHWCFG2_SPEC {
    const RESET_VALUE: u32 = 0x224d_d930;
}
