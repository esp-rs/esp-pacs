#[doc = "Register `BOX_MEM[%s]` reader"]
pub type R = crate::R<BOX_MEM_SPEC>;
#[doc = "Register `BOX_MEM[%s]` writer"]
pub type W = crate::W<BOX_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BOX_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "memory that stores BOX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`box_mem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`box_mem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOX_MEM_SPEC;
impl crate::RegisterSpec for BOX_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`box_mem::R`](R) reader structure"]
impl crate::Readable for BOX_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`box_mem::W`](W) writer structure"]
impl crate::Writable for BOX_MEM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOX_MEM[%s] to value 0"]
impl crate::Resettable for BOX_MEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
