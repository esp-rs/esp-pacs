#[doc = "Register `SET_PARA_PURPOSE` writer"]
pub type W = crate::W<SET_PARA_PURPOSE_SPEC>;
#[doc = "Field `PURPOSE_SET` writer - Set hmac purpose."]
pub type PURPOSE_SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_PARA_PURPOSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:3 - Set hmac purpose."]
    #[inline(always)]
    #[must_use]
    pub fn purpose_set(&mut self) -> PURPOSE_SET_W<SET_PARA_PURPOSE_SPEC, 0> {
        PURPOSE_SET_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HMAC parameter configuration register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_para_purpose::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_PARA_PURPOSE_SPEC;
impl crate::RegisterSpec for SET_PARA_PURPOSE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_para_purpose::W`](W) writer structure"]
impl crate::Writable for SET_PARA_PURPOSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_PARA_PURPOSE to value 0"]
impl crate::Resettable for SET_PARA_PURPOSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
