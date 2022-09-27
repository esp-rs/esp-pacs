#[doc = "Register `TZ0_STATUS` reader"]
pub struct R(crate::R<TZ0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZ0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZ0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZ0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TZ0_CBC_ON` reader - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
pub type TZ0_CBC_ON_R = crate::BitReader<bool>;
#[doc = "Field `TZ0_OST_ON` reader - Set and reset by hardware. If set, an one-shot mode action is on going"]
pub type TZ0_OST_ON_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
    #[inline(always)]
    pub fn tz0_cbc_on(&self) -> TZ0_CBC_ON_R {
        TZ0_CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set and reset by hardware. If set, an one-shot mode action is on going"]
    #[inline(always)]
    pub fn tz0_ost_on(&self) -> TZ0_OST_ON_R {
        TZ0_OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status of fault events.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tz0_status](index.html) module"]
pub struct TZ0_STATUS_SPEC;
impl crate::RegisterSpec for TZ0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tz0_status::R](R) reader structure"]
impl crate::Readable for TZ0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZ0_STATUS to value 0"]
impl crate::Resettable for TZ0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
