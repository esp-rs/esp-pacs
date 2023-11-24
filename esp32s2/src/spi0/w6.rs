#[doc = "Register `W6` reader"]
pub type R = crate::R<W6_SPEC>;
#[doc = "Register `W6` writer"]
pub type W = crate::W<W6_SPEC>;
#[doc = "Field `BUF6` reader - 32 bits data buffer 6, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF6_R = crate::FieldReader<u32>;
#[doc = "Field `BUF6` writer - 32 bits data buffer 6, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 6, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf6(&self) -> BUF6_R {
        BUF6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W6")
            .field("buf6", &format_args!("{}", self.buf6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 6, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf6(&mut self) -> BUF6_W<W6_SPEC> {
        BUF6_W::new(self, 0)
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
#[doc = "Data buffer 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W6_SPEC;
impl crate::RegisterSpec for W6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w6::R`](R) reader structure"]
impl crate::Readable for W6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w6::W`](W) writer structure"]
impl crate::Writable for W6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W6 to value 0"]
impl crate::Resettable for W6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
