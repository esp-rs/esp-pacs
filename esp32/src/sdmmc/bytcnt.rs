#[doc = "Register `BYTCNT` reader"]
pub struct R(crate::R<BYTCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYTCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BYTCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BYTCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BYTCNT` writer"]
pub struct W(crate::W<BYTCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYTCNT_SPEC>;
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
impl From<crate::W<BYTCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BYTCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE_COUNT` reader - Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer."]
pub type BYTE_COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `BYTE_COUNT` writer - Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer."]
pub type BYTE_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, BYTCNT_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer."]
    #[inline(always)]
    pub fn byte_count(&self) -> BYTE_COUNT_R {
        BYTE_COUNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BYTCNT")
            .field("byte_count", &format_args!("{}", self.byte_count().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BYTCNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes to be transferred, should be an integral multiple of Block Size for block transfers. For data transfers of undefined byte lengths, byte count should be set to 0. When byte count is set to 0, it is the responsibility of host to explicitly send stop/abort command to terminate data transfer."]
    #[inline(always)]
    #[must_use]
    pub fn byte_count(&mut self) -> BYTE_COUNT_W<0> {
        BYTE_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data transfer length configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bytcnt](index.html) module"]
pub struct BYTCNT_SPEC;
impl crate::RegisterSpec for BYTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bytcnt::R](R) reader structure"]
impl crate::Readable for BYTCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bytcnt::W](W) writer structure"]
impl crate::Writable for BYTCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BYTCNT to value 0x0200"]
impl crate::Resettable for BYTCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
