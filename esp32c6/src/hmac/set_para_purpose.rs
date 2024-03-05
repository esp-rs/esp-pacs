#[doc = "Register `SET_PARA_PURPOSE` writer"]
pub type W = crate::W<SET_PARA_PURPOSE_SPEC>;
#[doc = "Field `PURPOSE_SET` writer - Set hmac parameter purpose."]
pub type PURPOSE_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_PARA_PURPOSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:3 - Set hmac parameter purpose."]
    #[inline(always)]
    #[must_use]
    pub fn purpose_set(&mut self) -> PURPOSE_SET_W<SET_PARA_PURPOSE_SPEC> {
        PURPOSE_SET_W::new(self, 0)
    }
}
#[doc = "Configure purpose.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_para_purpose::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_PARA_PURPOSE_SPEC;
impl crate::RegisterSpec for SET_PARA_PURPOSE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_para_purpose::W`](W) writer structure"]
impl crate::Writable for SET_PARA_PURPOSE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SET_PARA_PURPOSE to value 0"]
impl crate::Resettable for SET_PARA_PURPOSE_SPEC {
    const RESET_VALUE: u32 = 0;
}
