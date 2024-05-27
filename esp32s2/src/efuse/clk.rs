#[doc = "Register `CLK` reader"]
pub type R = crate::R<CLK_SPEC>;
#[doc = "Register `CLK` writer"]
pub type W = crate::W<CLK_SPEC>;
#[doc = "Field `EFUSE_MEM_FORCE_PD` reader - If set, forces eFuse SRAM into power-saving mode."]
pub type EFUSE_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `EFUSE_MEM_FORCE_PD` writer - If set, forces eFuse SRAM into power-saving mode."]
pub type EFUSE_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - If set, forces to activate clock signal of eFuse SRAM."]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - If set, forces to activate clock signal of eFuse SRAM."]
pub type MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_MEM_FORCE_PU` reader - If set, forces eFuse SRAM into working mode."]
pub type EFUSE_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `EFUSE_MEM_FORCE_PU` writer - If set, forces eFuse SRAM into working mode."]
pub type EFUSE_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - If set, forces to enable clock signal of eFuse memory."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - If set, forces to enable clock signal of eFuse memory."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - If set, forces eFuse SRAM into power-saving mode."]
    #[inline(always)]
    pub fn efuse_mem_force_pd(&self) -> EFUSE_MEM_FORCE_PD_R {
        EFUSE_MEM_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set, forces to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set, forces eFuse SRAM into working mode."]
    #[inline(always)]
    pub fn efuse_mem_force_pu(&self) -> EFUSE_MEM_FORCE_PU_R {
        EFUSE_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - If set, forces to enable clock signal of eFuse memory."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK")
            .field("efuse_mem_force_pd", &self.efuse_mem_force_pd())
            .field("mem_clk_force_on", &self.mem_clk_force_on())
            .field("efuse_mem_force_pu", &self.efuse_mem_force_pu())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - If set, forces eFuse SRAM into power-saving mode."]
    #[inline(always)]
    #[must_use]
    pub fn efuse_mem_force_pd(&mut self) -> EFUSE_MEM_FORCE_PD_W<CLK_SPEC> {
        EFUSE_MEM_FORCE_PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - If set, forces to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W<CLK_SPEC> {
        MEM_CLK_FORCE_ON_W::new(self, 1)
    }
    #[doc = "Bit 2 - If set, forces eFuse SRAM into working mode."]
    #[inline(always)]
    #[must_use]
    pub fn efuse_mem_force_pu(&mut self) -> EFUSE_MEM_FORCE_PU_W<CLK_SPEC> {
        EFUSE_MEM_FORCE_PU_W::new(self, 2)
    }
    #[doc = "Bit 16 - If set, forces to enable clock signal of eFuse memory."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CLK_SPEC> {
        EN_W::new(self, 16)
    }
}
#[doc = "eFuse clock configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk::R`](R) reader structure"]
impl crate::Readable for CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk::W`](W) writer structure"]
impl crate::Writable for CLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK to value 0x02"]
impl crate::Resettable for CLK_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
