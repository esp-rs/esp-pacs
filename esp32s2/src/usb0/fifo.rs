#[doc = "Register `FIFO%s` reader"]
pub type R = crate::R<FIFO_SPEC>;
#[doc = "Register `FIFO%s` writer"]
pub type W = crate::W<FIFO_SPEC>;
#[doc = "Field `WORD` reader - "]
pub type WORD_R = crate::FieldReader<u32>;
#[doc = "Field `WORD` writer - "]
pub type WORD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO").field("word", &self.word()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn word(&mut self) -> WORD_W<FIFO_SPEC> {
        WORD_W::new(self, 0)
    }
}
#[doc = "Read and write data to the USB FIFOs through this register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo::R`](R) reader structure"]
impl crate::Readable for FIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo::W`](W) writer structure"]
impl crate::Writable for FIFO_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FIFO%s to value 0"]
impl crate::Resettable for FIFO_SPEC {}
