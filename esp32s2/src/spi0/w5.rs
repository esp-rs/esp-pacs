#[doc = "Register `W5` reader"]
pub struct R(crate::R<W5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W5` writer"]
pub struct W(crate::W<W5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W5_SPEC>;
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
impl From<crate::W<W5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF5` reader - 32 bits data buffer 5, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF5_R = crate::FieldReader<u32>;
#[doc = "Field `BUF5` writer - 32 bits data buffer 5, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF5_W<'a, const O: u8> = crate::FieldWriter<'a, W5_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 5, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf5(&self) -> BUF5_R {
        BUF5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W5")
            .field("buf5", &format_args!("{}", self.buf5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 5, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf5(&mut self) -> BUF5_W<0> {
        BUF5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data buffer 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w5](index.html) module"]
pub struct W5_SPEC;
impl crate::RegisterSpec for W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w5::R](R) reader structure"]
impl crate::Readable for W5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w5::W](W) writer structure"]
impl crate::Writable for W5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W5 to value 0"]
impl crate::Resettable for W5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
