#[doc = "Register `DBUS1_ACS_CNT` reader"]
pub type R = crate::R<DBUS1_ACS_CNT_SPEC>;
#[doc = "Field `DBUS1_ACS_CNT` reader - The bits are used to count the number of dbus1 access dcache."]
pub type DBUS1_ACS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to count the number of dbus1 access dcache."]
    #[inline(always)]
    pub fn dbus1_acs_cnt(&self) -> DBUS1_ACS_CNT_R {
        DBUS1_ACS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS1_ACS_CNT")
            .field(
                "dbus1_acs_cnt",
                &format_args!("{}", self.dbus1_acs_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBUS1_ACS_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus1_acs_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS1_ACS_CNT_SPEC;
impl crate::RegisterSpec for DBUS1_ACS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus1_acs_cnt::R`](R) reader structure"]
impl crate::Readable for DBUS1_ACS_CNT_SPEC {}
#[doc = "`reset()` method sets DBUS1_ACS_CNT to value 0"]
impl crate::Resettable for DBUS1_ACS_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
