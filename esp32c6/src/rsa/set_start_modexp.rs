#[doc = "Register `SET_START_MODEXP` writer"]
pub type W = crate::W<SET_START_MODEXP_SPEC>;
#[doc = "Field `SET_START_MODEXP` writer - start modular exponentiation"]
pub type SET_START_MODEXP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_START_MODEXP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - start modular exponentiation"]
    #[inline(always)]
    #[must_use]
    pub fn set_start_modexp(&mut self) -> SET_START_MODEXP_W<SET_START_MODEXP_SPEC> {
        SET_START_MODEXP_W::new(self, 0)
    }
}
#[doc = "RSA modular exponentiation trigger register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start_modexp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_START_MODEXP_SPEC;
impl crate::RegisterSpec for SET_START_MODEXP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_start_modexp::W`](W) writer structure"]
impl crate::Writable for SET_START_MODEXP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SET_START_MODEXP to value 0"]
impl crate::Resettable for SET_START_MODEXP_SPEC {
    const RESET_VALUE: u32 = 0;
}
