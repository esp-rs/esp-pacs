#[doc = "Register `MEM_CTRL` reader"]
pub type R = crate::R<MEM_CTRL_SPEC>;
#[doc = "Register `MEM_CTRL` writer"]
pub type W = crate::W<MEM_CTRL_SPEC>;
#[doc = "Field `EFUSE_MEM_LP_MODE` reader - Configures efuse memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type EFUSE_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `EFUSE_MEM_LP_MODE` writer - Configures efuse memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type EFUSE_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EFUSE_MEM_LP_EN` reader - Set this bit to power down efuse memory."]
pub type EFUSE_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `EFUSE_MEM_LP_EN` writer - Set this bit to power down efuse memory."]
pub type EFUSE_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_MEM_FORCE_CTRL` reader - Set this bit to force software control efuse memory, disbale hardware control."]
pub type EFUSE_MEM_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `EFUSE_MEM_FORCE_CTRL` writer - Set this bit to force software control efuse memory, disbale hardware control."]
pub type EFUSE_MEM_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUK_MEM_LP_MODE` reader - Configures huk memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type HUK_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `HUK_MEM_LP_MODE` writer - Configures huk memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
pub type HUK_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HUK_MEM_LP_EN` reader - Set this bit to power down huk memory."]
pub type HUK_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `HUK_MEM_LP_EN` writer - Set this bit to power down huk memory."]
pub type HUK_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUK_MEM_FORCE_CTRL` reader - Set this bit to force software control huk memory, disbale hardware control."]
pub type HUK_MEM_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `HUK_MEM_FORCE_CTRL` writer - Set this bit to force software control huk memory, disbale hardware control."]
pub type HUK_MEM_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures efuse memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn efuse_mem_lp_mode(&self) -> EFUSE_MEM_LP_MODE_R {
        EFUSE_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to power down efuse memory."]
    #[inline(always)]
    pub fn efuse_mem_lp_en(&self) -> EFUSE_MEM_LP_EN_R {
        EFUSE_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force software control efuse memory, disbale hardware control."]
    #[inline(always)]
    pub fn efuse_mem_force_ctrl(&self) -> EFUSE_MEM_FORCE_CTRL_R {
        EFUSE_MEM_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Configures huk memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn huk_mem_lp_mode(&self) -> HUK_MEM_LP_MODE_R {
        HUK_MEM_LP_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Set this bit to power down huk memory."]
    #[inline(always)]
    pub fn huk_mem_lp_en(&self) -> HUK_MEM_LP_EN_R {
        HUK_MEM_LP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to force software control huk memory, disbale hardware control."]
    #[inline(always)]
    pub fn huk_mem_force_ctrl(&self) -> HUK_MEM_FORCE_CTRL_R {
        HUK_MEM_FORCE_CTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CTRL")
            .field("efuse_mem_lp_mode", &self.efuse_mem_lp_mode())
            .field("efuse_mem_lp_en", &self.efuse_mem_lp_en())
            .field("efuse_mem_force_ctrl", &self.efuse_mem_force_ctrl())
            .field("huk_mem_lp_mode", &self.huk_mem_lp_mode())
            .field("huk_mem_lp_en", &self.huk_mem_lp_en())
            .field("huk_mem_force_ctrl", &self.huk_mem_force_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures efuse memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn efuse_mem_lp_mode(&mut self) -> EFUSE_MEM_LP_MODE_W<'_, MEM_CTRL_SPEC> {
        EFUSE_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to power down efuse memory."]
    #[inline(always)]
    pub fn efuse_mem_lp_en(&mut self) -> EFUSE_MEM_LP_EN_W<'_, MEM_CTRL_SPEC> {
        EFUSE_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force software control efuse memory, disbale hardware control."]
    #[inline(always)]
    pub fn efuse_mem_force_ctrl(&mut self) -> EFUSE_MEM_FORCE_CTRL_W<'_, MEM_CTRL_SPEC> {
        EFUSE_MEM_FORCE_CTRL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Configures huk memory low power mode in low power stage.\\\\ 0: deep sleep\\\\ 1: light sleep\\\\ 2(default): shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn huk_mem_lp_mode(&mut self) -> HUK_MEM_LP_MODE_W<'_, MEM_CTRL_SPEC> {
        HUK_MEM_LP_MODE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Set this bit to power down huk memory."]
    #[inline(always)]
    pub fn huk_mem_lp_en(&mut self) -> HUK_MEM_LP_EN_W<'_, MEM_CTRL_SPEC> {
        HUK_MEM_LP_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to force software control huk memory, disbale hardware control."]
    #[inline(always)]
    pub fn huk_mem_force_ctrl(&mut self) -> HUK_MEM_FORCE_CTRL_W<'_, MEM_CTRL_SPEC> {
        HUK_MEM_FORCE_CTRL_W::new(self, 7)
    }
}
#[doc = "configure rmemory power in lp system register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CTRL_SPEC;
impl crate::RegisterSpec for MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ctrl::R`](R) reader structure"]
impl crate::Readable for MEM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_ctrl::W`](W) writer structure"]
impl crate::Writable for MEM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CTRL to value 0x20"]
impl crate::Resettable for MEM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
