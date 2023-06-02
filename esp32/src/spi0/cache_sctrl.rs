#[doc = "Register `CACHE_SCTRL` reader"]
pub struct R(crate::R<CACHE_SCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_SCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_SCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_SCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_SCTRL` writer"]
pub struct W(crate::W<CACHE_SCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_SCTRL_SPEC>;
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
impl From<crate::W<CACHE_SCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_SCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_SRAM_DIO` reader - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
pub type USR_SRAM_DIO_R = crate::BitReader;
#[doc = "Field `USR_SRAM_DIO` writer - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
pub type USR_SRAM_DIO_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_SCTRL_SPEC, O>;
#[doc = "Field `USR_SRAM_QIO` reader - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
pub type USR_SRAM_QIO_R = crate::BitReader;
#[doc = "Field `USR_SRAM_QIO` writer - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
pub type USR_SRAM_QIO_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_SCTRL_SPEC, O>;
#[doc = "Field `USR_WR_SRAM_DUMMY` reader - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
pub type USR_WR_SRAM_DUMMY_R = crate::BitReader;
#[doc = "Field `USR_WR_SRAM_DUMMY` writer - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
pub type USR_WR_SRAM_DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_SCTRL_SPEC, O>;
#[doc = "Field `USR_RD_SRAM_DUMMY` reader - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
pub type USR_RD_SRAM_DUMMY_R = crate::BitReader;
#[doc = "Field `USR_RD_SRAM_DUMMY` writer - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
pub type USR_RD_SRAM_DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_SCTRL_SPEC, O>;
#[doc = "Field `CACHE_SRAM_USR_RCMD` reader - For SPI0 In the spi sram mode cache read sram for user define command."]
pub type CACHE_SRAM_USR_RCMD_R = crate::BitReader;
#[doc = "Field `CACHE_SRAM_USR_RCMD` writer - For SPI0 In the spi sram mode cache read sram for user define command."]
pub type CACHE_SRAM_USR_RCMD_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_SCTRL_SPEC, O>;
#[doc = "Field `SRAM_BYTES_LEN` reader - For SPI0 In the sram mode it is the byte length of spi read sram data."]
pub type SRAM_BYTES_LEN_R = crate::FieldReader;
#[doc = "Field `SRAM_BYTES_LEN` writer - For SPI0 In the sram mode it is the byte length of spi read sram data."]
pub type SRAM_BYTES_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, CACHE_SCTRL_SPEC, 8, O>;
#[doc = "Field `SRAM_DUMMY_CYCLELEN` reader - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SRAM_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SRAM_DUMMY_CYCLELEN` writer - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SRAM_DUMMY_CYCLELEN_W<'a, const O: u8> = crate::FieldWriter<'a, CACHE_SCTRL_SPEC, 8, O>;
#[doc = "Field `SRAM_ADDR_BITLEN` reader - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SRAM_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `SRAM_ADDR_BITLEN` writer - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SRAM_ADDR_BITLEN_W<'a, const O: u8> = crate::FieldWriter<'a, CACHE_SCTRL_SPEC, 6, O>;
#[doc = "Field `CACHE_SRAM_USR_WCMD` reader - For SPI0 In the spi sram mode cache write sram for user define command"]
pub type CACHE_SRAM_USR_WCMD_R = crate::BitReader;
#[doc = "Field `CACHE_SRAM_USR_WCMD` writer - For SPI0 In the spi sram mode cache write sram for user define command"]
pub type CACHE_SRAM_USR_WCMD_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_SCTRL_SPEC, O>;
impl R {
    #[doc = "Bit 1 - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn usr_sram_dio(&self) -> USR_SRAM_DIO_R {
        USR_SRAM_DIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    pub fn usr_sram_qio(&self) -> USR_SRAM_QIO_R {
        USR_SRAM_QIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    pub fn usr_wr_sram_dummy(&self) -> USR_WR_SRAM_DUMMY_R {
        USR_WR_SRAM_DUMMY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    pub fn usr_rd_sram_dummy(&self) -> USR_RD_SRAM_DUMMY_R {
        USR_RD_SRAM_DUMMY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI0 In the spi sram mode cache read sram for user define command."]
    #[inline(always)]
    pub fn cache_sram_usr_rcmd(&self) -> CACHE_SRAM_USR_RCMD_R {
        CACHE_SRAM_USR_RCMD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - For SPI0 In the sram mode it is the byte length of spi read sram data."]
    #[inline(always)]
    pub fn sram_bytes_len(&self) -> SRAM_BYTES_LEN_R {
        SRAM_BYTES_LEN_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:21 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_dummy_cyclelen(&self) -> SRAM_DUMMY_CYCLELEN_R {
        SRAM_DUMMY_CYCLELEN_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:27 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn sram_addr_bitlen(&self) -> SRAM_ADDR_BITLEN_R {
        SRAM_ADDR_BITLEN_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bit 28 - For SPI0 In the spi sram mode cache write sram for user define command"]
    #[inline(always)]
    pub fn cache_sram_usr_wcmd(&self) -> CACHE_SRAM_USR_WCMD_R {
        CACHE_SRAM_USR_WCMD_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SCTRL")
            .field(
                "usr_sram_dio",
                &format_args!("{}", self.usr_sram_dio().bit()),
            )
            .field(
                "usr_sram_qio",
                &format_args!("{}", self.usr_sram_qio().bit()),
            )
            .field(
                "usr_wr_sram_dummy",
                &format_args!("{}", self.usr_wr_sram_dummy().bit()),
            )
            .field(
                "usr_rd_sram_dummy",
                &format_args!("{}", self.usr_rd_sram_dummy().bit()),
            )
            .field(
                "cache_sram_usr_rcmd",
                &format_args!("{}", self.cache_sram_usr_rcmd().bit()),
            )
            .field(
                "sram_bytes_len",
                &format_args!("{}", self.sram_bytes_len().bits()),
            )
            .field(
                "sram_dummy_cyclelen",
                &format_args!("{}", self.sram_dummy_cyclelen().bits()),
            )
            .field(
                "sram_addr_bitlen",
                &format_args!("{}", self.sram_addr_bitlen().bits()),
            )
            .field(
                "cache_sram_usr_wcmd",
                &format_args!("{}", self.cache_sram_usr_wcmd().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_SCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - For SPI0 In the spi sram mode spi dual I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    #[must_use]
    pub fn usr_sram_dio(&mut self) -> USR_SRAM_DIO_W<1> {
        USR_SRAM_DIO_W::new(self)
    }
    #[doc = "Bit 2 - For SPI0 In the spi sram mode spi quad I/O mode enable 1: enable 0:disable"]
    #[inline(always)]
    #[must_use]
    pub fn usr_sram_qio(&mut self) -> USR_SRAM_QIO_W<2> {
        USR_SRAM_QIO_W::new(self)
    }
    #[doc = "Bit 3 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for write operations."]
    #[inline(always)]
    #[must_use]
    pub fn usr_wr_sram_dummy(&mut self) -> USR_WR_SRAM_DUMMY_W<3> {
        USR_WR_SRAM_DUMMY_W::new(self)
    }
    #[doc = "Bit 4 - For SPI0 In the spi sram mode it is the enable bit of dummy phase for read operations."]
    #[inline(always)]
    #[must_use]
    pub fn usr_rd_sram_dummy(&mut self) -> USR_RD_SRAM_DUMMY_W<4> {
        USR_RD_SRAM_DUMMY_W::new(self)
    }
    #[doc = "Bit 5 - For SPI0 In the spi sram mode cache read sram for user define command."]
    #[inline(always)]
    #[must_use]
    pub fn cache_sram_usr_rcmd(&mut self) -> CACHE_SRAM_USR_RCMD_W<5> {
        CACHE_SRAM_USR_RCMD_W::new(self)
    }
    #[doc = "Bits 6:13 - For SPI0 In the sram mode it is the byte length of spi read sram data."]
    #[inline(always)]
    #[must_use]
    pub fn sram_bytes_len(&mut self) -> SRAM_BYTES_LEN_W<6> {
        SRAM_BYTES_LEN_W::new(self)
    }
    #[doc = "Bits 14:21 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn sram_dummy_cyclelen(&mut self) -> SRAM_DUMMY_CYCLELEN_W<14> {
        SRAM_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bits 22:27 - For SPI0 In the sram mode it is the length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn sram_addr_bitlen(&mut self) -> SRAM_ADDR_BITLEN_W<22> {
        SRAM_ADDR_BITLEN_W::new(self)
    }
    #[doc = "Bit 28 - For SPI0 In the spi sram mode cache write sram for user define command"]
    #[inline(always)]
    #[must_use]
    pub fn cache_sram_usr_wcmd(&mut self) -> CACHE_SRAM_USR_WCMD_W<28> {
        CACHE_SRAM_USR_WCMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_sctrl](index.html) module"]
pub struct CACHE_SCTRL_SPEC;
impl crate::RegisterSpec for CACHE_SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_sctrl::R](R) reader structure"]
impl crate::Readable for CACHE_SCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_sctrl::W](W) writer structure"]
impl crate::Writable for CACHE_SCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_SCTRL to value 0x15c0_4830"]
impl crate::Resettable for CACHE_SCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x15c0_4830;
}
