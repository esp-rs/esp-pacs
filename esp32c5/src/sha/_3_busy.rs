#[doc = "Register `_3_BUSY` reader"]
pub type R = crate::R<_3_BUSY_SPEC>;
#[doc = "Field `_3_BUSY` reader - Sha3 busy state. 1'b0: idle. 1'b1: busy."]
pub type _3_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sha3 busy state. 1'b0: idle. 1'b1: busy."]
    #[inline(always)]
    pub fn _3_busy(&self) -> _3_BUSY_R {
        _3_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_3_BUSY")
            .field("_3_busy", &self._3_busy())
            .finish()
    }
}
#[doc = "Busy register.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_BUSY_SPEC;
impl crate::RegisterSpec for _3_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_busy::R`](R) reader structure"]
impl crate::Readable for _3_BUSY_SPEC {}
#[doc = "`reset()` method sets _3_BUSY to value 0"]
impl crate::Resettable for _3_BUSY_SPEC {
    const RESET_VALUE: u32 = 0;
}
