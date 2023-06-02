#[doc = "Register `DAINT` reader"]
pub struct R(crate::R<DAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INEPINT0` reader - "]
pub type INEPINT0_R = crate::BitReader;
#[doc = "Field `INEPINT1` reader - "]
pub type INEPINT1_R = crate::BitReader;
#[doc = "Field `INEPINT2` reader - "]
pub type INEPINT2_R = crate::BitReader;
#[doc = "Field `INEPINT3` reader - "]
pub type INEPINT3_R = crate::BitReader;
#[doc = "Field `INEPINT4` reader - "]
pub type INEPINT4_R = crate::BitReader;
#[doc = "Field `INEPINT5` reader - "]
pub type INEPINT5_R = crate::BitReader;
#[doc = "Field `INEPINT6` reader - "]
pub type INEPINT6_R = crate::BitReader;
#[doc = "Field `OUTEPINT0` reader - "]
pub type OUTEPINT0_R = crate::BitReader;
#[doc = "Field `OUTEPINT1` reader - "]
pub type OUTEPINT1_R = crate::BitReader;
#[doc = "Field `OUTEPINT2` reader - "]
pub type OUTEPINT2_R = crate::BitReader;
#[doc = "Field `OUTEPINT3` reader - "]
pub type OUTEPINT3_R = crate::BitReader;
#[doc = "Field `OUTEPINT4` reader - "]
pub type OUTEPINT4_R = crate::BitReader;
#[doc = "Field `OUTEPINT5` reader - "]
pub type OUTEPINT5_R = crate::BitReader;
#[doc = "Field `OUTEPINT6` reader - "]
pub type OUTEPINT6_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn inepint0(&self) -> INEPINT0_R {
        INEPINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn inepint1(&self) -> INEPINT1_R {
        INEPINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn inepint2(&self) -> INEPINT2_R {
        INEPINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn inepint3(&self) -> INEPINT3_R {
        INEPINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inepint4(&self) -> INEPINT4_R {
        INEPINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inepint5(&self) -> INEPINT5_R {
        INEPINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inepint6(&self) -> INEPINT6_R {
        INEPINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn outepint0(&self) -> OUTEPINT0_R {
        OUTEPINT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn outepint1(&self) -> OUTEPINT1_R {
        OUTEPINT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn outepint2(&self) -> OUTEPINT2_R {
        OUTEPINT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn outepint3(&self) -> OUTEPINT3_R {
        OUTEPINT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn outepint4(&self) -> OUTEPINT4_R {
        OUTEPINT4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn outepint5(&self) -> OUTEPINT5_R {
        OUTEPINT5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn outepint6(&self) -> OUTEPINT6_R {
        OUTEPINT6_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAINT")
            .field("inepint0", &format_args!("{}", self.inepint0().bit()))
            .field("inepint1", &format_args!("{}", self.inepint1().bit()))
            .field("inepint2", &format_args!("{}", self.inepint2().bit()))
            .field("inepint3", &format_args!("{}", self.inepint3().bit()))
            .field("inepint4", &format_args!("{}", self.inepint4().bit()))
            .field("inepint5", &format_args!("{}", self.inepint5().bit()))
            .field("inepint6", &format_args!("{}", self.inepint6().bit()))
            .field("outepint0", &format_args!("{}", self.outepint0().bit()))
            .field("outepint1", &format_args!("{}", self.outepint1().bit()))
            .field("outepint2", &format_args!("{}", self.outepint2().bit()))
            .field("outepint3", &format_args!("{}", self.outepint3().bit()))
            .field("outepint4", &format_args!("{}", self.outepint4().bit()))
            .field("outepint5", &format_args!("{}", self.outepint5().bit()))
            .field("outepint6", &format_args!("{}", self.outepint6().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DAINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daint](index.html) module"]
pub struct DAINT_SPEC;
impl crate::RegisterSpec for DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daint::R](R) reader structure"]
impl crate::Readable for DAINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
