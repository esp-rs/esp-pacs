#[doc = "Register `WORD1` reader"]
pub type R = crate::R<WORD1_SPEC>;
#[doc = "Register `WORD1` writer"]
pub type W = crate::W<WORD1_SPEC>;
#[doc = "Field `SEND_WORD` reader - This register is used as a quick_sent register when specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SEND_WORD_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_WORD` writer - This register is used as a quick_sent register when specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SEND_WORD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    pub fn send_word(&self) -> SEND_WORD_R {
        SEND_WORD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORD1")
            .field("send_word", &self.send_word())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    #[must_use]
    pub fn send_word(&mut self) -> SEND_WORD_W<WORD1_SPEC> {
        SEND_WORD_W::new(self, 0)
    }
}
#[doc = "Q0_WORD1 quick_sent register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`word1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WORD1_SPEC;
impl crate::RegisterSpec for WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`word1::R`](R) reader structure"]
impl crate::Readable for WORD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`word1::W`](W) writer structure"]
impl crate::Writable for WORD1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WORD1 to value 0"]
impl crate::Resettable for WORD1_SPEC {
    const RESET_VALUE: u32 = 0;
}
