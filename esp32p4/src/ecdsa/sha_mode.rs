#[doc = "Register `SHA_MODE` reader"]
pub type R = crate::R<SHA_MODE_SPEC>;
#[doc = "Register `SHA_MODE` writer"]
pub type W = crate::W<SHA_MODE_SPEC>;
#[doc = "Field `SHA_MODE` reader - The work mode bits of SHA Calculator in ECDSA Accelerator. 1: SHA-224. 2: SHA-256. Others: invalid."]
pub type SHA_MODE_R = crate::FieldReader;
#[doc = "Field `SHA_MODE` writer - The work mode bits of SHA Calculator in ECDSA Accelerator. 1: SHA-224. 2: SHA-256. Others: invalid."]
pub type SHA_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The work mode bits of SHA Calculator in ECDSA Accelerator. 1: SHA-224. 2: SHA-256. Others: invalid."]
    #[inline(always)]
    pub fn sha_mode(&self) -> SHA_MODE_R {
        SHA_MODE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA_MODE")
            .field("sha_mode", &self.sha_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - The work mode bits of SHA Calculator in ECDSA Accelerator. 1: SHA-224. 2: SHA-256. Others: invalid."]
    #[inline(always)]
    #[must_use]
    pub fn sha_mode(&mut self) -> SHA_MODE_W<SHA_MODE_SPEC> {
        SHA_MODE_W::new(self, 0)
    }
}
#[doc = "ECDSA control SHA register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sha_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA_MODE_SPEC;
impl crate::RegisterSpec for SHA_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_mode::R`](R) reader structure"]
impl crate::Readable for SHA_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sha_mode::W`](W) writer structure"]
impl crate::Writable for SHA_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHA_MODE to value 0"]
impl crate::Resettable for SHA_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
