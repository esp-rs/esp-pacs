#[doc = "Register `MEM_CACHE_SCTRL` reader"]
pub type R = crate::R<MEM_CACHE_SCTRL_SPEC>;
#[doc = "Register `MEM_CACHE_SCTRL` writer"]
pub type W = crate::W<MEM_CACHE_SCTRL_SPEC>;
#[doc = "Field `MEM_CACHE_USR_SADDR_4BYTE` reader - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable."]
pub type MEM_CACHE_USR_SADDR_4BYTE_R = crate::BitReader;
#[doc = "Field `MEM_CACHE_USR_SADDR_4BYTE` writer - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable."]
pub type MEM_CACHE_USR_SADDR_4BYTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_USR_SRAM_DIO` reader - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable"]
pub type MEM_USR_SRAM_DIO_R = crate::BitReader;
#[doc = "Field `MEM_USR_SRAM_DIO` writer - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable"]
pub type MEM_USR_SRAM_DIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_USR_SRAM_QIO` reader - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable"]
pub type MEM_USR_SRAM_QIO_R = crate::BitReader;
#[doc = "Field `MEM_USR_SRAM_QIO` writer - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable"]
pub type MEM_USR_SRAM_QIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_USR_WR_SRAM_DUMMY` reader - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations."]
pub type MEM_USR_WR_SRAM_DUMMY_R = crate::BitReader;
#[doc = "Field `MEM_USR_WR_SRAM_DUMMY` writer - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations."]
pub type MEM_USR_WR_SRAM_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_USR_RD_SRAM_DUMMY` reader - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations."]
pub type MEM_USR_RD_SRAM_DUMMY_R = crate::BitReader;
#[doc = "Field `MEM_USR_RD_SRAM_DUMMY` writer - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations."]
pub type MEM_USR_RD_SRAM_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CACHE_SRAM_USR_RCMD` reader - For SPI0, In the external RAM mode cache read external RAM for user define command."]
pub type MEM_CACHE_SRAM_USR_RCMD_R = crate::BitReader;
#[doc = "Field `MEM_CACHE_SRAM_USR_RCMD` writer - For SPI0, In the external RAM mode cache read external RAM for user define command."]
pub type MEM_CACHE_SRAM_USR_RCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SRAM_RDUMMY_CYCLELEN` reader - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1)."]
pub type MEM_SRAM_RDUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `MEM_SRAM_RDUMMY_CYCLELEN` writer - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1)."]
pub type MEM_SRAM_RDUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MEM_SRAM_ADDR_BITLEN` reader - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub type MEM_SRAM_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `MEM_SRAM_ADDR_BITLEN` writer - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub type MEM_SRAM_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MEM_CACHE_SRAM_USR_WCMD` reader - For SPI0, In the external RAM mode cache write sram for user define command"]
pub type MEM_CACHE_SRAM_USR_WCMD_R = crate::BitReader;
#[doc = "Field `MEM_CACHE_SRAM_USR_WCMD` writer - For SPI0, In the external RAM mode cache write sram for user define command"]
pub type MEM_CACHE_SRAM_USR_WCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SRAM_OCT` reader - reserved"]
pub type MEM_SRAM_OCT_R = crate::BitReader;
#[doc = "Field `MEM_SRAM_OCT` writer - reserved"]
pub type MEM_SRAM_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_SRAM_WDUMMY_CYCLELEN` reader - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1)."]
pub type MEM_SRAM_WDUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `MEM_SRAM_WDUMMY_CYCLELEN` writer - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1)."]
pub type MEM_SRAM_WDUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn mem_cache_usr_saddr_4byte(&self) -> MEM_CACHE_USR_SADDR_4BYTE_R {
        MEM_CACHE_USR_SADDR_4BYTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable"]
    #[inline(always)]
    pub fn mem_usr_sram_dio(&self) -> MEM_USR_SRAM_DIO_R {
        MEM_USR_SRAM_DIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable"]
    #[inline(always)]
    pub fn mem_usr_sram_qio(&self) -> MEM_USR_SRAM_QIO_R {
        MEM_USR_SRAM_QIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn mem_usr_wr_sram_dummy(&self) -> MEM_USR_WR_SRAM_DUMMY_R {
        MEM_USR_WR_SRAM_DUMMY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn mem_usr_rd_sram_dummy(&self) -> MEM_USR_RD_SRAM_DUMMY_R {
        MEM_USR_RD_SRAM_DUMMY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI0, In the external RAM mode cache read external RAM for user define command."]
    #[inline(always)]
    pub fn mem_cache_sram_usr_rcmd(&self) -> MEM_CACHE_SRAM_USR_RCMD_R {
        MEM_CACHE_SRAM_USR_RCMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn mem_sram_rdummy_cyclelen(&self) -> MEM_SRAM_RDUMMY_CYCLELEN_R {
        MEM_SRAM_RDUMMY_CYCLELEN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn mem_sram_addr_bitlen(&self) -> MEM_SRAM_ADDR_BITLEN_R {
        MEM_SRAM_ADDR_BITLEN_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - For SPI0, In the external RAM mode cache write sram for user define command"]
    #[inline(always)]
    pub fn mem_cache_sram_usr_wcmd(&self) -> MEM_CACHE_SRAM_USR_WCMD_R {
        MEM_CACHE_SRAM_USR_WCMD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - reserved"]
    #[inline(always)]
    pub fn mem_sram_oct(&self) -> MEM_SRAM_OCT_R {
        MEM_SRAM_OCT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:27 - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn mem_sram_wdummy_cyclelen(&self) -> MEM_SRAM_WDUMMY_CYCLELEN_R {
        MEM_SRAM_WDUMMY_CYCLELEN_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CACHE_SCTRL")
            .field(
                "mem_cache_usr_saddr_4byte",
                &self.mem_cache_usr_saddr_4byte(),
            )
            .field("mem_usr_sram_dio", &self.mem_usr_sram_dio())
            .field("mem_usr_sram_qio", &self.mem_usr_sram_qio())
            .field("mem_usr_wr_sram_dummy", &self.mem_usr_wr_sram_dummy())
            .field("mem_usr_rd_sram_dummy", &self.mem_usr_rd_sram_dummy())
            .field("mem_cache_sram_usr_rcmd", &self.mem_cache_sram_usr_rcmd())
            .field("mem_sram_rdummy_cyclelen", &self.mem_sram_rdummy_cyclelen())
            .field("mem_sram_addr_bitlen", &self.mem_sram_addr_bitlen())
            .field("mem_cache_sram_usr_wcmd", &self.mem_cache_sram_usr_wcmd())
            .field("mem_sram_oct", &self.mem_sram_oct())
            .field("mem_sram_wdummy_cyclelen", &self.mem_sram_wdummy_cyclelen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0, In the external RAM mode, cache read flash with 4 bytes command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn mem_cache_usr_saddr_4byte(
        &mut self,
    ) -> MEM_CACHE_USR_SADDR_4BYTE_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_CACHE_USR_SADDR_4BYTE_W::new(self, 0)
    }
    #[doc = "Bit 1 - For SPI0, In the external RAM mode, spi dual I/O mode enable, 1: enable, 0:disable"]
    #[inline(always)]
    pub fn mem_usr_sram_dio(&mut self) -> MEM_USR_SRAM_DIO_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_USR_SRAM_DIO_W::new(self, 1)
    }
    #[doc = "Bit 2 - For SPI0, In the external RAM mode, spi quad I/O mode enable, 1: enable, 0:disable"]
    #[inline(always)]
    pub fn mem_usr_sram_qio(&mut self) -> MEM_USR_SRAM_QIO_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_USR_SRAM_QIO_W::new(self, 2)
    }
    #[doc = "Bit 3 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn mem_usr_wr_sram_dummy(&mut self) -> MEM_USR_WR_SRAM_DUMMY_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_USR_WR_SRAM_DUMMY_W::new(self, 3)
    }
    #[doc = "Bit 4 - For SPI0, In the external RAM mode, it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn mem_usr_rd_sram_dummy(&mut self) -> MEM_USR_RD_SRAM_DUMMY_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_USR_RD_SRAM_DUMMY_W::new(self, 4)
    }
    #[doc = "Bit 5 - For SPI0, In the external RAM mode cache read external RAM for user define command."]
    #[inline(always)]
    pub fn mem_cache_sram_usr_rcmd(&mut self) -> MEM_CACHE_SRAM_USR_RCMD_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_CACHE_SRAM_USR_RCMD_W::new(self, 5)
    }
    #[doc = "Bits 6:11 - For SPI0, In the external RAM mode, it is the length in bits of read dummy phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn mem_sram_rdummy_cyclelen(&mut self) -> MEM_SRAM_RDUMMY_CYCLELEN_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_SRAM_RDUMMY_CYCLELEN_W::new(self, 6)
    }
    #[doc = "Bits 14:19 - For SPI0, In the external RAM mode, it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn mem_sram_addr_bitlen(&mut self) -> MEM_SRAM_ADDR_BITLEN_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_SRAM_ADDR_BITLEN_W::new(self, 14)
    }
    #[doc = "Bit 20 - For SPI0, In the external RAM mode cache write sram for user define command"]
    #[inline(always)]
    pub fn mem_cache_sram_usr_wcmd(&mut self) -> MEM_CACHE_SRAM_USR_WCMD_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_CACHE_SRAM_USR_WCMD_W::new(self, 20)
    }
    #[doc = "Bit 21 - reserved"]
    #[inline(always)]
    pub fn mem_sram_oct(&mut self) -> MEM_SRAM_OCT_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_SRAM_OCT_W::new(self, 21)
    }
    #[doc = "Bits 22:27 - For SPI0, In the external RAM mode, it is the length in bits of write dummy phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn mem_sram_wdummy_cyclelen(&mut self) -> MEM_SRAM_WDUMMY_CYCLELEN_W<MEM_CACHE_SCTRL_SPEC> {
        MEM_SRAM_WDUMMY_CYCLELEN_W::new(self, 22)
    }
}
#[doc = "SPI0 external RAM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_cache_sctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_cache_sctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CACHE_SCTRL_SPEC;
impl crate::RegisterSpec for MEM_CACHE_SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_cache_sctrl::R`](R) reader structure"]
impl crate::Readable for MEM_CACHE_SCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_cache_sctrl::W`](W) writer structure"]
impl crate::Writable for MEM_CACHE_SCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CACHE_SCTRL to value 0x0055_c070"]
impl crate::Resettable for MEM_CACHE_SCTRL_SPEC {
    const RESET_VALUE: u32 = 0x0055_c070;
}
