#[doc = "Register `W7` reader"]
pub type R = crate::R<W7_SPEC>;
#[doc = "Register `W7` writer"]
pub type W = crate::W<W7_SPEC>;
#[doc = "Field `BUF7` reader - 32 bits data buffer 7, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF7_R = crate::FieldReader<u32>;
#[doc = "Field `BUF7` writer - 32 bits data buffer 7, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 7, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf7(&self) -> BUF7_R {
        BUF7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W7")
            .field("buf7", &format_args!("{}", self.buf7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 7, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf7(&mut self) -> BUF7_W<W7_SPEC, 0> {
        BUF7_W::new(self)
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
#[doc = "Data buffer 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W7_SPEC;
impl crate::RegisterSpec for W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w7::R`](R) reader structure"]
impl crate::Readable for W7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w7::W`](W) writer structure"]
impl crate::Writable for W7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W7 to value 0"]
impl crate::Resettable for W7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
