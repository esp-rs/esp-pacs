#[doc = "Register `GHWCFG4` reader"]
pub struct R(crate::R<GHWCFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GHWCFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GHWCFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GHWCFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `G_NUMDEVPERIOEPS` reader - "]
pub type G_NUMDEVPERIOEPS_R = crate::FieldReader;
#[doc = "Field `G_PARTIALPWRDN` reader - "]
pub type G_PARTIALPWRDN_R = crate::BitReader;
#[doc = "Field `G_AHBFREQ` reader - "]
pub type G_AHBFREQ_R = crate::BitReader;
#[doc = "Field `G_HIBERNATION` reader - "]
pub type G_HIBERNATION_R = crate::BitReader;
#[doc = "Field `G_EXTENDEDHIBERNATION` reader - "]
pub type G_EXTENDEDHIBERNATION_R = crate::BitReader;
#[doc = "Field `G_ACGSUPT` reader - "]
pub type G_ACGSUPT_R = crate::BitReader;
#[doc = "Field `G_ENHANCEDLPMSUPT` reader - "]
pub type G_ENHANCEDLPMSUPT_R = crate::BitReader;
#[doc = "Field `G_PHYDATAWIDTH` reader - "]
pub type G_PHYDATAWIDTH_R = crate::FieldReader;
#[doc = "Field `G_NUMCTLEPS` reader - "]
pub type G_NUMCTLEPS_R = crate::FieldReader;
#[doc = "Field `G_IDDQFLTR` reader - "]
pub type G_IDDQFLTR_R = crate::BitReader;
#[doc = "Field `G_VBUSVALIDFLTR` reader - "]
pub type G_VBUSVALIDFLTR_R = crate::BitReader;
#[doc = "Field `G_AVALIDFLTR` reader - "]
pub type G_AVALIDFLTR_R = crate::BitReader;
#[doc = "Field `G_BVALIDFLTR` reader - "]
pub type G_BVALIDFLTR_R = crate::BitReader;
#[doc = "Field `G_SESSENDFLTR` reader - "]
pub type G_SESSENDFLTR_R = crate::BitReader;
#[doc = "Field `G_DEDFIFOMODE` reader - "]
pub type G_DEDFIFOMODE_R = crate::BitReader;
#[doc = "Field `G_INEPS` reader - "]
pub type G_INEPS_R = crate::FieldReader;
#[doc = "Field `G_DESCDMAENABLED` reader - "]
pub type G_DESCDMAENABLED_R = crate::BitReader;
#[doc = "Field `G_DESCDMA` reader - "]
pub type G_DESCDMA_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn g_numdevperioeps(&self) -> G_NUMDEVPERIOEPS_R {
        G_NUMDEVPERIOEPS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn g_partialpwrdn(&self) -> G_PARTIALPWRDN_R {
        G_PARTIALPWRDN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn g_ahbfreq(&self) -> G_AHBFREQ_R {
        G_AHBFREQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn g_hibernation(&self) -> G_HIBERNATION_R {
        G_HIBERNATION_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn g_extendedhibernation(&self) -> G_EXTENDEDHIBERNATION_R {
        G_EXTENDEDHIBERNATION_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn g_acgsupt(&self) -> G_ACGSUPT_R {
        G_ACGSUPT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn g_enhancedlpmsupt(&self) -> G_ENHANCEDLPMSUPT_R {
        G_ENHANCEDLPMSUPT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn g_phydatawidth(&self) -> G_PHYDATAWIDTH_R {
        G_PHYDATAWIDTH_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn g_numctleps(&self) -> G_NUMCTLEPS_R {
        G_NUMCTLEPS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn g_iddqfltr(&self) -> G_IDDQFLTR_R {
        G_IDDQFLTR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn g_vbusvalidfltr(&self) -> G_VBUSVALIDFLTR_R {
        G_VBUSVALIDFLTR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn g_avalidfltr(&self) -> G_AVALIDFLTR_R {
        G_AVALIDFLTR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn g_bvalidfltr(&self) -> G_BVALIDFLTR_R {
        G_BVALIDFLTR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn g_sessendfltr(&self) -> G_SESSENDFLTR_R {
        G_SESSENDFLTR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn g_dedfifomode(&self) -> G_DEDFIFOMODE_R {
        G_DEDFIFOMODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn g_ineps(&self) -> G_INEPS_R {
        G_INEPS_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn g_descdmaenabled(&self) -> G_DESCDMAENABLED_R {
        G_DESCDMAENABLED_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn g_descdma(&self) -> G_DESCDMA_R {
        G_DESCDMA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GHWCFG4")
            .field(
                "g_numdevperioeps",
                &format_args!("{}", self.g_numdevperioeps().bits()),
            )
            .field(
                "g_partialpwrdn",
                &format_args!("{}", self.g_partialpwrdn().bit()),
            )
            .field("g_ahbfreq", &format_args!("{}", self.g_ahbfreq().bit()))
            .field(
                "g_hibernation",
                &format_args!("{}", self.g_hibernation().bit()),
            )
            .field(
                "g_extendedhibernation",
                &format_args!("{}", self.g_extendedhibernation().bit()),
            )
            .field("g_acgsupt", &format_args!("{}", self.g_acgsupt().bit()))
            .field(
                "g_enhancedlpmsupt",
                &format_args!("{}", self.g_enhancedlpmsupt().bit()),
            )
            .field(
                "g_phydatawidth",
                &format_args!("{}", self.g_phydatawidth().bits()),
            )
            .field(
                "g_numctleps",
                &format_args!("{}", self.g_numctleps().bits()),
            )
            .field("g_iddqfltr", &format_args!("{}", self.g_iddqfltr().bit()))
            .field(
                "g_vbusvalidfltr",
                &format_args!("{}", self.g_vbusvalidfltr().bit()),
            )
            .field(
                "g_avalidfltr",
                &format_args!("{}", self.g_avalidfltr().bit()),
            )
            .field(
                "g_bvalidfltr",
                &format_args!("{}", self.g_bvalidfltr().bit()),
            )
            .field(
                "g_sessendfltr",
                &format_args!("{}", self.g_sessendfltr().bit()),
            )
            .field(
                "g_dedfifomode",
                &format_args!("{}", self.g_dedfifomode().bit()),
            )
            .field("g_ineps", &format_args!("{}", self.g_ineps().bits()))
            .field(
                "g_descdmaenabled",
                &format_args!("{}", self.g_descdmaenabled().bit()),
            )
            .field("g_descdma", &format_args!("{}", self.g_descdma().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GHWCFG4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ghwcfg4](index.html) module"]
pub struct GHWCFG4_SPEC;
impl crate::RegisterSpec for GHWCFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ghwcfg4::R](R) reader structure"]
impl crate::Readable for GHWCFG4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GHWCFG4 to value 0xd3f0_a030"]
impl crate::Resettable for GHWCFG4_SPEC {
    const RESET_VALUE: Self::Ux = 0xd3f0_a030;
}
