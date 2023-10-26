#[doc = "Register `SPI_MEM_PMS_CTRL` reader"]
pub type R = crate::R<SPI_MEM_PMS_CTRL_SPEC>;
#[doc = "Register `SPI_MEM_PMS_CTRL` writer"]
pub type W = crate::W<SPI_MEM_PMS_CTRL_SPEC>;
#[doc = "Field `SPI_MEM_REJECT_INT` reader - ******* Description ***********"]
pub type SPI_MEM_REJECT_INT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_REJECT_CLR` writer - ******* Description ***********"]
pub type SPI_MEM_REJECT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI_MEM_REJECT_CDE` reader - ******* Description ***********"]
pub type SPI_MEM_REJECT_CDE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - ******* Description ***********"]
    #[inline(always)]
    pub fn spi_mem_reject_int(&self) -> SPI_MEM_REJECT_INT_R {
        SPI_MEM_REJECT_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:6 - ******* Description ***********"]
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
    #[doc = "Bit 1 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_reject_clr(&mut self) -> SPI_MEM_REJECT_CLR_W<SPI_MEM_PMS_CTRL_SPEC, 1> {
        SPI_MEM_REJECT_CLR_W::new(self)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_pms_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_pms_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_PMS_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_PMS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_pms_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_PMS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_pms_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_PMS_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_PMS_CTRL to value 0"]
impl crate::Resettable for SPI_MEM_PMS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
