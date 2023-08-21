#[doc = "Register `CLK` reader"]
pub type R = crate::R<CLK_SPEC>;
#[doc = "Register `CLK` writer"]
pub type W = crate::W<CLK_SPEC>;
#[doc = "Field `EFUSE_MEM_FORCE_PD` reader - Set this bit to force eFuse SRAM into power-saving mode."]
pub type EFUSE_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `EFUSE_MEM_FORCE_PD` writer - Set this bit to force eFuse SRAM into power-saving mode."]
pub type EFUSE_MEM_FORCE_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - Set this bit and force to activate clock signal of eFuse SRAM."]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - Set this bit and force to activate clock signal of eFuse SRAM."]
pub type MEM_CLK_FORCE_ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EFUSE_MEM_FORCE_PU` reader - Set this bit to force eFuse SRAM into working mode."]
pub type EFUSE_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `EFUSE_MEM_FORCE_PU` writer - Set this bit to force eFuse SRAM into working mode."]
pub type EFUSE_MEM_FORCE_PU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN` reader - Set this bit and force to enable clock signal of eFuse memory."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Set this bit and force to enable clock signal of eFuse memory."]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to force eFuse SRAM into power-saving mode."]
    #[inline(always)]
    pub fn efuse_mem_force_pd(&self) -> EFUSE_MEM_FORCE_PD_R {
        EFUSE_MEM_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit and force to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force eFuse SRAM into working mode."]
    #[inline(always)]
    pub fn efuse_mem_force_pu(&self) -> EFUSE_MEM_FORCE_PU_R {
        EFUSE_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit and force to enable clock signal of eFuse memory."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK")
            .field(
                "efuse_mem_force_pd",
                &format_args!("{}", self.efuse_mem_force_pd().bit()),
            )
            .field(
                "mem_clk_force_on",
                &format_args!("{}", self.mem_clk_force_on().bit()),
            )
            .field(
                "efuse_mem_force_pu",
                &format_args!("{}", self.efuse_mem_force_pu().bit()),
            )
            .field("en", &format_args!("{}", self.en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force eFuse SRAM into power-saving mode."]
    #[inline(always)]
    #[must_use]
    pub fn efuse_mem_force_pd(&mut self) -> EFUSE_MEM_FORCE_PD_W<CLK_SPEC, 0> {
        EFUSE_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit and force to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W<CLK_SPEC, 1> {
        MEM_CLK_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to force eFuse SRAM into working mode."]
    #[inline(always)]
    #[must_use]
    pub fn efuse_mem_force_pu(&mut self) -> EFUSE_MEM_FORCE_PU_W<CLK_SPEC, 2> {
        EFUSE_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit and force to enable clock signal of eFuse memory."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CLK_SPEC, 16> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK to value 0x02"]
impl crate::Resettable for CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
