#[doc = "Register `Q1_WORD0` reader"]
pub type R = crate::R<Q1_WORD0_SPEC>;
#[doc = "Register `Q1_WORD0` writer"]
pub type W = crate::W<Q1_WORD0_SPEC>;
#[doc = "Field `SEND_Q1_WORD0` reader - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SEND_Q1_WORD0_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_Q1_WORD0` writer - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SEND_Q1_WORD0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    pub fn send_q1_word0(&self) -> SEND_Q1_WORD0_R {
        SEND_Q1_WORD0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Q1_WORD0")
            .field(
                "send_q1_word0",
                &format_args!("{}", self.send_q1_word0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<Q1_WORD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    #[must_use]
    pub fn send_q1_word0(&mut self) -> SEND_Q1_WORD0_W<Q1_WORD0_SPEC, 0> {
        SEND_Q1_WORD0_W::new(self)
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
#[doc = "Q1_WORD0 quick_sent register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`q1_word0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`q1_word0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Q1_WORD0_SPEC;
impl crate::RegisterSpec for Q1_WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`q1_word0::R`](R) reader structure"]
impl crate::Readable for Q1_WORD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`q1_word0::W`](W) writer structure"]
impl crate::Writable for Q1_WORD0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Q1_WORD0 to value 0"]
impl crate::Resettable for Q1_WORD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
