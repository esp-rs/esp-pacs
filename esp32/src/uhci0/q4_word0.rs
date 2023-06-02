#[doc = "Register `Q4_WORD0` reader"]
pub struct R(crate::R<Q4_WORD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<Q4_WORD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<Q4_WORD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<Q4_WORD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Q4_WORD0` writer"]
pub struct W(crate::W<Q4_WORD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<Q4_WORD0_SPEC>;
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
impl From<crate::W<Q4_WORD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<Q4_WORD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEND_Q4_WORD0` reader - This register stores the content of short packet's first dword"]
pub type SEND_Q4_WORD0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEND_Q4_WORD0` writer - This register stores the content of short packet's first dword"]
pub type SEND_Q4_WORD0_W<'a, const O: u8> = crate::FieldWriter<'a, Q4_WORD0_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the content of short packet's first dword"]
    #[inline(always)]
    pub fn send_q4_word0(&self) -> SEND_Q4_WORD0_R {
        SEND_Q4_WORD0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Q4_WORD0")
            .field(
                "send_q4_word0",
                &format_args!("{}", self.send_q4_word0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<Q4_WORD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the content of short packet's first dword"]
    #[inline(always)]
    #[must_use]
    pub fn send_q4_word0(&mut self) -> SEND_Q4_WORD0_W<0> {
        SEND_Q4_WORD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [q4_word0](index.html) module"]
pub struct Q4_WORD0_SPEC;
impl crate::RegisterSpec for Q4_WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [q4_word0::R](R) reader structure"]
impl crate::Readable for Q4_WORD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [q4_word0::W](W) writer structure"]
impl crate::Writable for Q4_WORD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Q4_WORD0 to value 0"]
impl crate::Resettable for Q4_WORD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
