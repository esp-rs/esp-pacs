#[doc = "Register `MAIN_STATE` reader"]
pub struct R(crate::R<MAIN_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAIN_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAIN_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAIN_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAIN_LAST_ST_STATE` reader - need_des"]
pub type MAIN_LAST_ST_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAIN_TAR_ST_STATE` reader - need_des"]
pub type MAIN_TAR_ST_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAIN_CUR_ST_STATE` reader - need_des"]
pub type MAIN_CUR_ST_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 11:17 - need_des"]
    #[inline(always)]
    pub fn main_last_st_state(&self) -> MAIN_LAST_ST_STATE_R {
        MAIN_LAST_ST_STATE_R::new(((self.bits >> 11) & 0x7f) as u8)
    }
    #[doc = "Bits 18:24 - need_des"]
    #[inline(always)]
    pub fn main_tar_st_state(&self) -> MAIN_TAR_ST_STATE_R {
        MAIN_TAR_ST_STATE_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - need_des"]
    #[inline(always)]
    pub fn main_cur_st_state(&self) -> MAIN_CUR_ST_STATE_R {
        MAIN_CUR_ST_STATE_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [main_state](index.html) module"]
pub struct MAIN_STATE_SPEC;
impl crate::RegisterSpec for MAIN_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [main_state::R](R) reader structure"]
impl crate::Readable for MAIN_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAIN_STATE to value 0x0810_0800"]
impl crate::Resettable for MAIN_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0810_0800;
}
