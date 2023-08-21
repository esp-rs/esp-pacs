#[doc = "Register `TZ2_STATUS` reader"]
pub type R = crate::R<TZ2_STATUS_SPEC>;
#[doc = "Field `TZ2_CBC_ON` reader - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
pub type TZ2_CBC_ON_R = crate::BitReader;
#[doc = "Field `TZ2_OST_ON` reader - Set and reset by hardware. If set, an one-shot mode action is on going"]
pub type TZ2_OST_ON_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
    #[inline(always)]
    pub fn tz2_cbc_on(&self) -> TZ2_CBC_ON_R {
        TZ2_CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set and reset by hardware. If set, an one-shot mode action is on going"]
    #[inline(always)]
    pub fn tz2_ost_on(&self) -> TZ2_OST_ON_R {
        TZ2_OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZ2_STATUS")
            .field("tz2_cbc_on", &format_args!("{}", self.tz2_cbc_on().bit()))
            .field("tz2_ost_on", &format_args!("{}", self.tz2_ost_on().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TZ2_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Status of fault events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tz2_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZ2_STATUS_SPEC;
impl crate::RegisterSpec for TZ2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tz2_status::R`](R) reader structure"]
impl crate::Readable for TZ2_STATUS_SPEC {}
#[doc = "`reset()` method sets TZ2_STATUS to value 0"]
impl crate::Resettable for TZ2_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
