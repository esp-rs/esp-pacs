#[doc = "Register `DBUS_PMS_TBL_ATTR` reader"]
pub struct R(crate::R<DBUS_PMS_TBL_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBUS_PMS_TBL_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBUS_PMS_TBL_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBUS_PMS_TBL_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBUS_PMS_TBL_ATTR` writer"]
pub struct W(crate::W<DBUS_PMS_TBL_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBUS_PMS_TBL_ATTR_SPEC>;
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
impl From<crate::W<DBUS_PMS_TBL_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBUS_PMS_TBL_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBUS_PMS_SCT1_ATTR` reader - The bit is used to configure attribute of the dbus permission control section1, bit0: load in world0, bit2: load in world1"]
pub type DBUS_PMS_SCT1_ATTR_R = crate::FieldReader;
#[doc = "Field `DBUS_PMS_SCT1_ATTR` writer - The bit is used to configure attribute of the dbus permission control section1, bit0: load in world0, bit2: load in world1"]
pub type DBUS_PMS_SCT1_ATTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, DBUS_PMS_TBL_ATTR_SPEC, 2, O>;
#[doc = "Field `DBUS_PMS_SCT2_ATTR` reader - The bit is used to configure attribute of the dbus permission control section2, bit0: load in world0, bit2: load in world1"]
pub type DBUS_PMS_SCT2_ATTR_R = crate::FieldReader;
#[doc = "Field `DBUS_PMS_SCT2_ATTR` writer - The bit is used to configure attribute of the dbus permission control section2, bit0: load in world0, bit2: load in world1"]
pub type DBUS_PMS_SCT2_ATTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, DBUS_PMS_TBL_ATTR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - The bit is used to configure attribute of the dbus permission control section1, bit0: load in world0, bit2: load in world1"]
    #[inline(always)]
    pub fn dbus_pms_sct1_attr(&self) -> DBUS_PMS_SCT1_ATTR_R {
        DBUS_PMS_SCT1_ATTR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The bit is used to configure attribute of the dbus permission control section2, bit0: load in world0, bit2: load in world1"]
    #[inline(always)]
    pub fn dbus_pms_sct2_attr(&self) -> DBUS_PMS_SCT2_ATTR_R {
        DBUS_PMS_SCT2_ATTR_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS_PMS_TBL_ATTR")
            .field(
                "dbus_pms_sct1_attr",
                &format_args!("{}", self.dbus_pms_sct1_attr().bits()),
            )
            .field(
                "dbus_pms_sct2_attr",
                &format_args!("{}", self.dbus_pms_sct2_attr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBUS_PMS_TBL_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - The bit is used to configure attribute of the dbus permission control section1, bit0: load in world0, bit2: load in world1"]
    #[inline(always)]
    #[must_use]
    pub fn dbus_pms_sct1_attr(&mut self) -> DBUS_PMS_SCT1_ATTR_W<0> {
        DBUS_PMS_SCT1_ATTR_W::new(self)
    }
    #[doc = "Bits 2:3 - The bit is used to configure attribute of the dbus permission control section2, bit0: load in world0, bit2: load in world1"]
    #[inline(always)]
    #[must_use]
    pub fn dbus_pms_sct2_attr(&mut self) -> DBUS_PMS_SCT2_ATTR_W<2> {
        DBUS_PMS_SCT2_ATTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbus_pms_tbl_attr](index.html) module"]
pub struct DBUS_PMS_TBL_ATTR_SPEC;
impl crate::RegisterSpec for DBUS_PMS_TBL_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbus_pms_tbl_attr::R](R) reader structure"]
impl crate::Readable for DBUS_PMS_TBL_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbus_pms_tbl_attr::W](W) writer structure"]
impl crate::Writable for DBUS_PMS_TBL_ATTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBUS_PMS_TBL_ATTR to value 0x0f"]
impl crate::Resettable for DBUS_PMS_TBL_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
