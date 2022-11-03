#[doc = "Register `SLC_APBWIN_WDATA` reader"]
pub struct R(crate::R<SLC_APBWIN_WDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_APBWIN_WDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_APBWIN_WDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_APBWIN_WDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_APBWIN_WDATA` writer"]
pub struct W(crate::W<SLC_APBWIN_WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_APBWIN_WDATA_SPEC>;
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
impl From<crate::W<SLC_APBWIN_WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_APBWIN_WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_APBWIN_WDATA` reader - *******Description***********"]
pub type SLC_APBWIN_WDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLC_APBWIN_WDATA` writer - *******Description***********"]
pub type SLC_APBWIN_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLC_APBWIN_WDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slc_apbwin_wdata(&self) -> SLC_APBWIN_WDATA_R {
        SLC_APBWIN_WDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc_apbwin_wdata(&mut self) -> SLC_APBWIN_WDATA_W<0> {
        SLC_APBWIN_WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_apbwin_wdata](index.html) module"]
pub struct SLC_APBWIN_WDATA_SPEC;
impl crate::RegisterSpec for SLC_APBWIN_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_apbwin_wdata::R](R) reader structure"]
impl crate::Readable for SLC_APBWIN_WDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_apbwin_wdata::W](W) writer structure"]
impl crate::Writable for SLC_APBWIN_WDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_APBWIN_WDATA to value 0"]
impl crate::Resettable for SLC_APBWIN_WDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
