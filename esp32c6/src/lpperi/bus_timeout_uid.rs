#[doc = "Register `BUS_TIMEOUT_UID` reader"]
pub struct R(crate::R<BUS_TIMEOUT_UID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_TIMEOUT_UID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_TIMEOUT_UID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_TIMEOUT_UID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LP_PERI_TIMEOUT_UID` reader - need_des"]
pub type LP_PERI_TIMEOUT_UID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - need_des"]
    #[inline(always)]
    pub fn lp_peri_timeout_uid(&self) -> LP_PERI_TIMEOUT_UID_R {
        LP_PERI_TIMEOUT_UID_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_timeout_uid](index.html) module"]
pub struct BUS_TIMEOUT_UID_SPEC;
impl crate::RegisterSpec for BUS_TIMEOUT_UID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_timeout_uid::R](R) reader structure"]
impl crate::Readable for BUS_TIMEOUT_UID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUS_TIMEOUT_UID to value 0"]
impl crate::Resettable for BUS_TIMEOUT_UID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
