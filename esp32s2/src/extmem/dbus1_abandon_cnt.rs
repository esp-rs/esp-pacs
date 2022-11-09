#[doc = "Register `DBUS1_ABANDON_CNT` reader"]
pub struct R(crate::R<DBUS1_ABANDON_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBUS1_ABANDON_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBUS1_ABANDON_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBUS1_ABANDON_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DBUS1_ABANDON_CNT` reader - The bits are used to count the number of the abandoned dbus1 access."]
pub type DBUS1_ABANDON_CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of the abandoned dbus1 access."]
    #[inline(always)]
    pub fn dbus1_abandon_cnt(&self) -> DBUS1_ABANDON_CNT_R {
        DBUS1_ABANDON_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbus1_abandon_cnt](index.html) module"]
pub struct DBUS1_ABANDON_CNT_SPEC;
impl crate::RegisterSpec for DBUS1_ABANDON_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbus1_abandon_cnt::R](R) reader structure"]
impl crate::Readable for DBUS1_ABANDON_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBUS1_ABANDON_CNT to value 0"]
impl crate::Resettable for DBUS1_ABANDON_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
