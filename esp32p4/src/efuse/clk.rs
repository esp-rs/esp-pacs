#[doc = "Register `CLK` reader"]
pub type R = crate::R<CLK_SPEC>;
#[doc = "Register `CLK` writer"]
pub type W = crate::W<CLK_SPEC>;
#[doc = "Field `MEM_FORCE_PD` reader - Set this bit to force eFuse SRAM into power-saving mode."]
pub type MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PD` writer - Set this bit to force eFuse SRAM into power-saving mode."]
pub type MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - Set this bit and force to activate clock signal of eFuse SRAM."]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - Set this bit and force to activate clock signal of eFuse SRAM."]
pub type MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FORCE_PU` reader - Set this bit to force eFuse SRAM into working mode."]
pub type MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PU` writer - Set this bit to force eFuse SRAM into working mode."]
pub type MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Set this bit to force enable eFuse register configuration clock signal."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Set this bit to force enable eFuse register configuration clock signal."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force eFuse SRAM into power-saving mode."]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit and force to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force eFuse SRAM into working mode."]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to force enable eFuse register configuration clock signal."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK")
            .field("mem_force_pd", &self.mem_force_pd())
            .field("mem_clk_force_on", &self.mem_clk_force_on())
            .field("mem_force_pu", &self.mem_force_pu())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force eFuse SRAM into power-saving mode."]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W<CLK_SPEC> {
        MEM_FORCE_PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit and force to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W<CLK_SPEC> {
        MEM_CLK_FORCE_ON_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force eFuse SRAM into working mode."]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W<CLK_SPEC> {
        MEM_FORCE_PU_W::new(self, 2)
    }
    #[doc = "Bit 16 - Set this bit to force enable eFuse register configuration clock signal."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<CLK_SPEC> {
        EN_W::new(self, 16)
    }
}
#[doc = "eFuse clcok configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for CLK_SPEC {
    const RESET_VALUE: u32 = 0;
}
