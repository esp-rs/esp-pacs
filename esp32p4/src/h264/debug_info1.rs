#[doc = "Register `DEBUG_INFO1` reader"]
pub type R = crate::R<DEBUG_INFO1_SPEC>;
#[doc = "Field `FME_CTRL_DEBUG_STATE` reader - Represents fme_ctrl module FSM info."]
pub type FME_CTRL_DEBUG_STATE_R = crate::FieldReader;
#[doc = "Field `DECI_CALC_DEBUG_STATE` reader - Represents deci_calc module's FSM info. DEV use only."]
pub type DECI_CALC_DEBUG_STATE_R = crate::FieldReader;
#[doc = "Field `DB_DEBUG_STATE` reader - Represents db module FSM info."]
pub type DB_DEBUG_STATE_R = crate::FieldReader;
#[doc = "Field `CAVLC_ENC_DEBUG_STATE` reader - Represents cavlc module enc FSM info."]
pub type CAVLC_ENC_DEBUG_STATE_R = crate::FieldReader;
#[doc = "Field `CAVLC_SCAN_DEBUG_STATE` reader - Represents cavlc module scan FSM info."]
pub type CAVLC_SCAN_DEBUG_STATE_R = crate::FieldReader;
#[doc = "Field `CAVLC_CTRL_DEBUG_STATE` reader - Represents cavlc module ctrl FSM info."]
pub type CAVLC_CTRL_DEBUG_STATE_R = crate::FieldReader;
#[doc = "Field `BS_BUFFER_DEBUG_STATE` reader - Represents bs buffer overflow info."]
pub type BS_BUFFER_DEBUG_STATE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Represents fme_ctrl module FSM info."]
    #[inline(always)]
    pub fn fme_ctrl_debug_state(&self) -> FME_CTRL_DEBUG_STATE_R {
        FME_CTRL_DEBUG_STATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Represents deci_calc module's FSM info. DEV use only."]
    #[inline(always)]
    pub fn deci_calc_debug_state(&self) -> DECI_CALC_DEBUG_STATE_R {
        DECI_CALC_DEBUG_STATE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Represents db module FSM info."]
    #[inline(always)]
    pub fn db_debug_state(&self) -> DB_DEBUG_STATE_R {
        DB_DEBUG_STATE_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Represents cavlc module enc FSM info."]
    #[inline(always)]
    pub fn cavlc_enc_debug_state(&self) -> CAVLC_ENC_DEBUG_STATE_R {
        CAVLC_ENC_DEBUG_STATE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Represents cavlc module scan FSM info."]
    #[inline(always)]
    pub fn cavlc_scan_debug_state(&self) -> CAVLC_SCAN_DEBUG_STATE_R {
        CAVLC_SCAN_DEBUG_STATE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Represents cavlc module ctrl FSM info."]
    #[inline(always)]
    pub fn cavlc_ctrl_debug_state(&self) -> CAVLC_CTRL_DEBUG_STATE_R {
        CAVLC_CTRL_DEBUG_STATE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Represents bs buffer overflow info."]
    #[inline(always)]
    pub fn bs_buffer_debug_state(&self) -> BS_BUFFER_DEBUG_STATE_R {
        BS_BUFFER_DEBUG_STATE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_INFO1")
            .field("fme_ctrl_debug_state", &self.fme_ctrl_debug_state())
            .field("deci_calc_debug_state", &self.deci_calc_debug_state())
            .field("db_debug_state", &self.db_debug_state())
            .field("cavlc_enc_debug_state", &self.cavlc_enc_debug_state())
            .field("cavlc_scan_debug_state", &self.cavlc_scan_debug_state())
            .field("cavlc_ctrl_debug_state", &self.cavlc_ctrl_debug_state())
            .field("bs_buffer_debug_state", &self.bs_buffer_debug_state())
            .finish()
    }
}
#[doc = "Debug information register1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_info1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_INFO1_SPEC;
impl crate::RegisterSpec for DEBUG_INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_info1::R`](R) reader structure"]
impl crate::Readable for DEBUG_INFO1_SPEC {}
#[doc = "`reset()` method sets DEBUG_INFO1 to value 0"]
impl crate::Resettable for DEBUG_INFO1_SPEC {
    const RESET_VALUE: u32 = 0;
}
