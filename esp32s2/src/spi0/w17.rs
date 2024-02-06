#[doc = "Register `W17` reader"]
pub type R = crate::R<W17_SPEC>;
#[doc = "Register `W17` writer"]
pub type W = crate::W<W17_SPEC>;
#[doc = "Field `BUF17` reader - 32 bits data buffer 17, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF17_R = crate::FieldReader<u32>;
#[doc = "Field `BUF17` writer - 32 bits data buffer 17, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF17_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 17, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf17(&self) -> BUF17_R {
        BUF17_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W17")
            .field("buf17", &format_args!("{}", self.buf17().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W17_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 17, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf17(&mut self) -> BUF17_W<W17_SPEC> {
        BUF17_W::new(self, 0)
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
#[doc = "Data buffer 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W17_SPEC;
impl crate::RegisterSpec for W17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w17::R`](R) reader structure"]
impl crate::Readable for W17_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w17::W`](W) writer structure"]
impl crate::Writable for W17_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W17 to value 0"]
impl crate::Resettable for W17_SPEC {
    const RESET_VALUE: u32 = 0;
}
