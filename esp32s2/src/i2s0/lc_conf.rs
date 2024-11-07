#[doc = "Register `LC_CONF` reader"]
pub type R = crate::R<LC_CONF_SPEC>;
#[doc = "Register `LC_CONF` writer"]
pub type W = crate::W<LC_CONF_SPEC>;
#[doc = "Field `IN_RST` reader - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
pub type IN_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_RST` reader - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
pub type OUT_RST_R = crate::BitReader;
#[doc = "Field `OUT_RST` writer - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
pub type OUT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_FIFO_RST` reader - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
pub type AHBM_FIFO_RST_R = crate::BitReader;
#[doc = "Field `AHBM_FIFO_RST` writer - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
pub type AHBM_FIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_RST` reader - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
pub type AHBM_RST_R = crate::BitReader;
#[doc = "Field `AHBM_RST` writer - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
pub type AHBM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_LOOP_TEST` reader - Set this bit to loop test inlink."]
pub type OUT_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST` writer - Set this bit to loop test inlink."]
pub type OUT_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_LOOP_TEST` reader - Set this bit to loop test outlink."]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - Set this bit to loop test outlink."]
pub type IN_LOOP_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` writer - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
pub type OUT_AUTO_WRBACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_NO_RESTART_CLR` reader - Reserved."]
pub type OUT_NO_RESTART_CLR_R = crate::BitReader;
#[doc = "Field `OUT_NO_RESTART_CLR` writer - Reserved."]
pub type OUT_NO_RESTART_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_MODE` reader - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
pub type OUT_EOF_MODE_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE` writer - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
pub type OUT_EOF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDSCR_BURST_EN` reader - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN` writer - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
pub type OUTDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDSCR_BURST_EN` reader - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
pub type INDSCR_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DATA_BURST_EN` reader - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
pub type OUT_DATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUT_DATA_BURST_EN` writer - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
pub type OUT_DATA_BURST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK_OWNER` reader - Set this bit to enable check owner bit by hardware."]
pub type CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `CHECK_OWNER` writer - Set this bit to enable check owner bit by hardware."]
pub type CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TRANS_EN` reader - Reserved."]
pub type MEM_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN` writer - Reserved."]
pub type MEM_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_MEM_BK_SIZE` reader - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
pub type EXT_MEM_BK_SIZE_R = crate::FieldReader;
#[doc = "Field `EXT_MEM_BK_SIZE` writer - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
pub type EXT_MEM_BK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to loop test inlink."]
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to loop test outlink."]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    pub fn out_no_restart_clr(&self) -> OUT_NO_RESTART_CLR_R {
        OUT_NO_RESTART_CLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
    #[inline(always)]
    pub fn out_data_burst_en(&self) -> OUT_DATA_BURST_EN_R {
        OUT_DATA_BURST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable check owner bit by hardware."]
    #[inline(always)]
    pub fn check_owner(&self) -> CHECK_OWNER_R {
        CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved."]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
    #[inline(always)]
    pub fn ext_mem_bk_size(&self) -> EXT_MEM_BK_SIZE_R {
        EXT_MEM_BK_SIZE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC_CONF")
            .field("in_rst", &self.in_rst())
            .field("out_rst", &self.out_rst())
            .field("ahbm_fifo_rst", &self.ahbm_fifo_rst())
            .field("ahbm_rst", &self.ahbm_rst())
            .field("out_loop_test", &self.out_loop_test())
            .field("in_loop_test", &self.in_loop_test())
            .field("out_auto_wrback", &self.out_auto_wrback())
            .field("out_no_restart_clr", &self.out_no_restart_clr())
            .field("out_eof_mode", &self.out_eof_mode())
            .field("outdscr_burst_en", &self.outdscr_burst_en())
            .field("indscr_burst_en", &self.indscr_burst_en())
            .field("out_data_burst_en", &self.out_data_burst_en())
            .field("check_owner", &self.check_owner())
            .field("mem_trans_en", &self.mem_trans_en())
            .field("ext_mem_bk_size", &self.ext_mem_bk_size())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn in_rst(&mut self) -> IN_RST_W<LC_CONF_SPEC> {
        IN_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn out_rst(&mut self) -> OUT_RST_W<LC_CONF_SPEC> {
        OUT_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<LC_CONF_SPEC> {
        AHBM_FIFO_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
    #[inline(always)]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<LC_CONF_SPEC> {
        AHBM_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to loop test inlink."]
    #[inline(always)]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<LC_CONF_SPEC> {
        OUT_LOOP_TEST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to loop test outlink."]
    #[inline(always)]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<LC_CONF_SPEC> {
        IN_LOOP_TEST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
    #[inline(always)]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<LC_CONF_SPEC> {
        OUT_AUTO_WRBACK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    pub fn out_no_restart_clr(&mut self) -> OUT_NO_RESTART_CLR_W<LC_CONF_SPEC> {
        OUT_NO_RESTART_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
    #[inline(always)]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<LC_CONF_SPEC> {
        OUT_EOF_MODE_W::new(self, 8)
    }
    #[doc = "Bit 9 - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
    #[inline(always)]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<LC_CONF_SPEC> {
        OUTDSCR_BURST_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
    #[inline(always)]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<LC_CONF_SPEC> {
        INDSCR_BURST_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
    #[inline(always)]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W<LC_CONF_SPEC> {
        OUT_DATA_BURST_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to enable check owner bit by hardware."]
    #[inline(always)]
    pub fn check_owner(&mut self) -> CHECK_OWNER_W<LC_CONF_SPEC> {
        CHECK_OWNER_W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved."]
    #[inline(always)]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<LC_CONF_SPEC> {
        MEM_TRANS_EN_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
    #[inline(always)]
    pub fn ext_mem_bk_size(&mut self) -> EXT_MEM_BK_SIZE_W<LC_CONF_SPEC> {
        EXT_MEM_BK_SIZE_W::new(self, 14)
    }
}
#[doc = "I2S DMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lc_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lc_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_CONF_SPEC;
impl crate::RegisterSpec for LC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_conf::R`](R) reader structure"]
impl crate::Readable for LC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lc_conf::W`](W) writer structure"]
impl crate::Writable for LC_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LC_CONF to value 0x0100"]
impl crate::Resettable for LC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
