#[doc = "Register `VDD_SOURCE_CNTL` reader"]
pub struct R(crate::R<VDD_SOURCE_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDD_SOURCE_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDD_SOURCE_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDD_SOURCE_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDD_SOURCE_CNTL` writer"]
pub struct W(crate::W<VDD_SOURCE_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDD_SOURCE_CNTL_SPEC>;
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
impl From<crate::W<VDD_SOURCE_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDD_SOURCE_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DETMODE_SEL` reader - need_des"]
pub type DETMODE_SEL_R = crate::FieldReader;
#[doc = "Field `DETMODE_SEL` writer - need_des"]
pub type DETMODE_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, VDD_SOURCE_CNTL_SPEC, 8, O>;
#[doc = "Field `VGOOD_EVENT_RECORD` reader - need_des"]
pub type VGOOD_EVENT_RECORD_R = crate::FieldReader;
#[doc = "Field `VBAT_EVENT_RECORD_CLR` writer - need_des"]
pub type VBAT_EVENT_RECORD_CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, VDD_SOURCE_CNTL_SPEC, 8, O>;
#[doc = "Field `BOD_SOURCE_ENA` reader - need_des"]
pub type BOD_SOURCE_ENA_R = crate::FieldReader;
#[doc = "Field `BOD_SOURCE_ENA` writer - need_des"]
pub type BOD_SOURCE_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, VDD_SOURCE_CNTL_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn detmode_sel(&self) -> DETMODE_SEL_R {
        DETMODE_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn vgood_event_record(&self) -> VGOOD_EVENT_RECORD_R {
        VGOOD_EVENT_RECORD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn bod_source_ena(&self) -> BOD_SOURCE_ENA_R {
        BOD_SOURCE_ENA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDD_SOURCE_CNTL")
            .field(
                "detmode_sel",
                &format_args!("{}", self.detmode_sel().bits()),
            )
            .field(
                "vgood_event_record",
                &format_args!("{}", self.vgood_event_record().bits()),
            )
            .field(
                "bod_source_ena",
                &format_args!("{}", self.bod_source_ena().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VDD_SOURCE_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn detmode_sel(&mut self) -> DETMODE_SEL_W<0> {
        DETMODE_SEL_W::new(self)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vbat_event_record_clr(&mut self) -> VBAT_EVENT_RECORD_CLR_W<16> {
        VBAT_EVENT_RECORD_CLR_W::new(self)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn bod_source_ena(&mut self) -> BOD_SOURCE_ENA_W<24> {
        BOD_SOURCE_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_source_cntl](index.html) module"]
pub struct VDD_SOURCE_CNTL_SPEC;
impl crate::RegisterSpec for VDD_SOURCE_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdd_source_cntl::R](R) reader structure"]
impl crate::Readable for VDD_SOURCE_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdd_source_cntl::W](W) writer structure"]
impl crate::Writable for VDD_SOURCE_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDD_SOURCE_CNTL to value 0x0400_00ff"]
impl crate::Resettable for VDD_SOURCE_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_00ff;
}
