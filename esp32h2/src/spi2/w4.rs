#[doc = "Register `W4` reader"]
pub type R = crate::R<W4_SPEC>;
#[doc = "Register `W4` writer"]
pub type W = crate::W<W4_SPEC>;
#[doc = "Field `BUF4` reader - data buffer"]
pub type BUF4_R = crate::FieldReader<u32>;
#[doc = "Field `BUF4` writer - data buffer"]
pub type BUF4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf4(&self) -> BUF4_R {
        BUF4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W4")
            .field("buf4", &format_args!("{}", self.buf4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf4(&mut self) -> BUF4_W<W4_SPEC, 0> {
        BUF4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI CPU-controlled buffer4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W4_SPEC;
impl crate::RegisterSpec for W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w4::R`](R) reader structure"]
impl crate::Readable for W4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w4::W`](W) writer structure"]
impl crate::Writable for W4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W4 to value 0"]
impl crate::Resettable for W4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
