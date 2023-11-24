#[doc = "Register `SPI_USER1` reader"]
pub type R = crate::R<SPI_USER1_SPEC>;
#[doc = "Register `SPI_USER1` writer"]
pub type W = crate::W<SPI_USER1_SPEC>;
#[doc = "Field `reg_usr_dummy_cyclelen` reader - The length in spi_clk cycles of \"dummy\" phase. The register value shall be (cycle_num-1)"]
pub type REG_USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `reg_usr_dummy_cyclelen` writer - The length in spi_clk cycles of \"dummy\" phase. The register value shall be (cycle_num-1)"]
pub type REG_USR_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_usr_miso_bitlen` reader - The length in bits of \"read-data\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_MISO_BITLEN_R = crate::FieldReader<u16>;
#[doc = "Field `reg_usr_miso_bitlen` writer - The length in bits of \"read-data\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_MISO_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `reg_usr_mosi_bitlen` reader - The length in bits of \"write-data\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_MOSI_BITLEN_R = crate::FieldReader<u16>;
#[doc = "Field `reg_usr_mosi_bitlen` writer - The length in bits of \"write-data\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_MOSI_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `reg_usr_addr_bitlen` reader - The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `reg_usr_addr_bitlen` writer - The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of \"dummy\" phase. The register value shall be (cycle_num-1)"]
    #[inline(always)]
    pub fn reg_usr_dummy_cyclelen(&self) -> REG_USR_DUMMY_CYCLELEN_R {
        REG_USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - The length in bits of \"read-data\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn reg_usr_miso_bitlen(&self) -> REG_USR_MISO_BITLEN_R {
        REG_USR_MISO_BITLEN_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 17:25 - The length in bits of \"write-data\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn reg_usr_mosi_bitlen(&self) -> REG_USR_MOSI_BITLEN_R {
        REG_USR_MOSI_BITLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bits 26:31 - The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    pub fn reg_usr_addr_bitlen(&self) -> REG_USR_ADDR_BITLEN_R {
        REG_USR_ADDR_BITLEN_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_USER1")
            .field(
                "reg_usr_addr_bitlen",
                &format_args!("{}", self.reg_usr_addr_bitlen().bits()),
            )
            .field(
                "reg_usr_mosi_bitlen",
                &format_args!("{}", self.reg_usr_mosi_bitlen().bits()),
            )
            .field(
                "reg_usr_miso_bitlen",
                &format_args!("{}", self.reg_usr_miso_bitlen().bits()),
            )
            .field(
                "reg_usr_dummy_cyclelen",
                &format_args!("{}", self.reg_usr_dummy_cyclelen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_USER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of \"dummy\" phase. The register value shall be (cycle_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_dummy_cyclelen(&mut self) -> REG_USR_DUMMY_CYCLELEN_W<SPI_USER1_SPEC> {
        REG_USR_DUMMY_CYCLELEN_W::new(self, 0)
    }
    #[doc = "Bits 8:16 - The length in bits of \"read-data\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_miso_bitlen(&mut self) -> REG_USR_MISO_BITLEN_W<SPI_USER1_SPEC> {
        REG_USR_MISO_BITLEN_W::new(self, 8)
    }
    #[doc = "Bits 17:25 - The length in bits of \"write-data\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_mosi_bitlen(&mut self) -> REG_USR_MOSI_BITLEN_W<SPI_USER1_SPEC> {
        REG_USR_MOSI_BITLEN_W::new(self, 17)
    }
    #[doc = "Bits 26:31 - The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_addr_bitlen(&mut self) -> REG_USR_ADDR_BITLEN_W<SPI_USER1_SPEC> {
        REG_USR_ADDR_BITLEN_W::new(self, 26)
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
#[doc = "The length in bits of \"address\" phase. The register value shall be (bit_num-1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_user1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_user1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_USER1_SPEC;
impl crate::RegisterSpec for SPI_USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_user1::R`](R) reader structure"]
impl crate::Readable for SPI_USER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_user1::W`](W) writer structure"]
impl crate::Writable for SPI_USER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_USER1 to value 0"]
impl crate::Resettable for SPI_USER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
