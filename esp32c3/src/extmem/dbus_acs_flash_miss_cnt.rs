#[doc = "Register `DBUS_ACS_FLASH_MISS_CNT` reader"]
pub type R = crate::R<DBUS_ACS_FLASH_MISS_CNT_SPEC>;
#[doc = "Field `DBUS_ACS_FLASH_MISS_CNT` reader - The bits are used to count the number of the cache miss caused by dbus access flash."]
pub type DBUS_ACS_FLASH_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to count the number of the cache miss caused by dbus access flash."]
    #[inline(always)]
    pub fn dbus_acs_flash_miss_cnt(&self) -> DBUS_ACS_FLASH_MISS_CNT_R {
        DBUS_ACS_FLASH_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS_ACS_FLASH_MISS_CNT")
            .field(
                "dbus_acs_flash_miss_cnt",
                &format_args!("{}", self.dbus_acs_flash_miss_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBUS_ACS_FLASH_MISS_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus_acs_flash_miss_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS_ACS_FLASH_MISS_CNT_SPEC;
impl crate::RegisterSpec for DBUS_ACS_FLASH_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus_acs_flash_miss_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS_ACS_FLASH_MISS_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS_ACS_FLASH_MISS_CNT to value 0"]
impl crate::Resettable for DBUS_ACS_FLASH_MISS_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
