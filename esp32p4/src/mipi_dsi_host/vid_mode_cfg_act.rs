#[doc = "Register `VID_MODE_CFG_ACT` reader"]
pub type R = crate::R<VID_MODE_CFG_ACT_SPEC>;
#[doc = "Field `VID_MODE_TYPE_ACT` reader - NA"]
pub type VID_MODE_TYPE_ACT_R = crate::FieldReader;
#[doc = "Field `LP_VSA_EN_ACT` reader - NA"]
pub type LP_VSA_EN_ACT_R = crate::BitReader;
#[doc = "Field `LP_VBP_EN_ACT` reader - NA"]
pub type LP_VBP_EN_ACT_R = crate::BitReader;
#[doc = "Field `LP_VFP_EN_ACT` reader - NA"]
pub type LP_VFP_EN_ACT_R = crate::BitReader;
#[doc = "Field `LP_VACT_EN_ACT` reader - NA"]
pub type LP_VACT_EN_ACT_R = crate::BitReader;
#[doc = "Field `LP_HBP_EN_ACT` reader - NA"]
pub type LP_HBP_EN_ACT_R = crate::BitReader;
#[doc = "Field `LP_HFP_EN_ACT` reader - NA"]
pub type LP_HFP_EN_ACT_R = crate::BitReader;
#[doc = "Field `FRAME_BTA_ACK_EN_ACT` reader - NA"]
pub type FRAME_BTA_ACK_EN_ACT_R = crate::BitReader;
#[doc = "Field `LP_CMD_EN_ACT` reader - NA"]
pub type LP_CMD_EN_ACT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn vid_mode_type_act(&self) -> VID_MODE_TYPE_ACT_R {
        VID_MODE_TYPE_ACT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn lp_vsa_en_act(&self) -> LP_VSA_EN_ACT_R {
        LP_VSA_EN_ACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn lp_vbp_en_act(&self) -> LP_VBP_EN_ACT_R {
        LP_VBP_EN_ACT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn lp_vfp_en_act(&self) -> LP_VFP_EN_ACT_R {
        LP_VFP_EN_ACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn lp_vact_en_act(&self) -> LP_VACT_EN_ACT_R {
        LP_VACT_EN_ACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn lp_hbp_en_act(&self) -> LP_HBP_EN_ACT_R {
        LP_HBP_EN_ACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn lp_hfp_en_act(&self) -> LP_HFP_EN_ACT_R {
        LP_HFP_EN_ACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn frame_bta_ack_en_act(&self) -> FRAME_BTA_ACK_EN_ACT_R {
        FRAME_BTA_ACK_EN_ACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn lp_cmd_en_act(&self) -> LP_CMD_EN_ACT_R {
        LP_CMD_EN_ACT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_MODE_CFG_ACT")
            .field(
                "vid_mode_type_act",
                &format_args!("{}", self.vid_mode_type_act().bits()),
            )
            .field(
                "lp_vsa_en_act",
                &format_args!("{}", self.lp_vsa_en_act().bit()),
            )
            .field(
                "lp_vbp_en_act",
                &format_args!("{}", self.lp_vbp_en_act().bit()),
            )
            .field(
                "lp_vfp_en_act",
                &format_args!("{}", self.lp_vfp_en_act().bit()),
            )
            .field(
                "lp_vact_en_act",
                &format_args!("{}", self.lp_vact_en_act().bit()),
            )
            .field(
                "lp_hbp_en_act",
                &format_args!("{}", self.lp_hbp_en_act().bit()),
            )
            .field(
                "lp_hfp_en_act",
                &format_args!("{}", self.lp_hfp_en_act().bit()),
            )
            .field(
                "frame_bta_ack_en_act",
                &format_args!("{}", self.frame_bta_ack_en_act().bit()),
            )
            .field(
                "lp_cmd_en_act",
                &format_args!("{}", self.lp_cmd_en_act().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VID_MODE_CFG_ACT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_mode_cfg_act::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_MODE_CFG_ACT_SPEC;
impl crate::RegisterSpec for VID_MODE_CFG_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_mode_cfg_act::R`](R) reader structure"]
impl crate::Readable for VID_MODE_CFG_ACT_SPEC {}
#[doc = "`reset()` method sets VID_MODE_CFG_ACT to value 0"]
impl crate::Resettable for VID_MODE_CFG_ACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
