#[doc = "Register `SLC1HOST_TOKEN_WDATA` reader"]
pub struct R(crate::R<SLC1HOST_TOKEN_WDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC1HOST_TOKEN_WDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC1HOST_TOKEN_WDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC1HOST_TOKEN_WDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC1HOST_TOKEN_WDATA` writer"]
pub struct W(crate::W<SLC1HOST_TOKEN_WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC1HOST_TOKEN_WDATA_SPEC>;
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
impl From<crate::W<SLC1HOST_TOKEN_WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC1HOST_TOKEN_WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC1HOST_TOKEN0_WD` reader - *******Description***********"]
pub type SLC1HOST_TOKEN0_WD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC1HOST_TOKEN0_WD` writer - *******Description***********"]
pub type SLC1HOST_TOKEN0_WD_W<'a, const O: u8> =
    crate::FieldWriter<'a, SLC1HOST_TOKEN_WDATA_SPEC, 12, O, u16, u16>;
#[doc = "Field `SLC1HOST_TOKEN1_WD` reader - *******Description***********"]
pub type SLC1HOST_TOKEN1_WD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLC1HOST_TOKEN1_WD` writer - *******Description***********"]
pub type SLC1HOST_TOKEN1_WD_W<'a, const O: u8> =
    crate::FieldWriter<'a, SLC1HOST_TOKEN_WDATA_SPEC, 12, O, u16, u16>;
impl R {
    #[doc = "Bits 0:11 - *******Description***********"]
    #[inline(always)]
    pub fn slc1host_token0_wd(&self) -> SLC1HOST_TOKEN0_WD_R {
        SLC1HOST_TOKEN0_WD_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - *******Description***********"]
    #[inline(always)]
    pub fn slc1host_token1_wd(&self) -> SLC1HOST_TOKEN1_WD_R {
        SLC1HOST_TOKEN1_WD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC1HOST_TOKEN_WDATA")
            .field(
                "slc1host_token0_wd",
                &format_args!("{}", self.slc1host_token0_wd().bits()),
            )
            .field(
                "slc1host_token1_wd",
                &format_args!("{}", self.slc1host_token1_wd().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC1HOST_TOKEN_WDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1host_token0_wd(&mut self) -> SLC1HOST_TOKEN0_WD_W<0> {
        SLC1HOST_TOKEN0_WD_W::new(self)
    }
    #[doc = "Bits 16:27 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1host_token1_wd(&mut self) -> SLC1HOST_TOKEN1_WD_W<16> {
        SLC1HOST_TOKEN1_WD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc1host_token_wdata](index.html) module"]
pub struct SLC1HOST_TOKEN_WDATA_SPEC;
impl crate::RegisterSpec for SLC1HOST_TOKEN_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc1host_token_wdata::R](R) reader structure"]
impl crate::Readable for SLC1HOST_TOKEN_WDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc1host_token_wdata::W](W) writer structure"]
impl crate::Writable for SLC1HOST_TOKEN_WDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC1HOST_TOKEN_WDATA to value 0"]
impl crate::Resettable for SLC1HOST_TOKEN_WDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
