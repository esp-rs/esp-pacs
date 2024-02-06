#[doc = "Register `LOCK_STATUS` reader"]
pub type R = crate::R<LOCK_STATUS_SPEC>;
#[doc = "Field `LOCK_STATUS` reader - read hareware lock status for debug"]
pub type LOCK_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - read hareware lock status for debug"]
    #[inline(always)]
    pub fn lock_status(&self) -> LOCK_STATUS_R {
        LOCK_STATUS_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK_STATUS")
            .field(
                "lock_status",
                &format_args!("{}", self.lock_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOCK_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "lock status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCK_STATUS_SPEC;
impl crate::RegisterSpec for LOCK_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock_status::R`](R) reader structure"]
impl crate::Readable for LOCK_STATUS_SPEC {}
#[doc = "`reset()` method sets LOCK_STATUS to value 0"]
impl crate::Resettable for LOCK_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
