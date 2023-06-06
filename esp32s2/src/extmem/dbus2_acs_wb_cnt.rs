#[doc = "Register `DBUS2_ACS_WB_CNT` reader"]
pub struct R(crate::R<DBUS2_ACS_WB_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBUS2_ACS_WB_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBUS2_ACS_WB_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBUS2_ACS_WB_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DBUS2_ACS_WB_CNT` reader - The bits are used to count the number of cache evictions by dbus2 access cache."]
pub type DBUS2_ACS_WB_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - The bits are used to count the number of cache evictions by dbus2 access cache."]
    #[inline(always)]
    pub fn dbus2_acs_wb_cnt(&self) -> DBUS2_ACS_WB_CNT_R {
        DBUS2_ACS_WB_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS2_ACS_WB_CNT")
            .field(
                "dbus2_acs_wb_cnt",
                &format_args!("{}", self.dbus2_acs_wb_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBUS2_ACS_WB_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbus2_acs_wb_cnt](index.html) module"]
pub struct DBUS2_ACS_WB_CNT_SPEC;
impl crate::RegisterSpec for DBUS2_ACS_WB_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbus2_acs_wb_cnt::R](R) reader structure"]
impl crate::Readable for DBUS2_ACS_WB_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBUS2_ACS_WB_CNT to value 0"]
impl crate::Resettable for DBUS2_ACS_WB_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
