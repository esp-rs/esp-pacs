#[doc = "Register `CERT_HASH_MEM[%s]` reader"]
pub type R = crate::R<CERT_HASH_MEM_SPEC>;
#[doc = "Register `CERT_HASH_MEM[%s]` writer"]
pub type W = crate::W<CERT_HASH_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RMA cert hash memory.\n\nYou can [`read`](crate::Reg::read) this register and get [`cert_hash_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cert_hash_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CERT_HASH_MEM_SPEC;
impl crate::RegisterSpec for CERT_HASH_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cert_hash_mem::R`](R) reader structure"]
impl crate::Readable for CERT_HASH_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cert_hash_mem::W`](W) writer structure"]
impl crate::Writable for CERT_HASH_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CERT_HASH_MEM[%s] to value 0"]
impl crate::Resettable for CERT_HASH_MEM_SPEC {}
