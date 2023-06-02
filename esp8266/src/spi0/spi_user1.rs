#[doc = "Register `SPI_USER1` reader"]
pub struct R(crate::R<SPI_USER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_USER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_USER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_USER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_USER1` writer"]
pub struct W(crate::W<SPI_USER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_USER1_SPEC>;
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
impl From<crate::W<SPI_USER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_USER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_usr_dummy_cyclelen` reader - The length in spi_clk cycles of \"dummy\" phase. The register value shall be (cycle_num-1)"]
pub type REG_USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `reg_usr_dummy_cyclelen` writer - The length in spi_clk cycles of \"dummy\" phase. The register value shall be (cycle_num-1)"]
pub type REG_USR_DUMMY_CYCLELEN_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_USER1_SPEC, 8, O>;
#[doc = "Field `reg_usr_miso_bitlen` reader - The length in bits of \"read-data\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_MISO_BITLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_usr_miso_bitlen` writer - The length in bits of \"read-data\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_MISO_BITLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_USER1_SPEC, 9, O, u16, u16>;
#[doc = "Field `reg_usr_mosi_bitlen` reader - The length in bits of \"write-data\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_MOSI_BITLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_usr_mosi_bitlen` writer - The length in bits of \"write-data\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_MOSI_BITLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_USER1_SPEC, 9, O, u16, u16>;
#[doc = "Field `reg_usr_addr_bitlen` reader - The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `reg_usr_addr_bitlen` writer - The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
pub type REG_USR_ADDR_BITLEN_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_USER1_SPEC, 6, O>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of \"dummy\" phase. The register value shall be (cycle_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_dummy_cyclelen(&mut self) -> REG_USR_DUMMY_CYCLELEN_W<0> {
        REG_USR_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bits 8:16 - The length in bits of \"read-data\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_miso_bitlen(&mut self) -> REG_USR_MISO_BITLEN_W<8> {
        REG_USR_MISO_BITLEN_W::new(self)
    }
    #[doc = "Bits 17:25 - The length in bits of \"write-data\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_mosi_bitlen(&mut self) -> REG_USR_MOSI_BITLEN_W<17> {
        REG_USR_MOSI_BITLEN_W::new(self)
    }
    #[doc = "Bits 26:31 - The length in bits of \"address\" phase. The register value shall be (bit_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_usr_addr_bitlen(&mut self) -> REG_USR_ADDR_BITLEN_W<26> {
        REG_USR_ADDR_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The length in bits of \"address\" phase. The register value shall be (bit_num-1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_user1](index.html) module"]
pub struct SPI_USER1_SPEC;
impl crate::RegisterSpec for SPI_USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_user1::R](R) reader structure"]
impl crate::Readable for SPI_USER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_user1::W](W) writer structure"]
impl crate::Writable for SPI_USER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_USER1 to value 0"]
impl crate::Resettable for SPI_USER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
