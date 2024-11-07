#[doc = "Register `SLAVE` reader"]
pub type R = crate::R<SLAVE_SPEC>;
#[doc = "Register `SLAVE` writer"]
pub type W = crate::W<SLAVE_SPEC>;
#[doc = "Field `SLV_RD_BUF_DONE` reader - The interrupt raw bit for the completion of read-buffer operation in the slave mode."]
pub type SLV_RD_BUF_DONE_R = crate::BitReader;
#[doc = "Field `SLV_RD_BUF_DONE` writer - The interrupt raw bit for the completion of read-buffer operation in the slave mode."]
pub type SLV_RD_BUF_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_BUF_DONE` reader - The interrupt raw bit for the completion of write-buffer operation in the slave mode."]
pub type SLV_WR_BUF_DONE_R = crate::BitReader;
#[doc = "Field `SLV_WR_BUF_DONE` writer - The interrupt raw bit for the completion of write-buffer operation in the slave mode."]
pub type SLV_WR_BUF_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RD_STA_DONE` reader - The interrupt raw bit for the completion of read-status operation in the slave mode."]
pub type SLV_RD_STA_DONE_R = crate::BitReader;
#[doc = "Field `SLV_RD_STA_DONE` writer - The interrupt raw bit for the completion of read-status operation in the slave mode."]
pub type SLV_RD_STA_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_STA_DONE` reader - The interrupt raw bit for the completion of write-status operation in the slave mode."]
pub type SLV_WR_STA_DONE_R = crate::BitReader;
#[doc = "Field `SLV_WR_STA_DONE` writer - The interrupt raw bit for the completion of write-status operation in the slave mode."]
pub type SLV_WR_STA_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_DONE` reader - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode."]
pub type TRANS_DONE_R = crate::BitReader;
#[doc = "Field `TRANS_DONE` writer - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode."]
pub type TRANS_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_EN` reader - Interrupt enable bits for the below 5 sources"]
pub type INT_EN_R = crate::FieldReader;
#[doc = "Field `INT_EN` writer - Interrupt enable bits for the below 5 sources"]
pub type INT_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CS_I_MODE` reader - In the slave mode this bits used to synchronize the input spi cs signal and eliminate spi cs jitter."]
pub type CS_I_MODE_R = crate::FieldReader;
#[doc = "Field `CS_I_MODE` writer - In the slave mode this bits used to synchronize the input spi cs signal and eliminate spi cs jitter."]
pub type CS_I_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLV_LAST_COMMAND` reader - In the slave mode it is the value of command."]
pub type SLV_LAST_COMMAND_R = crate::FieldReader;
#[doc = "Field `SLV_LAST_STATE` reader - In the slave mode it is the state of spi state machine."]
pub type SLV_LAST_STATE_R = crate::FieldReader;
#[doc = "Field `TRANS_CNT` reader - The operations counter in both the master mode and the slave mode. 4: read-status"]
pub type TRANS_CNT_R = crate::FieldReader;
#[doc = "Field `SLV_CMD_DEFINE` reader - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as: 1: write-status 2: write-buffer and 3: read-buffer."]
pub type SLV_CMD_DEFINE_R = crate::BitReader;
#[doc = "Field `SLV_CMD_DEFINE` writer - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as: 1: write-status 2: write-buffer and 3: read-buffer."]
pub type SLV_CMD_DEFINE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_RD_STA_EN` reader - write and read status enable in the slave mode"]
pub type SLV_WR_RD_STA_EN_R = crate::BitReader;
#[doc = "Field `SLV_WR_RD_STA_EN` writer - write and read status enable in the slave mode"]
pub type SLV_WR_RD_STA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_RD_BUF_EN` reader - write and read buffer enable in the slave mode"]
pub type SLV_WR_RD_BUF_EN_R = crate::BitReader;
#[doc = "Field `SLV_WR_RD_BUF_EN` writer - write and read buffer enable in the slave mode"]
pub type SLV_WR_RD_BUF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - 1: slave mode 0: master mode."]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - 1: slave mode 0: master mode."]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_RESET` reader - Software reset enable, reset the spi clock line cs line and data lines."]
pub type SYNC_RESET_R = crate::BitReader;
#[doc = "Field `SYNC_RESET` writer - Software reset enable, reset the spi clock line cs line and data lines."]
pub type SYNC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt raw bit for the completion of read-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&self) -> SLV_RD_BUF_DONE_R {
        SLV_RD_BUF_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for the completion of write-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&self) -> SLV_WR_BUF_DONE_R {
        SLV_WR_BUF_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for the completion of read-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_sta_done(&self) -> SLV_RD_STA_DONE_R {
        SLV_RD_STA_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for the completion of write-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_sta_done(&self) -> SLV_WR_STA_DONE_R {
        SLV_WR_STA_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode."]
    #[inline(always)]
    pub fn trans_done(&self) -> TRANS_DONE_R {
        TRANS_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:9 - Interrupt enable bits for the below 5 sources"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - In the slave mode this bits used to synchronize the input spi cs signal and eliminate spi cs jitter."]
    #[inline(always)]
    pub fn cs_i_mode(&self) -> CS_I_MODE_R {
        CS_I_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 17:19 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn slv_last_command(&self) -> SLV_LAST_COMMAND_R {
        SLV_LAST_COMMAND_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - In the slave mode it is the state of spi state machine."]
    #[inline(always)]
    pub fn slv_last_state(&self) -> SLV_LAST_STATE_R {
        SLV_LAST_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:26 - The operations counter in both the master mode and the slave mode. 4: read-status"]
    #[inline(always)]
    pub fn trans_cnt(&self) -> TRANS_CNT_R {
        TRANS_CNT_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as: 1: write-status 2: write-buffer and 3: read-buffer."]
    #[inline(always)]
    pub fn slv_cmd_define(&self) -> SLV_CMD_DEFINE_R {
        SLV_CMD_DEFINE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - write and read status enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_sta_en(&self) -> SLV_WR_RD_STA_EN_R {
        SLV_WR_RD_STA_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - write and read buffer enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_buf_en(&self) -> SLV_WR_RD_BUF_EN_R {
        SLV_WR_RD_BUF_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Software reset enable, reset the spi clock line cs line and data lines."]
    #[inline(always)]
    pub fn sync_reset(&self) -> SYNC_RESET_R {
        SYNC_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE")
            .field("slv_rd_buf_done", &self.slv_rd_buf_done())
            .field("slv_wr_buf_done", &self.slv_wr_buf_done())
            .field("slv_rd_sta_done", &self.slv_rd_sta_done())
            .field("slv_wr_sta_done", &self.slv_wr_sta_done())
            .field("trans_done", &self.trans_done())
            .field("int_en", &self.int_en())
            .field("cs_i_mode", &self.cs_i_mode())
            .field("slv_last_command", &self.slv_last_command())
            .field("slv_last_state", &self.slv_last_state())
            .field("trans_cnt", &self.trans_cnt())
            .field("slv_cmd_define", &self.slv_cmd_define())
            .field("slv_wr_rd_sta_en", &self.slv_wr_rd_sta_en())
            .field("slv_wr_rd_buf_en", &self.slv_wr_rd_buf_en())
            .field("mode", &self.mode())
            .field("sync_reset", &self.sync_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt raw bit for the completion of read-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&mut self) -> SLV_RD_BUF_DONE_W<SLAVE_SPEC> {
        SLV_RD_BUF_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for the completion of write-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&mut self) -> SLV_WR_BUF_DONE_W<SLAVE_SPEC> {
        SLV_WR_BUF_DONE_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt raw bit for the completion of read-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_sta_done(&mut self) -> SLV_RD_STA_DONE_W<SLAVE_SPEC> {
        SLV_RD_STA_DONE_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt raw bit for the completion of write-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_sta_done(&mut self) -> SLV_WR_STA_DONE_W<SLAVE_SPEC> {
        SLV_WR_STA_DONE_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode."]
    #[inline(always)]
    pub fn trans_done(&mut self) -> TRANS_DONE_W<SLAVE_SPEC> {
        TRANS_DONE_W::new(self, 4)
    }
    #[doc = "Bits 5:9 - Interrupt enable bits for the below 5 sources"]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W<SLAVE_SPEC> {
        INT_EN_W::new(self, 5)
    }
    #[doc = "Bits 10:11 - In the slave mode this bits used to synchronize the input spi cs signal and eliminate spi cs jitter."]
    #[inline(always)]
    pub fn cs_i_mode(&mut self) -> CS_I_MODE_W<SLAVE_SPEC> {
        CS_I_MODE_W::new(self, 10)
    }
    #[doc = "Bit 27 - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as: 1: write-status 2: write-buffer and 3: read-buffer."]
    #[inline(always)]
    pub fn slv_cmd_define(&mut self) -> SLV_CMD_DEFINE_W<SLAVE_SPEC> {
        SLV_CMD_DEFINE_W::new(self, 27)
    }
    #[doc = "Bit 28 - write and read status enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_sta_en(&mut self) -> SLV_WR_RD_STA_EN_W<SLAVE_SPEC> {
        SLV_WR_RD_STA_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - write and read buffer enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_buf_en(&mut self) -> SLV_WR_RD_BUF_EN_W<SLAVE_SPEC> {
        SLV_WR_RD_BUF_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<SLAVE_SPEC> {
        MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Software reset enable, reset the spi clock line cs line and data lines."]
    #[inline(always)]
    pub fn sync_reset(&mut self) -> SYNC_RESET_W<SLAVE_SPEC> {
        SYNC_RESET_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`slave::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE_SPEC;
impl crate::RegisterSpec for SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave::R`](R) reader structure"]
impl crate::Readable for SLAVE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave::W`](W) writer structure"]
impl crate::Writable for SLAVE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLAVE to value 0x20"]
impl crate::Resettable for SLAVE_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
