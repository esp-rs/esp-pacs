#[doc = "Register `PUBLIC_INFO_MEM[%s]` reader"]
pub type R = crate::R<PUBLIC_INFO_MEM_SPEC>;
#[doc = "Register `PUBLIC_INFO_MEM[%s]` writer"]
pub type W = crate::W<PUBLIC_INFO_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The memory that stores public key info.\n\nYou can [`read`](crate::Reg::read) this register and get [`public_info_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`public_info_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUBLIC_INFO_MEM_SPEC;
impl crate::RegisterSpec for PUBLIC_INFO_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`public_info_mem::R`](R) reader structure"]
impl crate::Readable for PUBLIC_INFO_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`public_info_mem::W`](W) writer structure"]
impl crate::Writable for PUBLIC_INFO_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUBLIC_INFO_MEM[%s] to value 0"]
impl crate::Resettable for PUBLIC_INFO_MEM_SPEC {}
