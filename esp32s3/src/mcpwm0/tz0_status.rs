#[doc = "Register `TZ0_STATUS` reader"]
pub type R = crate::R<TZ0_STATUS_SPEC>;
#[doc = "Field `TZ0_CBC_ON` reader - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
pub type TZ0_CBC_ON_R = crate::BitReader;
#[doc = "Field `TZ0_OST_ON` reader - Set and reset by hardware. If set, an one-shot mode action is on going"]
pub type TZ0_OST_ON_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
    #[inline(always)]
    pub fn tz0_cbc_on(&self) -> TZ0_CBC_ON_R {
        TZ0_CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set and reset by hardware. If set, an one-shot mode action is on going"]
    #[inline(always)]
    pub fn tz0_ost_on(&self) -> TZ0_OST_ON_R {
        TZ0_OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZ0_STATUS")
            .field("tz0_cbc_on", &format_args!("{}", self.tz0_cbc_on().bit()))
            .field("tz0_ost_on", &format_args!("{}", self.tz0_ost_on().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TZ0_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Status of fault events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tz0_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZ0_STATUS_SPEC;
impl crate::RegisterSpec for TZ0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tz0_status::R`](R) reader structure"]
impl crate::Readable for TZ0_STATUS_SPEC {}
#[doc = "`reset()` method sets TZ0_STATUS to value 0"]
impl crate::Resettable for TZ0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
