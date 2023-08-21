#[doc = "Register `TIMING_CALI` reader"]
pub type R = crate::R<TIMING_CALI_SPEC>;
#[doc = "Register `TIMING_CALI` writer"]
pub type W = crate::W<TIMING_CALI_SPEC>;
#[doc = "Field `TIMING_CLK_ENA` reader - The bit is used to enable timing adjust clock for all reading operations."]
pub type TIMING_CLK_ENA_R = crate::BitReader;
#[doc = "Field `TIMING_CLK_ENA` writer - The bit is used to enable timing adjust clock for all reading operations."]
pub type TIMING_CLK_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMING_CALI` reader - The bit is used to enable timing auto-calibration for all reading operations."]
pub type TIMING_CALI_R = crate::BitReader;
#[doc = "Field `TIMING_CALI` writer - The bit is used to enable timing auto-calibration for all reading operations."]
pub type TIMING_CALI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` reader - add extra dummy spi clock cycle length for spi clock calibration."]
pub type EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` writer - add extra dummy spi clock cycle length for spi clock calibration."]
pub type EXTRA_DUMMY_CYCLELEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn timing_clk_ena(&self) -> TIMING_CLK_ENA_R {
        TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn timing_cali(&self) -> TIMING_CALI_R {
        TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn extra_dummy_cyclelen(&self) -> EXTRA_DUMMY_CYCLELEN_R {
        EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMING_CALI")
            .field(
                "timing_clk_ena",
                &format_args!("{}", self.timing_clk_ena().bit()),
            )
            .field("timing_cali", &format_args!("{}", self.timing_cali().bit()))
            .field(
                "extra_dummy_cyclelen",
                &format_args!("{}", self.extra_dummy_cyclelen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMING_CALI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    #[must_use]
    pub fn timing_clk_ena(&mut self) -> TIMING_CLK_ENA_W<TIMING_CALI_SPEC, 0> {
        TIMING_CLK_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    #[must_use]
    pub fn timing_cali(&mut self) -> TIMING_CALI_W<TIMING_CALI_SPEC, 1> {
        TIMING_CALI_W::new(self)
    }
    #[doc = "Bits 2:4 - add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    #[must_use]
    pub fn extra_dummy_cyclelen(&mut self) -> EXTRA_DUMMY_CYCLELEN_W<TIMING_CALI_SPEC, 2> {
        EXTRA_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI0 timing calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing_cali::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing_cali::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMING_CALI_SPEC;
impl crate::RegisterSpec for TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timing_cali::R`](R) reader structure"]
impl crate::Readable for TIMING_CALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timing_cali::W`](W) writer structure"]
impl crate::Writable for TIMING_CALI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMING_CALI to value 0x01"]
impl crate::Resettable for TIMING_CALI_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
