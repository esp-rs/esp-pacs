#[doc = "Register `SLAVE1` reader"]
pub struct R(crate::R<SLAVE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE1` writer"]
pub struct W(crate::W<SLAVE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE1_SPEC>;
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
impl From<crate::W<SLAVE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_RDBUF_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
pub type SLV_RDBUF_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDBUF_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
pub type SLV_RDBUF_DUMMY_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE1_SPEC, O>;
#[doc = "Field `SLV_WRBUF_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
pub type SLV_WRBUF_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRBUF_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
pub type SLV_WRBUF_DUMMY_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE1_SPEC, O>;
#[doc = "Field `SLV_RDSTA_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for read-status operations."]
pub type SLV_RDSTA_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `SLV_RDSTA_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for read-status operations."]
pub type SLV_RDSTA_DUMMY_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE1_SPEC, O>;
#[doc = "Field `SLV_WRSTA_DUMMY_EN` reader - In the slave mode it is the enable bit of dummy phase for write-status operations."]
pub type SLV_WRSTA_DUMMY_EN_R = crate::BitReader;
#[doc = "Field `SLV_WRSTA_DUMMY_EN` writer - In the slave mode it is the enable bit of dummy phase for write-status operations."]
pub type SLV_WRSTA_DUMMY_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE1_SPEC, O>;
#[doc = "Field `SLV_WR_ADDR_BITLEN` reader - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
pub type SLV_WR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `SLV_WR_ADDR_BITLEN` writer - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
pub type SLV_WR_ADDR_BITLEN_W<'a, const O: u8> = crate::FieldWriter<'a, SLAVE1_SPEC, 6, O>;
#[doc = "Field `SLV_RD_ADDR_BITLEN` reader - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
pub type SLV_RD_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `SLV_RD_ADDR_BITLEN` writer - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
pub type SLV_RD_ADDR_BITLEN_W<'a, const O: u8> = crate::FieldWriter<'a, SLAVE1_SPEC, 6, O>;
#[doc = "Field `SLV_STATUS_READBACK` reader - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
pub type SLV_STATUS_READBACK_R = crate::BitReader;
#[doc = "Field `SLV_STATUS_READBACK` writer - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
pub type SLV_STATUS_READBACK_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE1_SPEC, O>;
#[doc = "Field `SLV_STATUS_FAST_EN` reader - In the slave mode enable fast read status."]
pub type SLV_STATUS_FAST_EN_R = crate::BitReader;
#[doc = "Field `SLV_STATUS_FAST_EN` writer - In the slave mode enable fast read status."]
pub type SLV_STATUS_FAST_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE1_SPEC, O>;
#[doc = "Field `SLV_STATUS_BITLEN` reader - In the slave mode it is the length of status bit."]
pub type SLV_STATUS_BITLEN_R = crate::FieldReader;
#[doc = "Field `SLV_STATUS_BITLEN` writer - In the slave mode it is the length of status bit."]
pub type SLV_STATUS_BITLEN_W<'a, const O: u8> = crate::FieldWriter<'a, SLAVE1_SPEC, 5, O>;
impl R {
    #[doc = "Bit 0 - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
    #[inline(always)]
    pub fn slv_rdbuf_dummy_en(&self) -> SLV_RDBUF_DUMMY_EN_R {
        SLV_RDBUF_DUMMY_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
    #[inline(always)]
    pub fn slv_wrbuf_dummy_en(&self) -> SLV_WRBUF_DUMMY_EN_R {
        SLV_WRBUF_DUMMY_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - In the slave mode it is the enable bit of dummy phase for read-status operations."]
    #[inline(always)]
    pub fn slv_rdsta_dummy_en(&self) -> SLV_RDSTA_DUMMY_EN_R {
        SLV_RDSTA_DUMMY_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In the slave mode it is the enable bit of dummy phase for write-status operations."]
    #[inline(always)]
    pub fn slv_wrsta_dummy_en(&self) -> SLV_WRSTA_DUMMY_EN_R {
        SLV_WRSTA_DUMMY_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_wr_addr_bitlen(&self) -> SLV_WR_ADDR_BITLEN_R {
        SLV_WR_ADDR_BITLEN_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn slv_rd_addr_bitlen(&self) -> SLV_RD_ADDR_BITLEN_R {
        SLV_RD_ADDR_BITLEN_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
    #[inline(always)]
    pub fn slv_status_readback(&self) -> SLV_STATUS_READBACK_R {
        SLV_STATUS_READBACK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - In the slave mode enable fast read status."]
    #[inline(always)]
    pub fn slv_status_fast_en(&self) -> SLV_STATUS_FAST_EN_R {
        SLV_STATUS_FAST_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - In the slave mode it is the length of status bit."]
    #[inline(always)]
    pub fn slv_status_bitlen(&self) -> SLV_STATUS_BITLEN_R {
        SLV_STATUS_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE1")
            .field(
                "slv_rdbuf_dummy_en",
                &format_args!("{}", self.slv_rdbuf_dummy_en().bit()),
            )
            .field(
                "slv_wrbuf_dummy_en",
                &format_args!("{}", self.slv_wrbuf_dummy_en().bit()),
            )
            .field(
                "slv_rdsta_dummy_en",
                &format_args!("{}", self.slv_rdsta_dummy_en().bit()),
            )
            .field(
                "slv_wrsta_dummy_en",
                &format_args!("{}", self.slv_wrsta_dummy_en().bit()),
            )
            .field(
                "slv_wr_addr_bitlen",
                &format_args!("{}", self.slv_wr_addr_bitlen().bits()),
            )
            .field(
                "slv_rd_addr_bitlen",
                &format_args!("{}", self.slv_rd_addr_bitlen().bits()),
            )
            .field(
                "slv_status_readback",
                &format_args!("{}", self.slv_status_readback().bit()),
            )
            .field(
                "slv_status_fast_en",
                &format_args!("{}", self.slv_status_fast_en().bit()),
            )
            .field(
                "slv_status_bitlen",
                &format_args!("{}", self.slv_status_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLAVE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - In the slave mode it is the enable bit of dummy phase for read-buffer operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdbuf_dummy_en(&mut self) -> SLV_RDBUF_DUMMY_EN_W<0> {
        SLV_RDBUF_DUMMY_EN_W::new(self)
    }
    #[doc = "Bit 1 - In the slave mode it is the enable bit of dummy phase for write-buffer operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrbuf_dummy_en(&mut self) -> SLV_WRBUF_DUMMY_EN_W<1> {
        SLV_WRBUF_DUMMY_EN_W::new(self)
    }
    #[doc = "Bit 2 - In the slave mode it is the enable bit of dummy phase for read-status operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdsta_dummy_en(&mut self) -> SLV_RDSTA_DUMMY_EN_W<2> {
        SLV_RDSTA_DUMMY_EN_W::new(self)
    }
    #[doc = "Bit 3 - In the slave mode it is the enable bit of dummy phase for write-status operations."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrsta_dummy_en(&mut self) -> SLV_WRSTA_DUMMY_EN_W<3> {
        SLV_WRSTA_DUMMY_EN_W::new(self)
    }
    #[doc = "Bits 4:9 - In the slave mode it is the address length in bits for write-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_addr_bitlen(&mut self) -> SLV_WR_ADDR_BITLEN_W<4> {
        SLV_WR_ADDR_BITLEN_W::new(self)
    }
    #[doc = "Bits 10:15 - In the slave mode it is the address length in bits for read-buffer operation. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_addr_bitlen(&mut self) -> SLV_RD_ADDR_BITLEN_W<10> {
        SLV_RD_ADDR_BITLEN_W::new(self)
    }
    #[doc = "Bit 25 - In the slave mode 1:read register of SPI_SLV_WR_STATUS 0: read register of SPI_RD_STATUS."]
    #[inline(always)]
    #[must_use]
    pub fn slv_status_readback(&mut self) -> SLV_STATUS_READBACK_W<25> {
        SLV_STATUS_READBACK_W::new(self)
    }
    #[doc = "Bit 26 - In the slave mode enable fast read status."]
    #[inline(always)]
    #[must_use]
    pub fn slv_status_fast_en(&mut self) -> SLV_STATUS_FAST_EN_W<26> {
        SLV_STATUS_FAST_EN_W::new(self)
    }
    #[doc = "Bits 27:31 - In the slave mode it is the length of status bit."]
    #[inline(always)]
    #[must_use]
    pub fn slv_status_bitlen(&mut self) -> SLV_STATUS_BITLEN_W<27> {
        SLV_STATUS_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave1](index.html) module"]
pub struct SLAVE1_SPEC;
impl crate::RegisterSpec for SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave1::R](R) reader structure"]
impl crate::Readable for SLAVE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave1::W](W) writer structure"]
impl crate::Writable for SLAVE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLAVE1 to value 0x0200_0000"]
impl crate::Resettable for SLAVE1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0000;
}
