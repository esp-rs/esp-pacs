#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `ICM_REG_AXI_CMD` reader - NA"]
pub type ICM_REG_AXI_CMD_R = crate::FieldReader;
#[doc = "Field `ICM_REG_AXI_CMD` writer - NA"]
pub type ICM_REG_AXI_CMD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ICM_REG_RD_WR_CHAN` reader - NA"]
pub type ICM_REG_RD_WR_CHAN_R = crate::BitReader;
#[doc = "Field `ICM_REG_RD_WR_CHAN` writer - NA"]
pub type ICM_REG_RD_WR_CHAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_REG_AXI_MASTER_PORT` reader - NA"]
pub type ICM_REG_AXI_MASTER_PORT_R = crate::FieldReader;
#[doc = "Field `ICM_REG_AXI_MASTER_PORT` writer - NA"]
pub type ICM_REG_AXI_MASTER_PORT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ICM_REG_AXI_ERR_BIT` reader - NA"]
pub type ICM_REG_AXI_ERR_BIT_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_SOFT_RESET_BIT` reader - NA"]
pub type ICM_REG_AXI_SOFT_RESET_BIT_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_SOFT_RESET_BIT` writer - NA"]
pub type ICM_REG_AXI_SOFT_RESET_BIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_REG_AXI_RD_WR_CMD` reader - NA"]
pub type ICM_REG_AXI_RD_WR_CMD_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_RD_WR_CMD` writer - NA"]
pub type ICM_REG_AXI_RD_WR_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_REG_AXI_CMD_EN` reader - NA"]
pub type ICM_REG_AXI_CMD_EN_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_CMD_EN` writer - NA"]
pub type ICM_REG_AXI_CMD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_cmd(&self) -> ICM_REG_AXI_CMD_R {
        ICM_REG_AXI_CMD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn icm_reg_rd_wr_chan(&self) -> ICM_REG_RD_WR_CHAN_R {
        ICM_REG_RD_WR_CHAN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_master_port(&self) -> ICM_REG_AXI_MASTER_PORT_R {
        ICM_REG_AXI_MASTER_PORT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_err_bit(&self) -> ICM_REG_AXI_ERR_BIT_R {
        ICM_REG_AXI_ERR_BIT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_soft_reset_bit(&self) -> ICM_REG_AXI_SOFT_RESET_BIT_R {
        ICM_REG_AXI_SOFT_RESET_BIT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_rd_wr_cmd(&self) -> ICM_REG_AXI_RD_WR_CMD_R {
        ICM_REG_AXI_RD_WR_CMD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_cmd_en(&self) -> ICM_REG_AXI_CMD_EN_R {
        ICM_REG_AXI_CMD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("icm_reg_axi_cmd", &self.icm_reg_axi_cmd())
            .field("icm_reg_rd_wr_chan", &self.icm_reg_rd_wr_chan())
            .field("icm_reg_axi_master_port", &self.icm_reg_axi_master_port())
            .field("icm_reg_axi_err_bit", &self.icm_reg_axi_err_bit())
            .field(
                "icm_reg_axi_soft_reset_bit",
                &self.icm_reg_axi_soft_reset_bit(),
            )
            .field("icm_reg_axi_rd_wr_cmd", &self.icm_reg_axi_rd_wr_cmd())
            .field("icm_reg_axi_cmd_en", &self.icm_reg_axi_cmd_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_cmd(&mut self) -> ICM_REG_AXI_CMD_W<CMD_SPEC> {
        ICM_REG_AXI_CMD_W::new(self, 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn icm_reg_rd_wr_chan(&mut self) -> ICM_REG_RD_WR_CHAN_W<CMD_SPEC> {
        ICM_REG_RD_WR_CHAN_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_master_port(&mut self) -> ICM_REG_AXI_MASTER_PORT_W<CMD_SPEC> {
        ICM_REG_AXI_MASTER_PORT_W::new(self, 8)
    }
    #[doc = "Bit 29 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_soft_reset_bit(&mut self) -> ICM_REG_AXI_SOFT_RESET_BIT_W<CMD_SPEC> {
        ICM_REG_AXI_SOFT_RESET_BIT_W::new(self, 29)
    }
    #[doc = "Bit 30 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_rd_wr_cmd(&mut self) -> ICM_REG_AXI_RD_WR_CMD_W<CMD_SPEC> {
        ICM_REG_AXI_RD_WR_CMD_W::new(self, 30)
    }
    #[doc = "Bit 31 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_cmd_en(&mut self) -> ICM_REG_AXI_CMD_EN_W<CMD_SPEC> {
        ICM_REG_AXI_CMD_EN_W::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
