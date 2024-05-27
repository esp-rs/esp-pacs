#[doc = "Register `LC_STATE0` reader"]
pub type R = crate::R<LC_STATE0_SPEC>;
#[doc = "Field `LC_STATE0` reader - "]
pub type LC_STATE0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lc_state0(&self) -> LC_STATE0_R {
        LC_STATE0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC_STATE0")
            .field("lc_state0", &self.lc_state0())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc_state0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_STATE0_SPEC;
impl crate::RegisterSpec for LC_STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_state0::R`](R) reader structure"]
impl crate::Readable for LC_STATE0_SPEC {}
#[doc = "`reset()` method sets LC_STATE0 to value 0"]
impl crate::Resettable for LC_STATE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
