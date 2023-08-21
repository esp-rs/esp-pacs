#[doc = "Register `Q5_WORD1` reader"]
pub type R = crate::R<Q5_WORD1_SPEC>;
#[doc = "Register `Q5_WORD1` writer"]
pub type W = crate::W<Q5_WORD1_SPEC>;
#[doc = "Field `SEND_Q5_WORD1` reader - This register stores the content of short packet's second dword"]
pub type SEND_Q5_WORD1_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_Q5_WORD1` writer - This register stores the content of short packet's second dword"]
pub type SEND_Q5_WORD1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the content of short packet's second dword"]
    #[inline(always)]
    pub fn send_q5_word1(&self) -> SEND_Q5_WORD1_R {
        SEND_Q5_WORD1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Q5_WORD1")
            .field(
                "send_q5_word1",
                &format_args!("{}", self.send_q5_word1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<Q5_WORD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the content of short packet's second dword"]
    #[inline(always)]
    #[must_use]
    pub fn send_q5_word1(&mut self) -> SEND_Q5_WORD1_W<Q5_WORD1_SPEC, 0> {
        SEND_Q5_WORD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q5_word1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q5_word1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Q5_WORD1_SPEC;
impl crate::RegisterSpec for Q5_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`q5_word1::R`](R) reader structure"]
impl crate::Readable for Q5_WORD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`q5_word1::W`](W) writer structure"]
impl crate::Writable for Q5_WORD1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Q5_WORD1 to value 0"]
impl crate::Resettable for Q5_WORD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
