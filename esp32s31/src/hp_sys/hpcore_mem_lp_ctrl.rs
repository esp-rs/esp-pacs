#[doc = "Register `HPCORE_MEM_LP_CTRL` reader"]
pub type R = crate::R<HPCORE_MEM_LP_CTRL_SPEC>;
#[doc = "Register `HPCORE_MEM_LP_CTRL` writer"]
pub type W = crate::W<HPCORE_MEM_LP_CTRL_SPEC>;
#[doc = "Field `HPCORE_MEM_LP_MODE` reader - Configures hpcore memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type HPCORE_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `HPCORE_MEM_LP_MODE` writer - Configures hpcore memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type HPCORE_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HPCORE_MEM_LP_EN` reader - Set this bit to power down hpcore memory."]
pub type HPCORE_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `HPCORE_MEM_LP_EN` writer - Set this bit to power down hpcore memory."]
pub type HPCORE_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCORE_MEM_LP_FORCE_CTRL` reader - Set this bit to force software control hpcore memory, disbale hardware control."]
pub type HPCORE_MEM_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `HPCORE_MEM_LP_FORCE_CTRL` writer - Set this bit to force software control hpcore memory, disbale hardware control."]
pub type HPCORE_MEM_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures hpcore memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hpcore_mem_lp_mode(&self) -> HPCORE_MEM_LP_MODE_R {
        HPCORE_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to power down hpcore memory."]
    #[inline(always)]
    pub fn hpcore_mem_lp_en(&self) -> HPCORE_MEM_LP_EN_R {
        HPCORE_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force software control hpcore memory, disbale hardware control."]
    #[inline(always)]
    pub fn hpcore_mem_lp_force_ctrl(&self) -> HPCORE_MEM_LP_FORCE_CTRL_R {
        HPCORE_MEM_LP_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPCORE_MEM_LP_CTRL")
            .field("hpcore_mem_lp_mode", &self.hpcore_mem_lp_mode())
            .field("hpcore_mem_lp_en", &self.hpcore_mem_lp_en())
            .field("hpcore_mem_lp_force_ctrl", &self.hpcore_mem_lp_force_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures hpcore memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn hpcore_mem_lp_mode(&mut self) -> HPCORE_MEM_LP_MODE_W<'_, HPCORE_MEM_LP_CTRL_SPEC> {
        HPCORE_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to power down hpcore memory."]
    #[inline(always)]
    pub fn hpcore_mem_lp_en(&mut self) -> HPCORE_MEM_LP_EN_W<'_, HPCORE_MEM_LP_CTRL_SPEC> {
        HPCORE_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force software control hpcore memory, disbale hardware control."]
    #[inline(always)]
    pub fn hpcore_mem_lp_force_ctrl(
        &mut self,
    ) -> HPCORE_MEM_LP_FORCE_CTRL_W<'_, HPCORE_MEM_LP_CTRL_SPEC> {
        HPCORE_MEM_LP_FORCE_CTRL_W::new(self, 3)
    }
}
#[doc = "HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPCORE_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for HPCORE_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpcore_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for HPCORE_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpcore_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for HPCORE_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPCORE_MEM_LP_CTRL to value 0"]
impl crate::Resettable for HPCORE_MEM_LP_CTRL_SPEC {}
