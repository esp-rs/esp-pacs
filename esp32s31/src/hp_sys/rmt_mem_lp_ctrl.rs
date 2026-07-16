#[doc = "Register `RMT_MEM_LP_CTRL` reader"]
pub type R = crate::R<RMT_MEM_LP_CTRL_SPEC>;
#[doc = "Register `RMT_MEM_LP_CTRL` writer"]
pub type W = crate::W<RMT_MEM_LP_CTRL_SPEC>;
#[doc = "Field `RMT_MEM_LP_MODE` reader - Configures rmt memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type RMT_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `RMT_MEM_LP_MODE` writer - Configures rmt memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type RMT_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RMT_MEM_LP_EN` reader - Set this bit to power down rmt memory."]
pub type RMT_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `RMT_MEM_LP_EN` writer - Set this bit to power down rmt memory."]
pub type RMT_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMT_MEM_LP_FORCE_CTRL` reader - Set this bit to force software control rmt memory, disbale hardware control."]
pub type RMT_MEM_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `RMT_MEM_LP_FORCE_CTRL` writer - Set this bit to force software control rmt memory, disbale hardware control."]
pub type RMT_MEM_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures rmt memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn rmt_mem_lp_mode(&self) -> RMT_MEM_LP_MODE_R {
        RMT_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to power down rmt memory."]
    #[inline(always)]
    pub fn rmt_mem_lp_en(&self) -> RMT_MEM_LP_EN_R {
        RMT_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force software control rmt memory, disbale hardware control."]
    #[inline(always)]
    pub fn rmt_mem_lp_force_ctrl(&self) -> RMT_MEM_LP_FORCE_CTRL_R {
        RMT_MEM_LP_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMT_MEM_LP_CTRL")
            .field("rmt_mem_lp_mode", &self.rmt_mem_lp_mode())
            .field("rmt_mem_lp_en", &self.rmt_mem_lp_en())
            .field("rmt_mem_lp_force_ctrl", &self.rmt_mem_lp_force_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures rmt memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn rmt_mem_lp_mode(&mut self) -> RMT_MEM_LP_MODE_W<'_, RMT_MEM_LP_CTRL_SPEC> {
        RMT_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to power down rmt memory."]
    #[inline(always)]
    pub fn rmt_mem_lp_en(&mut self) -> RMT_MEM_LP_EN_W<'_, RMT_MEM_LP_CTRL_SPEC> {
        RMT_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force software control rmt memory, disbale hardware control."]
    #[inline(always)]
    pub fn rmt_mem_lp_force_ctrl(&mut self) -> RMT_MEM_LP_FORCE_CTRL_W<'_, RMT_MEM_LP_CTRL_SPEC> {
        RMT_MEM_LP_FORCE_CTRL_W::new(self, 3)
    }
}
#[doc = "rmt memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMT_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for RMT_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmt_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for RMT_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmt_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for RMT_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RMT_MEM_LP_CTRL to value 0x06"]
impl crate::Resettable for RMT_MEM_LP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
