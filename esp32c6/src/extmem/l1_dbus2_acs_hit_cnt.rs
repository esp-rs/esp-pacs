#[doc = "Register `L1_DBUS2_ACS_HIT_CNT` reader"]
pub struct R(crate::R<L1_DBUS2_ACS_HIT_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_DBUS2_ACS_HIT_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_DBUS2_ACS_HIT_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_DBUS2_ACS_HIT_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_DBUS2_HIT_CNT` reader - The register records the number of hits when bus2 accesses L1-DCache."]
pub type L1_DBUS2_HIT_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of hits when bus2 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus2_hit_cnt(&self) -> L1_DBUS2_HIT_CNT_R {
        L1_DBUS2_HIT_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_DBUS2_ACS_HIT_CNT")
            .field(
                "l1_dbus2_hit_cnt",
                &format_args!("{}", self.l1_dbus2_hit_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_DBUS2_ACS_HIT_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L1-DCache bus2 Hit-Access Counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_dbus2_acs_hit_cnt](index.html) module"]
pub struct L1_DBUS2_ACS_HIT_CNT_SPEC;
impl crate::RegisterSpec for L1_DBUS2_ACS_HIT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_dbus2_acs_hit_cnt::R](R) reader structure"]
impl crate::Readable for L1_DBUS2_ACS_HIT_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_DBUS2_ACS_HIT_CNT to value 0"]
impl crate::Resettable for L1_DBUS2_ACS_HIT_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
