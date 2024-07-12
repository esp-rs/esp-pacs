#[doc = "Register `DEBUG_INFO2` reader"]
pub type R = crate::R<DEBUG_INFO2_SPEC>;
#[doc = "Field `P_RC_DONE_DEBUG_FLAG` reader - Represents p rate ctrl done status.\\\\0: not done\\\\1: done."]
pub type P_RC_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `P_P_I_CMP_DONE_DEBUG_FLAG` reader - Represents p p_i_cmp done status.\\\\0: not done\\\\1: done."]
pub type P_P_I_CMP_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `P_MV_MERGE_DONE_DEBUG_FLAG` reader - Represents p mv merge done status.\\\\0: not done\\\\1: done."]
pub type P_MV_MERGE_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `P_MOVE_ORI_DONE_DEBUG_FLAG` reader - Represents p move origin done status.\\\\0: not done\\\\1: done."]
pub type P_MOVE_ORI_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `P_MC_DONE_DEBUG_FLAG` reader - Represents p mc done status.\\\\0: not done\\\\1: done."]
pub type P_MC_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `P_IME_DONE_DEBUG_FLAG` reader - Represents p ime done status.\\\\0: not done\\\\1: done."]
pub type P_IME_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `P_GET_ORI_DONE_DEBUG_FLAG` reader - Represents p get origin done status.\\\\0: not done\\\\1: done."]
pub type P_GET_ORI_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `P_FME_DONE_DEBUG_FLAG` reader - Represents p fme done status.\\\\0: not done\\\\1: done."]
pub type P_FME_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `P_FETCH_DONE_DEBUG_FLAG` reader - Represents p fetch done status.\\\\0: not done\\\\1: done."]
pub type P_FETCH_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `P_DB_DONE_DEBUG_FLAG` reader - Represents p deblocking done status.\\\\0: not done\\\\1: done."]
pub type P_DB_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `P_BS_BUF_DONE_DEBUG_FLAG` reader - Represents p bitstream buffer done status.\\\\0: not done\\\\1: done."]
pub type P_BS_BUF_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG` reader - Represents dma move 2 ref mb line done status.\\\\0: not done\\\\1: done."]
pub type REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `I_P_I_CMP_DONE_DEBUG_FLAG` reader - Represents I p_i_cmp done status.\\\\0: not done\\\\1: done."]
pub type I_P_I_CMP_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `I_MOVE_ORI_DONE_DEBUG_FLAG` reader - Represents I move origin done status.\\\\0: not done\\\\1: done."]
pub type I_MOVE_ORI_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `I_GET_ORI_DONE_DEBUG_FLAG` reader - Represents I get origin done status.\\\\0: not done\\\\1: done."]
pub type I_GET_ORI_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `I_EC_DONE_DEBUG_FLAG` reader - Represents I encoder done status.\\\\0: not done\\\\1: done."]
pub type I_EC_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `I_DB_DONE_DEBUG_FLAG` reader - Represents I deblocking done status.\\\\0: not done\\\\1: done."]
pub type I_DB_DONE_DEBUG_FLAG_R = crate::BitReader;
#[doc = "Field `I_BS_BUF_DONE_DEBUG_FLAG` reader - Represents I bitstream buffer done status.\\\\0: not done\\\\1: done."]
pub type I_BS_BUF_DONE_DEBUG_FLAG_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents p rate ctrl done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_rc_done_debug_flag(&self) -> P_RC_DONE_DEBUG_FLAG_R {
        P_RC_DONE_DEBUG_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents p p_i_cmp done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_p_i_cmp_done_debug_flag(&self) -> P_P_I_CMP_DONE_DEBUG_FLAG_R {
        P_P_I_CMP_DONE_DEBUG_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents p mv merge done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_mv_merge_done_debug_flag(&self) -> P_MV_MERGE_DONE_DEBUG_FLAG_R {
        P_MV_MERGE_DONE_DEBUG_FLAG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents p move origin done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_move_ori_done_debug_flag(&self) -> P_MOVE_ORI_DONE_DEBUG_FLAG_R {
        P_MOVE_ORI_DONE_DEBUG_FLAG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents p mc done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_mc_done_debug_flag(&self) -> P_MC_DONE_DEBUG_FLAG_R {
        P_MC_DONE_DEBUG_FLAG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents p ime done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_ime_done_debug_flag(&self) -> P_IME_DONE_DEBUG_FLAG_R {
        P_IME_DONE_DEBUG_FLAG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents p get origin done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_get_ori_done_debug_flag(&self) -> P_GET_ORI_DONE_DEBUG_FLAG_R {
        P_GET_ORI_DONE_DEBUG_FLAG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents p fme done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_fme_done_debug_flag(&self) -> P_FME_DONE_DEBUG_FLAG_R {
        P_FME_DONE_DEBUG_FLAG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents p fetch done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_fetch_done_debug_flag(&self) -> P_FETCH_DONE_DEBUG_FLAG_R {
        P_FETCH_DONE_DEBUG_FLAG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents p deblocking done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_db_done_debug_flag(&self) -> P_DB_DONE_DEBUG_FLAG_R {
        P_DB_DONE_DEBUG_FLAG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents p bitstream buffer done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn p_bs_buf_done_debug_flag(&self) -> P_BS_BUF_DONE_DEBUG_FLAG_R {
        P_BS_BUF_DONE_DEBUG_FLAG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents dma move 2 ref mb line done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn ref_move_2mb_line_done_debug_flag(&self) -> REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG_R {
        REF_MOVE_2MB_LINE_DONE_DEBUG_FLAG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents I p_i_cmp done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_p_i_cmp_done_debug_flag(&self) -> I_P_I_CMP_DONE_DEBUG_FLAG_R {
        I_P_I_CMP_DONE_DEBUG_FLAG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents I move origin done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_move_ori_done_debug_flag(&self) -> I_MOVE_ORI_DONE_DEBUG_FLAG_R {
        I_MOVE_ORI_DONE_DEBUG_FLAG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents I get origin done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_get_ori_done_debug_flag(&self) -> I_GET_ORI_DONE_DEBUG_FLAG_R {
        I_GET_ORI_DONE_DEBUG_FLAG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents I encoder done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_ec_done_debug_flag(&self) -> I_EC_DONE_DEBUG_FLAG_R {
        I_EC_DONE_DEBUG_FLAG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents I deblocking done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_db_done_debug_flag(&self) -> I_DB_DONE_DEBUG_FLAG_R {
        I_DB_DONE_DEBUG_FLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents I bitstream buffer done status.\\\\0: not done\\\\1: done."]
    #[inline(always)]
    pub fn i_bs_buf_done_debug_flag(&self) -> I_BS_BUF_DONE_DEBUG_FLAG_R {
        I_BS_BUF_DONE_DEBUG_FLAG_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_INFO2")
            .field("p_rc_done_debug_flag", &self.p_rc_done_debug_flag())
            .field(
                "p_p_i_cmp_done_debug_flag",
                &self.p_p_i_cmp_done_debug_flag(),
            )
            .field(
                "p_mv_merge_done_debug_flag",
                &self.p_mv_merge_done_debug_flag(),
            )
            .field(
                "p_move_ori_done_debug_flag",
                &self.p_move_ori_done_debug_flag(),
            )
            .field("p_mc_done_debug_flag", &self.p_mc_done_debug_flag())
            .field("p_ime_done_debug_flag", &self.p_ime_done_debug_flag())
            .field(
                "p_get_ori_done_debug_flag",
                &self.p_get_ori_done_debug_flag(),
            )
            .field("p_fme_done_debug_flag", &self.p_fme_done_debug_flag())
            .field("p_fetch_done_debug_flag", &self.p_fetch_done_debug_flag())
            .field("p_db_done_debug_flag", &self.p_db_done_debug_flag())
            .field("p_bs_buf_done_debug_flag", &self.p_bs_buf_done_debug_flag())
            .field(
                "ref_move_2mb_line_done_debug_flag",
                &self.ref_move_2mb_line_done_debug_flag(),
            )
            .field(
                "i_p_i_cmp_done_debug_flag",
                &self.i_p_i_cmp_done_debug_flag(),
            )
            .field(
                "i_move_ori_done_debug_flag",
                &self.i_move_ori_done_debug_flag(),
            )
            .field(
                "i_get_ori_done_debug_flag",
                &self.i_get_ori_done_debug_flag(),
            )
            .field("i_ec_done_debug_flag", &self.i_ec_done_debug_flag())
            .field("i_db_done_debug_flag", &self.i_db_done_debug_flag())
            .field("i_bs_buf_done_debug_flag", &self.i_bs_buf_done_debug_flag())
            .finish()
    }
}
#[doc = "Debug information register2.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_info2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_INFO2_SPEC;
impl crate::RegisterSpec for DEBUG_INFO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_info2::R`](R) reader structure"]
impl crate::Readable for DEBUG_INFO2_SPEC {}
#[doc = "`reset()` method sets DEBUG_INFO2 to value 0"]
impl crate::Resettable for DEBUG_INFO2_SPEC {
    const RESET_VALUE: u32 = 0;
}
