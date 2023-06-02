#[doc = "Register `REMAINDER_BIT_NUM` reader"]
pub struct R(crate::R<REMAINDER_BIT_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REMAINDER_BIT_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REMAINDER_BIT_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REMAINDER_BIT_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REMAINDER_BIT_NUM` writer"]
pub struct W(crate::W<REMAINDER_BIT_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REMAINDER_BIT_NUM_SPEC>;
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
impl From<crate::W<REMAINDER_BIT_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REMAINDER_BIT_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REMAINDER_BIT_NUM` reader - Stores the Remainder Bit Number for the GCM operation."]
pub type REMAINDER_BIT_NUM_R = crate::FieldReader;
#[doc = "Field `REMAINDER_BIT_NUM` writer - Stores the Remainder Bit Number for the GCM operation."]
pub type REMAINDER_BIT_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, REMAINDER_BIT_NUM_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Stores the Remainder Bit Number for the GCM operation."]
    #[inline(always)]
    pub fn remainder_bit_num(&self) -> REMAINDER_BIT_NUM_R {
        REMAINDER_BIT_NUM_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAINDER_BIT_NUM")
            .field(
                "remainder_bit_num",
                &format_args!("{}", self.remainder_bit_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REMAINDER_BIT_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Stores the Remainder Bit Number for the GCM operation."]
    #[inline(always)]
    #[must_use]
    pub fn remainder_bit_num(&mut self) -> REMAINDER_BIT_NUM_W<0> {
        REMAINDER_BIT_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Remainder bit number of plaintext/ciphertext\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [remainder_bit_num](index.html) module"]
pub struct REMAINDER_BIT_NUM_SPEC;
impl crate::RegisterSpec for REMAINDER_BIT_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [remainder_bit_num::R](R) reader structure"]
impl crate::Readable for REMAINDER_BIT_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [remainder_bit_num::W](W) writer structure"]
impl crate::Writable for REMAINDER_BIT_NUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAINDER_BIT_NUM to value 0"]
impl crate::Resettable for REMAINDER_BIT_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
