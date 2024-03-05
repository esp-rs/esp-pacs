#[doc = "Register `RB_MEM[%s]` reader"]
pub type R = crate::R<RB_MEM_SPEC>;
#[doc = "Register `RB_MEM[%s]` writer"]
pub type W = crate::W<RB_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RB_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "memory that stores Rb\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rb_mem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rb_mem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RB_MEM_SPEC;
impl crate::RegisterSpec for RB_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rb_mem::R`](R) reader structure"]
impl crate::Readable for RB_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rb_mem::W`](W) writer structure"]
impl crate::Writable for RB_MEM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RB_MEM[%s] to value 0"]
impl crate::Resettable for RB_MEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
