///Register `MAIN_STATE` reader
pub type R = crate::R<MAIN_STATE_SPEC>;
///Field `MAIN_LAST_ST_STATE` reader - need_des
pub type MAIN_LAST_ST_STATE_R = crate::FieldReader;
///Field `MAIN_TAR_ST_STATE` reader - need_des
pub type MAIN_TAR_ST_STATE_R = crate::FieldReader;
///Field `MAIN_CUR_ST_STATE` reader - need_des
pub type MAIN_CUR_ST_STATE_R = crate::FieldReader;
impl R {
    ///Bits 11:17 - need_des
    #[inline(always)]
    pub fn main_last_st_state(&self) -> MAIN_LAST_ST_STATE_R {
        MAIN_LAST_ST_STATE_R::new(((self.bits >> 11) & 0x7f) as u8)
    }
    ///Bits 18:24 - need_des
    #[inline(always)]
    pub fn main_tar_st_state(&self) -> MAIN_TAR_ST_STATE_R {
        MAIN_TAR_ST_STATE_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    ///Bits 25:31 - need_des
    #[inline(always)]
    pub fn main_cur_st_state(&self) -> MAIN_CUR_ST_STATE_R {
        MAIN_CUR_ST_STATE_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAIN_STATE")
            .field("main_last_st_state", &self.main_last_st_state())
            .field("main_tar_st_state", &self.main_tar_st_state())
            .field("main_cur_st_state", &self.main_cur_st_state())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`main_state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MAIN_STATE_SPEC;
impl crate::RegisterSpec for MAIN_STATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`main_state::R`](R) reader structure
impl crate::Readable for MAIN_STATE_SPEC {}
///`reset()` method sets MAIN_STATE to value 0x0810_0800
impl crate::Resettable for MAIN_STATE_SPEC {
    const RESET_VALUE: u32 = 0x0810_0800;
}
