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
pub type INEPINT0_R = crate::BitReader<bool>;
#[doc = "Field `INEPINT1` reader - "]
pub type INEPINT1_R = crate::BitReader<bool>;
#[doc = "Field `INEPINT2` reader - "]
pub type INEPINT2_R = crate::BitReader<bool>;
#[doc = "Field `INEPINT3` reader - "]
pub type INEPINT3_R = crate::BitReader<bool>;
#[doc = "Field `INEPINT4` reader - "]
pub type INEPINT4_R = crate::BitReader<bool>;
#[doc = "Field `INEPINT5` reader - "]
pub type INEPINT5_R = crate::BitReader<bool>;
#[doc = "Field `INEPINT6` reader - "]
pub type INEPINT6_R = crate::BitReader<bool>;
#[doc = "Field `OUTEPINT0` reader - "]
pub type OUTEPINT0_R = crate::BitReader<bool>;
#[doc = "Field `OUTEPINT1` reader - "]
pub type OUTEPINT1_R = crate::BitReader<bool>;
#[doc = "Field `OUTEPINT2` reader - "]
pub type OUTEPINT2_R = crate::BitReader<bool>;
#[doc = "Field `OUTEPINT3` reader - "]
pub type OUTEPINT3_R = crate::BitReader<bool>;
#[doc = "Field `OUTEPINT4` reader - "]
pub type OUTEPINT4_R = crate::BitReader<bool>;
#[doc = "Field `OUTEPINT5` reader - "]
pub type OUTEPINT5_R = crate::BitReader<bool>;
#[doc = "Field `OUTEPINT6` reader - "]
pub type OUTEPINT6_R = crate::BitReader<bool>;
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
