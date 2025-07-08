#[doc = "Register `MEM_XTS_DESTROY` writer"]
pub type W = crate::W<MEM_XTS_DESTROY_SPEC>;
#[doc = "Field `XTS_DESTROY` writer - Set this bit to destroy encrypted result. This action should be asserted only when manual encryption status is 3. After this action, manual encryption status will become 0."]
pub type XTS_DESTROY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_XTS_DESTROY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to destroy encrypted result. This action should be asserted only when manual encryption status is 3. After this action, manual encryption status will become 0."]
    #[inline(always)]
    pub fn xts_destroy(&mut self) -> XTS_DESTROY_W<MEM_XTS_DESTROY_SPEC> {
        XTS_DESTROY_W::new(self, 0)
    }
}
#[doc = "Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_xts_destroy::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_XTS_DESTROY_SPEC;
impl crate::RegisterSpec for MEM_XTS_DESTROY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mem_xts_destroy::W`](W) writer structure"]
impl crate::Writable for MEM_XTS_DESTROY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_XTS_DESTROY to value 0"]
impl crate::Resettable for MEM_XTS_DESTROY_SPEC {}
