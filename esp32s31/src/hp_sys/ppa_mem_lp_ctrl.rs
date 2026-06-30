#[doc = "Register `PPA_MEM_LP_CTRL` reader"]
pub type R = crate::R<PPA_MEM_LP_CTRL_SPEC>;
#[doc = "Register `PPA_MEM_LP_CTRL` writer"]
pub type W = crate::W<PPA_MEM_LP_CTRL_SPEC>;
#[doc = "Field `PPA_SPRF_MEM_LP_MODE` reader - Configures ppa_sprf memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type PPA_SPRF_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `PPA_SPRF_MEM_LP_MODE` writer - Configures ppa_sprf memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type PPA_SPRF_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PPA_SPRF_MEM_LP_EN` reader - Set this bit to power down ppa_sprf memory."]
pub type PPA_SPRF_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `PPA_SPRF_MEM_LP_EN` writer - Set this bit to power down ppa_sprf memory."]
pub type PPA_SPRF_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPA_SPRF_MEM_FORCE_CTRL` reader - Set this bit to force software control ppa_sprf memory, disbale hardware control."]
pub type PPA_SPRF_MEM_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `PPA_SPRF_MEM_FORCE_CTRL` writer - Set this bit to force software control ppa_sprf memory, disbale hardware control."]
pub type PPA_SPRF_MEM_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures ppa_sprf memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn ppa_sprf_mem_lp_mode(&self) -> PPA_SPRF_MEM_LP_MODE_R {
        PPA_SPRF_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to power down ppa_sprf memory."]
    #[inline(always)]
    pub fn ppa_sprf_mem_lp_en(&self) -> PPA_SPRF_MEM_LP_EN_R {
        PPA_SPRF_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force software control ppa_sprf memory, disbale hardware control."]
    #[inline(always)]
    pub fn ppa_sprf_mem_force_ctrl(&self) -> PPA_SPRF_MEM_FORCE_CTRL_R {
        PPA_SPRF_MEM_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPA_MEM_LP_CTRL")
            .field("ppa_sprf_mem_lp_mode", &self.ppa_sprf_mem_lp_mode())
            .field("ppa_sprf_mem_lp_en", &self.ppa_sprf_mem_lp_en())
            .field("ppa_sprf_mem_force_ctrl", &self.ppa_sprf_mem_force_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures ppa_sprf memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn ppa_sprf_mem_lp_mode(&mut self) -> PPA_SPRF_MEM_LP_MODE_W<'_, PPA_MEM_LP_CTRL_SPEC> {
        PPA_SPRF_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to power down ppa_sprf memory."]
    #[inline(always)]
    pub fn ppa_sprf_mem_lp_en(&mut self) -> PPA_SPRF_MEM_LP_EN_W<'_, PPA_MEM_LP_CTRL_SPEC> {
        PPA_SPRF_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force software control ppa_sprf memory, disbale hardware control."]
    #[inline(always)]
    pub fn ppa_sprf_mem_force_ctrl(
        &mut self,
    ) -> PPA_SPRF_MEM_FORCE_CTRL_W<'_, PPA_MEM_LP_CTRL_SPEC> {
        PPA_SPRF_MEM_FORCE_CTRL_W::new(self, 3)
    }
}
#[doc = "vpu memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppa_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppa_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPA_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for PPA_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppa_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for PPA_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppa_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for PPA_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPA_MEM_LP_CTRL to value 0x06"]
impl crate::Resettable for PPA_MEM_LP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
