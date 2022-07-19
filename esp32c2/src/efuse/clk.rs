#[doc = "Register `CLK` reader"]
pub struct R(crate::R<CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK` writer"]
pub struct W(crate::W<CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFUSE_MEM_FORCE_PD` reader - Set this bit to force eFuse SRAM into power-saving mode."]
pub type EFUSE_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_MEM_FORCE_PD` writer - Set this bit to force eFuse SRAM into power-saving mode."]
pub type EFUSE_MEM_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, CLK_SPEC, bool, 0>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - Set this bit and force to activate clock signal of eFuse SRAM."]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - Set this bit and force to activate clock signal of eFuse SRAM."]
pub type MEM_CLK_FORCE_ON_W<'a> = crate::BitWriter<'a, u32, CLK_SPEC, bool, 1>;
#[doc = "Field `EFUSE_MEM_FORCE_PU` reader - Set this bit to force eFuse SRAM into working mode."]
pub type EFUSE_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_MEM_FORCE_PU` writer - Set this bit to force eFuse SRAM into working mode."]
pub type EFUSE_MEM_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, CLK_SPEC, bool, 2>;
#[doc = "Field `EN` reader - Set this bit and force to enable clock signal of eFuse memory."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Set this bit and force to enable clock signal of eFuse memory."]
pub type EN_W<'a> = crate::BitWriter<'a, u32, CLK_SPEC, bool, 16>;
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
impl W {
    #[doc = "Bit 0 - Set this bit to force eFuse SRAM into power-saving mode."]
    #[inline(always)]
    pub fn efuse_mem_force_pd(&mut self) -> EFUSE_MEM_FORCE_PD_W {
        EFUSE_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit and force to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W {
        MEM_CLK_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to force eFuse SRAM into working mode."]
    #[inline(always)]
    pub fn efuse_mem_force_pu(&mut self) -> EFUSE_MEM_FORCE_PU_W {
        EFUSE_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit and force to enable clock signal of eFuse memory."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eFuse clcok configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](index.html) module"]
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk::R](R) reader structure"]
impl crate::Readable for CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk::W](W) writer structure"]
impl crate::Writable for CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK to value 0x02"]
impl crate::Resettable for CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
