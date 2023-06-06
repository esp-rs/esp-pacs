#[doc = "Register `_0TXFIFO_POP` reader"]
pub struct R(crate::R<_0TXFIFO_POP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0TXFIFO_POP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0TXFIFO_POP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0TXFIFO_POP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_0TXFIFO_POP` writer"]
pub struct W(crate::W<_0TXFIFO_POP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_0TXFIFO_POP_SPEC>;
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
impl From<crate::W<_0TXFIFO_POP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_0TXFIFO_POP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_TXFIFO_RDATA` reader - "]
pub type SLC0_TXFIFO_RDATA_R = crate::FieldReader<u16>;
#[doc = "Field `SLC0_TXFIFO_POP` reader - "]
pub type SLC0_TXFIFO_POP_R = crate::BitReader;
#[doc = "Field `SLC0_TXFIFO_POP` writer - "]
pub type SLC0_TXFIFO_POP_W<'a, const O: u8> = crate::BitWriter<'a, _0TXFIFO_POP_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn slc0_txfifo_rdata(&self) -> SLC0_TXFIFO_RDATA_R {
        SLC0_TXFIFO_RDATA_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc0_txfifo_pop(&self) -> SLC0_TXFIFO_POP_R {
        SLC0_TXFIFO_POP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0TXFIFO_POP")
            .field(
                "slc0_txfifo_rdata",
                &format_args!("{}", self.slc0_txfifo_rdata().bits()),
            )
            .field(
                "slc0_txfifo_pop",
                &format_args!("{}", self.slc0_txfifo_pop().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0TXFIFO_POP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_txfifo_pop(&mut self) -> SLC0_TXFIFO_POP_W<16> {
        SLC0_TXFIFO_POP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0txfifo_pop](index.html) module"]
pub struct _0TXFIFO_POP_SPEC;
impl crate::RegisterSpec for _0TXFIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0txfifo_pop::R](R) reader structure"]
impl crate::Readable for _0TXFIFO_POP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_0txfifo_pop::W](W) writer structure"]
impl crate::Writable for _0TXFIFO_POP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _0TXFIFO_POP to value 0"]
impl crate::Resettable for _0TXFIFO_POP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
