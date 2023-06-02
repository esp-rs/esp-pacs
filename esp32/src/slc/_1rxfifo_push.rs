#[doc = "Register `_1RXFIFO_PUSH` reader"]
pub struct R(crate::R<_1RXFIFO_PUSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1RXFIFO_PUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1RXFIFO_PUSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1RXFIFO_PUSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_1RXFIFO_PUSH` writer"]
pub struct W(crate::W<_1RXFIFO_PUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_1RXFIFO_PUSH_SPEC>;
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
impl From<crate::W<_1RXFIFO_PUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_1RXFIFO_PUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC1_RXFIFO_WDATA` reader - "]
pub type SLC1_RXFIFO_WDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC1_RXFIFO_WDATA` writer - "]
pub type SLC1_RXFIFO_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, _1RXFIFO_PUSH_SPEC, 9, O, u16, u16>;
#[doc = "Field `SLC1_RXFIFO_PUSH` reader - "]
pub type SLC1_RXFIFO_PUSH_R = crate::BitReader;
#[doc = "Field `SLC1_RXFIFO_PUSH` writer - "]
pub type SLC1_RXFIFO_PUSH_W<'a, const O: u8> = crate::BitWriter<'a, _1RXFIFO_PUSH_SPEC, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn slc1_rxfifo_wdata(&self) -> SLC1_RXFIFO_WDATA_R {
        SLC1_RXFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_rxfifo_push(&self) -> SLC1_RXFIFO_PUSH_R {
        SLC1_RXFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1RXFIFO_PUSH")
            .field(
                "slc1_rxfifo_wdata",
                &format_args!("{}", self.slc1_rxfifo_wdata().bits()),
            )
            .field(
                "slc1_rxfifo_push",
                &format_args!("{}", self.slc1_rxfifo_push().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_1RXFIFO_PUSH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxfifo_wdata(&mut self) -> SLC1_RXFIFO_WDATA_W<0> {
        SLC1_RXFIFO_WDATA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_rxfifo_push(&mut self) -> SLC1_RXFIFO_PUSH_W<16> {
        SLC1_RXFIFO_PUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1rxfifo_push](index.html) module"]
pub struct _1RXFIFO_PUSH_SPEC;
impl crate::RegisterSpec for _1RXFIFO_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1rxfifo_push::R](R) reader structure"]
impl crate::Readable for _1RXFIFO_PUSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_1rxfifo_push::W](W) writer structure"]
impl crate::Writable for _1RXFIFO_PUSH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _1RXFIFO_PUSH to value 0"]
impl crate::Resettable for _1RXFIFO_PUSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
