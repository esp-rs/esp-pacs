#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATE_SPEC>;
#[doc = "Field `STATE` reader - Represents the working status of the AES accelerator. \\\\ In Typical AES working mode:\\\\ 0: IDLE\\\\ 1: WORK\\\\ 2: No effect\\\\ 3: No effect\\\\ In DMA-AES working mode:\\\\ 0: IDLE\\\\ 1: WORK\\\\ 2: DONE\\\\ 3: No effect\\\\"]
pub type STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the working status of the AES accelerator. \\\\ In Typical AES working mode:\\\\ 0: IDLE\\\\ 1: WORK\\\\ 2: No effect\\\\ 3: No effect\\\\ In DMA-AES working mode:\\\\ 0: IDLE\\\\ 1: WORK\\\\ 2: DONE\\\\ 3: No effect\\\\"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("state", &self.state())
            .finish()
    }
}
#[doc = "Operation status register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATE_SPEC {}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for STATE_SPEC {}
