#[doc = "Register `SYS_CTRL` reader"]
pub type R = crate::R<SYS_CTRL_SPEC>;
#[doc = "Register `SYS_CTRL` writer"]
pub type W = crate::W<SYS_CTRL_SPEC>;
#[doc = "Field `FRAME_START` writer - Configures whether or not to start encoding one frame.\\\\0: Invalid. No effect\\\\1: Start encoding one frame"]
pub type FRAME_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MOVE_START` writer - Configures whether or not to start moving reference data from external mem.\\\\0: Invalid. No effect\\\\1: H264 start moving two MB lines of reference frame from external mem to internal mem"]
pub type DMA_MOVE_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_MODE` reader - Configures H264 running mode. When field H264_DUAL_STREAM_MODE is set to 1, this field must be set to 1 too.\\\\0: GOP mode. Before every GOP first frame start, need reconfig reference frame DMA\\\\1: Frame mode. Before every frame start, need reconfig reference frame DMA"]
pub type FRAME_MODE_R = crate::BitReader;
#[doc = "Field `FRAME_MODE` writer - Configures H264 running mode. When field H264_DUAL_STREAM_MODE is set to 1, this field must be set to 1 too.\\\\0: GOP mode. Before every GOP first frame start, need reconfig reference frame DMA\\\\1: Frame mode. Before every frame start, need reconfig reference frame DMA"]
pub type FRAME_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_RST_PULSE` writer - Configures whether or not to reset H264 ip.\\\\0: Invalid. No effect\\\\1: Reset H264 ip"]
pub type SYS_RST_PULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Configures H264 running mode. When field H264_DUAL_STREAM_MODE is set to 1, this field must be set to 1 too.\\\\0: GOP mode. Before every GOP first frame start, need reconfig reference frame DMA\\\\1: Frame mode. Before every frame start, need reconfig reference frame DMA"]
    #[inline(always)]
    pub fn frame_mode(&self) -> FRAME_MODE_R {
        FRAME_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_CTRL")
            .field("frame_mode", &format_args!("{}", self.frame_mode().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to start encoding one frame.\\\\0: Invalid. No effect\\\\1: Start encoding one frame"]
    #[inline(always)]
    #[must_use]
    pub fn frame_start(&mut self) -> FRAME_START_W<SYS_CTRL_SPEC> {
        FRAME_START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to start moving reference data from external mem.\\\\0: Invalid. No effect\\\\1: H264 start moving two MB lines of reference frame from external mem to internal mem"]
    #[inline(always)]
    #[must_use]
    pub fn dma_move_start(&mut self) -> DMA_MOVE_START_W<SYS_CTRL_SPEC> {
        DMA_MOVE_START_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures H264 running mode. When field H264_DUAL_STREAM_MODE is set to 1, this field must be set to 1 too.\\\\0: GOP mode. Before every GOP first frame start, need reconfig reference frame DMA\\\\1: Frame mode. Before every frame start, need reconfig reference frame DMA"]
    #[inline(always)]
    #[must_use]
    pub fn frame_mode(&mut self) -> FRAME_MODE_W<SYS_CTRL_SPEC> {
        FRAME_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to reset H264 ip.\\\\0: Invalid. No effect\\\\1: Reset H264 ip"]
    #[inline(always)]
    #[must_use]
    pub fn sys_rst_pulse(&mut self) -> SYS_RST_PULSE_W<SYS_CTRL_SPEC> {
        SYS_RST_PULSE_W::new(self, 3)
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
#[doc = "H264 system level control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_CTRL_SPEC;
impl crate::RegisterSpec for SYS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_ctrl::R`](R) reader structure"]
impl crate::Readable for SYS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_ctrl::W`](W) writer structure"]
impl crate::Writable for SYS_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_CTRL to value 0"]
impl crate::Resettable for SYS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
