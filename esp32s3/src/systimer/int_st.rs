#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TARGET0_INT_ST` reader - interupt0 status"]
pub type TARGET0_INT_ST_R = crate::BitReader;
#[doc = "Field `TARGET1_INT_ST` reader - interupt1 status"]
pub type TARGET1_INT_ST_R = crate::BitReader;
#[doc = "Field `TARGET2_INT_ST` reader - interupt2 status"]
pub type TARGET2_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - interupt0 status"]
    #[inline(always)]
    pub fn target0_int_st(&self) -> TARGET0_INT_ST_R {
        TARGET0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interupt1 status"]
    #[inline(always)]
    pub fn target1_int_st(&self) -> TARGET1_INT_ST_R {
        TARGET1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interupt2 status"]
    #[inline(always)]
    pub fn target2_int_st(&self) -> TARGET2_INT_ST_R {
        TARGET2_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "target0_int_st",
                &format_args!("{}", self.target0_int_st().bit()),
            )
            .field(
                "target1_int_st",
                &format_args!("{}", self.target1_int_st().bit()),
            )
            .field(
                "target2_int_st",
                &format_args!("{}", self.target2_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "systimer interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
