#[doc = "Register `WORD0` reader"]
pub type R = crate::R<WORD0_SPEC>;
#[doc = "Register `WORD0` writer"]
pub type W = crate::W<WORD0_SPEC>;
#[doc = "Field `SEND_WORD` reader - a"]
pub type SEND_WORD_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_WORD` writer - a"]
pub type SEND_WORD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    pub fn send_word(&self) -> SEND_WORD_R {
        SEND_WORD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORD0")
            .field("send_word", &self.send_word().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WORD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    #[must_use]
    pub fn send_word(&mut self) -> SEND_WORD_W<WORD0_SPEC> {
        SEND_WORD_W::new(self, 0)
    }
}
#[doc = "a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`word0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WORD0_SPEC;
impl crate::RegisterSpec for WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`word0::R`](R) reader structure"]
impl crate::Readable for WORD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`word0::W`](W) writer structure"]
impl crate::Writable for WORD0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WORD0 to value 0"]
impl crate::Resettable for WORD0_SPEC {
    const RESET_VALUE: u32 = 0;
}
