#[doc = "Register `MEM_XTS_RELEASE` writer"]
pub type W = crate::W<MEM_XTS_RELEASE_SPEC>;
#[doc = "Field `XTS_RELEASE` writer - Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3."]
pub type XTS_RELEASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_XTS_RELEASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to release encrypted result to mspi. This action should only be asserted when manual encryption status is 2. After this action, manual encryption status will become 3."]
    #[inline(always)]
    pub fn xts_release(&mut self) -> XTS_RELEASE_W<MEM_XTS_RELEASE_SPEC> {
        XTS_RELEASE_W::new(self, 0)
    }
}
#[doc = "Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_release::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_XTS_RELEASE_SPEC;
impl crate::RegisterSpec for MEM_XTS_RELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mem_xts_release::W`](W) writer structure"]
impl crate::Writable for MEM_XTS_RELEASE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_XTS_RELEASE to value 0"]
impl crate::Resettable for MEM_XTS_RELEASE_SPEC {}
