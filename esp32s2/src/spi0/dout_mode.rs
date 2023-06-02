#[doc = "Register `DOUT_MODE` reader"]
pub struct R(crate::R<DOUT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUT_MODE` writer"]
pub struct W(crate::W<DOUT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUT_MODE_SPEC>;
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
impl From<crate::W<DOUT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT0_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT0_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT0_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT0_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, DOUT_MODE_SPEC, 3, O>;
#[doc = "Field `DOUT1_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT1_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT1_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT1_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, DOUT_MODE_SPEC, 3, O>;
#[doc = "Field `DOUT2_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT2_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT2_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT2_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, DOUT_MODE_SPEC, 3, O>;
#[doc = "Field `DOUT3_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT3_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT3_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT3_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, DOUT_MODE_SPEC, 3, O>;
#[doc = "Field `DOUT4_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT4_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT4_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT4_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, DOUT_MODE_SPEC, 3, O>;
#[doc = "Field `DOUT5_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT5_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT5_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT5_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, DOUT_MODE_SPEC, 3, O>;
#[doc = "Field `DOUT6_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT6_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT6_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT6_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, DOUT_MODE_SPEC, 3, O>;
#[doc = "Field `DOUT7_MODE` reader - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT7_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT7_MODE` writer - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
pub type DOUT7_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, DOUT_MODE_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT0_MODE_R {
        DOUT0_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT1_MODE_R {
        DOUT1_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT2_MODE_R {
        DOUT2_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT3_MODE_R {
        DOUT3_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout4_mode(&self) -> DOUT4_MODE_R {
        DOUT4_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout5_mode(&self) -> DOUT5_MODE_R {
        DOUT5_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout6_mode(&self) -> DOUT6_MODE_R {
        DOUT6_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dout7_mode(&self) -> DOUT7_MODE_R {
        DOUT7_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOUT_MODE")
            .field("dout0_mode", &format_args!("{}", self.dout0_mode().bits()))
            .field("dout1_mode", &format_args!("{}", self.dout1_mode().bits()))
            .field("dout2_mode", &format_args!("{}", self.dout2_mode().bits()))
            .field("dout3_mode", &format_args!("{}", self.dout3_mode().bits()))
            .field("dout4_mode", &format_args!("{}", self.dout4_mode().bits()))
            .field("dout5_mode", &format_args!("{}", self.dout5_mode().bits()))
            .field("dout6_mode", &format_args!("{}", self.dout6_mode().bits()))
            .field("dout7_mode", &format_args!("{}", self.dout7_mode().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOUT_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn dout0_mode(&mut self) -> DOUT0_MODE_W<0> {
        DOUT0_MODE_W::new(self)
    }
    #[doc = "Bits 3:5 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn dout1_mode(&mut self) -> DOUT1_MODE_W<3> {
        DOUT1_MODE_W::new(self)
    }
    #[doc = "Bits 6:8 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn dout2_mode(&mut self) -> DOUT2_MODE_W<6> {
        DOUT2_MODE_W::new(self)
    }
    #[doc = "Bits 9:11 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn dout3_mode(&mut self) -> DOUT3_MODE_W<9> {
        DOUT3_MODE_W::new(self)
    }
    #[doc = "Bits 12:14 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn dout4_mode(&mut self) -> DOUT4_MODE_W<12> {
        DOUT4_MODE_W::new(self)
    }
    #[doc = "Bits 15:17 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn dout5_mode(&mut self) -> DOUT5_MODE_W<15> {
        DOUT5_MODE_W::new(self)
    }
    #[doc = "Bits 18:20 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn dout6_mode(&mut self) -> DOUT6_MODE_W<18> {
        DOUT6_MODE_W::new(self)
    }
    #[doc = "Bits 21:23 - the output signals are delayed by system clock cycles, 0: output without delayed, 1: output with the posedge of clk_apb,2 output with the negedge of clk_apb, 3: output with the spi_clk. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn dout7_mode(&mut self) -> DOUT7_MODE_W<21> {
        DOUT7_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI output delay mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout_mode](index.html) module"]
pub struct DOUT_MODE_SPEC;
impl crate::RegisterSpec for DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dout_mode::R](R) reader structure"]
impl crate::Readable for DOUT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dout_mode::W](W) writer structure"]
impl crate::Writable for DOUT_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUT_MODE to value 0"]
impl crate::Resettable for DOUT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
