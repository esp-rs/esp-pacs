#[doc = "Register `IC_PRELOAD_CNT` reader"]
pub struct R(crate::R<IC_PRELOAD_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_PRELOAD_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_PRELOAD_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_PRELOAD_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IC_PRELOAD_CNT` reader - The bits are used to count the number of issued pre-load which include manual pre-load and conditional pre-load."]
pub type IC_PRELOAD_CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of issued pre-load which include manual pre-load and conditional pre-load."]
    #[inline(always)]
    pub fn ic_preload_cnt(&self) -> IC_PRELOAD_CNT_R {
        IC_PRELOAD_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_PRELOAD_CNT")
            .field(
                "ic_preload_cnt",
                &format_args!("{}", self.ic_preload_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IC_PRELOAD_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic_preload_cnt](index.html) module"]
pub struct IC_PRELOAD_CNT_SPEC;
impl crate::RegisterSpec for IC_PRELOAD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_preload_cnt::R](R) reader structure"]
impl crate::Readable for IC_PRELOAD_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_PRELOAD_CNT to value 0"]
impl crate::Resettable for IC_PRELOAD_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
