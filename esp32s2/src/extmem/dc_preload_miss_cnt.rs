#[doc = "Register `DC_PRELOAD_MISS_CNT` reader"]
pub struct R(crate::R<DC_PRELOAD_MISS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DC_PRELOAD_MISS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DC_PRELOAD_MISS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DC_PRELOAD_MISS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DC_PRELOAD_MISS_CNT` reader - The bits are used to count the number of missed pre-load which include manual pre-load and conditional pre-load."]
pub type DC_PRELOAD_MISS_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of missed pre-load which include manual pre-load and conditional pre-load."]
    #[inline(always)]
    pub fn dc_preload_miss_cnt(&self) -> DC_PRELOAD_MISS_CNT_R {
        DC_PRELOAD_MISS_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DC_PRELOAD_MISS_CNT")
            .field(
                "dc_preload_miss_cnt",
                &format_args!("{}", self.dc_preload_miss_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DC_PRELOAD_MISS_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_preload_miss_cnt](index.html) module"]
pub struct DC_PRELOAD_MISS_CNT_SPEC;
impl crate::RegisterSpec for DC_PRELOAD_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dc_preload_miss_cnt::R](R) reader structure"]
impl crate::Readable for DC_PRELOAD_MISS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DC_PRELOAD_MISS_CNT to value 0"]
impl crate::Resettable for DC_PRELOAD_MISS_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
