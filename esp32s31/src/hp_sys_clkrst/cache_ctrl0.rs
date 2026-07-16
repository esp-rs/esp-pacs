#[doc = "Register `CACHE_CTRL0` reader"]
pub type R = crate::R<CACHE_CTRL0_SPEC>;
#[doc = "Register `CACHE_CTRL0` writer"]
pub type W = crate::W<CACHE_CTRL0_SPEC>;
#[doc = "Field `CPU_ACACHE_CPU_CLK_EN` reader - need_des"]
pub type CPU_ACACHE_CPU_CLK_EN_R = crate::BitReader;
#[doc = "Field `CPU_ACACHE_CPU_CLK_EN` writer - need_des"]
pub type CPU_ACACHE_CPU_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_ACACHE_CPU_CLK_FORCE_ON` reader - need_des"]
pub type CPU_ACACHE_CPU_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CPU_ACACHE_CPU_CLK_FORCE_ON` writer - need_des"]
pub type CPU_ACACHE_CPU_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_ACACHE_RST_EN` reader - need_des"]
pub type CPU_ACACHE_RST_EN_R = crate::BitReader;
#[doc = "Field `CPU_ACACHE_RST_EN` writer - need_des"]
pub type CPU_ACACHE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM_ACACHE_MEM_CLK_EN` reader - need_des"]
pub type ROM_ACACHE_MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `ROM_ACACHE_MEM_CLK_EN` writer - need_des"]
pub type ROM_ACACHE_MEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM_ACACHE_MEM_CLK_FORCE_ON` reader - need_des"]
pub type ROM_ACACHE_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `ROM_ACACHE_MEM_CLK_FORCE_ON` writer - need_des"]
pub type ROM_ACACHE_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM_ACACHE_RST_EN` reader - need_des"]
pub type ROM_ACACHE_RST_EN_R = crate::BitReader;
#[doc = "Field `ROM_ACACHE_RST_EN` writer - need_des"]
pub type ROM_ACACHE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_CACHE_CPU_CLK_EN` reader - need_des"]
pub type CPU_CACHE_CPU_CLK_EN_R = crate::BitReader;
#[doc = "Field `CPU_CACHE_CPU_CLK_EN` writer - need_des"]
pub type CPU_CACHE_CPU_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_CACHE_CPU_CLK_FORCE_ON` reader - need_des"]
pub type CPU_CACHE_CPU_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `CPU_CACHE_CPU_CLK_FORCE_ON` writer - need_des"]
pub type CPU_CACHE_CPU_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU_CACHE_RST_EN` reader - need_des"]
pub type CPU_CACHE_RST_EN_R = crate::BitReader;
#[doc = "Field `CPU_CACHE_RST_EN` writer - need_des"]
pub type CPU_CACHE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSPI_CACHE_SYS_CLK_EN` reader - need_des"]
pub type MSPI_CACHE_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `MSPI_CACHE_SYS_CLK_EN` writer - need_des"]
pub type MSPI_CACHE_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSPI_CACHE_SYS_CLK_FORCE_ON` reader - need_des"]
pub type MSPI_CACHE_SYS_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MSPI_CACHE_SYS_CLK_FORCE_ON` writer - need_des"]
pub type MSPI_CACHE_SYS_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSPI_CACHE_RST_EN` reader - need_des"]
pub type MSPI_CACHE_RST_EN_R = crate::BitReader;
#[doc = "Field `MSPI_CACHE_RST_EN` writer - need_des"]
pub type MSPI_CACHE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_CLK_EN` reader - need_des"]
pub type DCACHE_CLK_EN_R = crate::BitReader;
#[doc = "Field `DCACHE_CLK_EN` writer - need_des"]
pub type DCACHE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_RST_EN` reader - need_des"]
pub type DCACHE_RST_EN_R = crate::BitReader;
#[doc = "Field `DCACHE_RST_EN` writer - need_des"]
pub type DCACHE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE0_CLK_EN` reader - need_des"]
pub type ICACHE0_CLK_EN_R = crate::BitReader;
#[doc = "Field `ICACHE0_CLK_EN` writer - need_des"]
pub type ICACHE0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE0_RST_EN` reader - need_des"]
pub type ICACHE0_RST_EN_R = crate::BitReader;
#[doc = "Field `ICACHE0_RST_EN` writer - need_des"]
pub type ICACHE0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_CLK_EN` reader - need_des"]
pub type ICACHE1_CLK_EN_R = crate::BitReader;
#[doc = "Field `ICACHE1_CLK_EN` writer - need_des"]
pub type ICACHE1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_RST_EN` reader - need_des"]
pub type ICACHE1_RST_EN_R = crate::BitReader;
#[doc = "Field `ICACHE1_RST_EN` writer - need_des"]
pub type ICACHE1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn cpu_acache_cpu_clk_en(&self) -> CPU_ACACHE_CPU_CLK_EN_R {
        CPU_ACACHE_CPU_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn cpu_acache_cpu_clk_force_on(&self) -> CPU_ACACHE_CPU_CLK_FORCE_ON_R {
        CPU_ACACHE_CPU_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn cpu_acache_rst_en(&self) -> CPU_ACACHE_RST_EN_R {
        CPU_ACACHE_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn rom_acache_mem_clk_en(&self) -> ROM_ACACHE_MEM_CLK_EN_R {
        ROM_ACACHE_MEM_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn rom_acache_mem_clk_force_on(&self) -> ROM_ACACHE_MEM_CLK_FORCE_ON_R {
        ROM_ACACHE_MEM_CLK_FORCE_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn rom_acache_rst_en(&self) -> ROM_ACACHE_RST_EN_R {
        ROM_ACACHE_RST_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn cpu_cache_cpu_clk_en(&self) -> CPU_CACHE_CPU_CLK_EN_R {
        CPU_CACHE_CPU_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn cpu_cache_cpu_clk_force_on(&self) -> CPU_CACHE_CPU_CLK_FORCE_ON_R {
        CPU_CACHE_CPU_CLK_FORCE_ON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn cpu_cache_rst_en(&self) -> CPU_CACHE_RST_EN_R {
        CPU_CACHE_RST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn mspi_cache_sys_clk_en(&self) -> MSPI_CACHE_SYS_CLK_EN_R {
        MSPI_CACHE_SYS_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn mspi_cache_sys_clk_force_on(&self) -> MSPI_CACHE_SYS_CLK_FORCE_ON_R {
        MSPI_CACHE_SYS_CLK_FORCE_ON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn mspi_cache_rst_en(&self) -> MSPI_CACHE_RST_EN_R {
        MSPI_CACHE_RST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn dcache_clk_en(&self) -> DCACHE_CLK_EN_R {
        DCACHE_CLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn dcache_rst_en(&self) -> DCACHE_RST_EN_R {
        DCACHE_RST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn icache0_clk_en(&self) -> ICACHE0_CLK_EN_R {
        ICACHE0_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn icache0_rst_en(&self) -> ICACHE0_RST_EN_R {
        ICACHE0_RST_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn icache1_clk_en(&self) -> ICACHE1_CLK_EN_R {
        ICACHE1_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn icache1_rst_en(&self) -> ICACHE1_RST_EN_R {
        ICACHE1_RST_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CTRL0")
            .field("cpu_acache_cpu_clk_en", &self.cpu_acache_cpu_clk_en())
            .field(
                "cpu_acache_cpu_clk_force_on",
                &self.cpu_acache_cpu_clk_force_on(),
            )
            .field("cpu_acache_rst_en", &self.cpu_acache_rst_en())
            .field("rom_acache_mem_clk_en", &self.rom_acache_mem_clk_en())
            .field(
                "rom_acache_mem_clk_force_on",
                &self.rom_acache_mem_clk_force_on(),
            )
            .field("rom_acache_rst_en", &self.rom_acache_rst_en())
            .field("cpu_cache_cpu_clk_en", &self.cpu_cache_cpu_clk_en())
            .field(
                "cpu_cache_cpu_clk_force_on",
                &self.cpu_cache_cpu_clk_force_on(),
            )
            .field("cpu_cache_rst_en", &self.cpu_cache_rst_en())
            .field("mspi_cache_sys_clk_en", &self.mspi_cache_sys_clk_en())
            .field(
                "mspi_cache_sys_clk_force_on",
                &self.mspi_cache_sys_clk_force_on(),
            )
            .field("mspi_cache_rst_en", &self.mspi_cache_rst_en())
            .field("dcache_clk_en", &self.dcache_clk_en())
            .field("dcache_rst_en", &self.dcache_rst_en())
            .field("icache0_clk_en", &self.icache0_clk_en())
            .field("icache0_rst_en", &self.icache0_rst_en())
            .field("icache1_clk_en", &self.icache1_clk_en())
            .field("icache1_rst_en", &self.icache1_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn cpu_acache_cpu_clk_en(&mut self) -> CPU_ACACHE_CPU_CLK_EN_W<'_, CACHE_CTRL0_SPEC> {
        CPU_ACACHE_CPU_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn cpu_acache_cpu_clk_force_on(
        &mut self,
    ) -> CPU_ACACHE_CPU_CLK_FORCE_ON_W<'_, CACHE_CTRL0_SPEC> {
        CPU_ACACHE_CPU_CLK_FORCE_ON_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn cpu_acache_rst_en(&mut self) -> CPU_ACACHE_RST_EN_W<'_, CACHE_CTRL0_SPEC> {
        CPU_ACACHE_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn rom_acache_mem_clk_en(&mut self) -> ROM_ACACHE_MEM_CLK_EN_W<'_, CACHE_CTRL0_SPEC> {
        ROM_ACACHE_MEM_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn rom_acache_mem_clk_force_on(
        &mut self,
    ) -> ROM_ACACHE_MEM_CLK_FORCE_ON_W<'_, CACHE_CTRL0_SPEC> {
        ROM_ACACHE_MEM_CLK_FORCE_ON_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn rom_acache_rst_en(&mut self) -> ROM_ACACHE_RST_EN_W<'_, CACHE_CTRL0_SPEC> {
        ROM_ACACHE_RST_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn cpu_cache_cpu_clk_en(&mut self) -> CPU_CACHE_CPU_CLK_EN_W<'_, CACHE_CTRL0_SPEC> {
        CPU_CACHE_CPU_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn cpu_cache_cpu_clk_force_on(
        &mut self,
    ) -> CPU_CACHE_CPU_CLK_FORCE_ON_W<'_, CACHE_CTRL0_SPEC> {
        CPU_CACHE_CPU_CLK_FORCE_ON_W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn cpu_cache_rst_en(&mut self) -> CPU_CACHE_RST_EN_W<'_, CACHE_CTRL0_SPEC> {
        CPU_CACHE_RST_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn mspi_cache_sys_clk_en(&mut self) -> MSPI_CACHE_SYS_CLK_EN_W<'_, CACHE_CTRL0_SPEC> {
        MSPI_CACHE_SYS_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn mspi_cache_sys_clk_force_on(
        &mut self,
    ) -> MSPI_CACHE_SYS_CLK_FORCE_ON_W<'_, CACHE_CTRL0_SPEC> {
        MSPI_CACHE_SYS_CLK_FORCE_ON_W::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn mspi_cache_rst_en(&mut self) -> MSPI_CACHE_RST_EN_W<'_, CACHE_CTRL0_SPEC> {
        MSPI_CACHE_RST_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn dcache_clk_en(&mut self) -> DCACHE_CLK_EN_W<'_, CACHE_CTRL0_SPEC> {
        DCACHE_CLK_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn dcache_rst_en(&mut self) -> DCACHE_RST_EN_W<'_, CACHE_CTRL0_SPEC> {
        DCACHE_RST_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn icache0_clk_en(&mut self) -> ICACHE0_CLK_EN_W<'_, CACHE_CTRL0_SPEC> {
        ICACHE0_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn icache0_rst_en(&mut self) -> ICACHE0_RST_EN_W<'_, CACHE_CTRL0_SPEC> {
        ICACHE0_RST_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn icache1_clk_en(&mut self) -> ICACHE1_CLK_EN_W<'_, CACHE_CTRL0_SPEC> {
        ICACHE1_CLK_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn icache1_rst_en(&mut self) -> ICACHE1_RST_EN_W<'_, CACHE_CTRL0_SPEC> {
        ICACHE1_RST_EN_W::new(self, 17)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CTRL0_SPEC;
impl crate::RegisterSpec for CACHE_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_ctrl0::R`](R) reader structure"]
impl crate::Readable for CACHE_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_ctrl0::W`](W) writer structure"]
impl crate::Writable for CACHE_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_CTRL0 to value 0x0001_5249"]
impl crate::Resettable for CACHE_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0001_5249;
}
