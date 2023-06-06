#[doc = "Register `IBUS_ACS_CNT` reader"]
pub struct R(crate::R<IBUS_ACS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBUS_ACS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBUS_ACS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBUS_ACS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBUS_ACS_CNT` reader - The bits are used to count the number of ibus access flash through icache."]
pub type IBUS_ACS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to count the number of ibus access flash through icache."]
    #[inline(always)]
    pub fn ibus_acs_cnt(&self) -> IBUS_ACS_CNT_R {
        IBUS_ACS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS_ACS_CNT")
            .field(
                "ibus_acs_cnt",
                &format_args!("{}", self.ibus_acs_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IBUS_ACS_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibus_acs_cnt](index.html) module"]
pub struct IBUS_ACS_CNT_SPEC;
impl crate::RegisterSpec for IBUS_ACS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibus_acs_cnt::R](R) reader structure"]
impl crate::Readable for IBUS_ACS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IBUS_ACS_CNT to value 0"]
impl crate::Resettable for IBUS_ACS_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
