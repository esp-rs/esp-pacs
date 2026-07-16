#[doc = "Register `VDD_SDIO_CFG` reader"]
pub type R = crate::R<VDD_SDIO_CFG_SPEC>;
#[doc = "Register `VDD_SDIO_CFG` writer"]
pub type W = crate::W<VDD_SDIO_CFG_SPEC>;
#[doc = "Field `SDIO_SW_POWER_SEL` reader - need_des"]
pub type SDIO_SW_POWER_SEL_R = crate::BitReader;
#[doc = "Field `SDIO_SW_POWER_SEL` writer - need_des"]
pub type SDIO_SW_POWER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_POWER_SEL_SW_CTRL` reader - need_des"]
pub type SDIO_POWER_SEL_SW_CTRL_R = crate::BitReader;
#[doc = "Field `SDIO_POWER_SEL_SW_CTRL` writer - need_des"]
pub type SDIO_POWER_SEL_SW_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_SWITCH_TIME` reader - need_des"]
pub type SDIO_SWITCH_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `SDIO_SWITCH_TIME` writer - need_des"]
pub type SDIO_SWITCH_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sdio_sw_power_sel(&self) -> SDIO_SW_POWER_SEL_R {
        SDIO_SW_POWER_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn sdio_power_sel_sw_ctrl(&self) -> SDIO_POWER_SEL_SW_CTRL_R {
        SDIO_POWER_SEL_SW_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11 - need_des"]
    #[inline(always)]
    pub fn sdio_switch_time(&self) -> SDIO_SWITCH_TIME_R {
        SDIO_SWITCH_TIME_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDD_SDIO_CFG")
            .field("sdio_sw_power_sel", &self.sdio_sw_power_sel())
            .field("sdio_power_sel_sw_ctrl", &self.sdio_power_sel_sw_ctrl())
            .field("sdio_switch_time", &self.sdio_switch_time())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sdio_sw_power_sel(&mut self) -> SDIO_SW_POWER_SEL_W<'_, VDD_SDIO_CFG_SPEC> {
        SDIO_SW_POWER_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn sdio_power_sel_sw_ctrl(&mut self) -> SDIO_POWER_SEL_SW_CTRL_W<'_, VDD_SDIO_CFG_SPEC> {
        SDIO_POWER_SEL_SW_CTRL_W::new(self, 1)
    }
    #[doc = "Bits 2:11 - need_des"]
    #[inline(always)]
    pub fn sdio_switch_time(&mut self) -> SDIO_SWITCH_TIME_W<'_, VDD_SDIO_CFG_SPEC> {
        SDIO_SWITCH_TIME_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_sdio_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdd_sdio_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDD_SDIO_CFG_SPEC;
impl crate::RegisterSpec for VDD_SDIO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_sdio_cfg::R`](R) reader structure"]
impl crate::Readable for VDD_SDIO_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vdd_sdio_cfg::W`](W) writer structure"]
impl crate::Writable for VDD_SDIO_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDD_SDIO_CFG to value 0x28"]
impl crate::Resettable for VDD_SDIO_CFG_SPEC {
    const RESET_VALUE: u32 = 0x28;
}
