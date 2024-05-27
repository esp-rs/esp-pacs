///Register `DEBUG_INFO0` reader
pub type R = crate::R<DEBUG_INFO0_SPEC>;
///Field `TOP_CTRL_INTER_DEBUG_STATE` reader - Represents top_ctrl_inter module FSM info.
pub type TOP_CTRL_INTER_DEBUG_STATE_R = crate::FieldReader;
///Field `TOP_CTRL_INTRA_DEBUG_STATE` reader - Represents top_ctrl_intra module FSM info.
pub type TOP_CTRL_INTRA_DEBUG_STATE_R = crate::FieldReader;
///Field `P_I_CMP_DEBUG_STATE` reader - Represents p_i_cmp module FSM info.
pub type P_I_CMP_DEBUG_STATE_R = crate::FieldReader;
///Field `MVD_DEBUG_STATE` reader - Represents mvd module FSM info.
pub type MVD_DEBUG_STATE_R = crate::FieldReader;
///Field `MC_CHROMA_IP_DEBUG_STATE` reader - Represents mc_chroma_ip module FSM info.
pub type MC_CHROMA_IP_DEBUG_STATE_R = crate::BitReader;
///Field `INTRA_16X16_CHROMA_CTRL_DEBUG_STATE` reader - Represents intra_16x16_chroma_ctrl module FSM info.
pub type INTRA_16X16_CHROMA_CTRL_DEBUG_STATE_R = crate::FieldReader;
///Field `INTRA_4X4_CTRL_DEBUG_STATE` reader - Represents intra_4x4_ctrl module FSM info.
pub type INTRA_4X4_CTRL_DEBUG_STATE_R = crate::FieldReader;
///Field `INTRA_TOP_CTRL_DEBUG_STATE` reader - Represents intra_top_ctrl module FSM info.
pub type INTRA_TOP_CTRL_DEBUG_STATE_R = crate::FieldReader;
///Field `IME_CTRL_DEBUG_STATE` reader - Represents ime_ctrl module FSM info.
pub type IME_CTRL_DEBUG_STATE_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Represents top_ctrl_inter module FSM info.
    #[inline(always)]
    pub fn top_ctrl_inter_debug_state(&self) -> TOP_CTRL_INTER_DEBUG_STATE_R {
        TOP_CTRL_INTER_DEBUG_STATE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Represents top_ctrl_intra module FSM info.
    #[inline(always)]
    pub fn top_ctrl_intra_debug_state(&self) -> TOP_CTRL_INTRA_DEBUG_STATE_R {
        TOP_CTRL_INTRA_DEBUG_STATE_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:9 - Represents p_i_cmp module FSM info.
    #[inline(always)]
    pub fn p_i_cmp_debug_state(&self) -> P_I_CMP_DEBUG_STATE_R {
        P_I_CMP_DEBUG_STATE_R::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bits 10:12 - Represents mvd module FSM info.
    #[inline(always)]
    pub fn mvd_debug_state(&self) -> MVD_DEBUG_STATE_R {
        MVD_DEBUG_STATE_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 13 - Represents mc_chroma_ip module FSM info.
    #[inline(always)]
    pub fn mc_chroma_ip_debug_state(&self) -> MC_CHROMA_IP_DEBUG_STATE_R {
        MC_CHROMA_IP_DEBUG_STATE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:17 - Represents intra_16x16_chroma_ctrl module FSM info.
    #[inline(always)]
    pub fn intra_16x16_chroma_ctrl_debug_state(&self) -> INTRA_16X16_CHROMA_CTRL_DEBUG_STATE_R {
        INTRA_16X16_CHROMA_CTRL_DEBUG_STATE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bits 18:21 - Represents intra_4x4_ctrl module FSM info.
    #[inline(always)]
    pub fn intra_4x4_ctrl_debug_state(&self) -> INTRA_4X4_CTRL_DEBUG_STATE_R {
        INTRA_4X4_CTRL_DEBUG_STATE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:24 - Represents intra_top_ctrl module FSM info.
    #[inline(always)]
    pub fn intra_top_ctrl_debug_state(&self) -> INTRA_TOP_CTRL_DEBUG_STATE_R {
        INTRA_TOP_CTRL_DEBUG_STATE_R::new(((self.bits >> 22) & 7) as u8)
    }
    ///Bits 25:27 - Represents ime_ctrl module FSM info.
    #[inline(always)]
    pub fn ime_ctrl_debug_state(&self) -> IME_CTRL_DEBUG_STATE_R {
        IME_CTRL_DEBUG_STATE_R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_INFO0")
            .field(
                "top_ctrl_inter_debug_state",
                &self.top_ctrl_inter_debug_state(),
            )
            .field(
                "top_ctrl_intra_debug_state",
                &self.top_ctrl_intra_debug_state(),
            )
            .field("p_i_cmp_debug_state", &self.p_i_cmp_debug_state())
            .field("mvd_debug_state", &self.mvd_debug_state())
            .field("mc_chroma_ip_debug_state", &self.mc_chroma_ip_debug_state())
            .field(
                "intra_16x16_chroma_ctrl_debug_state",
                &self.intra_16x16_chroma_ctrl_debug_state(),
            )
            .field(
                "intra_4x4_ctrl_debug_state",
                &self.intra_4x4_ctrl_debug_state(),
            )
            .field(
                "intra_top_ctrl_debug_state",
                &self.intra_top_ctrl_debug_state(),
            )
            .field("ime_ctrl_debug_state", &self.ime_ctrl_debug_state())
            .finish()
    }
}
/**Debug information register0.

You can [`read`](crate::generic::Reg::read) this register and get [`debug_info0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DEBUG_INFO0_SPEC;
impl crate::RegisterSpec for DEBUG_INFO0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`debug_info0::R`](R) reader structure
impl crate::Readable for DEBUG_INFO0_SPEC {}
///`reset()` method sets DEBUG_INFO0 to value 0
impl crate::Resettable for DEBUG_INFO0_SPEC {
    const RESET_VALUE: u32 = 0;
}
