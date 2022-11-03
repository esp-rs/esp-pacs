#[doc = "Register `PWR_STATE` reader"]
pub struct R(crate::R<PWR_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BACKUP_ST_STATE` reader - need_des"]
pub type BACKUP_ST_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_PWR_ST_STATE` reader - need_des"]
pub type LP_PWR_ST_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HP_PWR_ST_STATE` reader - need_des"]
pub type HP_PWR_ST_STATE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn backup_st_state(&self) -> BACKUP_ST_STATE_R {
        BACKUP_ST_STATE_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn lp_pwr_st_state(&self) -> LP_PWR_ST_STATE_R {
        LP_PWR_ST_STATE_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:31 - need_des"]
    #[inline(always)]
    pub fn hp_pwr_st_state(&self) -> HP_PWR_ST_STATE_R {
        HP_PWR_ST_STATE_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_state](index.html) module"]
pub struct PWR_STATE_SPEC;
impl crate::RegisterSpec for PWR_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_state::R](R) reader structure"]
impl crate::Readable for PWR_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PWR_STATE to value 0x0080_2000"]
impl crate::Resettable for PWR_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_2000;
}
