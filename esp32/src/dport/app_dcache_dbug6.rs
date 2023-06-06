#[doc = "Register `APP_DCACHE_DBUG6` reader"]
pub struct R(crate::R<APP_DCACHE_DBUG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_DCACHE_DBUG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_DCACHE_DBUG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_DCACHE_DBUG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APP_IRAM0ADDR_IA` reader - "]
pub type APP_IRAM0ADDR_IA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn app_iram0addr_ia(&self) -> APP_IRAM0ADDR_IA_R {
        APP_IRAM0ADDR_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG6")
            .field(
                "app_iram0addr_ia",
                &format_args!("{}", self.app_iram0addr_ia().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_DCACHE_DBUG6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_dcache_dbug6](index.html) module"]
pub struct APP_DCACHE_DBUG6_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_dcache_dbug6::R](R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APP_DCACHE_DBUG6 to value 0"]
impl crate::Resettable for APP_DCACHE_DBUG6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
