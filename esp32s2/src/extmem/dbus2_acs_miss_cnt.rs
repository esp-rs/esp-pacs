#[doc = "Register `DBUS2_ACS_MISS_CNT` reader"]
pub type R = crate::R<DBUS2_ACS_MISS_CNT_SPEC>;
#[doc = "Field `DBUS2_ACS_MISS_CNT` reader - The bits are used to count the number of the cache miss caused by dbus2 access."]
pub type DBUS2_ACS_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to count the number of the cache miss caused by dbus2 access."]
    #[inline(always)]
    pub fn dbus2_acs_miss_cnt(&self) -> DBUS2_ACS_MISS_CNT_R {
        DBUS2_ACS_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS2_ACS_MISS_CNT")
            .field("dbus2_acs_miss_cnt", &self.dbus2_acs_miss_cnt())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`dbus2_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS2_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for DBUS2_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus2_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS2_ACS_MISS_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS2_ACS_MISS_CNT to value 0"]
impl crate::Resettable for DBUS2_ACS_MISS_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
