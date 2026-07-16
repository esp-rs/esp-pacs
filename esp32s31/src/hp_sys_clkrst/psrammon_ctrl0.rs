#[doc = "Register `PSRAMMON_CTRL0` reader"]
pub type R = crate::R<PSRAMMON_CTRL0_SPEC>;
#[doc = "Register `PSRAMMON_CTRL0` writer"]
pub type W = crate::W<PSRAMMON_CTRL0_SPEC>;
#[doc = "Field `PSRAM_MON_SYS_CLK_EN` reader - need_des"]
pub type PSRAM_MON_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `PSRAM_MON_SYS_CLK_EN` writer - need_des"]
pub type PSRAM_MON_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_MON_RST_EN` reader - need_des"]
pub type PSRAM_MON_RST_EN_R = crate::BitReader;
#[doc = "Field `PSRAM_MON_RST_EN` writer - need_des"]
pub type PSRAM_MON_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_MON_FORCE_NORST` reader - need_des"]
pub type PSRAM_MON_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `PSRAM_MON_FORCE_NORST` writer - need_des"]
pub type PSRAM_MON_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn psram_mon_sys_clk_en(&self) -> PSRAM_MON_SYS_CLK_EN_R {
        PSRAM_MON_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn psram_mon_rst_en(&self) -> PSRAM_MON_RST_EN_R {
        PSRAM_MON_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn psram_mon_force_norst(&self) -> PSRAM_MON_FORCE_NORST_R {
        PSRAM_MON_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSRAMMON_CTRL0")
            .field("psram_mon_sys_clk_en", &self.psram_mon_sys_clk_en())
            .field("psram_mon_rst_en", &self.psram_mon_rst_en())
            .field("psram_mon_force_norst", &self.psram_mon_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn psram_mon_sys_clk_en(&mut self) -> PSRAM_MON_SYS_CLK_EN_W<'_, PSRAMMON_CTRL0_SPEC> {
        PSRAM_MON_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn psram_mon_rst_en(&mut self) -> PSRAM_MON_RST_EN_W<'_, PSRAMMON_CTRL0_SPEC> {
        PSRAM_MON_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn psram_mon_force_norst(&mut self) -> PSRAM_MON_FORCE_NORST_W<'_, PSRAMMON_CTRL0_SPEC> {
        PSRAM_MON_FORCE_NORST_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`psrammon_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psrammon_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRAMMON_CTRL0_SPEC;
impl crate::RegisterSpec for PSRAMMON_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psrammon_ctrl0::R`](R) reader structure"]
impl crate::Readable for PSRAMMON_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psrammon_ctrl0::W`](W) writer structure"]
impl crate::Writable for PSRAMMON_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAMMON_CTRL0 to value 0"]
impl crate::Resettable for PSRAMMON_CTRL0_SPEC {}
