#[doc = "Register `CACHE_SCTRL` reader"]
pub type R = crate::R<CACHE_SCTRL_SPEC>;
#[doc = "Register `CACHE_SCTRL` writer"]
pub type W = crate::W<CACHE_SCTRL_SPEC>;
#[doc = "Field `CACHE_USR_SCMD_4BYTE` reader - Set this bit to enable SPI0 read Ext_RAM with 32 bits address. The value of SPI_MEM_SRAM_ADDR_BITLEN should be 31."]
pub type CACHE_USR_SCMD_4BYTE_R = crate::BitReader;
#[doc = "Field `CACHE_USR_SCMD_4BYTE` writer - Set this bit to enable SPI0 read Ext_RAM with 32 bits address. The value of SPI_MEM_SRAM_ADDR_BITLEN should be 31."]
pub type CACHE_USR_SCMD_4BYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_SRAM_DIO` reader - Set the bit to enable 2-bm in all the phases of SPI0 Ext_RAM transfer."]
pub type USR_SRAM_DIO_R = crate::BitReader;
#[doc = "Field `USR_SRAM_DIO` writer - Set the bit to enable 2-bm in all the phases of SPI0 Ext_RAM transfer."]
pub type USR_SRAM_DIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_SRAM_QIO` reader - Set the bit to enable QPI mode in all SPI0 Ext_RAM transfer."]
pub type USR_SRAM_QIO_R = crate::BitReader;
#[doc = "Field `USR_SRAM_QIO` writer - Set the bit to enable QPI mode in all SPI0 Ext_RAM transfer."]
pub type USR_SRAM_QIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_WR_SRAM_DUMMY` reader - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in write operations."]
pub type USR_WR_SRAM_DUMMY_R = crate::BitReader;
#[doc = "Field `USR_WR_SRAM_DUMMY` writer - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in write operations."]
pub type USR_WR_SRAM_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_RD_SRAM_DUMMY` reader - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in read operations."]
pub type USR_RD_SRAM_DUMMY_R = crate::BitReader;
#[doc = "Field `USR_RD_SRAM_DUMMY` writer - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in read operations."]
pub type USR_RD_SRAM_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_SRAM_USR_RCMD` reader - 1: The command value of SPI0 read Ext_RAM is SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE. 0: The value is 0x2."]
pub type CACHE_SRAM_USR_RCMD_R = crate::BitReader;
#[doc = "Field `CACHE_SRAM_USR_RCMD` writer - 1: The command value of SPI0 read Ext_RAM is SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE. 0: The value is 0x2."]
pub type CACHE_SRAM_USR_RCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_RDUMMY_CYCLELEN` reader - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in read data transfer."]
pub type SRAM_RDUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SRAM_RDUMMY_CYCLELEN` writer - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in read data transfer."]
pub type SRAM_RDUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SRAM_ADDR_BITLEN` reader - When SPI0 accesses to Ext_RAM, it is the length in bits of ADDR phase. The register value shall be (bit_num-1)."]
pub type SRAM_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `SRAM_ADDR_BITLEN` writer - When SPI0 accesses to Ext_RAM, it is the length in bits of ADDR phase. The register value shall be (bit_num-1)."]
pub type SRAM_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CACHE_SRAM_USR_WCMD` reader - 1: The command value of SPI0 write Ext_RAM is SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE. 0: The value is 0x3."]
pub type CACHE_SRAM_USR_WCMD_R = crate::BitReader;
#[doc = "Field `CACHE_SRAM_USR_WCMD` writer - 1: The command value of SPI0 write Ext_RAM is SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE. 0: The value is 0x3."]
pub type CACHE_SRAM_USR_WCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_OCT` reader - Set the bit to enable OPI mode in all SPI0 Ext_RAM transfer."]
pub type SRAM_OCT_R = crate::BitReader;
#[doc = "Field `SRAM_OCT` writer - Set the bit to enable OPI mode in all SPI0 Ext_RAM transfer."]
pub type SRAM_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM_WDUMMY_CYCLELEN` reader - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in write data transfer."]
pub type SRAM_WDUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SRAM_WDUMMY_CYCLELEN` writer - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in write data transfer."]
pub type SRAM_WDUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable SPI0 read Ext_RAM with 32 bits address. The value of SPI_MEM_SRAM_ADDR_BITLEN should be 31."]
    #[inline(always)]
    pub fn cache_usr_scmd_4byte(&self) -> CACHE_USR_SCMD_4BYTE_R {
        CACHE_USR_SCMD_4BYTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set the bit to enable 2-bm in all the phases of SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn usr_sram_dio(&self) -> USR_SRAM_DIO_R {
        USR_SRAM_DIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set the bit to enable QPI mode in all SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn usr_sram_qio(&self) -> USR_SRAM_QIO_R {
        USR_SRAM_QIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&self) -> USR_WR_SRAM_DUMMY_R {
        USR_WR_SRAM_DUMMY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&self) -> USR_RD_SRAM_DUMMY_R {
        USR_RD_SRAM_DUMMY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: The command value of SPI0 read Ext_RAM is SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE. 0: The value is 0x2."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&self) -> CACHE_SRAM_USR_RCMD_R {
        CACHE_SRAM_USR_RCMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in read data transfer."]
    #[inline(always)]
    pub fn sram_rdummy_cyclelen(&self) -> SRAM_RDUMMY_CYCLELEN_R {
        SRAM_RDUMMY_CYCLELEN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - When SPI0 accesses to Ext_RAM, it is the length in bits of ADDR phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&self) -> SRAM_ADDR_BITLEN_R {
        SRAM_ADDR_BITLEN_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - 1: The command value of SPI0 write Ext_RAM is SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE. 0: The value is 0x3."]
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&self) -> CACHE_SRAM_USR_WCMD_R {
        CACHE_SRAM_USR_WCMD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set the bit to enable OPI mode in all SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn sram_oct(&self) -> SRAM_OCT_R {
        SRAM_OCT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:27 - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in write data transfer."]
    #[inline(always)]
    pub fn sram_wdummy_cyclelen(&self) -> SRAM_WDUMMY_CYCLELEN_R {
        SRAM_WDUMMY_CYCLELEN_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SCTRL")
            .field("cache_usr_scmd_4byte", &self.cache_usr_scmd_4byte())
            .field("usr_sram_dio", &self.usr_sram_dio())
            .field("usr_sram_qio", &self.usr_sram_qio())
            .field("usr_wr_sram_dummy", &self.usr_wr_sram_dummy())
            .field("usr_rd_sram_dummy", &self.usr_rd_sram_dummy())
            .field("cache_sram_usr_rcmd", &self.cache_sram_usr_rcmd())
            .field("sram_rdummy_cyclelen", &self.sram_rdummy_cyclelen())
            .field("sram_addr_bitlen", &self.sram_addr_bitlen())
            .field("cache_sram_usr_wcmd", &self.cache_sram_usr_wcmd())
            .field("sram_oct", &self.sram_oct())
            .field("sram_wdummy_cyclelen", &self.sram_wdummy_cyclelen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable SPI0 read Ext_RAM with 32 bits address. The value of SPI_MEM_SRAM_ADDR_BITLEN should be 31."]
    #[inline(always)]
    pub fn cache_usr_scmd_4byte(&mut self) -> CACHE_USR_SCMD_4BYTE_W<CACHE_SCTRL_SPEC> {
        CACHE_USR_SCMD_4BYTE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set the bit to enable 2-bm in all the phases of SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn usr_sram_dio(&mut self) -> USR_SRAM_DIO_W<CACHE_SCTRL_SPEC> {
        USR_SRAM_DIO_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set the bit to enable QPI mode in all SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn usr_sram_qio(&mut self) -> USR_SRAM_QIO_W<CACHE_SCTRL_SPEC> {
        USR_SRAM_QIO_W::new(self, 2)
    }
    #[doc = "Bit 3 - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&mut self) -> USR_WR_SRAM_DUMMY_W<CACHE_SCTRL_SPEC> {
        USR_WR_SRAM_DUMMY_W::new(self, 3)
    }
    #[doc = "Bit 4 - When SPI0 accesses to Ext_RAM, set this bit to enable DUMMY phase in read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&mut self) -> USR_RD_SRAM_DUMMY_W<CACHE_SCTRL_SPEC> {
        USR_RD_SRAM_DUMMY_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: The command value of SPI0 read Ext_RAM is SPI_MEM_CACHE_SRAM_USR_WR_CMD_VALUE. 0: The value is 0x2."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&mut self) -> CACHE_SRAM_USR_RCMD_W<CACHE_SCTRL_SPEC> {
        CACHE_SRAM_USR_RCMD_W::new(self, 5)
    }
    #[doc = "Bits 6:11 - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in read data transfer."]
    #[inline(always)]
    pub fn sram_rdummy_cyclelen(&mut self) -> SRAM_RDUMMY_CYCLELEN_W<CACHE_SCTRL_SPEC> {
        SRAM_RDUMMY_CYCLELEN_W::new(self, 6)
    }
    #[doc = "Bits 14:19 - When SPI0 accesses to Ext_RAM, it is the length in bits of ADDR phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&mut self) -> SRAM_ADDR_BITLEN_W<CACHE_SCTRL_SPEC> {
        SRAM_ADDR_BITLEN_W::new(self, 14)
    }
    #[doc = "Bit 20 - 1: The command value of SPI0 write Ext_RAM is SPI_MEM_CACHE_SRAM_USR_RD_CMD_VALUE. 0: The value is 0x3."]
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&mut self) -> CACHE_SRAM_USR_WCMD_W<CACHE_SCTRL_SPEC> {
        CACHE_SRAM_USR_WCMD_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set the bit to enable OPI mode in all SPI0 Ext_RAM transfer."]
    #[inline(always)]
    pub fn sram_oct(&mut self) -> SRAM_OCT_W<CACHE_SCTRL_SPEC> {
        SRAM_OCT_W::new(self, 21)
    }
    #[doc = "Bits 22:27 - When SPI0 accesses to Ext_RAM, it is the SPI_CLK cycles minus 1 of DUMMY phase in write data transfer."]
    #[inline(always)]
    pub fn sram_wdummy_cyclelen(&mut self) -> SRAM_WDUMMY_CYCLELEN_W<CACHE_SCTRL_SPEC> {
        SRAM_WDUMMY_CYCLELEN_W::new(self, 22)
    }
}
#[doc = "SPI0 external RAM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_SCTRL_SPEC;
impl crate::RegisterSpec for CACHE_SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_sctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_SCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_sctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_SCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_SCTRL to value 0x0055_c070"]
impl crate::Resettable for CACHE_SCTRL_SPEC {
    const RESET_VALUE: u32 = 0x0055_c070;
}
