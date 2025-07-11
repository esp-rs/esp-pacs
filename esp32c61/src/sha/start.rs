#[doc = "Register `START` reader"]
pub type R = crate::R<START_SPEC>;
#[doc = "Field `START` reader - Write 1 to start Typical SHA calculation."]
pub type START_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 1:31 - Write 1 to start Typical SHA calculation."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("START")
            .field("start", &self.start())
            .finish()
    }
}
#[doc = "Starts the SHA accelerator for Typical SHA operation\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start::R`](R) reader structure"]
impl crate::Readable for START_SPEC {}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {}
