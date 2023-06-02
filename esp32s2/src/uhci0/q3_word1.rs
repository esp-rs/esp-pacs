#[doc = "Register `Q3_WORD1` reader"]
pub struct R(crate::R<Q3_WORD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<Q3_WORD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<Q3_WORD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<Q3_WORD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Q3_WORD1` writer"]
pub struct W(crate::W<Q3_WORD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<Q3_WORD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<Q3_WORD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<Q3_WORD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEND_Q3_WORD1` reader - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SEND_Q3_WORD1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEND_Q3_WORD1` writer - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
pub type SEND_Q3_WORD1_W<'a, const O: u8> = crate::FieldWriter<'a, Q3_WORD1_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    pub fn send_q3_word1(&self) -> SEND_Q3_WORD1_R {
        SEND_Q3_WORD1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Q3_WORD1")
            .field(
                "send_q3_word1",
                &format_args!("{}", self.send_q3_word1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<Q3_WORD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used as a quick_sent register when mode is specified by UHCI_ALWAYS_SEND_NUM or UHCI_SINGLE_SEND_NUM."]
    #[inline(always)]
    #[must_use]
    pub fn send_q3_word1(&mut self) -> SEND_Q3_WORD1_W<0> {
        SEND_Q3_WORD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Q3_WORD1 quick_sent register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [q3_word1](index.html) module"]
pub struct Q3_WORD1_SPEC;
impl crate::RegisterSpec for Q3_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [q3_word1::R](R) reader structure"]
impl crate::Readable for Q3_WORD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [q3_word1::W](W) writer structure"]
impl crate::Writable for Q3_WORD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Q3_WORD1 to value 0"]
impl crate::Resettable for Q3_WORD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
