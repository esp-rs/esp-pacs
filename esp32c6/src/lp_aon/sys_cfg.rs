#[doc = "Register `SYS_CFG` reader"]
pub type R = crate::R<SYS_CFG_SPEC>;
#[doc = "Register `SYS_CFG` writer"]
pub type W = crate::W<SYS_CFG_SPEC>;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` reader - need_des"]
pub type FORCE_DOWNLOAD_BOOT_R = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` writer - need_des"]
pub type FORCE_DOWNLOAD_BOOT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HPSYS_SW_RESET` writer - need_des"]
pub type HPSYS_SW_RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
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
            .field(
                "force_download_boot",
                &format_args!("{}", self.force_download_boot().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYS_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_download_boot(&mut self) -> FORCE_DOWNLOAD_BOOT_W<SYS_CFG_SPEC, 30> {
        FORCE_DOWNLOAD_BOOT_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hpsys_sw_reset(&mut self) -> HPSYS_SW_RESET_W<SYS_CFG_SPEC, 31> {
        HPSYS_SW_RESET_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYS_CFG to value 0"]
impl crate::Resettable for SYS_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
