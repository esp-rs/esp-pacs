#[doc = "Register `IBUS_PMS_TBL_BOUNDARY2` reader"]
pub struct R(crate::R<IBUS_PMS_TBL_BOUNDARY2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBUS_PMS_TBL_BOUNDARY2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBUS_PMS_TBL_BOUNDARY2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBUS_PMS_TBL_BOUNDARY2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBUS_PMS_TBL_BOUNDARY2` writer"]
pub struct W(crate::W<IBUS_PMS_TBL_BOUNDARY2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBUS_PMS_TBL_BOUNDARY2_SPEC>;
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
impl From<crate::W<IBUS_PMS_TBL_BOUNDARY2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBUS_PMS_TBL_BOUNDARY2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBUS_PMS_BOUNDARY2` reader - The bit is used to configure the ibus permission control section boundary2"]
pub type IBUS_PMS_BOUNDARY2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IBUS_PMS_BOUNDARY2` writer - The bit is used to configure the ibus permission control section boundary2"]
pub type IBUS_PMS_BOUNDARY2_W<'a, const O: u8> =
    crate::FieldWriter<'a, IBUS_PMS_TBL_BOUNDARY2_SPEC, 12, O, u16, u16>;
impl R {
    #[doc = "Bits 0:11 - The bit is used to configure the ibus permission control section boundary2"]
    #[inline(always)]
    pub fn ibus_pms_boundary2(&self) -> IBUS_PMS_BOUNDARY2_R {
        IBUS_PMS_BOUNDARY2_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS_PMS_TBL_BOUNDARY2")
            .field(
                "ibus_pms_boundary2",
                &format_args!("{}", self.ibus_pms_boundary2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IBUS_PMS_TBL_BOUNDARY2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - The bit is used to configure the ibus permission control section boundary2"]
    #[inline(always)]
    #[must_use]
    pub fn ibus_pms_boundary2(&mut self) -> IBUS_PMS_BOUNDARY2_W<0> {
        IBUS_PMS_BOUNDARY2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibus_pms_tbl_boundary2](index.html) module"]
pub struct IBUS_PMS_TBL_BOUNDARY2_SPEC;
impl crate::RegisterSpec for IBUS_PMS_TBL_BOUNDARY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibus_pms_tbl_boundary2::R](R) reader structure"]
impl crate::Readable for IBUS_PMS_TBL_BOUNDARY2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibus_pms_tbl_boundary2::W](W) writer structure"]
impl crate::Writable for IBUS_PMS_TBL_BOUNDARY2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IBUS_PMS_TBL_BOUNDARY2 to value 0x0800"]
impl crate::Resettable for IBUS_PMS_TBL_BOUNDARY2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
