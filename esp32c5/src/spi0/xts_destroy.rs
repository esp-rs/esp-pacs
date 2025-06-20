#[doc = "Register `XTS_DESTROY` writer"]
pub type W = crate::W<XTS_DESTROY_SPEC>;
#[doc = "Field `XTS_DESTROY` writer - Set this bit to destroy encrypted result. This action should be asserted only when manual encryption status is 3. After this action, manual encryption status will become 0."]
pub type XTS_DESTROY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTS_DESTROY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to destroy encrypted result. This action should be asserted only when manual encryption status is 3. After this action, manual encryption status will become 0."]
    #[inline(always)]
    pub fn xts_destroy(&mut self) -> XTS_DESTROY_W<XTS_DESTROY_SPEC> {
        XTS_DESTROY_W::new(self, 0)
    }
}
#[doc = "Manual Encryption physical address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xts_destroy::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTS_DESTROY_SPEC;
impl crate::RegisterSpec for XTS_DESTROY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`xts_destroy::W`](W) writer structure"]
impl crate::Writable for XTS_DESTROY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTS_DESTROY to value 0"]
impl crate::Resettable for XTS_DESTROY_SPEC {
    const RESET_VALUE: u32 = 0;
}
