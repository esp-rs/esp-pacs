#[doc = "Register `SPI_WR_STATUS` reader"]
pub struct R(crate::R<SPI_WR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_WR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_WR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_WR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_WR_STATUS` writer"]
pub struct W(crate::W<SPI_WR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_WR_STATUS_SPEC>;
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
impl From<crate::W<SPI_WR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_WR_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slv_wr_status` reader - In the slave mode, this register are the status register for the master to write into."]
pub type SLV_WR_STATUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `slv_wr_status` writer - In the slave mode, this register are the status register for the master to write into."]
pub type SLV_WR_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_WR_STATUS_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - In the slave mode, this register are the status register for the master to write into."]
    #[inline(always)]
    pub fn slv_wr_status(&self) -> SLV_WR_STATUS_R {
        SLV_WR_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_WR_STATUS")
            .field(
                "slv_wr_status",
                &format_args!("{}", self.slv_wr_status().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_WR_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - In the slave mode, this register are the status register for the master to write into."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_status(&mut self) -> SLV_WR_STATUS_W<0> {
        SLV_WR_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In the slave mode, this register are the status register for the master to write into.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_wr_status](index.html) module"]
pub struct SPI_WR_STATUS_SPEC;
impl crate::RegisterSpec for SPI_WR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_wr_status::R](R) reader structure"]
impl crate::Readable for SPI_WR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_wr_status::W](W) writer structure"]
impl crate::Writable for SPI_WR_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_WR_STATUS to value 0"]
impl crate::Resettable for SPI_WR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
