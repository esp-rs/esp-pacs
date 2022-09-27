#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TARGET0_INT_RAW` reader - interupt0 raw"]
pub type TARGET0_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TARGET1_INT_RAW` reader - interupt1 raw"]
pub type TARGET1_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TARGET2_INT_RAW` reader - interupt2 raw"]
pub type TARGET2_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - interupt0 raw"]
    #[inline(always)]
    pub fn target0_int_raw(&self) -> TARGET0_INT_RAW_R {
        TARGET0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interupt1 raw"]
    #[inline(always)]
    pub fn target1_int_raw(&self) -> TARGET1_INT_RAW_R {
        TARGET1_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interupt2 raw"]
    #[inline(always)]
    pub fn target2_int_raw(&self) -> TARGET2_INT_RAW_R {
        TARGET2_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "SYSTIMER_INT_RAW.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
