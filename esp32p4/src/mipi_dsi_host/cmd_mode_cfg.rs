#[doc = "Register `CMD_MODE_CFG` reader"]
pub type R = crate::R<CMD_MODE_CFG_SPEC>;
#[doc = "Register `CMD_MODE_CFG` writer"]
pub type W = crate::W<CMD_MODE_CFG_SPEC>;
#[doc = "Field `TEAR_FX_EN` reader - NA"]
pub type TEAR_FX_EN_R = crate::BitReader;
#[doc = "Field `TEAR_FX_EN` writer - NA"]
pub type TEAR_FX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_RQST_EN` reader - NA"]
pub type ACK_RQST_EN_R = crate::BitReader;
#[doc = "Field `ACK_RQST_EN` writer - NA"]
pub type ACK_RQST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SW_0P_TX` reader - NA"]
pub type GEN_SW_0P_TX_R = crate::BitReader;
#[doc = "Field `GEN_SW_0P_TX` writer - NA"]
pub type GEN_SW_0P_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SW_1P_TX` reader - NA"]
pub type GEN_SW_1P_TX_R = crate::BitReader;
#[doc = "Field `GEN_SW_1P_TX` writer - NA"]
pub type GEN_SW_1P_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SW_2P_TX` reader - NA"]
pub type GEN_SW_2P_TX_R = crate::BitReader;
#[doc = "Field `GEN_SW_2P_TX` writer - NA"]
pub type GEN_SW_2P_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SR_0P_TX` reader - NA"]
pub type GEN_SR_0P_TX_R = crate::BitReader;
#[doc = "Field `GEN_SR_0P_TX` writer - NA"]
pub type GEN_SR_0P_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SR_1P_TX` reader - NA"]
pub type GEN_SR_1P_TX_R = crate::BitReader;
#[doc = "Field `GEN_SR_1P_TX` writer - NA"]
pub type GEN_SR_1P_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_SR_2P_TX` reader - NA"]
pub type GEN_SR_2P_TX_R = crate::BitReader;
#[doc = "Field `GEN_SR_2P_TX` writer - NA"]
pub type GEN_SR_2P_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_LW_TX` reader - NA"]
pub type GEN_LW_TX_R = crate::BitReader;
#[doc = "Field `GEN_LW_TX` writer - NA"]
pub type GEN_LW_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_SW_0P_TX` reader - NA"]
pub type DCS_SW_0P_TX_R = crate::BitReader;
#[doc = "Field `DCS_SW_0P_TX` writer - NA"]
pub type DCS_SW_0P_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_SW_1P_TX` reader - NA"]
pub type DCS_SW_1P_TX_R = crate::BitReader;
#[doc = "Field `DCS_SW_1P_TX` writer - NA"]
pub type DCS_SW_1P_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_SR_0P_TX` reader - NA"]
pub type DCS_SR_0P_TX_R = crate::BitReader;
#[doc = "Field `DCS_SR_0P_TX` writer - NA"]
pub type DCS_SR_0P_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCS_LW_TX` reader - NA"]
pub type DCS_LW_TX_R = crate::BitReader;
#[doc = "Field `DCS_LW_TX` writer - NA"]
pub type DCS_LW_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAX_RD_PKT_SIZE` reader - NA"]
pub type MAX_RD_PKT_SIZE_R = crate::BitReader;
#[doc = "Field `MAX_RD_PKT_SIZE` writer - NA"]
pub type MAX_RD_PKT_SIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn tear_fx_en(&self) -> TEAR_FX_EN_R {
        TEAR_FX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ack_rqst_en(&self) -> ACK_RQST_EN_R {
        ACK_RQST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn gen_sw_0p_tx(&self) -> GEN_SW_0P_TX_R {
        GEN_SW_0P_TX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn gen_sw_1p_tx(&self) -> GEN_SW_1P_TX_R {
        GEN_SW_1P_TX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn gen_sw_2p_tx(&self) -> GEN_SW_2P_TX_R {
        GEN_SW_2P_TX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn gen_sr_0p_tx(&self) -> GEN_SR_0P_TX_R {
        GEN_SR_0P_TX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn gen_sr_1p_tx(&self) -> GEN_SR_1P_TX_R {
        GEN_SR_1P_TX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn gen_sr_2p_tx(&self) -> GEN_SR_2P_TX_R {
        GEN_SR_2P_TX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn gen_lw_tx(&self) -> GEN_LW_TX_R {
        GEN_LW_TX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn dcs_sw_0p_tx(&self) -> DCS_SW_0P_TX_R {
        DCS_SW_0P_TX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn dcs_sw_1p_tx(&self) -> DCS_SW_1P_TX_R {
        DCS_SW_1P_TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn dcs_sr_0p_tx(&self) -> DCS_SR_0P_TX_R {
        DCS_SR_0P_TX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn dcs_lw_tx(&self) -> DCS_LW_TX_R {
        DCS_LW_TX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    pub fn max_rd_pkt_size(&self) -> MAX_RD_PKT_SIZE_R {
        MAX_RD_PKT_SIZE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_MODE_CFG")
            .field("tear_fx_en", &self.tear_fx_en().bit())
            .field("ack_rqst_en", &self.ack_rqst_en().bit())
            .field("gen_sw_0p_tx", &self.gen_sw_0p_tx().bit())
            .field("gen_sw_1p_tx", &self.gen_sw_1p_tx().bit())
            .field("gen_sw_2p_tx", &self.gen_sw_2p_tx().bit())
            .field("gen_sr_0p_tx", &self.gen_sr_0p_tx().bit())
            .field("gen_sr_1p_tx", &self.gen_sr_1p_tx().bit())
            .field("gen_sr_2p_tx", &self.gen_sr_2p_tx().bit())
            .field("gen_lw_tx", &self.gen_lw_tx().bit())
            .field("dcs_sw_0p_tx", &self.dcs_sw_0p_tx().bit())
            .field("dcs_sw_1p_tx", &self.dcs_sw_1p_tx().bit())
            .field("dcs_sr_0p_tx", &self.dcs_sr_0p_tx().bit())
            .field("dcs_lw_tx", &self.dcs_lw_tx().bit())
            .field("max_rd_pkt_size", &self.max_rd_pkt_size().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD_MODE_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn tear_fx_en(&mut self) -> TEAR_FX_EN_W<CMD_MODE_CFG_SPEC> {
        TEAR_FX_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ack_rqst_en(&mut self) -> ACK_RQST_EN_W<CMD_MODE_CFG_SPEC> {
        ACK_RQST_EN_W::new(self, 1)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sw_0p_tx(&mut self) -> GEN_SW_0P_TX_W<CMD_MODE_CFG_SPEC> {
        GEN_SW_0P_TX_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sw_1p_tx(&mut self) -> GEN_SW_1P_TX_W<CMD_MODE_CFG_SPEC> {
        GEN_SW_1P_TX_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sw_2p_tx(&mut self) -> GEN_SW_2P_TX_W<CMD_MODE_CFG_SPEC> {
        GEN_SW_2P_TX_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sr_0p_tx(&mut self) -> GEN_SR_0P_TX_W<CMD_MODE_CFG_SPEC> {
        GEN_SR_0P_TX_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sr_1p_tx(&mut self) -> GEN_SR_1P_TX_W<CMD_MODE_CFG_SPEC> {
        GEN_SR_1P_TX_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_sr_2p_tx(&mut self) -> GEN_SR_2P_TX_W<CMD_MODE_CFG_SPEC> {
        GEN_SR_2P_TX_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gen_lw_tx(&mut self) -> GEN_LW_TX_W<CMD_MODE_CFG_SPEC> {
        GEN_LW_TX_W::new(self, 14)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn dcs_sw_0p_tx(&mut self) -> DCS_SW_0P_TX_W<CMD_MODE_CFG_SPEC> {
        DCS_SW_0P_TX_W::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn dcs_sw_1p_tx(&mut self) -> DCS_SW_1P_TX_W<CMD_MODE_CFG_SPEC> {
        DCS_SW_1P_TX_W::new(self, 17)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn dcs_sr_0p_tx(&mut self) -> DCS_SR_0P_TX_W<CMD_MODE_CFG_SPEC> {
        DCS_SR_0P_TX_W::new(self, 18)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn dcs_lw_tx(&mut self) -> DCS_LW_TX_W<CMD_MODE_CFG_SPEC> {
        DCS_LW_TX_W::new(self, 19)
    }
    #[doc = "Bit 24 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn max_rd_pkt_size(&mut self) -> MAX_RD_PKT_SIZE_W<CMD_MODE_CFG_SPEC> {
        MAX_RD_PKT_SIZE_W::new(self, 24)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_mode_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_mode_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_MODE_CFG_SPEC;
impl crate::RegisterSpec for CMD_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_mode_cfg::R`](R) reader structure"]
impl crate::Readable for CMD_MODE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd_mode_cfg::W`](W) writer structure"]
impl crate::Writable for CMD_MODE_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_MODE_CFG to value 0"]
impl crate::Resettable for CMD_MODE_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
