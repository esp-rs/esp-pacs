#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `FSM_RST` writer - fsm reset"]
pub type FSM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG_START` writer - start to compress a new pic(in dma reg mode)"]
pub type JPEG_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QNR_PRESITION` reader - 0:8bit qnr,1:12bit qnr(TBD)"]
pub type QNR_PRESITION_R = crate::BitReader;
#[doc = "Field `QNR_PRESITION` writer - 0:8bit qnr,1:12bit qnr(TBD)"]
pub type QNR_PRESITION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FF_CHECK_EN` reader - enable whether to add \"00\" after \"ff\""]
pub type FF_CHECK_EN_R = crate::BitReader;
#[doc = "Field `FF_CHECK_EN` writer - enable whether to add \"00\" after \"ff\""]
pub type FF_CHECK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_SEL` reader - 0:yuv444,1:yuv422, 2:yuv420"]
pub type SAMPLE_SEL_R = crate::FieldReader;
#[doc = "Field `SAMPLE_SEL` writer - 0:yuv444,1:yuv422, 2:yuv420"]
pub type SAMPLE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA_LINKLIST_MODE` reader - 1:use linklist to configure dma"]
pub type DMA_LINKLIST_MODE_R = crate::BitReader;
#[doc = "Field `DEBUG_DIRECT_OUT_EN` reader - 0:normal mode,1:debug mode for direct output from input"]
pub type DEBUG_DIRECT_OUT_EN_R = crate::BitReader;
#[doc = "Field `DEBUG_DIRECT_OUT_EN` writer - 0:normal mode,1:debug mode for direct output from input"]
pub type DEBUG_DIRECT_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GRAY_SEL` reader - 0:use non-fifo way to access qnr ram,1:use fifo way to access qnr ram"]
pub type GRAY_SEL_R = crate::BitReader;
#[doc = "Field `GRAY_SEL` writer - 0:use non-fifo way to access qnr ram,1:use fifo way to access qnr ram"]
pub type GRAY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LQNR_TBL_SEL` reader - choose luminance quntization table id(TBD)"]
pub type LQNR_TBL_SEL_R = crate::FieldReader;
#[doc = "Field `LQNR_TBL_SEL` writer - choose luminance quntization table id(TBD)"]
pub type LQNR_TBL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CQNR_TBL_SEL` reader - choose chrominance quntization table id (TBD)"]
pub type CQNR_TBL_SEL_R = crate::FieldReader;
#[doc = "Field `CQNR_TBL_SEL` writer - choose chrominance quntization table id (TBD)"]
pub type CQNR_TBL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COLOR_SPACE` reader - configure picture's color space:0-rb888,1-yuv422,2-rgb565, 3-gray"]
pub type COLOR_SPACE_R = crate::FieldReader;
#[doc = "Field `COLOR_SPACE` writer - configure picture's color space:0-rb888,1-yuv422,2-rgb565, 3-gray"]
pub type COLOR_SPACE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DHT_FIFO_EN` reader - 0:use non-fifo way to write dht len_total/codemin/value table,1:use fifo way to write dht len_total/codemin/value table. Reading dht len_total/codemin/value table only has nonfifo way"]
pub type DHT_FIFO_EN_R = crate::BitReader;
#[doc = "Field `DHT_FIFO_EN` writer - 0:use non-fifo way to write dht len_total/codemin/value table,1:use fifo way to write dht len_total/codemin/value table. Reading dht len_total/codemin/value table only has nonfifo way"]
pub type DHT_FIFO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - force memory's clock enabled"]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - force memory's clock enabled"]
pub type MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JFIF_VER` reader - decode pause period to trigger decode_timeout int, the timeout periods =2 power (reg_decode_timeout_thres) -1"]
pub type JFIF_VER_R = crate::FieldReader;
#[doc = "Field `JFIF_VER` writer - decode pause period to trigger decode_timeout int, the timeout periods =2 power (reg_decode_timeout_thres) -1"]
pub type JFIF_VER_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DECODE_TIMEOUT_TASK_SEL` reader - 0: software use reset to abort decode process ,1: decoder abort decode process by itself"]
pub type DECODE_TIMEOUT_TASK_SEL_R = crate::BitReader;
#[doc = "Field `DECODE_TIMEOUT_TASK_SEL` writer - 0: software use reset to abort decode process ,1: decoder abort decode process by itself"]
pub type DECODE_TIMEOUT_TASK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFT_RST` reader - when set to 1, soft reset JPEG module except jpeg_reg module"]
pub type SOFT_RST_R = crate::BitReader;
#[doc = "Field `SOFT_RST` writer - when set to 1, soft reset JPEG module except jpeg_reg module"]
pub type SOFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_RST` reader - fifo reset"]
pub type FIFO_RST_R = crate::BitReader;
#[doc = "Field `FIFO_RST` writer - fifo reset"]
pub type FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIXEL_REV` reader - reverse the source color pixel"]
pub type PIXEL_REV_R = crate::BitReader;
#[doc = "Field `PIXEL_REV` writer - reverse the source color pixel"]
pub type PIXEL_REV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAILER_EN` reader - set this bit to add EOI of \"0xffd9\" at the end of bitstream"]
pub type TAILER_EN_R = crate::BitReader;
#[doc = "Field `TAILER_EN` writer - set this bit to add EOI of \"0xffd9\" at the end of bitstream"]
pub type TAILER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSE_EN` reader - set this bit to pause jpeg encoding"]
pub type PAUSE_EN_R = crate::BitReader;
#[doc = "Field `PAUSE_EN` writer - set this bit to pause jpeg encoding"]
pub type PAUSE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FORCE_PD` reader - 0: no operation,1:force jpeg memory to power down"]
pub type MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PD` writer - 0: no operation,1:force jpeg memory to power down"]
pub type MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FORCE_PU` reader - 0: no operation,1:force jpeg memory to power up"]
pub type MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PU` writer - 0: no operation,1:force jpeg memory to power up"]
pub type MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - 0:encoder mode, 1: decoder mode"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - 0:encoder mode, 1: decoder mode"]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - 0:8bit qnr,1:12bit qnr(TBD)"]
    #[inline(always)]
    pub fn qnr_presition(&self) -> QNR_PRESITION_R {
        QNR_PRESITION_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable whether to add \"00\" after \"ff\""]
    #[inline(always)]
    pub fn ff_check_en(&self) -> FF_CHECK_EN_R {
        FF_CHECK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 0:yuv444,1:yuv422, 2:yuv420"]
    #[inline(always)]
    pub fn sample_sel(&self) -> SAMPLE_SEL_R {
        SAMPLE_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 1:use linklist to configure dma"]
    #[inline(always)]
    pub fn dma_linklist_mode(&self) -> DMA_LINKLIST_MODE_R {
        DMA_LINKLIST_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0:normal mode,1:debug mode for direct output from input"]
    #[inline(always)]
    pub fn debug_direct_out_en(&self) -> DEBUG_DIRECT_OUT_EN_R {
        DEBUG_DIRECT_OUT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0:use non-fifo way to access qnr ram,1:use fifo way to access qnr ram"]
    #[inline(always)]
    pub fn gray_sel(&self) -> GRAY_SEL_R {
        GRAY_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - choose luminance quntization table id(TBD)"]
    #[inline(always)]
    pub fn lqnr_tbl_sel(&self) -> LQNR_TBL_SEL_R {
        LQNR_TBL_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - choose chrominance quntization table id (TBD)"]
    #[inline(always)]
    pub fn cqnr_tbl_sel(&self) -> CQNR_TBL_SEL_R {
        CQNR_TBL_SEL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - configure picture's color space:0-rb888,1-yuv422,2-rgb565, 3-gray"]
    #[inline(always)]
    pub fn color_space(&self) -> COLOR_SPACE_R {
        COLOR_SPACE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 0:use non-fifo way to write dht len_total/codemin/value table,1:use fifo way to write dht len_total/codemin/value table. Reading dht len_total/codemin/value table only has nonfifo way"]
    #[inline(always)]
    pub fn dht_fifo_en(&self) -> DHT_FIFO_EN_R {
        DHT_FIFO_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - force memory's clock enabled"]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - decode pause period to trigger decode_timeout int, the timeout periods =2 power (reg_decode_timeout_thres) -1"]
    #[inline(always)]
    pub fn jfif_ver(&self) -> JFIF_VER_R {
        JFIF_VER_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - 0: software use reset to abort decode process ,1: decoder abort decode process by itself"]
    #[inline(always)]
    pub fn decode_timeout_task_sel(&self) -> DECODE_TIMEOUT_TASK_SEL_R {
        DECODE_TIMEOUT_TASK_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - when set to 1, soft reset JPEG module except jpeg_reg module"]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - fifo reset"]
    #[inline(always)]
    pub fn fifo_rst(&self) -> FIFO_RST_R {
        FIFO_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reverse the source color pixel"]
    #[inline(always)]
    pub fn pixel_rev(&self) -> PIXEL_REV_R {
        PIXEL_REV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - set this bit to add EOI of \"0xffd9\" at the end of bitstream"]
    #[inline(always)]
    pub fn tailer_en(&self) -> TAILER_EN_R {
        TAILER_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - set this bit to pause jpeg encoding"]
    #[inline(always)]
    pub fn pause_en(&self) -> PAUSE_EN_R {
        PAUSE_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 0: no operation,1:force jpeg memory to power down"]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: no operation,1:force jpeg memory to power up"]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0:encoder mode, 1: decoder mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("qnr_presition", &self.qnr_presition())
            .field("ff_check_en", &self.ff_check_en())
            .field("sample_sel", &self.sample_sel())
            .field("dma_linklist_mode", &self.dma_linklist_mode())
            .field("debug_direct_out_en", &self.debug_direct_out_en())
            .field("gray_sel", &self.gray_sel())
            .field("lqnr_tbl_sel", &self.lqnr_tbl_sel())
            .field("cqnr_tbl_sel", &self.cqnr_tbl_sel())
            .field("color_space", &self.color_space())
            .field("dht_fifo_en", &self.dht_fifo_en())
            .field("mem_clk_force_on", &self.mem_clk_force_on())
            .field("jfif_ver", &self.jfif_ver())
            .field("decode_timeout_task_sel", &self.decode_timeout_task_sel())
            .field("soft_rst", &self.soft_rst())
            .field("fifo_rst", &self.fifo_rst())
            .field("pixel_rev", &self.pixel_rev())
            .field("tailer_en", &self.tailer_en())
            .field("pause_en", &self.pause_en())
            .field("mem_force_pd", &self.mem_force_pd())
            .field("mem_force_pu", &self.mem_force_pu())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - fsm reset"]
    #[inline(always)]
    pub fn fsm_rst(&mut self) -> FSM_RST_W<CONFIG_SPEC> {
        FSM_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - start to compress a new pic(in dma reg mode)"]
    #[inline(always)]
    pub fn jpeg_start(&mut self) -> JPEG_START_W<CONFIG_SPEC> {
        JPEG_START_W::new(self, 1)
    }
    #[doc = "Bit 2 - 0:8bit qnr,1:12bit qnr(TBD)"]
    #[inline(always)]
    pub fn qnr_presition(&mut self) -> QNR_PRESITION_W<CONFIG_SPEC> {
        QNR_PRESITION_W::new(self, 2)
    }
    #[doc = "Bit 3 - enable whether to add \"00\" after \"ff\""]
    #[inline(always)]
    pub fn ff_check_en(&mut self) -> FF_CHECK_EN_W<CONFIG_SPEC> {
        FF_CHECK_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - 0:yuv444,1:yuv422, 2:yuv420"]
    #[inline(always)]
    pub fn sample_sel(&mut self) -> SAMPLE_SEL_W<CONFIG_SPEC> {
        SAMPLE_SEL_W::new(self, 4)
    }
    #[doc = "Bit 7 - 0:normal mode,1:debug mode for direct output from input"]
    #[inline(always)]
    pub fn debug_direct_out_en(&mut self) -> DEBUG_DIRECT_OUT_EN_W<CONFIG_SPEC> {
        DEBUG_DIRECT_OUT_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - 0:use non-fifo way to access qnr ram,1:use fifo way to access qnr ram"]
    #[inline(always)]
    pub fn gray_sel(&mut self) -> GRAY_SEL_W<CONFIG_SPEC> {
        GRAY_SEL_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - choose luminance quntization table id(TBD)"]
    #[inline(always)]
    pub fn lqnr_tbl_sel(&mut self) -> LQNR_TBL_SEL_W<CONFIG_SPEC> {
        LQNR_TBL_SEL_W::new(self, 9)
    }
    #[doc = "Bits 11:12 - choose chrominance quntization table id (TBD)"]
    #[inline(always)]
    pub fn cqnr_tbl_sel(&mut self) -> CQNR_TBL_SEL_W<CONFIG_SPEC> {
        CQNR_TBL_SEL_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - configure picture's color space:0-rb888,1-yuv422,2-rgb565, 3-gray"]
    #[inline(always)]
    pub fn color_space(&mut self) -> COLOR_SPACE_W<CONFIG_SPEC> {
        COLOR_SPACE_W::new(self, 13)
    }
    #[doc = "Bit 15 - 0:use non-fifo way to write dht len_total/codemin/value table,1:use fifo way to write dht len_total/codemin/value table. Reading dht len_total/codemin/value table only has nonfifo way"]
    #[inline(always)]
    pub fn dht_fifo_en(&mut self) -> DHT_FIFO_EN_W<CONFIG_SPEC> {
        DHT_FIFO_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - force memory's clock enabled"]
    #[inline(always)]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W<CONFIG_SPEC> {
        MEM_CLK_FORCE_ON_W::new(self, 16)
    }
    #[doc = "Bits 17:22 - decode pause period to trigger decode_timeout int, the timeout periods =2 power (reg_decode_timeout_thres) -1"]
    #[inline(always)]
    pub fn jfif_ver(&mut self) -> JFIF_VER_W<CONFIG_SPEC> {
        JFIF_VER_W::new(self, 17)
    }
    #[doc = "Bit 23 - 0: software use reset to abort decode process ,1: decoder abort decode process by itself"]
    #[inline(always)]
    pub fn decode_timeout_task_sel(&mut self) -> DECODE_TIMEOUT_TASK_SEL_W<CONFIG_SPEC> {
        DECODE_TIMEOUT_TASK_SEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - when set to 1, soft reset JPEG module except jpeg_reg module"]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SOFT_RST_W<CONFIG_SPEC> {
        SOFT_RST_W::new(self, 24)
    }
    #[doc = "Bit 25 - fifo reset"]
    #[inline(always)]
    pub fn fifo_rst(&mut self) -> FIFO_RST_W<CONFIG_SPEC> {
        FIFO_RST_W::new(self, 25)
    }
    #[doc = "Bit 26 - reverse the source color pixel"]
    #[inline(always)]
    pub fn pixel_rev(&mut self) -> PIXEL_REV_W<CONFIG_SPEC> {
        PIXEL_REV_W::new(self, 26)
    }
    #[doc = "Bit 27 - set this bit to add EOI of \"0xffd9\" at the end of bitstream"]
    #[inline(always)]
    pub fn tailer_en(&mut self) -> TAILER_EN_W<CONFIG_SPEC> {
        TAILER_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - set this bit to pause jpeg encoding"]
    #[inline(always)]
    pub fn pause_en(&mut self) -> PAUSE_EN_W<CONFIG_SPEC> {
        PAUSE_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - 0: no operation,1:force jpeg memory to power down"]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W<CONFIG_SPEC> {
        MEM_FORCE_PD_W::new(self, 29)
    }
    #[doc = "Bit 30 - 0: no operation,1:force jpeg memory to power up"]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W<CONFIG_SPEC> {
        MEM_FORCE_PU_W::new(self, 30)
    }
    #[doc = "Bit 31 - 0:encoder mode, 1: decoder mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<CONFIG_SPEC> {
        MODE_W::new(self, 31)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x0040_8958"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x0040_8958;
}
