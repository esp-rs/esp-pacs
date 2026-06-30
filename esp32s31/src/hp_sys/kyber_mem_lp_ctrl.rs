#[doc = "Register `KYBER_MEM_LP_CTRL` reader"]
pub type R = crate::R<KYBER_MEM_LP_CTRL_SPEC>;
#[doc = "Register `KYBER_MEM_LP_CTRL` writer"]
pub type W = crate::W<KYBER_MEM_LP_CTRL_SPEC>;
#[doc = "Field `KYBER_MEM_LP_MODE` reader - Configures kyber memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type KYBER_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `KYBER_MEM_LP_MODE` writer - Configures kyber memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type KYBER_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KYBER_MEM_LP_EN` reader - Set this bit to power down kyber memory."]
pub type KYBER_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `KYBER_MEM_LP_EN` writer - Set this bit to power down kyber memory."]
pub type KYBER_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KYBER_MEM_LP_FORCE_CTRL` reader - Set this bit to force software control kyber memory, disbale hardware control."]
pub type KYBER_MEM_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `KYBER_MEM_LP_FORCE_CTRL` writer - Set this bit to force software control kyber memory, disbale hardware control."]
pub type KYBER_MEM_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures kyber memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn kyber_mem_lp_mode(&self) -> KYBER_MEM_LP_MODE_R {
        KYBER_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to power down kyber memory."]
    #[inline(always)]
    pub fn kyber_mem_lp_en(&self) -> KYBER_MEM_LP_EN_R {
        KYBER_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force software control kyber memory, disbale hardware control."]
    #[inline(always)]
    pub fn kyber_mem_lp_force_ctrl(&self) -> KYBER_MEM_LP_FORCE_CTRL_R {
        KYBER_MEM_LP_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KYBER_MEM_LP_CTRL")
            .field("kyber_mem_lp_mode", &self.kyber_mem_lp_mode())
            .field("kyber_mem_lp_en", &self.kyber_mem_lp_en())
            .field("kyber_mem_lp_force_ctrl", &self.kyber_mem_lp_force_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures kyber memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn kyber_mem_lp_mode(&mut self) -> KYBER_MEM_LP_MODE_W<'_, KYBER_MEM_LP_CTRL_SPEC> {
        KYBER_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to power down kyber memory."]
    #[inline(always)]
    pub fn kyber_mem_lp_en(&mut self) -> KYBER_MEM_LP_EN_W<'_, KYBER_MEM_LP_CTRL_SPEC> {
        KYBER_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force software control kyber memory, disbale hardware control."]
    #[inline(always)]
    pub fn kyber_mem_lp_force_ctrl(
        &mut self,
    ) -> KYBER_MEM_LP_FORCE_CTRL_W<'_, KYBER_MEM_LP_CTRL_SPEC> {
        KYBER_MEM_LP_FORCE_CTRL_W::new(self, 3)
    }
}
#[doc = "HP CORE0 & HP CORE1 memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`kyber_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kyber_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KYBER_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for KYBER_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`kyber_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for KYBER_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`kyber_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for KYBER_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KYBER_MEM_LP_CTRL to value 0x06"]
impl crate::Resettable for KYBER_MEM_LP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
