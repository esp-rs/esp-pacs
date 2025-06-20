#[doc = "Register `INFO_MEM[%s]` reader"]
pub type R = crate::R<INFO_MEM_SPEC>;
#[doc = "Register `INFO_MEM[%s]` writer"]
pub type W = crate::W<INFO_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The memory that stores HUK info.\n\nYou can [`read`](crate::Reg::read) this register and get [`info_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFO_MEM_SPEC;
impl crate::RegisterSpec for INFO_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`info_mem::R`](R) reader structure"]
impl crate::Readable for INFO_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`info_mem::W`](W) writer structure"]
impl crate::Writable for INFO_MEM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INFO_MEM[%s] to value 0"]
impl crate::Resettable for INFO_MEM_SPEC {
    const RESET_VALUE: u8 = 0;
}
