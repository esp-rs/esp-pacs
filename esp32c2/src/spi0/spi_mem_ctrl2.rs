#[doc = "Register `SPI_MEM_CTRL2` reader"]
pub struct R(crate::R<SPI_MEM_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_CTRL2` writer"]
pub struct W(crate::W<SPI_MEM_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_CTRL2_SPEC>;
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
impl From<crate::W<SPI_MEM_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_CS_SETUP_TIME` reader - (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
pub type SPI_MEM_CS_SETUP_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_CS_SETUP_TIME` writer - (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
pub type SPI_MEM_CS_SETUP_TIME_W<'a> =
    crate::FieldWriter<'a, u32, SPI_MEM_CTRL2_SPEC, u8, u8, 5, 0>;
#[doc = "Field `SPI_MEM_CS_HOLD_TIME` reader - Spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
pub type SPI_MEM_CS_HOLD_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_CS_HOLD_TIME` writer - Spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
pub type SPI_MEM_CS_HOLD_TIME_W<'a> = crate::FieldWriter<'a, u32, SPI_MEM_CTRL2_SPEC, u8, u8, 5, 5>;
#[doc = "Field `SPI_MEM_CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type SPI_MEM_CS_HOLD_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_MEM_CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type SPI_MEM_CS_HOLD_DELAY_W<'a> =
    crate::FieldWriter<'a, u32, SPI_MEM_CTRL2_SPEC, u8, u8, 6, 25>;
#[doc = "Field `SPI_MEM_SYNC_RESET` writer - The FSM will be reset."]
pub type SPI_MEM_SYNC_RESET_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_CTRL2_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:4 - (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
    #[inline(always)]
    pub fn spi_mem_cs_setup_time(&self) -> SPI_MEM_CS_SETUP_TIME_R {
        SPI_MEM_CS_SETUP_TIME_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_time(&self) -> SPI_MEM_CS_HOLD_TIME_R {
        SPI_MEM_CS_HOLD_TIME_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_delay(&self) -> SPI_MEM_CS_HOLD_DELAY_R {
        SPI_MEM_CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
    #[inline(always)]
    pub fn spi_mem_cs_setup_time(&mut self) -> SPI_MEM_CS_SETUP_TIME_W {
        SPI_MEM_CS_SETUP_TIME_W::new(self)
    }
    #[doc = "Bits 5:9 - Spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_time(&mut self) -> SPI_MEM_CS_HOLD_TIME_W {
        SPI_MEM_CS_HOLD_TIME_W::new(self)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn spi_mem_cs_hold_delay(&mut self) -> SPI_MEM_CS_HOLD_DELAY_W {
        SPI_MEM_CS_HOLD_DELAY_W::new(self)
    }
    #[doc = "Bit 31 - The FSM will be reset."]
    #[inline(always)]
    pub fn spi_mem_sync_reset(&mut self) -> SPI_MEM_SYNC_RESET_W {
        SPI_MEM_SYNC_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 control2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ctrl2](index.html) module"]
pub struct SPI_MEM_CTRL2_SPEC;
impl crate::RegisterSpec for SPI_MEM_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_ctrl2::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_ctrl2::W](W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_CTRL2 to value 0x21"]
impl crate::Resettable for SPI_MEM_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
