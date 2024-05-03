#[doc = "Register `ACCESS_CHECK` reader"]
pub type R = crate::R<ACCESS_CHECK_SPEC>;
#[doc = "Field `PRO` reader - "]
pub type PRO_R = crate::BitReader;
#[doc = "Field `APP` reader - "]
pub type APP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro(&self) -> PRO_R {
        PRO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app(&self) -> APP_R {
        APP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACCESS_CHECK")
            .field("pro", &self.pro().bit())
            .field("app", &self.app().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ACCESS_CHECK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`access_check::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACCESS_CHECK_SPEC;
impl crate::RegisterSpec for ACCESS_CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`access_check::R`](R) reader structure"]
impl crate::Readable for ACCESS_CHECK_SPEC {}
#[doc = "`reset()` method sets ACCESS_CHECK to value 0"]
impl crate::Resettable for ACCESS_CHECK_SPEC {
    const RESET_VALUE: u32 = 0;
}
