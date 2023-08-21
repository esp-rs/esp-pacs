#[doc = "Register `SLAVE2` reader"]
pub type R = crate::R<SLAVE2_SPEC>;
#[doc = "Register `SLAVE2` writer"]
pub type W = crate::W<SLAVE2_SPEC>;
#[doc = "Field `SLV_RDSTA_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1)."]
pub type SLV_RDSTA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SLV_RDSTA_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1)."]
pub type SLV_RDSTA_DUMMY_CYCLELEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLV_WRSTA_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1)."]
pub type SLV_WRSTA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SLV_WRSTA_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1)."]
pub type SLV_WRSTA_DUMMY_CYCLELEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLV_RDBUF_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1)."]
pub type SLV_RDBUF_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SLV_RDBUF_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1)."]
pub type SLV_RDBUF_DUMMY_CYCLELEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SLV_WRBUF_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1)."]
pub type SLV_WRBUF_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SLV_WRBUF_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1)."]
pub type SLV_WRBUF_DUMMY_CYCLELEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_rdsta_dummy_cyclelen(&self) -> SLV_RDSTA_DUMMY_CYCLELEN_R {
        SLV_RDSTA_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_wrsta_dummy_cyclelen(&self) -> SLV_WRSTA_DUMMY_CYCLELEN_R {
        SLV_WRSTA_DUMMY_CYCLELEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_rdbuf_dummy_cyclelen(&self) -> SLV_RDBUF_DUMMY_CYCLELEN_R {
        SLV_RDBUF_DUMMY_CYCLELEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn slv_wrbuf_dummy_cyclelen(&self) -> SLV_WRBUF_DUMMY_CYCLELEN_R {
        SLV_WRBUF_DUMMY_CYCLELEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE2")
            .field(
                "slv_rdsta_dummy_cyclelen",
                &format_args!("{}", self.slv_rdsta_dummy_cyclelen().bits()),
            )
            .field(
                "slv_wrsta_dummy_cyclelen",
                &format_args!("{}", self.slv_wrsta_dummy_cyclelen().bits()),
            )
            .field(
                "slv_rdbuf_dummy_cyclelen",
                &format_args!("{}", self.slv_rdbuf_dummy_cyclelen().bits()),
            )
            .field(
                "slv_wrbuf_dummy_cyclelen",
                &format_args!("{}", self.slv_wrbuf_dummy_cyclelen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLAVE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdsta_dummy_cyclelen(&mut self) -> SLV_RDSTA_DUMMY_CYCLELEN_W<SLAVE2_SPEC, 0> {
        SLV_RDSTA_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bits 8:15 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrsta_dummy_cyclelen(&mut self) -> SLV_WRSTA_DUMMY_CYCLELEN_W<SLAVE2_SPEC, 8> {
        SLV_WRSTA_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bits 16:23 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdbuf_dummy_cyclelen(&mut self) -> SLV_RDBUF_DUMMY_CYCLELEN_W<SLAVE2_SPEC, 16> {
        SLV_RDBUF_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bits 24:31 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrbuf_dummy_cyclelen(&mut self) -> SLV_WRBUF_DUMMY_CYCLELEN_W<SLAVE2_SPEC, 24> {
        SLV_WRBUF_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLAVE2_SPEC;
impl crate::RegisterSpec for SLAVE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave2::R`](R) reader structure"]
impl crate::Readable for SLAVE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slave2::W`](W) writer structure"]
impl crate::Writable for SLAVE2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLAVE2 to value 0"]
impl crate::Resettable for SLAVE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
