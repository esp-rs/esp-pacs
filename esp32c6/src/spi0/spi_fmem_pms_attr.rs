#[doc = "Register `SPI_FMEM_PMS%s_ATTR` reader"]
pub struct R(crate::R<SPI_FMEM_PMS_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FMEM_PMS_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FMEM_PMS_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FMEM_PMS_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_FMEM_PMS%s_ATTR` writer"]
pub struct W(crate::W<SPI_FMEM_PMS_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FMEM_PMS_ATTR_SPEC>;
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
impl From<crate::W<SPI_FMEM_PMS_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_FMEM_PMS_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_FMEM_PMS_RD_ATTR` reader - 1: SPI1 flash ACE section %s read accessible. 0: Not allowed."]
pub type SPI_FMEM_PMS_RD_ATTR_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_PMS_RD_ATTR` writer - 1: SPI1 flash ACE section %s read accessible. 0: Not allowed."]
pub type SPI_FMEM_PMS_RD_ATTR_W<'a, const O: u8> = crate::BitWriter<'a, SPI_FMEM_PMS_ATTR_SPEC, O>;
#[doc = "Field `SPI_FMEM_PMS_WR_ATTR` reader - 1: SPI1 flash ACE section %s write accessible. 0: Not allowed."]
pub type SPI_FMEM_PMS_WR_ATTR_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_PMS_WR_ATTR` writer - 1: SPI1 flash ACE section %s write accessible. 0: Not allowed."]
pub type SPI_FMEM_PMS_WR_ATTR_W<'a, const O: u8> = crate::BitWriter<'a, SPI_FMEM_PMS_ATTR_SPEC, O>;
#[doc = "Field `SPI_FMEM_PMS_ECC` reader - SPI1 flash ACE section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash ACE section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG."]
pub type SPI_FMEM_PMS_ECC_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_PMS_ECC` writer - SPI1 flash ACE section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash ACE section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG."]
pub type SPI_FMEM_PMS_ECC_W<'a, const O: u8> = crate::BitWriter<'a, SPI_FMEM_PMS_ATTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - 1: SPI1 flash ACE section %s read accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_fmem_pms_rd_attr(&self) -> SPI_FMEM_PMS_RD_ATTR_R {
        SPI_FMEM_PMS_RD_ATTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: SPI1 flash ACE section %s write accessible. 0: Not allowed."]
    #[inline(always)]
    pub fn spi_fmem_pms_wr_attr(&self) -> SPI_FMEM_PMS_WR_ATTR_R {
        SPI_FMEM_PMS_WR_ATTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI1 flash ACE section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash ACE section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG."]
    #[inline(always)]
    pub fn spi_fmem_pms_ecc(&self) -> SPI_FMEM_PMS_ECC_R {
        SPI_FMEM_PMS_ECC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_FMEM_PMS_ATTR")
            .field(
                "spi_fmem_pms_rd_attr",
                &format_args!("{}", self.spi_fmem_pms_rd_attr().bit()),
            )
            .field(
                "spi_fmem_pms_wr_attr",
                &format_args!("{}", self.spi_fmem_pms_wr_attr().bit()),
            )
            .field(
                "spi_fmem_pms_ecc",
                &format_args!("{}", self.spi_fmem_pms_ecc().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_FMEM_PMS_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1: SPI1 flash ACE section %s read accessible. 0: Not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_pms_rd_attr(&mut self) -> SPI_FMEM_PMS_RD_ATTR_W<0> {
        SPI_FMEM_PMS_RD_ATTR_W::new(self)
    }
    #[doc = "Bit 1 - 1: SPI1 flash ACE section %s write accessible. 0: Not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_pms_wr_attr(&mut self) -> SPI_FMEM_PMS_WR_ATTR_W<1> {
        SPI_FMEM_PMS_WR_ATTR_W::new(self)
    }
    #[doc = "Bit 2 - SPI1 flash ACE section %s ECC mode, 1: enable ECC mode. 0: Disable it. The flash ACE section %s is configured by registers SPI_FMEM_PMS%s_ADDR_REG and SPI_FMEM_PMS%s_SIZE_REG."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_pms_ecc(&mut self) -> SPI_FMEM_PMS_ECC_W<2> {
        SPI_FMEM_PMS_ECC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI flash ACE section %s attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fmem_pms_attr](index.html) module"]
pub struct SPI_FMEM_PMS_ATTR_SPEC;
impl crate::RegisterSpec for SPI_FMEM_PMS_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_fmem_pms_attr::R](R) reader structure"]
impl crate::Readable for SPI_FMEM_PMS_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_fmem_pms_attr::W](W) writer structure"]
impl crate::Writable for SPI_FMEM_PMS_ATTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_FMEM_PMS%s_ATTR to value 0x03"]
impl crate::Resettable for SPI_FMEM_PMS_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
