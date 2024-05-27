///Register `MODE_CFG` reader
pub type R = crate::R<MODE_CFG_SPEC>;
///Register `MODE_CFG` writer
pub type W = crate::W<MODE_CFG_SPEC>;
///Field `CMD_VIDEO_MODE` reader - NA
pub type CMD_VIDEO_MODE_R = crate::BitReader;
///Field `CMD_VIDEO_MODE` writer - NA
pub type CMD_VIDEO_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn cmd_video_mode(&self) -> CMD_VIDEO_MODE_R {
        CMD_VIDEO_MODE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE_CFG")
            .field("cmd_video_mode", &self.cmd_video_mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn cmd_video_mode(&mut self) -> CMD_VIDEO_MODE_W<MODE_CFG_SPEC> {
        CMD_VIDEO_MODE_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`mode_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MODE_CFG_SPEC;
impl crate::RegisterSpec for MODE_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mode_cfg::R`](R) reader structure
impl crate::Readable for MODE_CFG_SPEC {}
///`write(|w| ..)` method takes [`mode_cfg::W`](W) writer structure
impl crate::Writable for MODE_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODE_CFG to value 0x01
impl crate::Resettable for MODE_CFG_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
