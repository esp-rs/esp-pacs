#[doc = "Register `REGDMA_CTRL0` reader"]
pub type R = crate::R<REGDMA_CTRL0_SPEC>;
#[doc = "Register `REGDMA_CTRL0` writer"]
pub type W = crate::W<REGDMA_CTRL0_SPEC>;
#[doc = "Field `REG_REGDMA_SYS_CLK_EN` reader - need_des"]
pub type REG_REGDMA_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_REGDMA_SYS_CLK_EN` writer - need_des"]
pub type REG_REGDMA_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REGDMA_RST_EN` reader - need_des"]
pub type REG_REGDMA_RST_EN_R = crate::BitReader;
#[doc = "Field `REG_REGDMA_RST_EN` writer - need_des"]
pub type REG_REGDMA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_regdma_sys_clk_en(&self) -> REG_REGDMA_SYS_CLK_EN_R {
        REG_REGDMA_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_regdma_rst_en(&self) -> REG_REGDMA_RST_EN_R {
        REG_REGDMA_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_CTRL0")
            .field("reg_regdma_sys_clk_en", &self.reg_regdma_sys_clk_en())
            .field("reg_regdma_rst_en", &self.reg_regdma_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_regdma_sys_clk_en(&mut self) -> REG_REGDMA_SYS_CLK_EN_W<'_, REGDMA_CTRL0_SPEC> {
        REG_REGDMA_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_regdma_rst_en(&mut self) -> REG_REGDMA_RST_EN_W<'_, REGDMA_CTRL0_SPEC> {
        REG_REGDMA_RST_EN_W::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_CTRL0_SPEC;
impl crate::RegisterSpec for REGDMA_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_ctrl0::R`](R) reader structure"]
impl crate::Readable for REGDMA_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regdma_ctrl0::W`](W) writer structure"]
impl crate::Writable for REGDMA_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGDMA_CTRL0 to value 0x01"]
impl crate::Resettable for REGDMA_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
