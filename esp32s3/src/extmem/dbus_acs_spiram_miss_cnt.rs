#[doc = "Register `DBUS_ACS_SPIRAM_MISS_CNT` reader"]
pub struct R(crate::R<DBUS_ACS_SPIRAM_MISS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBUS_ACS_SPIRAM_MISS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBUS_ACS_SPIRAM_MISS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBUS_ACS_SPIRAM_MISS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DBUS_ACS_SPIRAM_MISS_CNT` reader - The bits are used to count the number of the cache miss caused by dbus access spiram."]
pub type DBUS_ACS_SPIRAM_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to count the number of the cache miss caused by dbus access spiram."]
    #[inline(always)]
    pub fn dbus_acs_spiram_miss_cnt(&self) -> DBUS_ACS_SPIRAM_MISS_CNT_R {
        DBUS_ACS_SPIRAM_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS_ACS_SPIRAM_MISS_CNT")
            .field(
                "dbus_acs_spiram_miss_cnt",
                &format_args!("{}", self.dbus_acs_spiram_miss_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBUS_ACS_SPIRAM_MISS_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbus_acs_spiram_miss_cnt](index.html) module"]
pub struct DBUS_ACS_SPIRAM_MISS_CNT_SPEC;
impl crate::RegisterSpec for DBUS_ACS_SPIRAM_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbus_acs_spiram_miss_cnt::R](R) reader structure"]
impl crate::Readable for DBUS_ACS_SPIRAM_MISS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBUS_ACS_SPIRAM_MISS_CNT to value 0"]
impl crate::Resettable for DBUS_ACS_SPIRAM_MISS_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
