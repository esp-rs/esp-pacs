#[doc = "Register `SHA1_BUSY` reader"]
pub type R = crate::R<SHA1_BUSY_SPEC>;
#[doc = "Field `SHA1_BUSY` reader - SHA-1 operation status: 1 if the SHA accelerator is processing data, 0 if it is idle."]
pub type SHA1_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SHA-1 operation status: 1 if the SHA accelerator is processing data, 0 if it is idle."]
    #[inline(always)]
    pub fn sha1_busy(&self) -> SHA1_BUSY_R {
        SHA1_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA1_BUSY")
            .field("sha1_busy", &self.sha1_busy())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sha1_busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA1_BUSY_SPEC;
impl crate::RegisterSpec for SHA1_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha1_busy::R`](R) reader structure"]
impl crate::Readable for SHA1_BUSY_SPEC {}
#[doc = "`reset()` method sets SHA1_BUSY to value 0"]
impl crate::Resettable for SHA1_BUSY_SPEC {}
