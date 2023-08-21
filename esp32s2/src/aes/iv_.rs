#[doc = "Register `IV_%s` reader"]
pub type R = crate::R<IV__SPEC>;
#[doc = "Register `IV_%s` writer"]
pub type W = crate::W<IV__SPEC>;
#[doc = "Field `IV` reader - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
pub type IV_R = crate::FieldReader<u32>;
#[doc = "Field `IV` writer - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
pub type IV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_")
            .field("iv", &format_args!("{}", self.iv().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IV__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit initialization vector"]
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IV_W<IV__SPEC, 0> {
        IV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "initialization vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IV__SPEC;
impl crate::RegisterSpec for IV__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv_::R`](R) reader structure"]
impl crate::Readable for IV__SPEC {}
#[doc = "`write(|w| ..)` method takes [`iv_::W`](W) writer structure"]
impl crate::Writable for IV__SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IV_%s to value 0"]
impl crate::Resettable for IV__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
