#[doc = "Register `MODE_CFG` reader"]
pub type R = crate::R<MODE_CFG_SPEC>;
#[doc = "Register `MODE_CFG` writer"]
pub type W = crate::W<MODE_CFG_SPEC>;
#[doc = "Field `CMD_VIDEO_MODE` reader - NA"]
pub type CMD_VIDEO_MODE_R = crate::BitReader;
#[doc = "Field `CMD_VIDEO_MODE` writer - NA"]
pub type CMD_VIDEO_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn cmd_video_mode(&self) -> CMD_VIDEO_MODE_R {
        CMD_VIDEO_MODE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE_CFG")
            .field(
                "cmd_video_mode",
                &format_args!("{}", self.cmd_video_mode().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODE_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_video_mode(&mut self) -> CMD_VIDEO_MODE_W<MODE_CFG_SPEC> {
        CMD_VIDEO_MODE_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_CFG_SPEC;
impl crate::RegisterSpec for MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_cfg::R`](R) reader structure"]
impl crate::Readable for MODE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode_cfg::W`](W) writer structure"]
impl crate::Writable for MODE_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE_CFG to value 0x01"]
impl crate::Resettable for MODE_CFG_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
