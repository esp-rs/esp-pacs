#[doc = "Register `SET_ME` writer"]
pub type W = crate::W<SET_ME_SPEC>;
#[doc = "Field `SET_ME` writer - Write 1 to this register to start DS operation."]
pub type SET_ME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_ME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to this register to start DS operation."]
    #[inline(always)]
    pub fn set_me(&mut self) -> SET_ME_W<'_, SET_ME_SPEC> {
        SET_ME_W::new(self, 0)
    }
}
#[doc = "Starts DS operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_me::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_ME_SPEC;
impl crate::RegisterSpec for SET_ME_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_me::W`](W) writer structure"]
impl crate::Writable for SET_ME_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET_ME to value 0"]
impl crate::Resettable for SET_ME_SPEC {}
