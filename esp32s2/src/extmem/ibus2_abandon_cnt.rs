#[doc = "Register `IBUS2_ABANDON_CNT` reader"]
pub struct R(crate::R<IBUS2_ABANDON_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBUS2_ABANDON_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBUS2_ABANDON_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBUS2_ABANDON_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBUS2_ABANDON_CNT` reader - The bits are used to count the number of the abandoned ibus2 access."]
pub type IBUS2_ABANDON_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of the abandoned ibus2 access."]
    #[inline(always)]
    pub fn ibus2_abandon_cnt(&self) -> IBUS2_ABANDON_CNT_R {
        IBUS2_ABANDON_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS2_ABANDON_CNT")
            .field(
                "ibus2_abandon_cnt",
                &format_args!("{}", self.ibus2_abandon_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IBUS2_ABANDON_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibus2_abandon_cnt](index.html) module"]
pub struct IBUS2_ABANDON_CNT_SPEC;
impl crate::RegisterSpec for IBUS2_ABANDON_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibus2_abandon_cnt::R](R) reader structure"]
impl crate::Readable for IBUS2_ABANDON_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IBUS2_ABANDON_CNT to value 0"]
impl crate::Resettable for IBUS2_ABANDON_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
