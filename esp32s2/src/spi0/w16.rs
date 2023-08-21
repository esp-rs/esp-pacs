#[doc = "Register `W16` reader"]
pub type R = crate::R<W16_SPEC>;
#[doc = "Register `W16` writer"]
pub type W = crate::W<W16_SPEC>;
#[doc = "Field `BUF16` reader - 32 bits data buffer 16, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF16_R = crate::FieldReader<u32>;
#[doc = "Field `BUF16` writer - 32 bits data buffer 16, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF16_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 16, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf16(&self) -> BUF16_R {
        BUF16_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W16")
            .field("buf16", &format_args!("{}", self.buf16().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W16_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 16, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf16(&mut self) -> BUF16_W<W16_SPEC, 0> {
        BUF16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data buffer 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W16_SPEC;
impl crate::RegisterSpec for W16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w16::R`](R) reader structure"]
impl crate::Readable for W16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w16::W`](W) writer structure"]
impl crate::Writable for W16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W16 to value 0"]
impl crate::Resettable for W16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
