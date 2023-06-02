#[doc = "Register `SPI_MEM_PMS_CTRL` reader"]
pub struct R(crate::R<SPI_MEM_PMS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_PMS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_PMS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_PMS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_PMS_CTRL` writer"]
pub struct W(crate::W<SPI_MEM_PMS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_PMS_CTRL_SPEC>;
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
impl From<crate::W<SPI_MEM_PMS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_PMS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_REJECT_INT` reader - reg_spi_mem_reject_int"]
pub type SPI_MEM_REJECT_INT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_REJECT_CLR` writer - reg_spi_mem_reject_clr"]
pub type SPI_MEM_REJECT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_PMS_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_REJECT_CDE` reader - reg_spi_mem_reject_cde"]
pub type SPI_MEM_REJECT_CDE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - reg_spi_mem_reject_int"]
    #[inline(always)]
    pub fn spi_mem_reject_int(&self) -> SPI_MEM_REJECT_INT_R {
        SPI_MEM_REJECT_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:6 - reg_spi_mem_reject_cde"]
    #[inline(always)]
    pub fn spi_mem_reject_cde(&self) -> SPI_MEM_REJECT_CDE_R {
        SPI_MEM_REJECT_CDE_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_PMS_CTRL")
            .field(
                "spi_mem_reject_int",
                &format_args!("{}", self.spi_mem_reject_int().bit()),
            )
            .field(
                "spi_mem_reject_cde",
                &format_args!("{}", self.spi_mem_reject_cde().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_PMS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - reg_spi_mem_reject_clr"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_reject_clr(&mut self) -> SPI_MEM_REJECT_CLR_W<1> {
        SPI_MEM_REJECT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_SPI_MEM_PMS_CTRL_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_pms_ctrl](index.html) module"]
pub struct SPI_MEM_PMS_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_PMS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_pms_ctrl::R](R) reader structure"]
impl crate::Readable for SPI_MEM_PMS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_pms_ctrl::W](W) writer structure"]
impl crate::Writable for SPI_MEM_PMS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_PMS_CTRL to value 0"]
impl crate::Resettable for SPI_MEM_PMS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
