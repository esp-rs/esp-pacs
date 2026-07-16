#[doc = "Register `SYS_HP_SDMMC_CTRL` reader"]
pub type R = crate::R<SYS_HP_SDMMC_CTRL_SPEC>;
#[doc = "Register `SYS_HP_SDMMC_CTRL` writer"]
pub type W = crate::W<SYS_HP_SDMMC_CTRL_SPEC>;
#[doc = "Field `SYS_SDMMC_AHB2AXI_POST_WRITE_EN` reader - "]
pub type SYS_SDMMC_AHB2AXI_POST_WRITE_EN_R = crate::BitReader;
#[doc = "Field `SYS_SDMMC_AHB2AXI_POST_WRITE_EN` writer - "]
pub type SYS_SDMMC_AHB2AXI_POST_WRITE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_SDMMC_RST_EN` reader - "]
pub type SYS_SDMMC_RST_EN_R = crate::BitReader;
#[doc = "Field `SYS_SDMMC_RST_EN` writer - "]
pub type SYS_SDMMC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_SDMMC_FORCE_NORST` reader - "]
pub type SYS_SDMMC_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `SYS_SDMMC_FORCE_NORST` writer - "]
pub type SYS_SDMMC_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sys_sdmmc_ahb2axi_post_write_en(&self) -> SYS_SDMMC_AHB2AXI_POST_WRITE_EN_R {
        SYS_SDMMC_AHB2AXI_POST_WRITE_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_sdmmc_rst_en(&self) -> SYS_SDMMC_RST_EN_R {
        SYS_SDMMC_RST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_sdmmc_force_norst(&self) -> SYS_SDMMC_FORCE_NORST_R {
        SYS_SDMMC_FORCE_NORST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_HP_SDMMC_CTRL")
            .field(
                "sys_sdmmc_ahb2axi_post_write_en",
                &self.sys_sdmmc_ahb2axi_post_write_en(),
            )
            .field("sys_sdmmc_rst_en", &self.sys_sdmmc_rst_en())
            .field("sys_sdmmc_force_norst", &self.sys_sdmmc_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sys_sdmmc_ahb2axi_post_write_en(
        &mut self,
    ) -> SYS_SDMMC_AHB2AXI_POST_WRITE_EN_W<'_, SYS_HP_SDMMC_CTRL_SPEC> {
        SYS_SDMMC_AHB2AXI_POST_WRITE_EN_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_sdmmc_rst_en(&mut self) -> SYS_SDMMC_RST_EN_W<'_, SYS_HP_SDMMC_CTRL_SPEC> {
        SYS_SDMMC_RST_EN_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_sdmmc_force_norst(&mut self) -> SYS_SDMMC_FORCE_NORST_W<'_, SYS_HP_SDMMC_CTRL_SPEC> {
        SYS_SDMMC_FORCE_NORST_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_sdmmc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_sdmmc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_HP_SDMMC_CTRL_SPEC;
impl crate::RegisterSpec for SYS_HP_SDMMC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_hp_sdmmc_ctrl::R`](R) reader structure"]
impl crate::Readable for SYS_HP_SDMMC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_hp_sdmmc_ctrl::W`](W) writer structure"]
impl crate::Writable for SYS_HP_SDMMC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_HP_SDMMC_CTRL to value 0"]
impl crate::Resettable for SYS_HP_SDMMC_CTRL_SPEC {}
