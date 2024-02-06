#[doc = "Register `W12` reader"]
pub type R = crate::R<W12_SPEC>;
#[doc = "Register `W12` writer"]
pub type W = crate::W<W12_SPEC>;
#[doc = "Field `BUF12` reader - 32 bits data buffer 12, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF12_R = crate::FieldReader<u32>;
#[doc = "Field `BUF12` writer - 32 bits data buffer 12, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF12_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 12, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf12(&self) -> BUF12_R {
        BUF12_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W12")
            .field("buf12", &format_args!("{}", self.buf12().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 12, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf12(&mut self) -> BUF12_W<W12_SPEC> {
        BUF12_W::new(self, 0)
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
#[doc = "Data buffer 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W12_SPEC;
impl crate::RegisterSpec for W12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w12::R`](R) reader structure"]
impl crate::Readable for W12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w12::W`](W) writer structure"]
impl crate::Writable for W12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W12 to value 0"]
impl crate::Resettable for W12_SPEC {
    const RESET_VALUE: u32 = 0;
}
