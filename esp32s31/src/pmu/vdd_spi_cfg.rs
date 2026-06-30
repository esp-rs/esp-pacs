#[doc = "Register `VDD_SPI_CFG` reader"]
pub type R = crate::R<VDD_SPI_CFG_SPEC>;
#[doc = "Register `VDD_SPI_CFG` writer"]
pub type W = crate::W<VDD_SPI_CFG_SPEC>;
#[doc = "Field `FLASH_SW_POWER_SEL` reader - need_des"]
pub type FLASH_SW_POWER_SEL_R = crate::BitReader;
#[doc = "Field `FLASH_SW_POWER_SEL` writer - need_des"]
pub type FLASH_SW_POWER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_POWER_SEL_SW_CTRL` reader - need_des"]
pub type FLASH_POWER_SEL_SW_CTRL_R = crate::BitReader;
#[doc = "Field `FLASH_POWER_SEL_SW_CTRL` writer - need_des"]
pub type FLASH_POWER_SEL_SW_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_SWITCH_TIME` reader - need_des"]
pub type FLASH_SWITCH_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_SWITCH_TIME` writer - need_des"]
pub type FLASH_SWITCH_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `FLASH_STABLE_TIME` reader - need_des"]
pub type FLASH_STABLE_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_STABLE_TIME` writer - need_des"]
pub type FLASH_STABLE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `FLASH_COOLDOWN_TIME` reader - need_des"]
pub type FLASH_COOLDOWN_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_COOLDOWN_TIME` writer - need_des"]
pub type FLASH_COOLDOWN_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn flash_sw_power_sel(&self) -> FLASH_SW_POWER_SEL_R {
        FLASH_SW_POWER_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn flash_power_sel_sw_ctrl(&self) -> FLASH_POWER_SEL_SW_CTRL_R {
        FLASH_POWER_SEL_SW_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11 - need_des"]
    #[inline(always)]
    pub fn flash_switch_time(&self) -> FLASH_SWITCH_TIME_R {
        FLASH_SWITCH_TIME_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    pub fn flash_stable_time(&self) -> FLASH_STABLE_TIME_R {
        FLASH_STABLE_TIME_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn flash_cooldown_time(&self) -> FLASH_COOLDOWN_TIME_R {
        FLASH_COOLDOWN_TIME_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDD_SPI_CFG")
            .field("flash_sw_power_sel", &self.flash_sw_power_sel())
            .field("flash_power_sel_sw_ctrl", &self.flash_power_sel_sw_ctrl())
            .field("flash_switch_time", &self.flash_switch_time())
            .field("flash_stable_time", &self.flash_stable_time())
            .field("flash_cooldown_time", &self.flash_cooldown_time())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn flash_sw_power_sel(&mut self) -> FLASH_SW_POWER_SEL_W<'_, VDD_SPI_CFG_SPEC> {
        FLASH_SW_POWER_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn flash_power_sel_sw_ctrl(&mut self) -> FLASH_POWER_SEL_SW_CTRL_W<'_, VDD_SPI_CFG_SPEC> {
        FLASH_POWER_SEL_SW_CTRL_W::new(self, 1)
    }
    #[doc = "Bits 2:11 - need_des"]
    #[inline(always)]
    pub fn flash_switch_time(&mut self) -> FLASH_SWITCH_TIME_W<'_, VDD_SPI_CFG_SPEC> {
        FLASH_SWITCH_TIME_W::new(self, 2)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    pub fn flash_stable_time(&mut self) -> FLASH_STABLE_TIME_W<'_, VDD_SPI_CFG_SPEC> {
        FLASH_STABLE_TIME_W::new(self, 12)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn flash_cooldown_time(&mut self) -> FLASH_COOLDOWN_TIME_W<'_, VDD_SPI_CFG_SPEC> {
        FLASH_COOLDOWN_TIME_W::new(self, 22)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_spi_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdd_spi_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDD_SPI_CFG_SPEC;
impl crate::RegisterSpec for VDD_SPI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_spi_cfg::R`](R) reader structure"]
impl crate::Readable for VDD_SPI_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vdd_spi_cfg::W`](W) writer structure"]
impl crate::Writable for VDD_SPI_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDD_SPI_CFG to value 0x0288_04b0"]
impl crate::Resettable for VDD_SPI_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0288_04b0;
}
