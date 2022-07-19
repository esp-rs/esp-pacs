#[doc = "Register `SPI_MEM_USER1` reader"]
pub struct R(crate::R<SPI_MEM_USER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_USER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_USER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_USER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_USER1` writer"]
pub struct W(crate::W<SPI_MEM_USER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_USER1_SPEC>;
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
impl From<crate::W<SPI_MEM_USER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_USER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_USR_DUMMY_CYCLELEN` reader - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
pub type SPI_MEM_USR_DUMMY_CYCLELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_USR_DUMMY_CYCLELEN` writer - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
pub type SPI_MEM_USR_DUMMY_CYCLELEN_W<'a> =
    crate::FieldWriter<'a, u32, SPI_MEM_USER1_SPEC, u8, u8, 6, 0>;
#[doc = "Field `SPI_MEM_USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SPI_MEM_USR_ADDR_BITLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_USR_ADDR_BITLEN` writer - The length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SPI_MEM_USR_ADDR_BITLEN_W<'a> =
    crate::FieldWriter<'a, u32, SPI_MEM_USER1_SPEC, u8, u8, 6, 26>;
impl R {
    #[doc = "Bits 0:5 - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_dummy_cyclelen(&self) -> SPI_MEM_USR_DUMMY_CYCLELEN_R {
        SPI_MEM_USR_DUMMY_CYCLELEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_addr_bitlen(&self) -> SPI_MEM_USR_ADDR_BITLEN_R {
        SPI_MEM_USR_ADDR_BITLEN_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_dummy_cyclelen(&mut self) -> SPI_MEM_USR_DUMMY_CYCLELEN_W {
        SPI_MEM_USR_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_addr_bitlen(&mut self) -> SPI_MEM_USR_ADDR_BITLEN_W {
        SPI_MEM_USR_ADDR_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 user1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_user1](index.html) module"]
pub struct SPI_MEM_USER1_SPEC;
impl crate::RegisterSpec for SPI_MEM_USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_user1::R](R) reader structure"]
impl crate::Readable for SPI_MEM_USER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_user1::W](W) writer structure"]
impl crate::Writable for SPI_MEM_USER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_USER1 to value 0x5c00_0007"]
impl crate::Resettable for SPI_MEM_USER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5c00_0007
    }
}
