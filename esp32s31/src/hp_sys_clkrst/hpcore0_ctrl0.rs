#[doc = "Register `HPCORE0_CTRL0` reader"]
pub type R = crate::R<HPCORE0_CTRL0_SPEC>;
#[doc = "Register `HPCORE0_CTRL0` writer"]
pub type W = crate::W<HPCORE0_CTRL0_SPEC>;
#[doc = "Field `CORE0_CLIC_CLK_EN` reader - need_des"]
pub type CORE0_CLIC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CORE0_CLIC_CLK_EN` writer - need_des"]
pub type CORE0_CLIC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_CPU_CLK_EN` reader - need_des"]
pub type CORE0_CPU_CLK_EN_R = crate::BitReader;
#[doc = "Field `CORE0_CPU_CLK_EN` writer - need_des"]
pub type CORE0_CPU_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_GLOBAL_RST_EN` reader - need_des"]
pub type CORE0_GLOBAL_RST_EN_R = crate::BitReader;
#[doc = "Field `CORE0_GLOBAL_RST_EN` writer - need_des"]
pub type CORE0_GLOBAL_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_FORCE_NORST` reader - need_des"]
pub type CORE0_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `CORE0_FORCE_NORST` writer - need_des"]
pub type CORE0_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn core0_clic_clk_en(&self) -> CORE0_CLIC_CLK_EN_R {
        CORE0_CLIC_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn core0_cpu_clk_en(&self) -> CORE0_CPU_CLK_EN_R {
        CORE0_CPU_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn core0_global_rst_en(&self) -> CORE0_GLOBAL_RST_EN_R {
        CORE0_GLOBAL_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn core0_force_norst(&self) -> CORE0_FORCE_NORST_R {
        CORE0_FORCE_NORST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPCORE0_CTRL0")
            .field("core0_clic_clk_en", &self.core0_clic_clk_en())
            .field("core0_cpu_clk_en", &self.core0_cpu_clk_en())
            .field("core0_global_rst_en", &self.core0_global_rst_en())
            .field("core0_force_norst", &self.core0_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn core0_clic_clk_en(&mut self) -> CORE0_CLIC_CLK_EN_W<'_, HPCORE0_CTRL0_SPEC> {
        CORE0_CLIC_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn core0_cpu_clk_en(&mut self) -> CORE0_CPU_CLK_EN_W<'_, HPCORE0_CTRL0_SPEC> {
        CORE0_CPU_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn core0_global_rst_en(&mut self) -> CORE0_GLOBAL_RST_EN_W<'_, HPCORE0_CTRL0_SPEC> {
        CORE0_GLOBAL_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn core0_force_norst(&mut self) -> CORE0_FORCE_NORST_W<'_, HPCORE0_CTRL0_SPEC> {
        CORE0_FORCE_NORST_W::new(self, 3)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore0_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore0_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPCORE0_CTRL0_SPEC;
impl crate::RegisterSpec for HPCORE0_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpcore0_ctrl0::R`](R) reader structure"]
impl crate::Readable for HPCORE0_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpcore0_ctrl0::W`](W) writer structure"]
impl crate::Writable for HPCORE0_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPCORE0_CTRL0 to value 0x03"]
impl crate::Resettable for HPCORE0_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
