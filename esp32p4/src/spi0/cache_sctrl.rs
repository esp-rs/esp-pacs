///Register `CACHE_SCTRL` reader
pub type R = crate::R<CACHE_SCTRL_SPEC>;
///Register `CACHE_SCTRL` writer
pub type W = crate::W<CACHE_SCTRL_SPEC>;
///Field `CACHE_USR_SADDR_4BYTE` reader - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable.
pub type CACHE_USR_SADDR_4BYTE_R = crate::BitReader;
///Field `CACHE_USR_SADDR_4BYTE` writer - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable.
pub type CACHE_USR_SADDR_4BYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USR_SRAM_DIO` reader - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable
pub type USR_SRAM_DIO_R = crate::BitReader;
///Field `USR_SRAM_DIO` writer - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable
pub type USR_SRAM_DIO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USR_SRAM_QIO` reader - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable
pub type USR_SRAM_QIO_R = crate::BitReader;
///Field `USR_SRAM_QIO` writer - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable
pub type USR_SRAM_QIO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USR_WR_SRAM_DUMMY` reader - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations.
pub type USR_WR_SRAM_DUMMY_R = crate::BitReader;
///Field `USR_WR_SRAM_DUMMY` writer - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations.
pub type USR_WR_SRAM_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USR_RD_SRAM_DUMMY` reader - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations.
pub type USR_RD_SRAM_DUMMY_R = crate::BitReader;
///Field `USR_RD_SRAM_DUMMY` writer - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations.
pub type USR_RD_SRAM_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHE_SRAM_USR_RCMD` reader - For SPI0, In the external RAM mode cache read external RAM for user define command.
pub type CACHE_SRAM_USR_RCMD_R = crate::BitReader;
///Field `CACHE_SRAM_USR_RCMD` writer - For SPI0, In the external RAM mode cache read external RAM for user define command.
pub type CACHE_SRAM_USR_RCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM_RDUMMY_CYCLELEN` reader - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1).
pub type SRAM_RDUMMY_CYCLELEN_R = crate::FieldReader;
///Field `SRAM_RDUMMY_CYCLELEN` writer - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1).
pub type SRAM_RDUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SRAM_ADDR_BITLEN` reader - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1).
pub type SRAM_ADDR_BITLEN_R = crate::FieldReader;
///Field `SRAM_ADDR_BITLEN` writer - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1).
pub type SRAM_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CACHE_SRAM_USR_WCMD` reader - For SPI0, In the external RAM mode cache write sram for user define command
pub type CACHE_SRAM_USR_WCMD_R = crate::BitReader;
///Field `CACHE_SRAM_USR_WCMD` writer - For SPI0, In the external RAM mode cache write sram for user define command
pub type CACHE_SRAM_USR_WCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM_OCT` reader - reserved
pub type SRAM_OCT_R = crate::BitReader;
///Field `SRAM_OCT` writer - reserved
pub type SRAM_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM_WDUMMY_CYCLELEN` reader - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1).
pub type SRAM_WDUMMY_CYCLELEN_R = crate::FieldReader;
///Field `SRAM_WDUMMY_CYCLELEN` writer - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1).
pub type SRAM_WDUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bit 0 - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable.
    #[inline(always)]
    pub fn cache_usr_saddr_4byte(&self) -> CACHE_USR_SADDR_4BYTE_R {
        CACHE_USR_SADDR_4BYTE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable
    #[inline(always)]
    pub fn usr_sram_dio(&self) -> USR_SRAM_DIO_R {
        USR_SRAM_DIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable
    #[inline(always)]
    pub fn usr_sram_qio(&self) -> USR_SRAM_QIO_R {
        USR_SRAM_QIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations.
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&self) -> USR_WR_SRAM_DUMMY_R {
        USR_WR_SRAM_DUMMY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations.
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&self) -> USR_RD_SRAM_DUMMY_R {
        USR_RD_SRAM_DUMMY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - For SPI0, In the external RAM mode cache read external RAM for user define command.
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&self) -> CACHE_SRAM_USR_RCMD_R {
        CACHE_SRAM_USR_RCMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:11 - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn sram_rdummy_cyclelen(&self) -> SRAM_RDUMMY_CYCLELEN_R {
        SRAM_RDUMMY_CYCLELEN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    ///Bits 14:19 - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn sram_addr_bitlen(&self) -> SRAM_ADDR_BITLEN_R {
        SRAM_ADDR_BITLEN_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    ///Bit 20 - For SPI0, In the external RAM mode cache write sram for user define command
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&self) -> CACHE_SRAM_USR_WCMD_R {
        CACHE_SRAM_USR_WCMD_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - reserved
    #[inline(always)]
    pub fn sram_oct(&self) -> SRAM_OCT_R {
        SRAM_OCT_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:27 - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    pub fn sram_wdummy_cyclelen(&self) -> SRAM_WDUMMY_CYCLELEN_R {
        SRAM_WDUMMY_CYCLELEN_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SCTRL")
            .field("cache_usr_saddr_4byte", &self.cache_usr_saddr_4byte())
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
    ///Bit 0 - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable.
    #[inline(always)]
    #[must_use]
    pub fn cache_usr_saddr_4byte(&mut self) -> CACHE_USR_SADDR_4BYTE_W<CACHE_SCTRL_SPEC> {
        CACHE_USR_SADDR_4BYTE_W::new(self, 0)
    }
    ///Bit 1 - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable
    #[inline(always)]
    #[must_use]
    pub fn usr_sram_dio(&mut self) -> USR_SRAM_DIO_W<CACHE_SCTRL_SPEC> {
        USR_SRAM_DIO_W::new(self, 1)
    }
    ///Bit 2 - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable
    #[inline(always)]
    #[must_use]
    pub fn usr_sram_qio(&mut self) -> USR_SRAM_QIO_W<CACHE_SCTRL_SPEC> {
        USR_SRAM_QIO_W::new(self, 2)
    }
    ///Bit 3 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations.
    #[inline(always)]
    #[must_use]
    pub fn usr_wr_sram_dummy(&mut self) -> USR_WR_SRAM_DUMMY_W<CACHE_SCTRL_SPEC> {
        USR_WR_SRAM_DUMMY_W::new(self, 3)
    }
    ///Bit 4 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations.
    #[inline(always)]
    #[must_use]
    pub fn usr_rd_sram_dummy(&mut self) -> USR_RD_SRAM_DUMMY_W<CACHE_SCTRL_SPEC> {
        USR_RD_SRAM_DUMMY_W::new(self, 4)
    }
    ///Bit 5 - For SPI0, In the external RAM mode cache read external RAM for user define command.
    #[inline(always)]
    #[must_use]
    pub fn cache_sram_usr_rcmd(&mut self) -> CACHE_SRAM_USR_RCMD_W<CACHE_SCTRL_SPEC> {
        CACHE_SRAM_USR_RCMD_W::new(self, 5)
    }
    ///Bits 6:11 - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    #[must_use]
    pub fn sram_rdummy_cyclelen(&mut self) -> SRAM_RDUMMY_CYCLELEN_W<CACHE_SCTRL_SPEC> {
        SRAM_RDUMMY_CYCLELEN_W::new(self, 6)
    }
    ///Bits 14:19 - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1).
    #[inline(always)]
    #[must_use]
    pub fn sram_addr_bitlen(&mut self) -> SRAM_ADDR_BITLEN_W<CACHE_SCTRL_SPEC> {
        SRAM_ADDR_BITLEN_W::new(self, 14)
    }
    ///Bit 20 - For SPI0, In the external RAM mode cache write sram for user define command
    #[inline(always)]
    #[must_use]
    pub fn cache_sram_usr_wcmd(&mut self) -> CACHE_SRAM_USR_WCMD_W<CACHE_SCTRL_SPEC> {
        CACHE_SRAM_USR_WCMD_W::new(self, 20)
    }
    ///Bit 21 - reserved
    #[inline(always)]
    #[must_use]
    pub fn sram_oct(&mut self) -> SRAM_OCT_W<CACHE_SCTRL_SPEC> {
        SRAM_OCT_W::new(self, 21)
    }
    ///Bits 22:27 - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1).
    #[inline(always)]
    #[must_use]
    pub fn sram_wdummy_cyclelen(&mut self) -> SRAM_WDUMMY_CYCLELEN_W<CACHE_SCTRL_SPEC> {
        SRAM_WDUMMY_CYCLELEN_W::new(self, 22)
    }
}
/**SPI0 external RAM control register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_sctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_sctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_SCTRL_SPEC;
impl crate::RegisterSpec for CACHE_SCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_sctrl::R`](R) reader structure
impl crate::Readable for CACHE_SCTRL_SPEC {}
///`write(|w| ..)` method takes [`cache_sctrl::W`](W) writer structure
impl crate::Writable for CACHE_SCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_SCTRL to value 0x0055_c070
impl crate::Resettable for CACHE_SCTRL_SPEC {
    const RESET_VALUE: u32 = 0x0055_c070;
}
