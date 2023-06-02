#[doc = "Register `GHWCFG3` reader"]
pub struct R(crate::R<GHWCFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GHWCFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GHWCFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GHWCFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `XFERSIZEWIDTH` reader - "]
pub type XFERSIZEWIDTH_R = crate::FieldReader;
#[doc = "Field `PKTSIZEWIDTH` reader - "]
pub type PKTSIZEWIDTH_R = crate::FieldReader;
#[doc = "Field `OTGEN` reader - "]
pub type OTGEN_R = crate::BitReader;
#[doc = "Field `I2CINTSEL` reader - "]
pub type I2CINTSEL_R = crate::BitReader;
#[doc = "Field `VNDCTLSUPT` reader - "]
pub type VNDCTLSUPT_R = crate::BitReader;
#[doc = "Field `OPTFEATURE` reader - "]
pub type OPTFEATURE_R = crate::BitReader;
#[doc = "Field `RSTTYPE` reader - "]
pub type RSTTYPE_R = crate::BitReader;
#[doc = "Field `ADPSUPPORT` reader - "]
pub type ADPSUPPORT_R = crate::BitReader;
#[doc = "Field `HSICMODE` reader - "]
pub type HSICMODE_R = crate::BitReader;
#[doc = "Field `BCSUPPORT` reader - "]
pub type BCSUPPORT_R = crate::BitReader;
#[doc = "Field `LPMMODE` reader - "]
pub type LPMMODE_R = crate::BitReader;
#[doc = "Field `DFIFODEPTH` reader - "]
pub type DFIFODEPTH_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn xfersizewidth(&self) -> XFERSIZEWIDTH_R {
        XFERSIZEWIDTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pktsizewidth(&self) -> PKTSIZEWIDTH_R {
        PKTSIZEWIDTH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn otgen(&self) -> OTGEN_R {
        OTGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2cintsel(&self) -> I2CINTSEL_R {
        I2CINTSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn vndctlsupt(&self) -> VNDCTLSUPT_R {
        VNDCTLSUPT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn optfeature(&self) -> OPTFEATURE_R {
        OPTFEATURE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rsttype(&self) -> RSTTYPE_R {
        RSTTYPE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpsupport(&self) -> ADPSUPPORT_R {
        ADPSUPPORT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hsicmode(&self) -> HSICMODE_R {
        HSICMODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn bcsupport(&self) -> BCSUPPORT_R {
        BCSUPPORT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lpmmode(&self) -> LPMMODE_R {
        LPMMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn dfifodepth(&self) -> DFIFODEPTH_R {
        DFIFODEPTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GHWCFG3")
            .field(
                "xfersizewidth",
                &format_args!("{}", self.xfersizewidth().bits()),
            )
            .field(
                "pktsizewidth",
                &format_args!("{}", self.pktsizewidth().bits()),
            )
            .field("otgen", &format_args!("{}", self.otgen().bit()))
            .field("i2cintsel", &format_args!("{}", self.i2cintsel().bit()))
            .field("vndctlsupt", &format_args!("{}", self.vndctlsupt().bit()))
            .field("optfeature", &format_args!("{}", self.optfeature().bit()))
            .field("rsttype", &format_args!("{}", self.rsttype().bit()))
            .field("adpsupport", &format_args!("{}", self.adpsupport().bit()))
            .field("hsicmode", &format_args!("{}", self.hsicmode().bit()))
            .field("bcsupport", &format_args!("{}", self.bcsupport().bit()))
            .field("lpmmode", &format_args!("{}", self.lpmmode().bit()))
            .field("dfifodepth", &format_args!("{}", self.dfifodepth().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GHWCFG3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ghwcfg3](index.html) module"]
pub struct GHWCFG3_SPEC;
impl crate::RegisterSpec for GHWCFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ghwcfg3::R](R) reader structure"]
impl crate::Readable for GHWCFG3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GHWCFG3 to value 0x0100_04b5"]
impl crate::Resettable for GHWCFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_04b5;
}
