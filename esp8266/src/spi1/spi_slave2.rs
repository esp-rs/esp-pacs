#[doc = "Register `SPI_SLAVE2` reader"]
pub type R = crate::R<SPI_SLAVE2_SPEC>;
#[doc = "Register `SPI_SLAVE2` writer"]
pub type W = crate::W<SPI_SLAVE2_SPEC>;
#[doc = "Field `slv_rdsta_dummy_cyclelen` reader - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"read-status\" operations. Theregister value shall be (cycle_num-1)"]
pub type SLV_RDSTA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `slv_rdsta_dummy_cyclelen` writer - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"read-status\" operations. Theregister value shall be (cycle_num-1)"]
pub type SLV_RDSTA_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `slv_wrsta_dummy_cyclelen` reader - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"write-status\" operations. Theregister value shall be (cycle_num-1)"]
pub type SLV_WRSTA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `slv_wrsta_dummy_cyclelen` writer - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"write-status\" operations. Theregister value shall be (cycle_num-1)"]
pub type SLV_WRSTA_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `slv_rdbuf_dummy_cyclelen` reader - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"read-buffer\" operations. The registervalue shall be (cycle_num-1)"]
pub type SLV_RDBUF_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `slv_rdbuf_dummy_cyclelen` writer - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"read-buffer\" operations. The registervalue shall be (cycle_num-1)"]
pub type SLV_RDBUF_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `slv_wrbuf_dummy_cyclelen` reader - In the slave mode, it is the length in spi_clk cycles \"dummy\" phase for \"write-buffer\" operations. The registervalue shall be (cycle_num-1)"]
pub type SLV_WRBUF_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `slv_wrbuf_dummy_cyclelen` writer - In the slave mode, it is the length in spi_clk cycles \"dummy\" phase for \"write-buffer\" operations. The registervalue shall be (cycle_num-1)"]
pub type SLV_WRBUF_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"read-status\" operations. Theregister value shall be (cycle_num-1)"]
    #[inline(always)]
    pub fn slv_rdsta_dummy_cyclelen(&self) -> SLV_RDSTA_DUMMY_CYCLELEN_R {
        SLV_RDSTA_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"write-status\" operations. Theregister value shall be (cycle_num-1)"]
    #[inline(always)]
    pub fn slv_wrsta_dummy_cyclelen(&self) -> SLV_WRSTA_DUMMY_CYCLELEN_R {
        SLV_WRSTA_DUMMY_CYCLELEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"read-buffer\" operations. The registervalue shall be (cycle_num-1)"]
    #[inline(always)]
    pub fn slv_rdbuf_dummy_cyclelen(&self) -> SLV_RDBUF_DUMMY_CYCLELEN_R {
        SLV_RDBUF_DUMMY_CYCLELEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - In the slave mode, it is the length in spi_clk cycles \"dummy\" phase for \"write-buffer\" operations. The registervalue shall be (cycle_num-1)"]
    #[inline(always)]
    pub fn slv_wrbuf_dummy_cyclelen(&self) -> SLV_WRBUF_DUMMY_CYCLELEN_R {
        SLV_WRBUF_DUMMY_CYCLELEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SLAVE2")
            .field(
                "slv_wrbuf_dummy_cyclelen",
                &format_args!("{}", self.slv_wrbuf_dummy_cyclelen().bits()),
            )
            .field(
                "slv_rdbuf_dummy_cyclelen",
                &format_args!("{}", self.slv_rdbuf_dummy_cyclelen().bits()),
            )
            .field(
                "slv_wrsta_dummy_cyclelen",
                &format_args!("{}", self.slv_wrsta_dummy_cyclelen().bits()),
            )
            .field(
                "slv_rdsta_dummy_cyclelen",
                &format_args!("{}", self.slv_rdsta_dummy_cyclelen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SLAVE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"read-status\" operations. Theregister value shall be (cycle_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdsta_dummy_cyclelen(&mut self) -> SLV_RDSTA_DUMMY_CYCLELEN_W<SPI_SLAVE2_SPEC> {
        SLV_RDSTA_DUMMY_CYCLELEN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"write-status\" operations. Theregister value shall be (cycle_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrsta_dummy_cyclelen(&mut self) -> SLV_WRSTA_DUMMY_CYCLELEN_W<SPI_SLAVE2_SPEC> {
        SLV_WRSTA_DUMMY_CYCLELEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - In the slave mode, it is the length in spi_clk cycles of \"dummy\" phase for \"read-buffer\" operations. The registervalue shall be (cycle_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn slv_rdbuf_dummy_cyclelen(&mut self) -> SLV_RDBUF_DUMMY_CYCLELEN_W<SPI_SLAVE2_SPEC> {
        SLV_RDBUF_DUMMY_CYCLELEN_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - In the slave mode, it is the length in spi_clk cycles \"dummy\" phase for \"write-buffer\" operations. The registervalue shall be (cycle_num-1)"]
    #[inline(always)]
    #[must_use]
    pub fn slv_wrbuf_dummy_cyclelen(&mut self) -> SLV_WRBUF_DUMMY_CYCLELEN_W<SPI_SLAVE2_SPEC> {
        SLV_WRBUF_DUMMY_CYCLELEN_W::new(self, 24)
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
#[doc = "In the slave mode, it is the length in spi_clk cycles \"dummy\" phase for \"write-buffer\" operations. The registervalue shall be (cycle_num-1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_slave2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_slave2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SLAVE2_SPEC;
impl crate::RegisterSpec for SPI_SLAVE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_slave2::R`](R) reader structure"]
impl crate::Readable for SPI_SLAVE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_slave2::W`](W) writer structure"]
impl crate::Writable for SPI_SLAVE2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_SLAVE2 to value 0"]
impl crate::Resettable for SPI_SLAVE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
