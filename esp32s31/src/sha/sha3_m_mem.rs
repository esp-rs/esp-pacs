#[doc = "Register `SHA3_M_MEM[%s]` reader"]
pub type R = crate::R<SHA3_M_MEM_SPEC>;
#[doc = "Register `SHA3_M_MEM[%s]` writer"]
pub type W = crate::W<SHA3_M_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SHA3, SHAKE M memory which contains message.\n\nYou can [`read`](crate::Reg::read) this register and get [`sha3_m_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha3_m_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHA3_M_MEM_SPEC;
impl crate::RegisterSpec for SHA3_M_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha3_m_mem::R`](R) reader structure"]
impl crate::Readable for SHA3_M_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sha3_m_mem::W`](W) writer structure"]
impl crate::Writable for SHA3_M_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA3_M_MEM[%s] to value 0"]
impl crate::Resettable for SHA3_M_MEM_SPEC {}
