#[doc = "Register `SPI_SMEM_S_ECC_CTRL` reader"]
pub type R = crate::R<SPI_SMEM_S_ECC_CTRL_SPEC>;
#[doc = "Register `SPI_SMEM_S_ECC_CTRL` writer"]
pub type W = crate::W<SPI_SMEM_S_ECC_CTRL_SPEC>;
#[doc = "Field `SPI_SMEM_S_ECC_ERR_INT_EN` reader - "]
pub type SPI_SMEM_S_ECC_ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_ECC_ERR_INT_EN` writer - "]
pub type SPI_SMEM_S_ECC_ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_S_PAGE_SIZE` reader - "]
pub type SPI_SMEM_S_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `SPI_SMEM_S_PAGE_SIZE` writer - "]
pub type SPI_SMEM_S_PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_S_ECC_ADDR_EN` reader - "]
pub type SPI_SMEM_S_ECC_ADDR_EN_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_S_ECC_ADDR_EN` writer - "]
pub type SPI_SMEM_S_ECC_ADDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_smem_s_ecc_err_int_en(&self) -> SPI_SMEM_S_ECC_ERR_INT_EN_R {
        SPI_SMEM_S_ECC_ERR_INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn spi_smem_s_page_size(&self) -> SPI_SMEM_S_PAGE_SIZE_R {
        SPI_SMEM_S_PAGE_SIZE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_smem_s_ecc_addr_en(&self) -> SPI_SMEM_S_ECC_ADDR_EN_R {
        SPI_SMEM_S_ECC_ADDR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_S_ECC_CTRL")
            .field(
                "spi_smem_s_ecc_err_int_en",
                &self.spi_smem_s_ecc_err_int_en(),
            )
            .field("spi_smem_s_page_size", &self.spi_smem_s_page_size())
            .field("spi_smem_s_ecc_addr_en", &self.spi_smem_s_ecc_addr_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_smem_s_ecc_err_int_en(
        &mut self,
    ) -> SPI_SMEM_S_ECC_ERR_INT_EN_W<'_, SPI_SMEM_S_ECC_CTRL_SPEC> {
        SPI_SMEM_S_ECC_ERR_INT_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn spi_smem_s_page_size(&mut self) -> SPI_SMEM_S_PAGE_SIZE_W<'_, SPI_SMEM_S_ECC_CTRL_SPEC> {
        SPI_SMEM_S_PAGE_SIZE_W::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi_smem_s_ecc_addr_en(
        &mut self,
    ) -> SPI_SMEM_S_ECC_ADDR_EN_W<'_, SPI_SMEM_S_ECC_CTRL_SPEC> {
        SPI_SMEM_S_ECC_ADDR_EN_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_s_ecc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_s_ecc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_S_ECC_CTRL_SPEC;
impl crate::RegisterSpec for SPI_SMEM_S_ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_s_ecc_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_S_ECC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_s_ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_SMEM_S_ECC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_S_ECC_CTRL to value 0x0008_0000"]
impl crate::Resettable for SPI_SMEM_S_ECC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0008_0000;
}
