#[doc = "Register `FH_STATUS` reader"]
pub type R = crate::R<FH_STATUS_SPEC>;
#[doc = "Field `CBC_ON` reader - Represents whether or not an cycle-by-cycle mode action is on going.\\\\0:No action\\\\1: On going"]
pub type CBC_ON_R = crate::BitReader;
#[doc = "Field `OST_ON` reader - Represents whether or not an one-shot mode action is on going.\\\\0:No action\\\\1: On going"]
pub type OST_ON_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not an cycle-by-cycle mode action is on going.\\\\0:No action\\\\1: On going"]
    #[inline(always)]
    pub fn cbc_on(&self) -> CBC_ON_R {
        CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether or not an one-shot mode action is on going.\\\\0:No action\\\\1: On going"]
    #[inline(always)]
    pub fn ost_on(&self) -> OST_ON_R {
        OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH_STATUS")
            .field("cbc_on", &format_args!("{}", self.cbc_on().bit()))
            .field("ost_on", &format_args!("{}", self.ost_on().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FH_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Fault events status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH_STATUS_SPEC;
impl crate::RegisterSpec for FH_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh_status::R`](R) reader structure"]
impl crate::Readable for FH_STATUS_SPEC {}
#[doc = "`reset()` method sets FH_STATUS to value 0"]
impl crate::Resettable for FH_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
