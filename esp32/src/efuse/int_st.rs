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
#[doc = "Field `READ_DONE_INT_ST` reader - read done interrupt status"]
pub type READ_DONE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `PGM_DONE_INT_ST` reader - program done interrupt status"]
pub type PGM_DONE_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - read done interrupt status"]
    #[inline(always)]
    pub fn read_done_int_st(&self) -> READ_DONE_INT_ST_R {
        READ_DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - program done interrupt status"]
    #[inline(always)]
    pub fn pgm_done_int_st(&self) -> PGM_DONE_INT_ST_R {
        PGM_DONE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
