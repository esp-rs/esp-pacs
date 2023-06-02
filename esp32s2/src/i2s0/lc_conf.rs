#[doc = "Register `LC_CONF` reader"]
pub struct R(crate::R<LC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LC_CONF` writer"]
pub struct W(crate::W<LC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LC_CONF_SPEC>;
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
impl From<crate::W<LC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_RST` reader - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
pub type IN_RST_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `OUT_RST` reader - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
pub type OUT_RST_R = crate::BitReader;
#[doc = "Field `OUT_RST` writer - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
pub type OUT_RST_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `AHBM_FIFO_RST` reader - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
pub type AHBM_FIFO_RST_R = crate::BitReader;
#[doc = "Field `AHBM_FIFO_RST` writer - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
pub type AHBM_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `AHBM_RST` reader - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
pub type AHBM_RST_R = crate::BitReader;
#[doc = "Field `AHBM_RST` writer - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
pub type AHBM_RST_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `OUT_LOOP_TEST` reader - Set this bit to loop test inlink."]
pub type OUT_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST` writer - Set this bit to loop test inlink."]
pub type OUT_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `IN_LOOP_TEST` reader - Set this bit to loop test outlink."]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - Set this bit to loop test outlink."]
pub type IN_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` writer - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
pub type OUT_AUTO_WRBACK_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `OUT_NO_RESTART_CLR` reader - Reserved."]
pub type OUT_NO_RESTART_CLR_R = crate::BitReader;
#[doc = "Field `OUT_NO_RESTART_CLR` writer - Reserved."]
pub type OUT_NO_RESTART_CLR_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `OUT_EOF_MODE` reader - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
pub type OUT_EOF_MODE_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE` writer - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
pub type OUT_EOF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `OUTDSCR_BURST_EN` reader - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN` writer - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
pub type OUTDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `INDSCR_BURST_EN` reader - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
pub type INDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `OUT_DATA_BURST_EN` reader - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
pub type OUT_DATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUT_DATA_BURST_EN` writer - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
pub type OUT_DATA_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `CHECK_OWNER` reader - Set this bit to enable check owner bit by hardware."]
pub type CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `CHECK_OWNER` writer - Set this bit to enable check owner bit by hardware."]
pub type CHECK_OWNER_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `MEM_TRANS_EN` reader - Reserved."]
pub type MEM_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN` writer - Reserved."]
pub type MEM_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, LC_CONF_SPEC, O>;
#[doc = "Field `EXT_MEM_BK_SIZE` reader - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
pub type EXT_MEM_BK_SIZE_R = crate::FieldReader;
#[doc = "Field `EXT_MEM_BK_SIZE` writer - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
pub type EXT_MEM_BK_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, LC_CONF_SPEC, 2, O>;
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
            .field("in_rst", &format_args!("{}", self.in_rst().bit()))
            .field("out_rst", &format_args!("{}", self.out_rst().bit()))
            .field(
                "ahbm_fifo_rst",
                &format_args!("{}", self.ahbm_fifo_rst().bit()),
            )
            .field("ahbm_rst", &format_args!("{}", self.ahbm_rst().bit()))
            .field(
                "out_loop_test",
                &format_args!("{}", self.out_loop_test().bit()),
            )
            .field(
                "in_loop_test",
                &format_args!("{}", self.in_loop_test().bit()),
            )
            .field(
                "out_auto_wrback",
                &format_args!("{}", self.out_auto_wrback().bit()),
            )
            .field(
                "out_no_restart_clr",
                &format_args!("{}", self.out_no_restart_clr().bit()),
            )
            .field(
                "out_eof_mode",
                &format_args!("{}", self.out_eof_mode().bit()),
            )
            .field(
                "outdscr_burst_en",
                &format_args!("{}", self.outdscr_burst_en().bit()),
            )
            .field(
                "indscr_burst_en",
                &format_args!("{}", self.indscr_burst_en().bit()),
            )
            .field(
                "out_data_burst_en",
                &format_args!("{}", self.out_data_burst_en().bit()),
            )
            .field("check_owner", &format_args!("{}", self.check_owner().bit()))
            .field(
                "mem_trans_en",
                &format_args!("{}", self.mem_trans_en().bit()),
            )
            .field(
                "ext_mem_bk_size",
                &format_args!("{}", self.ext_mem_bk_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset in-DMA FSM. Set this bit before the DMA configuration."]
    #[inline(always)]
    #[must_use]
    pub fn in_rst(&mut self) -> IN_RST_W<0> {
        IN_RST_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to reset out-DMA FSM. Set this bit before the DMA configuration."]
    #[inline(always)]
    #[must_use]
    pub fn out_rst(&mut self) -> OUT_RST_W<1> {
        OUT_RST_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to reset AHB interface cmdFIFO of DMA. Set this bit before the DMA configuration."]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<2> {
        AHBM_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to reset AHB interface of DMA. Set this bit before the DMA configuration."]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<3> {
        AHBM_RST_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to loop test inlink."]
    #[inline(always)]
    #[must_use]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<4> {
        OUT_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to loop test outlink."]
    #[inline(always)]
    #[must_use]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<5> {
        IN_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to enable outlink-written-back automatically when out buffer is transmitted done."]
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<6> {
        OUT_AUTO_WRBACK_W::new(self)
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn out_no_restart_clr(&mut self) -> OUT_NO_RESTART_CLR_W<7> {
        OUT_NO_RESTART_CLR_W::new(self)
    }
    #[doc = "Bit 8 - DMA out EOF flag generation mode. 1: When DMA has popped all data from the FIFO. 0: When AHB has pushed all data to the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<8> {
        OUT_EOF_MODE_W::new(self)
    }
    #[doc = "Bit 9 - DMA outlink descriptor transfer mode configuration bit. 1: Prepare outlink descriptor with burst mode. 0: Prepare outlink descriptor with byte mode."]
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<9> {
        OUTDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 10 - DMA inlink descriptor transfer mode configuration bit. 1: Prepare inlink descriptor with burst mode. 0: Prepare inlink descriptor with byte mode."]
    #[inline(always)]
    #[must_use]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<10> {
        INDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 11 - Transmitter data transfer mode configuration bit. 1: Prepare out data with burst mode. 0: Prepare out data with byte mode."]
    #[inline(always)]
    #[must_use]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W<11> {
        OUT_DATA_BURST_EN_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to enable check owner bit by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn check_owner(&mut self) -> CHECK_OWNER_W<12> {
        CHECK_OWNER_W::new(self)
    }
    #[doc = "Bit 13 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<13> {
        MEM_TRANS_EN_W::new(self)
    }
    #[doc = "Bits 14:15 - DMA access external memory block size. 0: 16 bytes. 1: 32 bytes. 2: 64 bytes. 3: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn ext_mem_bk_size(&mut self) -> EXT_MEM_BK_SIZE_W<14> {
        EXT_MEM_BK_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S DMA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_conf](index.html) module"]
pub struct LC_CONF_SPEC;
impl crate::RegisterSpec for LC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_conf::R](R) reader structure"]
impl crate::Readable for LC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lc_conf::W](W) writer structure"]
impl crate::Writable for LC_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LC_CONF to value 0x0100"]
impl crate::Resettable for LC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
