#[doc = "Register `SPI_SLAVE` reader"]
pub struct R(crate::R<SPI_SLAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_SLAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_SLAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_SLAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_SLAVE` writer"]
pub struct W(crate::W<SPI_SLAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_SLAVE_SPEC>;
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
impl From<crate::W<SPI_SLAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_SLAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slv_rd_buf_done` reader - The interrupt raw bit for the completement of \"read-buffer\" operation in the slave mode."]
pub type SLV_RD_BUF_DONE_R = crate::BitReader;
#[doc = "Field `slv_rd_buf_done` writer - The interrupt raw bit for the completement of \"read-buffer\" operation in the slave mode."]
pub type SLV_RD_BUF_DONE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `slv_wr_buf_done` reader - The interrupt raw bit for the completement of \"write-buffer\" operation in the slave mode."]
pub type SLV_WR_BUF_DONE_R = crate::BitReader;
#[doc = "Field `slv_wr_buf_done` writer - The interrupt raw bit for the completement of \"write-buffer\" operation in the slave mode."]
pub type SLV_WR_BUF_DONE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `slv_rd_sta_done` reader - The interrupt raw bit for the completement of \"read-status\" operation in the slave mode."]
pub type SLV_RD_STA_DONE_R = crate::BitReader;
#[doc = "Field `slv_rd_sta_done` writer - The interrupt raw bit for the completement of \"read-status\" operation in the slave mode."]
pub type SLV_RD_STA_DONE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `slv_wr_sta_done` reader - The interrupt raw bit for the completement of \"write-status\" operation in the slave mode."]
pub type SLV_WR_STA_DONE_R = crate::BitReader;
#[doc = "Field `slv_wr_sta_done` writer - The interrupt raw bit for the completement of \"write-status\" operation in the slave mode."]
pub type SLV_WR_STA_DONE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `spi_trans_done` reader - The interrupt raw bit for the completement of any operation in both the master mode and the slave mode."]
pub type SPI_TRANS_DONE_R = crate::BitReader;
#[doc = "Field `spi_trans_done` writer - The interrupt raw bit for the completement of any operation in both the master mode and the slave mode."]
pub type SPI_TRANS_DONE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `spi_int_en` reader - Interrupt enable bits for the below 5 sources"]
pub type SPI_INT_EN_R = crate::FieldReader;
#[doc = "Field `spi_int_en` writer - Interrupt enable bits for the below 5 sources"]
pub type SPI_INT_EN_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_SLAVE_SPEC, 5, O>;
#[doc = "Field `interrupt_rb_enable` reader - Enable buffer read interrupts"]
pub type INTERRUPT_RB_ENABLE_R = crate::BitReader;
#[doc = "Field `interrupt_rb_enable` writer - Enable buffer read interrupts"]
pub type INTERRUPT_RB_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `interrupt_wb_enable` reader - Enable buffer write interrupts"]
pub type INTERRUPT_WB_ENABLE_R = crate::BitReader;
#[doc = "Field `interrupt_wb_enable` writer - Enable buffer write interrupts"]
pub type INTERRUPT_WB_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `interrupt_rs_enable` reader - Enable status read interrupts"]
pub type INTERRUPT_RS_ENABLE_R = crate::BitReader;
#[doc = "Field `interrupt_rs_enable` writer - Enable status read interrupts"]
pub type INTERRUPT_RS_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `interrupt_ws_enable` reader - Enable status write interrupts"]
pub type INTERRUPT_WS_ENABLE_R = crate::BitReader;
#[doc = "Field `interrupt_ws_enable` writer - Enable status write interrupts"]
pub type INTERRUPT_WS_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `interrupt_trans_enable` reader - Enable TRANS interrupts"]
pub type INTERRUPT_TRANS_ENABLE_R = crate::BitReader;
#[doc = "Field `interrupt_trans_enable` writer - Enable TRANS interrupts"]
pub type INTERRUPT_TRANS_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `spi_trans_cnt` reader - The operations counter in both the master mode and the slave mode."]
pub type SPI_TRANS_CNT_R = crate::FieldReader;
#[doc = "Field `slv_cmd_define` reader - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as 1: \"write-status\"; 4: \"read-status\"; 2: \"write-buffer\" and 3: \"read-buffer\"."]
pub type SLV_CMD_DEFINE_R = crate::BitReader;
#[doc = "Field `slv_cmd_define` writer - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as 1: \"write-status\"; 4: \"read-status\"; 2: \"write-buffer\" and 3: \"read-buffer\"."]
pub type SLV_CMD_DEFINE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `sta_enable` reader - Enable read/write buffer"]
pub type STA_ENABLE_R = crate::BitReader;
#[doc = "Field `sta_enable` writer - Enable read/write buffer"]
pub type STA_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `spi_buffer_enable` reader - Enable read/write buffer"]
pub type SPI_BUFFER_ENABLE_R = crate::BitReader;
#[doc = "Field `spi_buffer_enable` writer - Enable read/write buffer"]
pub type SPI_BUFFER_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `spi_slave_mode` reader - 1: slave mode, 0: master mode."]
pub type SPI_SLAVE_MODE_R = crate::BitReader;
#[doc = "Field `spi_slave_mode` writer - 1: slave mode, 0: master mode."]
pub type SPI_SLAVE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
#[doc = "Field `spi_sync_reset` reader - It is the synchronous reset signal of the module. This bit is self-cleared by hardware."]
pub type SPI_SYNC_RESET_R = crate::BitReader;
#[doc = "Field `spi_sync_reset` writer - It is the synchronous reset signal of the module. This bit is self-cleared by hardware."]
pub type SPI_SYNC_RESET_W<'a, const O: u8> = crate::BitWriter<'a, SPI_SLAVE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The interrupt raw bit for the completement of \"read-buffer\" operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&self) -> SLV_RD_BUF_DONE_R {
        SLV_RD_BUF_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for the completement of \"write-buffer\" operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&self) -> SLV_WR_BUF_DONE_R {
        SLV_WR_BUF_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for the completement of \"read-status\" operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_sta_done(&self) -> SLV_RD_STA_DONE_R {
        SLV_RD_STA_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for the completement of \"write-status\" operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_sta_done(&self) -> SLV_WR_STA_DONE_R {
        SLV_WR_STA_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for the completement of any operation in both the master mode and the slave mode."]
    #[inline(always)]
    pub fn spi_trans_done(&self) -> SPI_TRANS_DONE_R {
        SPI_TRANS_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:9 - Interrupt enable bits for the below 5 sources"]
    #[inline(always)]
    pub fn spi_int_en(&self) -> SPI_INT_EN_R {
        SPI_INT_EN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enable buffer read interrupts"]
    #[inline(always)]
    pub fn interrupt_rb_enable(&self) -> INTERRUPT_RB_ENABLE_R {
        INTERRUPT_RB_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable buffer write interrupts"]
    #[inline(always)]
    pub fn interrupt_wb_enable(&self) -> INTERRUPT_WB_ENABLE_R {
        INTERRUPT_WB_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable status read interrupts"]
    #[inline(always)]
    pub fn interrupt_rs_enable(&self) -> INTERRUPT_RS_ENABLE_R {
        INTERRUPT_RS_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable status write interrupts"]
    #[inline(always)]
    pub fn interrupt_ws_enable(&self) -> INTERRUPT_WS_ENABLE_R {
        INTERRUPT_WS_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable TRANS interrupts"]
    #[inline(always)]
    pub fn interrupt_trans_enable(&self) -> INTERRUPT_TRANS_ENABLE_R {
        INTERRUPT_TRANS_ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 23:26 - The operations counter in both the master mode and the slave mode."]
    #[inline(always)]
    pub fn spi_trans_cnt(&self) -> SPI_TRANS_CNT_R {
        SPI_TRANS_CNT_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as 1: \"write-status\"; 4: \"read-status\"; 2: \"write-buffer\" and 3: \"read-buffer\"."]
    #[inline(always)]
    pub fn slv_cmd_define(&self) -> SLV_CMD_DEFINE_R {
        SLV_CMD_DEFINE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable read/write buffer"]
    #[inline(always)]
    pub fn sta_enable(&self) -> STA_ENABLE_R {
        STA_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable read/write buffer"]
    #[inline(always)]
    pub fn spi_buffer_enable(&self) -> SPI_BUFFER_ENABLE_R {
        SPI_BUFFER_ENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: slave mode, 0: master mode."]
    #[inline(always)]
    pub fn spi_slave_mode(&self) -> SPI_SLAVE_MODE_R {
        SPI_SLAVE_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - It is the synchronous reset signal of the module. This bit is self-cleared by hardware."]
    #[inline(always)]
    pub fn spi_sync_reset(&self) -> SPI_SYNC_RESET_R {
        SPI_SYNC_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SLAVE")
            .field(
                "spi_sync_reset",
                &format_args!("{}", self.spi_sync_reset().bit()),
            )
            .field(
                "spi_slave_mode",
                &format_args!("{}", self.spi_slave_mode().bit()),
            )
            .field(
                "slv_cmd_define",
                &format_args!("{}", self.slv_cmd_define().bit()),
            )
            .field(
                "spi_trans_cnt",
                &format_args!("{}", self.spi_trans_cnt().bits()),
            )
            .field("spi_int_en", &format_args!("{}", self.spi_int_en().bits()))
            .field(
                "spi_trans_done",
                &format_args!("{}", self.spi_trans_done().bit()),
            )
            .field(
                "slv_wr_sta_done",
                &format_args!("{}", self.slv_wr_sta_done().bit()),
            )
            .field(
                "slv_rd_sta_done",
                &format_args!("{}", self.slv_rd_sta_done().bit()),
            )
            .field(
                "slv_wr_buf_done",
                &format_args!("{}", self.slv_wr_buf_done().bit()),
            )
            .field(
                "slv_rd_buf_done",
                &format_args!("{}", self.slv_rd_buf_done().bit()),
            )
            .field(
                "spi_buffer_enable",
                &format_args!("{}", self.spi_buffer_enable().bit()),
            )
            .field("sta_enable", &format_args!("{}", self.sta_enable().bit()))
            .field(
                "interrupt_trans_enable",
                &format_args!("{}", self.interrupt_trans_enable().bit()),
            )
            .field(
                "interrupt_ws_enable",
                &format_args!("{}", self.interrupt_ws_enable().bit()),
            )
            .field(
                "interrupt_rs_enable",
                &format_args!("{}", self.interrupt_rs_enable().bit()),
            )
            .field(
                "interrupt_wb_enable",
                &format_args!("{}", self.interrupt_wb_enable().bit()),
            )
            .field(
                "interrupt_rb_enable",
                &format_args!("{}", self.interrupt_rb_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SLAVE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt raw bit for the completement of \"read-buffer\" operation in the slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_buf_done(&mut self) -> SLV_RD_BUF_DONE_W<0> {
        SLV_RD_BUF_DONE_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt raw bit for the completement of \"write-buffer\" operation in the slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_buf_done(&mut self) -> SLV_WR_BUF_DONE_W<1> {
        SLV_WR_BUF_DONE_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt raw bit for the completement of \"read-status\" operation in the slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_sta_done(&mut self) -> SLV_RD_STA_DONE_W<2> {
        SLV_RD_STA_DONE_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt raw bit for the completement of \"write-status\" operation in the slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_sta_done(&mut self) -> SLV_WR_STA_DONE_W<3> {
        SLV_WR_STA_DONE_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt raw bit for the completement of any operation in both the master mode and the slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_trans_done(&mut self) -> SPI_TRANS_DONE_W<4> {
        SPI_TRANS_DONE_W::new(self)
    }
    #[doc = "Bits 5:9 - Interrupt enable bits for the below 5 sources"]
    #[inline(always)]
    #[must_use]
    pub fn spi_int_en(&mut self) -> SPI_INT_EN_W<5> {
        SPI_INT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Enable buffer read interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_rb_enable(&mut self) -> INTERRUPT_RB_ENABLE_W<5> {
        INTERRUPT_RB_ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - Enable buffer write interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_wb_enable(&mut self) -> INTERRUPT_WB_ENABLE_W<6> {
        INTERRUPT_WB_ENABLE_W::new(self)
    }
    #[doc = "Bit 7 - Enable status read interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_rs_enable(&mut self) -> INTERRUPT_RS_ENABLE_W<7> {
        INTERRUPT_RS_ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Enable status write interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_ws_enable(&mut self) -> INTERRUPT_WS_ENABLE_W<8> {
        INTERRUPT_WS_ENABLE_W::new(self)
    }
    #[doc = "Bit 9 - Enable TRANS interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_trans_enable(&mut self) -> INTERRUPT_TRANS_ENABLE_W<9> {
        INTERRUPT_TRANS_ENABLE_W::new(self)
    }
    #[doc = "Bit 27 - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as 1: \"write-status\"; 4: \"read-status\"; 2: \"write-buffer\" and 3: \"read-buffer\"."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd_define(&mut self) -> SLV_CMD_DEFINE_W<27> {
        SLV_CMD_DEFINE_W::new(self)
    }
    #[doc = "Bit 28 - Enable read/write buffer"]
    #[inline(always)]
    #[must_use]
    pub fn sta_enable(&mut self) -> STA_ENABLE_W<28> {
        STA_ENABLE_W::new(self)
    }
    #[doc = "Bit 29 - Enable read/write buffer"]
    #[inline(always)]
    #[must_use]
    pub fn spi_buffer_enable(&mut self) -> SPI_BUFFER_ENABLE_W<29> {
        SPI_BUFFER_ENABLE_W::new(self)
    }
    #[doc = "Bit 30 - 1: slave mode, 0: master mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slave_mode(&mut self) -> SPI_SLAVE_MODE_W<30> {
        SPI_SLAVE_MODE_W::new(self)
    }
    #[doc = "Bit 31 - It is the synchronous reset signal of the module. This bit is self-cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn spi_sync_reset(&mut self) -> SPI_SYNC_RESET_W<31> {
        SPI_SYNC_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "It is the synchronous reset signal of the module. This bit is self-cleared by hardware.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_slave](index.html) module"]
pub struct SPI_SLAVE_SPEC;
impl crate::RegisterSpec for SPI_SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_slave::R](R) reader structure"]
impl crate::Readable for SPI_SLAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_slave::W](W) writer structure"]
impl crate::Writable for SPI_SLAVE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_SLAVE to value 0"]
impl crate::Resettable for SPI_SLAVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
