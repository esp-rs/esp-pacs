#[doc = "Register `SYS_CFG` reader"]
pub type R = crate::R<SYS_CFG_SPEC>;
#[doc = "Register `SYS_CFG` writer"]
pub type W = crate::W<SYS_CFG_SPEC>;
#[doc = "Field `ANA_FIB_SWD_ENABLE` reader - need_des"]
pub type ANA_FIB_SWD_ENABLE_R = crate::BitReader;
#[doc = "Field `ANA_FIB_CK_GLITCH_ENABLE` reader - need_des"]
pub type ANA_FIB_CK_GLITCH_ENABLE_R = crate::BitReader;
#[doc = "Field `ANA_FIB_BOD_ENABLE` reader - need_des"]
pub type ANA_FIB_BOD_ENABLE_R = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` reader - need_des"]
pub type FORCE_DOWNLOAD_BOOT_R = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` writer - need_des"]
pub type FORCE_DOWNLOAD_BOOT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPSYS_SW_RESET` writer - need_des"]
pub type HPSYS_SW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn ana_fib_swd_enable(&self) -> ANA_FIB_SWD_ENABLE_R {
        ANA_FIB_SWD_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn ana_fib_ck_glitch_enable(&self) -> ANA_FIB_CK_GLITCH_ENABLE_R {
        ANA_FIB_CK_GLITCH_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn ana_fib_bod_enable(&self) -> ANA_FIB_BOD_ENABLE_R {
        ANA_FIB_BOD_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn force_download_boot(&self) -> FORCE_DOWNLOAD_BOOT_R {
        FORCE_DOWNLOAD_BOOT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_CFG")
            .field("ana_fib_swd_enable", &self.ana_fib_swd_enable())
            .field("ana_fib_ck_glitch_enable", &self.ana_fib_ck_glitch_enable())
            .field("ana_fib_bod_enable", &self.ana_fib_bod_enable())
            .field("force_download_boot", &self.force_download_boot())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_download_boot(&mut self) -> FORCE_DOWNLOAD_BOOT_W<SYS_CFG_SPEC> {
        FORCE_DOWNLOAD_BOOT_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hpsys_sw_reset(&mut self) -> HPSYS_SW_RESET_W<SYS_CFG_SPEC> {
        HPSYS_SW_RESET_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets SYS_CFG to value 0x07"]
impl crate::Resettable for SYS_CFG_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
