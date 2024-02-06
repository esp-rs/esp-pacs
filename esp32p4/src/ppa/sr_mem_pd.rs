#[doc = "Register `SR_MEM_PD` reader"]
pub type R = crate::R<SR_MEM_PD_SPEC>;
#[doc = "Register `SR_MEM_PD` writer"]
pub type W = crate::W<SR_MEM_PD_SPEC>;
#[doc = "Field `SR_MEM_CLK_ENA` reader - Set this bit to force clock enable of scaling and rotating engine's data memory."]
pub type SR_MEM_CLK_ENA_R = crate::BitReader;
#[doc = "Field `SR_MEM_CLK_ENA` writer - Set this bit to force clock enable of scaling and rotating engine's data memory."]
pub type SR_MEM_CLK_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_MEM_FORCE_PD` reader - Set this bit to force power down scaling and rotating engine's data memory."]
pub type SR_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `SR_MEM_FORCE_PD` writer - Set this bit to force power down scaling and rotating engine's data memory."]
pub type SR_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_MEM_FORCE_PU` reader - Set this bit to force power up scaling and rotating engine's data memory."]
pub type SR_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `SR_MEM_FORCE_PU` writer - Set this bit to force power up scaling and rotating engine's data memory."]
pub type SR_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force clock enable of scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn sr_mem_clk_ena(&self) -> SR_MEM_CLK_ENA_R {
        SR_MEM_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force power down scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn sr_mem_force_pd(&self) -> SR_MEM_FORCE_PD_R {
        SR_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power up scaling and rotating engine's data memory."]
    #[inline(always)]
    pub fn sr_mem_force_pu(&self) -> SR_MEM_FORCE_PU_R {
        SR_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_MEM_PD")
            .field(
                "sr_mem_clk_ena",
                &format_args!("{}", self.sr_mem_clk_ena().bit()),
            )
            .field(
                "sr_mem_force_pd",
                &format_args!("{}", self.sr_mem_force_pd().bit()),
            )
            .field(
                "sr_mem_force_pu",
                &format_args!("{}", self.sr_mem_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SR_MEM_PD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force clock enable of scaling and rotating engine's data memory."]
    #[inline(always)]
    #[must_use]
    pub fn sr_mem_clk_ena(&mut self) -> SR_MEM_CLK_ENA_W<SR_MEM_PD_SPEC> {
        SR_MEM_CLK_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force power down scaling and rotating engine's data memory."]
    #[inline(always)]
    #[must_use]
    pub fn sr_mem_force_pd(&mut self) -> SR_MEM_FORCE_PD_W<SR_MEM_PD_SPEC> {
        SR_MEM_FORCE_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power up scaling and rotating engine's data memory."]
    #[inline(always)]
    #[must_use]
    pub fn sr_mem_force_pu(&mut self) -> SR_MEM_FORCE_PU_W<SR_MEM_PD_SPEC> {
        SR_MEM_FORCE_PU_W::new(self, 2)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SR memory power done register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr_mem_pd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_mem_pd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_MEM_PD_SPEC;
impl crate::RegisterSpec for SR_MEM_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_mem_pd::R`](R) reader structure"]
impl crate::Readable for SR_MEM_PD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr_mem_pd::W`](W) writer structure"]
impl crate::Writable for SR_MEM_PD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR_MEM_PD to value 0"]
impl crate::Resettable for SR_MEM_PD_SPEC {
    const RESET_VALUE: u32 = 0;
}
