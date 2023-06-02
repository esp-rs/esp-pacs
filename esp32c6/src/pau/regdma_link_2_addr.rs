#[doc = "Register `REGDMA_LINK_2_ADDR` reader"]
pub struct R(crate::R<REGDMA_LINK_2_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGDMA_LINK_2_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGDMA_LINK_2_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGDMA_LINK_2_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGDMA_LINK_2_ADDR` writer"]
pub struct W(crate::W<REGDMA_LINK_2_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGDMA_LINK_2_ADDR_SPEC>;
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
impl From<crate::W<REGDMA_LINK_2_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGDMA_LINK_2_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINK_ADDR_2` reader - Link_2_addr reg"]
pub type LINK_ADDR_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LINK_ADDR_2` writer - Link_2_addr reg"]
pub type LINK_ADDR_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGDMA_LINK_2_ADDR_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Link_2_addr reg"]
    #[inline(always)]
    pub fn link_addr_2(&self) -> LINK_ADDR_2_R {
        LINK_ADDR_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_LINK_2_ADDR")
            .field(
                "link_addr_2",
                &format_args!("{}", self.link_addr_2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_LINK_2_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link_2_addr reg"]
    #[inline(always)]
    #[must_use]
    pub fn link_addr_2(&mut self) -> LINK_ADDR_2_W<0> {
        LINK_ADDR_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link_2_addr\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regdma_link_2_addr](index.html) module"]
pub struct REGDMA_LINK_2_ADDR_SPEC;
impl crate::RegisterSpec for REGDMA_LINK_2_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regdma_link_2_addr::R](R) reader structure"]
impl crate::Readable for REGDMA_LINK_2_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regdma_link_2_addr::W](W) writer structure"]
impl crate::Writable for REGDMA_LINK_2_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGDMA_LINK_2_ADDR to value 0"]
impl crate::Resettable for REGDMA_LINK_2_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
