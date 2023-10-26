#[doc = "Register `W15` reader"]
pub type R = crate::R<W15_SPEC>;
#[doc = "Register `W15` writer"]
pub type W = crate::W<W15_SPEC>;
#[doc = "Field `BUF15` reader - 32 bits data buffer 15, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF15_R = crate::FieldReader<u32>;
#[doc = "Field `BUF15` writer - 32 bits data buffer 15, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 15, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf15(&self) -> BUF15_R {
        BUF15_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W15")
            .field("buf15", &format_args!("{}", self.buf15().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W15_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 15, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf15(&mut self) -> BUF15_W<W15_SPEC, 0> {
        BUF15_W::new(self)
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
#[doc = "Data buffer 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W15_SPEC;
impl crate::RegisterSpec for W15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w15::R`](R) reader structure"]
impl crate::Readable for W15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w15::W`](W) writer structure"]
impl crate::Writable for W15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W15 to value 0"]
impl crate::Resettable for W15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
