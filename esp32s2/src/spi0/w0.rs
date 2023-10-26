#[doc = "Register `W0` reader"]
pub type R = crate::R<W0_SPEC>;
#[doc = "Register `W0` writer"]
pub type W = crate::W<W0_SPEC>;
#[doc = "Field `BUF0` reader - 32 bits data buffer 0, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF0_R = crate::FieldReader<u32>;
#[doc = "Field `BUF0` writer - 32 bits data buffer 0, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 0, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf0(&self) -> BUF0_R {
        BUF0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0")
            .field("buf0", &format_args!("{}", self.buf0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 0, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf0(&mut self) -> BUF0_W<W0_SPEC, 0> {
        BUF0_W::new(self)
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
#[doc = "Data buffer 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W0_SPEC;
impl crate::RegisterSpec for W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w0::R`](R) reader structure"]
impl crate::Readable for W0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w0::W`](W) writer structure"]
impl crate::Writable for W0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W0 to value 0"]
impl crate::Resettable for W0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
