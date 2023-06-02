#[doc = "Register `SYS_CFG` reader"]
pub struct R(crate::R<SYS_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CFG` writer"]
pub struct W(crate::W<SYS_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYS_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANA_FIB_SWD_ENABLE` reader - need_des"]
pub type ANA_FIB_SWD_ENABLE_R = crate::BitReader;
#[doc = "Field `ANA_FIB_CK_GLITCH_ENABLE` reader - need_des"]
pub type ANA_FIB_CK_GLITCH_ENABLE_R = crate::BitReader;
#[doc = "Field `ANA_FIB_BOD_ENABLE` reader - need_des"]
pub type ANA_FIB_BOD_ENABLE_R = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` reader - need_des"]
pub type FORCE_DOWNLOAD_BOOT_R = crate::BitReader;
#[doc = "Field `FORCE_DOWNLOAD_BOOT` writer - need_des"]
pub type FORCE_DOWNLOAD_BOOT_W<'a, const O: u8> = crate::BitWriter<'a, SYS_CFG_SPEC, O>;
#[doc = "Field `HPSYS_SW_RESET` writer - need_des"]
pub type HPSYS_SW_RESET_W<'a, const O: u8> = crate::BitWriter<'a, SYS_CFG_SPEC, O>;
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
            .field(
                "ana_fib_swd_enable",
                &format_args!("{}", self.ana_fib_swd_enable().bit()),
            )
            .field(
                "ana_fib_ck_glitch_enable",
                &format_args!("{}", self.ana_fib_ck_glitch_enable().bit()),
            )
            .field(
                "ana_fib_bod_enable",
                &format_args!("{}", self.ana_fib_bod_enable().bit()),
            )
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
    pub fn force_download_boot(&mut self) -> FORCE_DOWNLOAD_BOOT_W<30> {
        FORCE_DOWNLOAD_BOOT_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hpsys_sw_reset(&mut self) -> HPSYS_SW_RESET_W<31> {
        HPSYS_SW_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_cfg](index.html) module"]
pub struct SYS_CFG_SPEC;
impl crate::RegisterSpec for SYS_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_cfg::R](R) reader structure"]
impl crate::Readable for SYS_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_cfg::W](W) writer structure"]
impl crate::Writable for SYS_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYS_CFG to value 0x07"]
impl crate::Resettable for SYS_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
