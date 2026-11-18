#[doc = "Register `SRM_MEM_PD` reader"]
pub type R = crate::R<SRM_MEM_PD_SPEC>;
#[doc = "Register `SRM_MEM_PD` writer"]
pub type W = crate::W<SRM_MEM_PD_SPEC>;
#[doc = "Field `SRM_MEM_CLK_ENA` reader - Set this bit to force clock enable of scaling and rotating engine's data memory."]
pub type SRM_MEM_CLK_ENA_R = crate::BitReader;
#[doc = "Field `SRM_MEM_CLK_ENA` writer - Set this bit to force clock enable of scaling and rotating engine's data memory."]
pub type SRM_MEM_CLK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRM_MEM_FORCE_PD` reader - Set this bit to force power down scaling and rotating engine's data memory."]
pub type SRM_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `SRM_MEM_FORCE_PD` writer - Set this bit to force power down scaling and rotating engine's data memory."]
pub type SRM_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRM_MEM_FORCE_PU` reader - Set this bit to force power up scaling and rotating engine's data memory."]
pub type SRM_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `SRM_MEM_FORCE_PU` writer - Set this bit to force power up scaling and rotating engine's data memory."]
pub type SRM_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force clock enable of scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn srm_mem_clk_ena(&self) -> SRM_MEM_CLK_ENA_R {
        SRM_MEM_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force power down scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn srm_mem_force_pd(&self) -> SRM_MEM_FORCE_PD_R {
        SRM_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power up scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn srm_mem_force_pu(&self) -> SRM_MEM_FORCE_PU_R {
        SRM_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRM_MEM_PD")
            .field("srm_mem_clk_ena", &self.srm_mem_clk_ena())
            .field("srm_mem_force_pd", &self.srm_mem_force_pd())
            .field("srm_mem_force_pu", &self.srm_mem_force_pu())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force clock enable of scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn srm_mem_clk_ena(&mut self) -> SRM_MEM_CLK_ENA_W<'_, SRM_MEM_PD_SPEC> {
        SRM_MEM_CLK_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force power down scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn srm_mem_force_pd(&mut self) -> SRM_MEM_FORCE_PD_W<'_, SRM_MEM_PD_SPEC> {
        SRM_MEM_FORCE_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power up scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn srm_mem_force_pu(&mut self) -> SRM_MEM_FORCE_PU_W<'_, SRM_MEM_PD_SPEC> {
        SRM_MEM_FORCE_PU_W::new(self, 2)
    }
}
#[doc = "SR memory power done register\n\nYou can [`read`](crate::Reg::read) this register and get [`srm_mem_pd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srm_mem_pd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRM_MEM_PD_SPEC;
impl crate::RegisterSpec for SRM_MEM_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srm_mem_pd::R`](R) reader structure"]
impl crate::Readable for SRM_MEM_PD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srm_mem_pd::W`](W) writer structure"]
impl crate::Writable for SRM_MEM_PD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRM_MEM_PD to value 0"]
impl crate::Resettable for SRM_MEM_PD_SPEC {}
