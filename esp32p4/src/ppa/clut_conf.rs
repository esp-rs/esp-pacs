#[doc = "Register `CLUT_CONF` reader"]
pub type R = crate::R<CLUT_CONF_SPEC>;
#[doc = "Register `CLUT_CONF` writer"]
pub type W = crate::W<CLUT_CONF_SPEC>;
#[doc = "Field `APB_FIFO_MASK` reader - 1'b0: fifo mode to wr/rd clut0/clut1 RAM through register PPA_SR_CLUT_DATA_REG/PPA_BLEND0_CLUT_DATA_REG/PPA_BLEND1_CLUT_DATA_REG. 1'b1: memory mode to wr/rd sr/blend0/blend1 clut RAM. The bit 11 and 10 of the waddr should be 01 to access sr clut and should be 10 to access blend0 clut and should be 11 to access blend 1 clut in memory mode."]
pub type APB_FIFO_MASK_R = crate::BitReader;
#[doc = "Field `APB_FIFO_MASK` writer - 1'b0: fifo mode to wr/rd clut0/clut1 RAM through register PPA_SR_CLUT_DATA_REG/PPA_BLEND0_CLUT_DATA_REG/PPA_BLEND1_CLUT_DATA_REG. 1'b1: memory mode to wr/rd sr/blend0/blend1 clut RAM. The bit 11 and 10 of the waddr should be 01 to access sr clut and should be 10 to access blend0 clut and should be 11 to access blend 1 clut in memory mode."]
pub type APB_FIFO_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND0_CLUT_MEM_RST` reader - Write 1 then write 0 to this bit to reset BLEND0 CLUT."]
pub type BLEND0_CLUT_MEM_RST_R = crate::BitReader;
#[doc = "Field `BLEND0_CLUT_MEM_RST` writer - Write 1 then write 0 to this bit to reset BLEND0 CLUT."]
pub type BLEND0_CLUT_MEM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND1_CLUT_MEM_RST` reader - Write 1 then write 0 to this bit to reset BLEND1 CLUT."]
pub type BLEND1_CLUT_MEM_RST_R = crate::BitReader;
#[doc = "Field `BLEND1_CLUT_MEM_RST` writer - Write 1 then write 0 to this bit to reset BLEND1 CLUT."]
pub type BLEND1_CLUT_MEM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND0_CLUT_MEM_RDADDR_RST` reader - Write 1 then write 0 to reset the read address of BLEND0 CLUT in fifo mode."]
pub type BLEND0_CLUT_MEM_RDADDR_RST_R = crate::BitReader;
#[doc = "Field `BLEND0_CLUT_MEM_RDADDR_RST` writer - Write 1 then write 0 to reset the read address of BLEND0 CLUT in fifo mode."]
pub type BLEND0_CLUT_MEM_RDADDR_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND1_CLUT_MEM_RDADDR_RST` reader - Write 1 then write 0 to reset the read address of BLEND1 CLUT in fifo mode."]
pub type BLEND1_CLUT_MEM_RDADDR_RST_R = crate::BitReader;
#[doc = "Field `BLEND1_CLUT_MEM_RDADDR_RST` writer - Write 1 then write 0 to reset the read address of BLEND1 CLUT in fifo mode."]
pub type BLEND1_CLUT_MEM_RDADDR_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND0_CLUT_MEM_FORCE_PD` reader - 1: force power down BLEND CLUT memory."]
pub type BLEND0_CLUT_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `BLEND0_CLUT_MEM_FORCE_PD` writer - 1: force power down BLEND CLUT memory."]
pub type BLEND0_CLUT_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND0_CLUT_MEM_FORCE_PU` reader - 1: force power up BLEND CLUT memory."]
pub type BLEND0_CLUT_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `BLEND0_CLUT_MEM_FORCE_PU` writer - 1: force power up BLEND CLUT memory."]
pub type BLEND0_CLUT_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND0_CLUT_MEM_CLK_ENA` reader - 1: Force clock on for BLEND CLUT memory."]
pub type BLEND0_CLUT_MEM_CLK_ENA_R = crate::BitReader;
#[doc = "Field `BLEND0_CLUT_MEM_CLK_ENA` writer - 1: Force clock on for BLEND CLUT memory."]
pub type BLEND0_CLUT_MEM_CLK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1'b0: fifo mode to wr/rd clut0/clut1 RAM through register PPA_SR_CLUT_DATA_REG/PPA_BLEND0_CLUT_DATA_REG/PPA_BLEND1_CLUT_DATA_REG. 1'b1: memory mode to wr/rd sr/blend0/blend1 clut RAM. The bit 11 and 10 of the waddr should be 01 to access sr clut and should be 10 to access blend0 clut and should be 11 to access blend 1 clut in memory mode."]
    #[inline(always)]
    pub fn apb_fifo_mask(&self) -> APB_FIFO_MASK_R {
        APB_FIFO_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 then write 0 to this bit to reset BLEND0 CLUT."]
    #[inline(always)]
    pub fn blend0_clut_mem_rst(&self) -> BLEND0_CLUT_MEM_RST_R {
        BLEND0_CLUT_MEM_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 then write 0 to this bit to reset BLEND1 CLUT."]
    #[inline(always)]
    pub fn blend1_clut_mem_rst(&self) -> BLEND1_CLUT_MEM_RST_R {
        BLEND1_CLUT_MEM_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 then write 0 to reset the read address of BLEND0 CLUT in fifo mode."]
    #[inline(always)]
    pub fn blend0_clut_mem_rdaddr_rst(&self) -> BLEND0_CLUT_MEM_RDADDR_RST_R {
        BLEND0_CLUT_MEM_RDADDR_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 then write 0 to reset the read address of BLEND1 CLUT in fifo mode."]
    #[inline(always)]
    pub fn blend1_clut_mem_rdaddr_rst(&self) -> BLEND1_CLUT_MEM_RDADDR_RST_R {
        BLEND1_CLUT_MEM_RDADDR_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: force power down BLEND CLUT memory."]
    #[inline(always)]
    pub fn blend0_clut_mem_force_pd(&self) -> BLEND0_CLUT_MEM_FORCE_PD_R {
        BLEND0_CLUT_MEM_FORCE_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: force power up BLEND CLUT memory."]
    #[inline(always)]
    pub fn blend0_clut_mem_force_pu(&self) -> BLEND0_CLUT_MEM_FORCE_PU_R {
        BLEND0_CLUT_MEM_FORCE_PU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: Force clock on for BLEND CLUT memory."]
    #[inline(always)]
    pub fn blend0_clut_mem_clk_ena(&self) -> BLEND0_CLUT_MEM_CLK_ENA_R {
        BLEND0_CLUT_MEM_CLK_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLUT_CONF")
            .field("apb_fifo_mask", &self.apb_fifo_mask())
            .field("blend0_clut_mem_rst", &self.blend0_clut_mem_rst())
            .field("blend1_clut_mem_rst", &self.blend1_clut_mem_rst())
            .field(
                "blend0_clut_mem_rdaddr_rst",
                &self.blend0_clut_mem_rdaddr_rst(),
            )
            .field(
                "blend1_clut_mem_rdaddr_rst",
                &self.blend1_clut_mem_rdaddr_rst(),
            )
            .field("blend0_clut_mem_force_pd", &self.blend0_clut_mem_force_pd())
            .field("blend0_clut_mem_force_pu", &self.blend0_clut_mem_force_pu())
            .field("blend0_clut_mem_clk_ena", &self.blend0_clut_mem_clk_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1'b0: fifo mode to wr/rd clut0/clut1 RAM through register PPA_SR_CLUT_DATA_REG/PPA_BLEND0_CLUT_DATA_REG/PPA_BLEND1_CLUT_DATA_REG. 1'b1: memory mode to wr/rd sr/blend0/blend1 clut RAM. The bit 11 and 10 of the waddr should be 01 to access sr clut and should be 10 to access blend0 clut and should be 11 to access blend 1 clut in memory mode."]
    #[inline(always)]
    pub fn apb_fifo_mask(&mut self) -> APB_FIFO_MASK_W<'_, CLUT_CONF_SPEC> {
        APB_FIFO_MASK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 then write 0 to this bit to reset BLEND0 CLUT."]
    #[inline(always)]
    pub fn blend0_clut_mem_rst(&mut self) -> BLEND0_CLUT_MEM_RST_W<'_, CLUT_CONF_SPEC> {
        BLEND0_CLUT_MEM_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 then write 0 to this bit to reset BLEND1 CLUT."]
    #[inline(always)]
    pub fn blend1_clut_mem_rst(&mut self) -> BLEND1_CLUT_MEM_RST_W<'_, CLUT_CONF_SPEC> {
        BLEND1_CLUT_MEM_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 then write 0 to reset the read address of BLEND0 CLUT in fifo mode."]
    #[inline(always)]
    pub fn blend0_clut_mem_rdaddr_rst(
        &mut self,
    ) -> BLEND0_CLUT_MEM_RDADDR_RST_W<'_, CLUT_CONF_SPEC> {
        BLEND0_CLUT_MEM_RDADDR_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 then write 0 to reset the read address of BLEND1 CLUT in fifo mode."]
    #[inline(always)]
    pub fn blend1_clut_mem_rdaddr_rst(
        &mut self,
    ) -> BLEND1_CLUT_MEM_RDADDR_RST_W<'_, CLUT_CONF_SPEC> {
        BLEND1_CLUT_MEM_RDADDR_RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: force power down BLEND CLUT memory."]
    #[inline(always)]
    pub fn blend0_clut_mem_force_pd(&mut self) -> BLEND0_CLUT_MEM_FORCE_PD_W<'_, CLUT_CONF_SPEC> {
        BLEND0_CLUT_MEM_FORCE_PD_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: force power up BLEND CLUT memory."]
    #[inline(always)]
    pub fn blend0_clut_mem_force_pu(&mut self) -> BLEND0_CLUT_MEM_FORCE_PU_W<'_, CLUT_CONF_SPEC> {
        BLEND0_CLUT_MEM_FORCE_PU_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1: Force clock on for BLEND CLUT memory."]
    #[inline(always)]
    pub fn blend0_clut_mem_clk_ena(&mut self) -> BLEND0_CLUT_MEM_CLK_ENA_W<'_, CLUT_CONF_SPEC> {
        BLEND0_CLUT_MEM_CLK_ENA_W::new(self, 7)
    }
}
#[doc = "CLUT configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`clut_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clut_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLUT_CONF_SPEC;
impl crate::RegisterSpec for CLUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clut_conf::R`](R) reader structure"]
impl crate::Readable for CLUT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clut_conf::W`](W) writer structure"]
impl crate::Writable for CLUT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLUT_CONF to value 0"]
impl crate::Resettable for CLUT_CONF_SPEC {}
