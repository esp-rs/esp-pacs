#[doc = "Register `SPI_EXT3` reader"]
pub type R = crate::R<SPI_EXT3_SPEC>;
#[doc = "Register `SPI_EXT3` writer"]
pub type W = crate::W<SPI_EXT3_SPEC>;
#[doc = "Field `reg_int_hold_ena` reader - This register is for two SPI masters to share the same cs, clock and data signals."]
pub type REG_INT_HOLD_ENA_R = crate::FieldReader;
#[doc = "Field `reg_int_hold_ena` writer - This register is for two SPI masters to share the same cs, clock and data signals."]
pub type REG_INT_HOLD_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs, clock and data signals."]
    #[inline(always)]
    pub fn reg_int_hold_ena(&self) -> REG_INT_HOLD_ENA_R {
        REG_INT_HOLD_ENA_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_EXT3")
            .field(
                "reg_int_hold_ena",
                &format_args!("{}", self.reg_int_hold_ena().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_EXT3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs, clock and data signals."]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_hold_ena(&mut self) -> REG_INT_HOLD_ENA_W<SPI_EXT3_SPEC> {
        REG_INT_HOLD_ENA_W::new(self, 0)
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
#[doc = "This register is for two SPI masters to share the same cs, clock and data signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ext3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ext3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_EXT3_SPEC;
impl crate::RegisterSpec for SPI_EXT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ext3::R`](R) reader structure"]
impl crate::Readable for SPI_EXT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ext3::W`](W) writer structure"]
impl crate::Writable for SPI_EXT3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_EXT3 to value 0"]
impl crate::Resettable for SPI_EXT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
