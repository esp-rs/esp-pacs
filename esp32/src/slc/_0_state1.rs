#[doc = "Register `_0_STATE1` reader"]
pub type R = crate::R<_0_STATE1_SPEC>;
#[doc = "Field `SLC0_STATE1` reader - "]
pub type SLC0_STATE1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_state1(&self) -> SLC0_STATE1_R {
        SLC0_STATE1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_STATE1")
            .field("slc0_state1", &self.slc0_state1())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_state1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0_STATE1_SPEC;
impl crate::RegisterSpec for _0_STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_state1::R`](R) reader structure"]
impl crate::Readable for _0_STATE1_SPEC {}
#[doc = "`reset()` method sets _0_STATE1 to value 0"]
impl crate::Resettable for _0_STATE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
