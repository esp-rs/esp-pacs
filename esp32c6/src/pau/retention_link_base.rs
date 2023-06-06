#[doc = "Register `RETENTION_LINK_BASE` reader"]
pub struct R(crate::R<RETENTION_LINK_BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_LINK_BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_LINK_BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_LINK_BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_LINK_BASE` writer"]
pub struct W(crate::W<RETENTION_LINK_BASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_LINK_BASE_SPEC>;
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
impl From<crate::W<RETENTION_LINK_BASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_LINK_BASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINK_BASE_ADDR` reader - retention dma link base"]
pub type LINK_BASE_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `LINK_BASE_ADDR` writer - retention dma link base"]
pub type LINK_BASE_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, RETENTION_LINK_BASE_SPEC, 27, O, u32>;
impl R {
    #[doc = "Bits 0:26 - retention dma link base"]
    #[inline(always)]
    pub fn link_base_addr(&self) -> LINK_BASE_ADDR_R {
        LINK_BASE_ADDR_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_LINK_BASE")
            .field(
                "link_base_addr",
                &format_args!("{}", self.link_base_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_LINK_BASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:26 - retention dma link base"]
    #[inline(always)]
    #[must_use]
    pub fn link_base_addr(&mut self) -> LINK_BASE_ADDR_W<0> {
        LINK_BASE_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "retention dma link base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_link_base](index.html) module"]
pub struct RETENTION_LINK_BASE_SPEC;
impl crate::RegisterSpec for RETENTION_LINK_BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_link_base::R](R) reader structure"]
impl crate::Readable for RETENTION_LINK_BASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_link_base::W](W) writer structure"]
impl crate::Writable for RETENTION_LINK_BASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETENTION_LINK_BASE to value 0"]
impl crate::Resettable for RETENTION_LINK_BASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
