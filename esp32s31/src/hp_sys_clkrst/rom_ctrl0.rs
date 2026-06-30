#[doc = "Register `ROM_CTRL0` reader"]
pub type R = crate::R<ROM_CTRL0_SPEC>;
#[doc = "Register `ROM_CTRL0` writer"]
pub type W = crate::W<ROM_CTRL0_SPEC>;
#[doc = "Field `REG_ROM_MEM_CLK_EN` reader - need_des"]
pub type REG_ROM_MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_ROM_MEM_CLK_EN` writer - need_des"]
pub type REG_ROM_MEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ROM_MEM_CLK_FORCE_ON` reader - need_des"]
pub type REG_ROM_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `REG_ROM_MEM_CLK_FORCE_ON` writer - need_des"]
pub type REG_ROM_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_ROM_RST_EN` reader - need_des"]
pub type REG_ROM_RST_EN_R = crate::BitReader;
#[doc = "Field `REG_ROM_RST_EN` writer - need_des"]
pub type REG_ROM_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_rom_mem_clk_en(&self) -> REG_ROM_MEM_CLK_EN_R {
        REG_ROM_MEM_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_rom_mem_clk_force_on(&self) -> REG_ROM_MEM_CLK_FORCE_ON_R {
        REG_ROM_MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_rom_rst_en(&self) -> REG_ROM_RST_EN_R {
        REG_ROM_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_CTRL0")
            .field("reg_rom_mem_clk_en", &self.reg_rom_mem_clk_en())
            .field("reg_rom_mem_clk_force_on", &self.reg_rom_mem_clk_force_on())
            .field("reg_rom_rst_en", &self.reg_rom_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_rom_mem_clk_en(&mut self) -> REG_ROM_MEM_CLK_EN_W<'_, ROM_CTRL0_SPEC> {
        REG_ROM_MEM_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn reg_rom_mem_clk_force_on(&mut self) -> REG_ROM_MEM_CLK_FORCE_ON_W<'_, ROM_CTRL0_SPEC> {
        REG_ROM_MEM_CLK_FORCE_ON_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn reg_rom_rst_en(&mut self) -> REG_ROM_RST_EN_W<'_, ROM_CTRL0_SPEC> {
        REG_ROM_RST_EN_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_CTRL0_SPEC;
impl crate::RegisterSpec for ROM_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_ctrl0::R`](R) reader structure"]
impl crate::Readable for ROM_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_ctrl0::W`](W) writer structure"]
impl crate::Writable for ROM_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROM_CTRL0 to value 0x01"]
impl crate::Resettable for ROM_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
