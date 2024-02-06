#[doc = "Register `SHA256_BUSY` reader"]
pub type R = crate::R<SHA256_BUSY_SPEC>;
#[doc = "Field `SHA256_BUSY` reader - SHA-256 operation status: 1 if the SHA accelerator is processing data, 0 if it is idle."]
pub type SHA256_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SHA-256 operation status: 1 if the SHA accelerator is processing data, 0 if it is idle."]
    #[inline(always)]
    pub fn sha256_busy(&self) -> SHA256_BUSY_R {
        SHA256_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA256_BUSY")
            .field("sha256_busy", &format_args!("{}", self.sha256_busy().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHA256_BUSY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sha256_busy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA256_BUSY_SPEC;
impl crate::RegisterSpec for SHA256_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha256_busy::R`](R) reader structure"]
impl crate::Readable for SHA256_BUSY_SPEC {}
#[doc = "`reset()` method sets SHA256_BUSY to value 0"]
impl crate::Resettable for SHA256_BUSY_SPEC {
    const RESET_VALUE: u32 = 0;
}
