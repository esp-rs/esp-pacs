#[doc = "Register `SPI_RD_STATUS` reader"]
pub type R = crate::R<SPI_RD_STATUS_SPEC>;
#[doc = "Register `SPI_RD_STATUS` writer"]
pub type W = crate::W<SPI_RD_STATUS_SPEC>;
#[doc = "Field `slv_rd_status` reader - In the slave mode, this register are the status register for the master to read out."]
pub type SLV_RD_STATUS_R = crate::FieldReader<u32>;
#[doc = "Field `slv_rd_status` writer - In the slave mode, this register are the status register for the master to read out."]
pub type SLV_RD_STATUS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - In the slave mode, this register are the status register for the master to read out."]
    #[inline(always)]
    pub fn slv_rd_status(&self) -> SLV_RD_STATUS_R {
        SLV_RD_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_RD_STATUS")
            .field(
                "slv_rd_status",
                &format_args!("{}", self.slv_rd_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_RD_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - In the slave mode, this register are the status register for the master to read out."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_status(&mut self) -> SLV_RD_STATUS_W<SPI_RD_STATUS_SPEC> {
        SLV_RD_STATUS_W::new(self, 0)
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
#[doc = "In the slave mode, this register are the status register for the master to read out.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_rd_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_rd_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_RD_STATUS_SPEC;
impl crate::RegisterSpec for SPI_RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_rd_status::R`](R) reader structure"]
impl crate::Readable for SPI_RD_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_rd_status::W`](W) writer structure"]
impl crate::Writable for SPI_RD_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_RD_STATUS to value 0"]
impl crate::Resettable for SPI_RD_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
