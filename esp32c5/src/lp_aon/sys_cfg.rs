#[doc = "Register `SYS_CFG` reader"]
pub type R = crate::R<SYS_CFG_SPEC>;
#[doc = "Register `SYS_CFG` writer"]
pub type W = crate::W<SYS_CFG_SPEC>;
#[doc = "Field `FORCE_DOWNLOAD_BOOT_STATUS` reader - get force download mode status"]
pub type FORCE_DOWNLOAD_BOOT_STATUS_R = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` reader - enable chip entry download mode or not 1: enable 0: no operation"]
pub type FORCE_DOWNLOAD_BOOT_R = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` writer - enable chip entry download mode or not 1: enable 0: no operation"]
pub type FORCE_DOWNLOAD_BOOT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPSYS_SW_RESET` writer - enable hp system reset by software or not 1: reset 0: no operation"]
pub type HPSYS_SW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29 - get force download mode status"]
    #[inline(always)]
    pub fn force_download_boot_status(&self) -> FORCE_DOWNLOAD_BOOT_STATUS_R {
        FORCE_DOWNLOAD_BOOT_STATUS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable chip entry download mode or not 1: enable 0: no operation"]
    #[inline(always)]
    pub fn force_download_boot(&self) -> FORCE_DOWNLOAD_BOOT_R {
        FORCE_DOWNLOAD_BOOT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_CFG")
            .field(
                "force_download_boot_status",
                &self.force_download_boot_status(),
            )
            .field("force_download_boot", &self.force_download_boot())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - enable chip entry download mode or not 1: enable 0: no operation"]
    #[inline(always)]
    pub fn force_download_boot(&mut self) -> FORCE_DOWNLOAD_BOOT_W<SYS_CFG_SPEC> {
        FORCE_DOWNLOAD_BOOT_W::new(self, 30)
    }
    #[doc = "Bit 31 - enable hp system reset by software or not 1: reset 0: no operation"]
    #[inline(always)]
    pub fn hpsys_sw_reset(&mut self) -> HPSYS_SW_RESET_W<SYS_CFG_SPEC> {
        HPSYS_SW_RESET_W::new(self, 31)
    }
}
#[doc = "configure system register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_CFG_SPEC;
impl crate::RegisterSpec for SYS_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_cfg::R`](R) reader structure"]
impl crate::Readable for SYS_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_cfg::W`](W) writer structure"]
impl crate::Writable for SYS_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_CFG to value 0"]
impl crate::Resettable for SYS_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
