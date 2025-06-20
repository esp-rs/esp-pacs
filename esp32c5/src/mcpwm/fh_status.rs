#[doc = "Register `FH%s_STATUS` reader"]
pub type R = crate::R<FH_STATUS_SPEC>;
#[doc = "Field `TZ_CBC_ON` reader - Represents whether or not an cycle-by-cycle mode action is on going.\\\\0:No action\\\\1: On going"]
pub type TZ_CBC_ON_R = crate::BitReader;
#[doc = "Field `TZ_OST_ON` reader - Represents whether or not an one-shot mode action is on going.\\\\0:No action\\\\1: On going"]
pub type TZ_OST_ON_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not an cycle-by-cycle mode action is on going.\\\\0:No action\\\\1: On going"]
    #[inline(always)]
    pub fn tz_cbc_on(&self) -> TZ_CBC_ON_R {
        TZ_CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether or not an one-shot mode action is on going.\\\\0:No action\\\\1: On going"]
    #[inline(always)]
    pub fn tz_ost_on(&self) -> TZ_OST_ON_R {
        TZ_OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH_STATUS")
            .field("tz_cbc_on", &self.tz_cbc_on())
            .field("tz_ost_on", &self.tz_ost_on())
            .finish()
    }
}
#[doc = "Fault events status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fh_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH_STATUS_SPEC;
impl crate::RegisterSpec for FH_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh_status::R`](R) reader structure"]
impl crate::Readable for FH_STATUS_SPEC {}
#[doc = "`reset()` method sets FH%s_STATUS to value 0"]
impl crate::Resettable for FH_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
