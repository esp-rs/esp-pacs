#[doc = "Register `FIFO_START_ADDR` reader"]
pub struct R(crate::R<FIFO_START_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_START_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_START_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_START_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_START_ADDR` writer"]
pub struct W(crate::W<FIFO_START_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_START_ADDR_SPEC>;
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
impl From<crate::W<FIFO_START_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_START_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_START_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_start_addr](index.html) module"]
pub struct FIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for FIFO_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_start_addr::R](R) reader structure"]
impl crate::Readable for FIFO_START_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_start_addr::W](W) writer structure"]
impl crate::Writable for FIFO_START_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO_START_ADDR to value 0"]
impl crate::Resettable for FIFO_START_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
