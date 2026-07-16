#[doc = "Register `ICM_CTRL0` reader"]
pub type R = crate::R<ICM_CTRL0_SPEC>;
#[doc = "Register `ICM_CTRL0` writer"]
pub type W = crate::W<ICM_CTRL0_SPEC>;
#[doc = "Field `ICM_CPU_CLK_EN` reader - need_des"]
pub type ICM_CPU_CLK_EN_R = crate::BitReader;
#[doc = "Field `ICM_CPU_CLK_EN` writer - need_des"]
pub type ICM_CPU_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_MEM_CLK_EN` reader - need_des"]
pub type ICM_MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `ICM_MEM_CLK_EN` writer - need_des"]
pub type ICM_MEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_SYS_CLK_EN` reader - need_des"]
pub type ICM_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `ICM_SYS_CLK_EN` writer - need_des"]
pub type ICM_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_APB_CLK_EN` reader - need_des"]
pub type ICM_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `ICM_APB_CLK_EN` writer - need_des"]
pub type ICM_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPUICM_GATED_CLK_FORCE_ON` reader - need_des"]
pub type CPUICM_GATED_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CPUICM_GATED_CLK_FORCE_ON` writer - need_des"]
pub type CPUICM_GATED_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn icm_cpu_clk_en(&self) -> ICM_CPU_CLK_EN_R {
        ICM_CPU_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn icm_mem_clk_en(&self) -> ICM_MEM_CLK_EN_R {
        ICM_MEM_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn icm_sys_clk_en(&self) -> ICM_SYS_CLK_EN_R {
        ICM_SYS_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn icm_apb_clk_en(&self) -> ICM_APB_CLK_EN_R {
        ICM_APB_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn cpuicm_gated_clk_force_on(&self) -> CPUICM_GATED_CLK_FORCE_ON_R {
        CPUICM_GATED_CLK_FORCE_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_CTRL0")
            .field("icm_cpu_clk_en", &self.icm_cpu_clk_en())
            .field("icm_mem_clk_en", &self.icm_mem_clk_en())
            .field("icm_sys_clk_en", &self.icm_sys_clk_en())
            .field("icm_apb_clk_en", &self.icm_apb_clk_en())
            .field(
                "cpuicm_gated_clk_force_on",
                &self.cpuicm_gated_clk_force_on(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn icm_cpu_clk_en(&mut self) -> ICM_CPU_CLK_EN_W<'_, ICM_CTRL0_SPEC> {
        ICM_CPU_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn icm_mem_clk_en(&mut self) -> ICM_MEM_CLK_EN_W<'_, ICM_CTRL0_SPEC> {
        ICM_MEM_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn icm_sys_clk_en(&mut self) -> ICM_SYS_CLK_EN_W<'_, ICM_CTRL0_SPEC> {
        ICM_SYS_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn icm_apb_clk_en(&mut self) -> ICM_APB_CLK_EN_W<'_, ICM_CTRL0_SPEC> {
        ICM_APB_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn cpuicm_gated_clk_force_on(&mut self) -> CPUICM_GATED_CLK_FORCE_ON_W<'_, ICM_CTRL0_SPEC> {
        CPUICM_GATED_CLK_FORCE_ON_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_CTRL0_SPEC;
impl crate::RegisterSpec for ICM_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_ctrl0::R`](R) reader structure"]
impl crate::Readable for ICM_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_ctrl0::W`](W) writer structure"]
impl crate::Writable for ICM_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_CTRL0 to value 0x0f"]
impl crate::Resettable for ICM_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
