#[doc = "Register `VID_MODE_CFG` reader"]
pub type R = crate::R<VID_MODE_CFG_SPEC>;
#[doc = "Register `VID_MODE_CFG` writer"]
pub type W = crate::W<VID_MODE_CFG_SPEC>;
#[doc = "Field `VID_MODE_TYPE` reader - NA"]
pub type VID_MODE_TYPE_R = crate::FieldReader;
#[doc = "Field `VID_MODE_TYPE` writer - NA"]
pub type VID_MODE_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_VSA_EN` reader - NA"]
pub type LP_VSA_EN_R = crate::BitReader;
#[doc = "Field `LP_VSA_EN` writer - NA"]
pub type LP_VSA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VBP_EN` reader - NA"]
pub type LP_VBP_EN_R = crate::BitReader;
#[doc = "Field `LP_VBP_EN` writer - NA"]
pub type LP_VBP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VFP_EN` reader - NA"]
pub type LP_VFP_EN_R = crate::BitReader;
#[doc = "Field `LP_VFP_EN` writer - NA"]
pub type LP_VFP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VACT_EN` reader - NA"]
pub type LP_VACT_EN_R = crate::BitReader;
#[doc = "Field `LP_VACT_EN` writer - NA"]
pub type LP_VACT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_HBP_EN` reader - NA"]
pub type LP_HBP_EN_R = crate::BitReader;
#[doc = "Field `LP_HBP_EN` writer - NA"]
pub type LP_HBP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_HFP_EN` reader - NA"]
pub type LP_HFP_EN_R = crate::BitReader;
#[doc = "Field `LP_HFP_EN` writer - NA"]
pub type LP_HFP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_BTA_ACK_EN` reader - NA"]
pub type FRAME_BTA_ACK_EN_R = crate::BitReader;
#[doc = "Field `FRAME_BTA_ACK_EN` writer - NA"]
pub type FRAME_BTA_ACK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CMD_EN` reader - NA"]
pub type LP_CMD_EN_R = crate::BitReader;
#[doc = "Field `LP_CMD_EN` writer - NA"]
pub type LP_CMD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPG_EN` reader - NA"]
pub type VPG_EN_R = crate::BitReader;
#[doc = "Field `VPG_EN` writer - NA"]
pub type VPG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPG_MODE` reader - NA"]
pub type VPG_MODE_R = crate::BitReader;
#[doc = "Field `VPG_MODE` writer - NA"]
pub type VPG_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPG_ORIENTATION` reader - NA"]
pub type VPG_ORIENTATION_R = crate::BitReader;
#[doc = "Field `VPG_ORIENTATION` writer - NA"]
pub type VPG_ORIENTATION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn vid_mode_type(&self) -> VID_MODE_TYPE_R {
        VID_MODE_TYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn lp_vsa_en(&self) -> LP_VSA_EN_R {
        LP_VSA_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn lp_vbp_en(&self) -> LP_VBP_EN_R {
        LP_VBP_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn lp_vfp_en(&self) -> LP_VFP_EN_R {
        LP_VFP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn lp_vact_en(&self) -> LP_VACT_EN_R {
        LP_VACT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn lp_hbp_en(&self) -> LP_HBP_EN_R {
        LP_HBP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn lp_hfp_en(&self) -> LP_HFP_EN_R {
        LP_HFP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn frame_bta_ack_en(&self) -> FRAME_BTA_ACK_EN_R {
        FRAME_BTA_ACK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn lp_cmd_en(&self) -> LP_CMD_EN_R {
        LP_CMD_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn vpg_en(&self) -> VPG_EN_R {
        VPG_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn vpg_mode(&self) -> VPG_MODE_R {
        VPG_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    pub fn vpg_orientation(&self) -> VPG_ORIENTATION_R {
        VPG_ORIENTATION_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_MODE_CFG")
            .field(
                "vid_mode_type",
                &format_args!("{}", self.vid_mode_type().bits()),
            )
            .field("lp_vsa_en", &format_args!("{}", self.lp_vsa_en().bit()))
            .field("lp_vbp_en", &format_args!("{}", self.lp_vbp_en().bit()))
            .field("lp_vfp_en", &format_args!("{}", self.lp_vfp_en().bit()))
            .field("lp_vact_en", &format_args!("{}", self.lp_vact_en().bit()))
            .field("lp_hbp_en", &format_args!("{}", self.lp_hbp_en().bit()))
            .field("lp_hfp_en", &format_args!("{}", self.lp_hfp_en().bit()))
            .field(
                "frame_bta_ack_en",
                &format_args!("{}", self.frame_bta_ack_en().bit()),
            )
            .field("lp_cmd_en", &format_args!("{}", self.lp_cmd_en().bit()))
            .field("vpg_en", &format_args!("{}", self.vpg_en().bit()))
            .field("vpg_mode", &format_args!("{}", self.vpg_mode().bit()))
            .field(
                "vpg_orientation",
                &format_args!("{}", self.vpg_orientation().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VID_MODE_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vid_mode_type(&mut self) -> VID_MODE_TYPE_W<VID_MODE_CFG_SPEC> {
        VID_MODE_TYPE_W::new(self, 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn lp_vsa_en(&mut self) -> LP_VSA_EN_W<VID_MODE_CFG_SPEC> {
        LP_VSA_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn lp_vbp_en(&mut self) -> LP_VBP_EN_W<VID_MODE_CFG_SPEC> {
        LP_VBP_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn lp_vfp_en(&mut self) -> LP_VFP_EN_W<VID_MODE_CFG_SPEC> {
        LP_VFP_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn lp_vact_en(&mut self) -> LP_VACT_EN_W<VID_MODE_CFG_SPEC> {
        LP_VACT_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn lp_hbp_en(&mut self) -> LP_HBP_EN_W<VID_MODE_CFG_SPEC> {
        LP_HBP_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn lp_hfp_en(&mut self) -> LP_HFP_EN_W<VID_MODE_CFG_SPEC> {
        LP_HFP_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn frame_bta_ack_en(&mut self) -> FRAME_BTA_ACK_EN_W<VID_MODE_CFG_SPEC> {
        FRAME_BTA_ACK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cmd_en(&mut self) -> LP_CMD_EN_W<VID_MODE_CFG_SPEC> {
        LP_CMD_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vpg_en(&mut self) -> VPG_EN_W<VID_MODE_CFG_SPEC> {
        VPG_EN_W::new(self, 16)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vpg_mode(&mut self) -> VPG_MODE_W<VID_MODE_CFG_SPEC> {
        VPG_MODE_W::new(self, 20)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn vpg_orientation(&mut self) -> VPG_ORIENTATION_W<VID_MODE_CFG_SPEC> {
        VPG_ORIENTATION_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid_mode_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vid_mode_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VID_MODE_CFG_SPEC;
impl crate::RegisterSpec for VID_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_mode_cfg::R`](R) reader structure"]
impl crate::Readable for VID_MODE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vid_mode_cfg::W`](W) writer structure"]
impl crate::Writable for VID_MODE_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VID_MODE_CFG to value 0"]
impl crate::Resettable for VID_MODE_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
