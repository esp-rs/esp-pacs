#[doc = "Register `NAND_FLASH_SPI_SEQ0` reader"]
pub type R = crate::R<NAND_FLASH_SPI_SEQ0_SPEC>;
#[doc = "Field `SEQ_TAIL_FLG` reader - "]
pub type SEQ_TAIL_FLG_R = crate::BitReader;
#[doc = "Field `SR_CHK_EN` reader - "]
pub type SR_CHK_EN_R = crate::BitReader;
#[doc = "Field `DIN_INDEX` reader - "]
pub type DIN_INDEX_R = crate::FieldReader;
#[doc = "Field `ADDR_INDEX` reader - "]
pub type ADDR_INDEX_R = crate::FieldReader;
#[doc = "Field `REQ_OR_CFG` reader - "]
pub type REQ_OR_CFG_R = crate::BitReader;
#[doc = "Field `CMD_INDEX` reader - "]
pub type CMD_INDEX_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn seq_tail_flg(&self) -> SEQ_TAIL_FLG_R {
        SEQ_TAIL_FLG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sr_chk_en(&self) -> SR_CHK_EN_R {
        SR_CHK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn din_index(&self) -> DIN_INDEX_R {
        DIN_INDEX_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn addr_index(&self) -> ADDR_INDEX_R {
        ADDR_INDEX_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn req_or_cfg(&self) -> REQ_OR_CFG_R {
        REQ_OR_CFG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMD_INDEX_R {
        CMD_INDEX_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_SPI_SEQ0")
            .field("seq_tail_flg", &self.seq_tail_flg())
            .field("sr_chk_en", &self.sr_chk_en())
            .field("din_index", &self.din_index())
            .field("addr_index", &self.addr_index())
            .field("req_or_cfg", &self.req_or_cfg())
            .field("cmd_index", &self.cmd_index())
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_spi_seq0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_SPI_SEQ0_SPEC;
impl crate::RegisterSpec for NAND_FLASH_SPI_SEQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_spi_seq0::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_SPI_SEQ0_SPEC {}
#[doc = "`reset()` method sets NAND_FLASH_SPI_SEQ0 to value 0"]
impl crate::Resettable for NAND_FLASH_SPI_SEQ0_SPEC {}
