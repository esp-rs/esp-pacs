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
#[doc = "Field `OVERFLOW_ST` reader - need_des"]
pub type OVERFLOW_ST_R = crate::BitReader<bool>;
#[doc = "Field `SOC_WAKEUP_INT_ST` reader - need_des"]
pub type SOC_WAKEUP_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn overflow_st(&self) -> OVERFLOW_ST_R {
        OVERFLOW_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_int_st(&self) -> SOC_WAKEUP_INT_ST_R {
        SOC_WAKEUP_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
