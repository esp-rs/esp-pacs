#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS_SETUP_TIME` reader - (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
pub type CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `CS_SETUP_TIME` writer - (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
pub type CS_SETUP_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL2_SPEC, 5, O>;
#[doc = "Field `CS_HOLD_TIME` reader - Spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
pub type CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `CS_HOLD_TIME` writer - Spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
pub type CS_HOLD_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL2_SPEC, 5, O>;
#[doc = "Field `CS_HOLD_DELAY` reader - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `CS_HOLD_DELAY` writer - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
pub type CS_HOLD_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL2_SPEC, 6, O>;
#[doc = "Field `SYNC_RESET` writer - The FSM will be reset."]
pub type SYNC_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CTRL2_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    pub fn cs_hold_delay(&self) -> CS_HOLD_DELAY_R {
        CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field(
                "cs_setup_time",
                &format_args!("{}", self.cs_setup_time().bits()),
            )
            .field(
                "cs_hold_time",
                &format_args!("{}", self.cs_hold_time().bits()),
            )
            .field(
                "cs_hold_delay",
                &format_args!("{}", self.cs_hold_delay().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - (cycles-1) of prepare phase by spi clock this bits are combined with spi_mem_cs_setup bit."]
    #[inline(always)]
    #[must_use]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W<0> {
        CS_SETUP_TIME_W::new(self)
    }
    #[doc = "Bits 5:9 - Spi cs signal is delayed to inactive by spi clock this bits are combined with spi_mem_cs_hold bit."]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W<5> {
        CS_HOLD_TIME_W::new(self)
    }
    #[doc = "Bits 25:30 - These bits are used to set the minimum CS high time tSHSL between SPI burst transfer when accesses to flash. tSHSL is (SPI_MEM_CS_HOLD_DELAY\\[5:0\\] + 1) MSPI core clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold_delay(&mut self) -> CS_HOLD_DELAY_W<25> {
        CS_HOLD_DELAY_W::new(self)
    }
    #[doc = "Bit 31 - The FSM will be reset."]
    #[inline(always)]
    #[must_use]
    pub fn sync_reset(&mut self) -> SYNC_RESET_W<31> {
        SYNC_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 control2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x21"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
