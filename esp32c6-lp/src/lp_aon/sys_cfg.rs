///Register `SYS_CFG` reader
pub type R = crate::R<SYS_CFG_SPEC>;
///Register `SYS_CFG` writer
pub type W = crate::W<SYS_CFG_SPEC>;
///Field `FORCE_DOWNLOAD_BOOT` reader - need_des
pub type FORCE_DOWNLOAD_BOOT_R = crate::BitReader;
///Field `FORCE_DOWNLOAD_BOOT` writer - need_des
pub type FORCE_DOWNLOAD_BOOT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPSYS_SW_RESET` writer - need_des
pub type HPSYS_SW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn force_download_boot(&self) -> FORCE_DOWNLOAD_BOOT_R {
        FORCE_DOWNLOAD_BOOT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_CFG")
            .field("force_download_boot", &self.force_download_boot())
            .finish()
    }
}
impl W {
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn force_download_boot(&mut self) -> FORCE_DOWNLOAD_BOOT_W<SYS_CFG_SPEC> {
        FORCE_DOWNLOAD_BOOT_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hpsys_sw_reset(&mut self) -> HPSYS_SW_RESET_W<SYS_CFG_SPEC> {
        HPSYS_SW_RESET_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`sys_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYS_CFG_SPEC;
impl crate::RegisterSpec for SYS_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sys_cfg::R`](R) reader structure
impl crate::Readable for SYS_CFG_SPEC {}
///`write(|w| ..)` method takes [`sys_cfg::W`](W) writer structure
impl crate::Writable for SYS_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYS_CFG to value 0
impl crate::Resettable for SYS_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
