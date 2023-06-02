#[doc = "Register `IBUS0_ACS_MISS_CNT` reader"]
pub struct R(crate::R<IBUS0_ACS_MISS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBUS0_ACS_MISS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBUS0_ACS_MISS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBUS0_ACS_MISS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBUS0_ACS_MISS_CNT` reader - The bits are used to count the number of the cache miss caused by ibus0 access."]
pub type IBUS0_ACS_MISS_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to count the number of the cache miss caused by ibus0 access."]
    #[inline(always)]
    pub fn ibus0_acs_miss_cnt(&self) -> IBUS0_ACS_MISS_CNT_R {
        IBUS0_ACS_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS0_ACS_MISS_CNT")
            .field(
                "ibus0_acs_miss_cnt",
                &format_args!("{}", self.ibus0_acs_miss_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IBUS0_ACS_MISS_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibus0_acs_miss_cnt](index.html) module"]
pub struct IBUS0_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for IBUS0_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibus0_acs_miss_cnt::R](R) reader structure"]
impl crate::Readable for IBUS0_ACS_MISS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IBUS0_ACS_MISS_CNT to value 0"]
impl crate::Resettable for IBUS0_ACS_MISS_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
