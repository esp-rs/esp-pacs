#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `AXI_CMD` reader - "]
pub type AXI_CMD_R = crate::FieldReader;
#[doc = "Field `AXI_CMD` writer - "]
pub type AXI_CMD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RD_WR_CHAN` reader - "]
pub type RD_WR_CHAN_R = crate::BitReader;
#[doc = "Field `RD_WR_CHAN` writer - "]
pub type RD_WR_CHAN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_MASTER_PORT` reader - "]
pub type AXI_MASTER_PORT_R = crate::FieldReader;
#[doc = "Field `AXI_MASTER_PORT` writer - "]
pub type AXI_MASTER_PORT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AXI_ERR_BIT` reader - "]
pub type AXI_ERR_BIT_R = crate::BitReader;
#[doc = "Field `AXI_SOFT_RESET_BIT` reader - "]
pub type AXI_SOFT_RESET_BIT_R = crate::BitReader;
#[doc = "Field `AXI_SOFT_RESET_BIT` writer - "]
pub type AXI_SOFT_RESET_BIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_RD_WR_CMD` reader - "]
pub type AXI_RD_WR_CMD_R = crate::BitReader;
#[doc = "Field `AXI_RD_WR_CMD` writer - "]
pub type AXI_RD_WR_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_CMD_EN` reader - "]
pub type AXI_CMD_EN_R = crate::BitReader;
#[doc = "Field `AXI_CMD_EN` writer - "]
pub type AXI_CMD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn axi_cmd(&self) -> AXI_CMD_R {
        AXI_CMD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rd_wr_chan(&self) -> RD_WR_CHAN_R {
        RD_WR_CHAN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn axi_master_port(&self) -> AXI_MASTER_PORT_R {
        AXI_MASTER_PORT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn axi_err_bit(&self) -> AXI_ERR_BIT_R {
        AXI_ERR_BIT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn axi_soft_reset_bit(&self) -> AXI_SOFT_RESET_BIT_R {
        AXI_SOFT_RESET_BIT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn axi_rd_wr_cmd(&self) -> AXI_RD_WR_CMD_R {
        AXI_RD_WR_CMD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn axi_cmd_en(&self) -> AXI_CMD_EN_R {
        AXI_CMD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("axi_cmd", &self.axi_cmd())
            .field("rd_wr_chan", &self.rd_wr_chan())
            .field("axi_master_port", &self.axi_master_port())
            .field("axi_err_bit", &self.axi_err_bit())
            .field("axi_soft_reset_bit", &self.axi_soft_reset_bit())
            .field("axi_rd_wr_cmd", &self.axi_rd_wr_cmd())
            .field("axi_cmd_en", &self.axi_cmd_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn axi_cmd(&mut self) -> AXI_CMD_W<'_, CMD_SPEC> {
        AXI_CMD_W::new(self, 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rd_wr_chan(&mut self) -> RD_WR_CHAN_W<'_, CMD_SPEC> {
        RD_WR_CHAN_W::new(self, 7)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn axi_master_port(&mut self) -> AXI_MASTER_PORT_W<'_, CMD_SPEC> {
        AXI_MASTER_PORT_W::new(self, 8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn axi_soft_reset_bit(&mut self) -> AXI_SOFT_RESET_BIT_W<'_, CMD_SPEC> {
        AXI_SOFT_RESET_BIT_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn axi_rd_wr_cmd(&mut self) -> AXI_RD_WR_CMD_W<'_, CMD_SPEC> {
        AXI_RD_WR_CMD_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn axi_cmd_en(&mut self) -> AXI_CMD_EN_W<'_, CMD_SPEC> {
        AXI_CMD_EN_W::new(self, 31)
    }
}
#[doc = "QoS indirect command\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {}
