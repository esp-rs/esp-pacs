#[doc = "Register `BLK3_WDATA4` reader"]
pub struct R(crate::R<BLK3_WDATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK3_WDATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK3_WDATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK3_WDATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SECURE_VERSION` reader - "]
pub type SECURE_VERSION_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn secure_version(&self) -> SECURE_VERSION_R {
        SECURE_VERSION_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_WDATA4")
            .field(
                "secure_version",
                &format_args!("{}", self.secure_version().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK3_WDATA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk3_wdata4](index.html) module"]
pub struct BLK3_WDATA4_SPEC;
impl crate::RegisterSpec for BLK3_WDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk3_wdata4::R](R) reader structure"]
impl crate::Readable for BLK3_WDATA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK3_WDATA4 to value 0"]
impl crate::Resettable for BLK3_WDATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
