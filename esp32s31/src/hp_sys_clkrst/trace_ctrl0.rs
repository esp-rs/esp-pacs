#[doc = "Register `TRACE_CTRL0` reader"]
pub type R = crate::R<TRACE_CTRL0_SPEC>;
#[doc = "Register `TRACE_CTRL0` writer"]
pub type W = crate::W<TRACE_CTRL0_SPEC>;
#[doc = "Field `TRACE_CPU_CLK_EN` reader - need_des"]
pub type TRACE_CPU_CLK_EN_R = crate::BitReader;
#[doc = "Field `TRACE_CPU_CLK_EN` writer - need_des"]
pub type TRACE_CPU_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_SYS_CLK_EN` reader - need_des"]
pub type TRACE_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `TRACE_SYS_CLK_EN` writer - need_des"]
pub type TRACE_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_CPU_CLK_FORCE_ON` reader - need_des"]
pub type TRACE_CPU_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `TRACE_CPU_CLK_FORCE_ON` writer - need_des"]
pub type TRACE_CPU_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_SYS_CLK_FORCE_ON` reader - need_des"]
pub type TRACE_SYS_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `TRACE_SYS_CLK_FORCE_ON` writer - need_des"]
pub type TRACE_SYS_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE0_RST_EN` reader - need_des"]
pub type TRACE0_RST_EN_R = crate::BitReader;
#[doc = "Field `TRACE0_RST_EN` writer - need_des"]
pub type TRACE0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE1_RST_EN` reader - need_des"]
pub type TRACE1_RST_EN_R = crate::BitReader;
#[doc = "Field `TRACE1_RST_EN` writer - need_des"]
pub type TRACE1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE0_FORCE_NORST` reader - need_des"]
pub type TRACE0_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `TRACE0_FORCE_NORST` writer - need_des"]
pub type TRACE0_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE1_FORCE_NORST` reader - need_des"]
pub type TRACE1_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `TRACE1_FORCE_NORST` writer - need_des"]
pub type TRACE1_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn trace_cpu_clk_en(&self) -> TRACE_CPU_CLK_EN_R {
        TRACE_CPU_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn trace_sys_clk_en(&self) -> TRACE_SYS_CLK_EN_R {
        TRACE_SYS_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn trace_cpu_clk_force_on(&self) -> TRACE_CPU_CLK_FORCE_ON_R {
        TRACE_CPU_CLK_FORCE_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn trace_sys_clk_force_on(&self) -> TRACE_SYS_CLK_FORCE_ON_R {
        TRACE_SYS_CLK_FORCE_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn trace0_rst_en(&self) -> TRACE0_RST_EN_R {
        TRACE0_RST_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn trace1_rst_en(&self) -> TRACE1_RST_EN_R {
        TRACE1_RST_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn trace0_force_norst(&self) -> TRACE0_FORCE_NORST_R {
        TRACE0_FORCE_NORST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn trace1_force_norst(&self) -> TRACE1_FORCE_NORST_R {
        TRACE1_FORCE_NORST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACE_CTRL0")
            .field("trace_cpu_clk_en", &self.trace_cpu_clk_en())
            .field("trace_sys_clk_en", &self.trace_sys_clk_en())
            .field("trace_cpu_clk_force_on", &self.trace_cpu_clk_force_on())
            .field("trace_sys_clk_force_on", &self.trace_sys_clk_force_on())
            .field("trace0_rst_en", &self.trace0_rst_en())
            .field("trace1_rst_en", &self.trace1_rst_en())
            .field("trace0_force_norst", &self.trace0_force_norst())
            .field("trace1_force_norst", &self.trace1_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn trace_cpu_clk_en(&mut self) -> TRACE_CPU_CLK_EN_W<'_, TRACE_CTRL0_SPEC> {
        TRACE_CPU_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn trace_sys_clk_en(&mut self) -> TRACE_SYS_CLK_EN_W<'_, TRACE_CTRL0_SPEC> {
        TRACE_SYS_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn trace_cpu_clk_force_on(&mut self) -> TRACE_CPU_CLK_FORCE_ON_W<'_, TRACE_CTRL0_SPEC> {
        TRACE_CPU_CLK_FORCE_ON_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn trace_sys_clk_force_on(&mut self) -> TRACE_SYS_CLK_FORCE_ON_W<'_, TRACE_CTRL0_SPEC> {
        TRACE_SYS_CLK_FORCE_ON_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn trace0_rst_en(&mut self) -> TRACE0_RST_EN_W<'_, TRACE_CTRL0_SPEC> {
        TRACE0_RST_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn trace1_rst_en(&mut self) -> TRACE1_RST_EN_W<'_, TRACE_CTRL0_SPEC> {
        TRACE1_RST_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn trace0_force_norst(&mut self) -> TRACE0_FORCE_NORST_W<'_, TRACE_CTRL0_SPEC> {
        TRACE0_FORCE_NORST_W::new(self, 6)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn trace1_force_norst(&mut self) -> TRACE1_FORCE_NORST_W<'_, TRACE_CTRL0_SPEC> {
        TRACE1_FORCE_NORST_W::new(self, 7)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`trace_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trace_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACE_CTRL0_SPEC;
impl crate::RegisterSpec for TRACE_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trace_ctrl0::R`](R) reader structure"]
impl crate::Readable for TRACE_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trace_ctrl0::W`](W) writer structure"]
impl crate::Writable for TRACE_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRACE_CTRL0 to value 0x03"]
impl crate::Resettable for TRACE_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
