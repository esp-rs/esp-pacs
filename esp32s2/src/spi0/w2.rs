#[doc = "Register `W2` reader"]
pub struct R(crate::R<W2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W2` writer"]
pub struct W(crate::W<W2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W2_SPEC>;
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
impl From<crate::W<W2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF2` reader - 32 bits data buffer 2, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF2_R = crate::FieldReader<u32>;
#[doc = "Field `BUF2` writer - 32 bits data buffer 2, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF2_W<'a, const O: u8> = crate::FieldWriter<'a, W2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 2, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf2(&self) -> BUF2_R {
        BUF2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W2")
            .field("buf2", &format_args!("{}", self.buf2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 2, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf2(&mut self) -> BUF2_W<0> {
        BUF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data buffer 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w2](index.html) module"]
pub struct W2_SPEC;
impl crate::RegisterSpec for W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w2::R](R) reader structure"]
impl crate::Readable for W2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w2::W](W) writer structure"]
impl crate::Writable for W2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W2 to value 0"]
impl crate::Resettable for W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
