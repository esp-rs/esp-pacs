#[doc = "Register `POWER_VDD_FLASH` reader"]
pub type R = crate::R<POWER_VDD_FLASH_SPEC>;
#[doc = "Register `POWER_VDD_FLASH` writer"]
pub type W = crate::W<POWER_VDD_FLASH_SPEC>;
#[doc = "Field `FLASH_LDO_SW_EN_TIEL` reader - need_des"]
pub type FLASH_LDO_SW_EN_TIEL_R = crate::BitReader;
#[doc = "Field `FLASH_LDO_SW_EN_TIEL` writer - need_des"]
pub type FLASH_LDO_SW_EN_TIEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_LDO_POWER_SEL` reader - need_des"]
pub type FLASH_LDO_POWER_SEL_R = crate::BitReader;
#[doc = "Field `FLASH_LDO_POWER_SEL` writer - need_des"]
pub type FLASH_LDO_POWER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_LDO_SW_EN_POWER_SEL` reader - need_des"]
pub type FLASH_LDO_SW_EN_POWER_SEL_R = crate::BitReader;
#[doc = "Field `FLASH_LDO_SW_EN_POWER_SEL` writer - need_des"]
pub type FLASH_LDO_SW_EN_POWER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_LDO_WAIT_TARGET` reader - need_des"]
pub type FLASH_LDO_WAIT_TARGET_R = crate::FieldReader;
#[doc = "Field `FLASH_LDO_WAIT_TARGET` writer - need_des"]
pub type FLASH_LDO_WAIT_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLASH_LDO_TIEL_EN` reader - need_des"]
pub type FLASH_LDO_TIEL_EN_R = crate::BitReader;
#[doc = "Field `FLASH_LDO_TIEL_EN` writer - need_des"]
pub type FLASH_LDO_TIEL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_LDO_TIEL` reader - need_des"]
pub type FLASH_LDO_TIEL_R = crate::BitReader;
#[doc = "Field `FLASH_LDO_TIEL` writer - need_des"]
pub type FLASH_LDO_TIEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_LDO_SW_UPDATE` writer - need_des"]
pub type FLASH_LDO_SW_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_sw_en_tiel(&self) -> FLASH_LDO_SW_EN_TIEL_R {
        FLASH_LDO_SW_EN_TIEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_power_sel(&self) -> FLASH_LDO_POWER_SEL_R {
        FLASH_LDO_POWER_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_sw_en_power_sel(&self) -> FLASH_LDO_SW_EN_POWER_SEL_R {
        FLASH_LDO_SW_EN_POWER_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_wait_target(&self) -> FLASH_LDO_WAIT_TARGET_R {
        FLASH_LDO_WAIT_TARGET_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_tiel_en(&self) -> FLASH_LDO_TIEL_EN_R {
        FLASH_LDO_TIEL_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_tiel(&self) -> FLASH_LDO_TIEL_R {
        FLASH_LDO_TIEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_VDD_FLASH")
            .field("flash_ldo_sw_en_tiel", &self.flash_ldo_sw_en_tiel())
            .field("flash_ldo_power_sel", &self.flash_ldo_power_sel())
            .field(
                "flash_ldo_sw_en_power_sel",
                &self.flash_ldo_sw_en_power_sel(),
            )
            .field("flash_ldo_wait_target", &self.flash_ldo_wait_target())
            .field("flash_ldo_tiel_en", &self.flash_ldo_tiel_en())
            .field("flash_ldo_tiel", &self.flash_ldo_tiel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_sw_en_tiel(&mut self) -> FLASH_LDO_SW_EN_TIEL_W<'_, POWER_VDD_FLASH_SPEC> {
        FLASH_LDO_SW_EN_TIEL_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_power_sel(&mut self) -> FLASH_LDO_POWER_SEL_W<'_, POWER_VDD_FLASH_SPEC> {
        FLASH_LDO_POWER_SEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_sw_en_power_sel(
        &mut self,
    ) -> FLASH_LDO_SW_EN_POWER_SEL_W<'_, POWER_VDD_FLASH_SPEC> {
        FLASH_LDO_SW_EN_POWER_SEL_W::new(self, 24)
    }
    #[doc = "Bits 25:28 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_wait_target(&mut self) -> FLASH_LDO_WAIT_TARGET_W<'_, POWER_VDD_FLASH_SPEC> {
        FLASH_LDO_WAIT_TARGET_W::new(self, 25)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_tiel_en(&mut self) -> FLASH_LDO_TIEL_EN_W<'_, POWER_VDD_FLASH_SPEC> {
        FLASH_LDO_TIEL_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_tiel(&mut self) -> FLASH_LDO_TIEL_W<'_, POWER_VDD_FLASH_SPEC> {
        FLASH_LDO_TIEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn flash_ldo_sw_update(&mut self) -> FLASH_LDO_SW_UPDATE_W<'_, POWER_VDD_FLASH_SPEC> {
        FLASH_LDO_SW_UPDATE_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_vdd_flash::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_vdd_flash::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_VDD_FLASH_SPEC;
impl crate::RegisterSpec for POWER_VDD_FLASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_vdd_flash::R`](R) reader structure"]
impl crate::Readable for POWER_VDD_FLASH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_vdd_flash::W`](W) writer structure"]
impl crate::Writable for POWER_VDD_FLASH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_VDD_FLASH to value 0x1e00_0000"]
impl crate::Resettable for POWER_VDD_FLASH_SPEC {
    const RESET_VALUE: u32 = 0x1e00_0000;
}
