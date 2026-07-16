#[doc = "Register `ECC_CTRL` reader"]
pub type R = crate::R<ECC_CTRL_SPEC>;
#[doc = "Register `ECC_CTRL` writer"]
pub type W = crate::W<ECC_CTRL_SPEC>;
#[doc = "Field `ECC_ERR_CNT` reader - "]
pub type ECC_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `ECC_ERR_CNT` writer - "]
pub type ECC_ERR_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_FMEM_S_ECC_ERR_INT_NUM` reader - "]
pub type SPI_FMEM_S_ECC_ERR_INT_NUM_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_S_ECC_ERR_INT_NUM` writer - "]
pub type SPI_FMEM_S_ECC_ERR_INT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPI_FMEM_S_ECC_ERR_INT_EN` reader - "]
pub type SPI_FMEM_S_ECC_ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_ECC_ERR_INT_EN` writer - "]
pub type SPI_FMEM_S_ECC_ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_PAGE_SIZE` reader - "]
pub type SPI_FMEM_S_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_S_PAGE_SIZE` writer - "]
pub type SPI_FMEM_S_PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_FMEM_S_ECC_ADDR_EN` reader - "]
pub type SPI_FMEM_S_ECC_ADDR_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_ECC_ADDR_EN` writer - "]
pub type SPI_FMEM_S_ECC_ADDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_ECC_ADDR_EN` reader - "]
pub type USR_ECC_ADDR_EN_R = crate::BitReader;
#[doc = "Field `USR_ECC_ADDR_EN` writer - "]
pub type USR_ECC_ADDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_CONTINUE_RECORD_ERR_EN` reader - "]
pub type ECC_CONTINUE_RECORD_ERR_EN_R = crate::BitReader;
#[doc = "Field `ECC_CONTINUE_RECORD_ERR_EN` writer - "]
pub type ECC_CONTINUE_RECORD_ERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR_BITS` reader - "]
pub type ECC_ERR_BITS_R = crate::FieldReader;
#[doc = "Field `ECC_ERR_BITS` writer - "]
pub type ECC_ERR_BITS_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 5:10"]
    #[inline(always)]
    pub fn ecc_err_cnt(&self) -> ECC_ERR_CNT_R {
        ECC_ERR_CNT_R::new(((self.bits >> 5) & 0x3f) as u8)
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    pub fn spi_fmem_s_ecc_err_int_num(&self) -> SPI_FMEM_S_ECC_ERR_INT_NUM_R {
        SPI_FMEM_S_ECC_ERR_INT_NUM_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_fmem_s_ecc_err_int_en(&self) -> SPI_FMEM_S_ECC_ERR_INT_EN_R {
        SPI_FMEM_S_ECC_ERR_INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn spi_fmem_s_page_size(&self) -> SPI_FMEM_S_PAGE_SIZE_R {
        SPI_FMEM_S_PAGE_SIZE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_fmem_s_ecc_addr_en(&self) -> SPI_FMEM_S_ECC_ADDR_EN_R {
        SPI_FMEM_S_ECC_ADDR_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn usr_ecc_addr_en(&self) -> USR_ECC_ADDR_EN_R {
        USR_ECC_ADDR_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ecc_continue_record_err_en(&self) -> ECC_CONTINUE_RECORD_ERR_EN_R {
        ECC_CONTINUE_RECORD_ERR_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn ecc_err_bits(&self) -> ECC_ERR_BITS_R {
        ECC_ERR_BITS_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_CTRL")
            .field("ecc_err_cnt", &self.ecc_err_cnt())
            .field(
                "spi_fmem_s_ecc_err_int_num",
                &self.spi_fmem_s_ecc_err_int_num(),
            )
            .field(
                "spi_fmem_s_ecc_err_int_en",
                &self.spi_fmem_s_ecc_err_int_en(),
            )
            .field("spi_fmem_s_page_size", &self.spi_fmem_s_page_size())
            .field("spi_fmem_s_ecc_addr_en", &self.spi_fmem_s_ecc_addr_en())
            .field("usr_ecc_addr_en", &self.usr_ecc_addr_en())
            .field(
                "ecc_continue_record_err_en",
                &self.ecc_continue_record_err_en(),
            )
            .field("ecc_err_bits", &self.ecc_err_bits())
            .finish()
    }
}
impl W {
    #[doc = "Bits 5:10"]
    #[inline(always)]
    pub fn ecc_err_cnt(&mut self) -> ECC_ERR_CNT_W<'_, ECC_CTRL_SPEC> {
        ECC_ERR_CNT_W::new(self, 5)
    }
    #[doc = "Bits 11:16"]
    #[inline(always)]
    pub fn spi_fmem_s_ecc_err_int_num(
        &mut self,
    ) -> SPI_FMEM_S_ECC_ERR_INT_NUM_W<'_, ECC_CTRL_SPEC> {
        SPI_FMEM_S_ECC_ERR_INT_NUM_W::new(self, 11)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi_fmem_s_ecc_err_int_en(&mut self) -> SPI_FMEM_S_ECC_ERR_INT_EN_W<'_, ECC_CTRL_SPEC> {
        SPI_FMEM_S_ECC_ERR_INT_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn spi_fmem_s_page_size(&mut self) -> SPI_FMEM_S_PAGE_SIZE_W<'_, ECC_CTRL_SPEC> {
        SPI_FMEM_S_PAGE_SIZE_W::new(self, 18)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_fmem_s_ecc_addr_en(&mut self) -> SPI_FMEM_S_ECC_ADDR_EN_W<'_, ECC_CTRL_SPEC> {
        SPI_FMEM_S_ECC_ADDR_EN_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn usr_ecc_addr_en(&mut self) -> USR_ECC_ADDR_EN_W<'_, ECC_CTRL_SPEC> {
        USR_ECC_ADDR_EN_W::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ecc_continue_record_err_en(
        &mut self,
    ) -> ECC_CONTINUE_RECORD_ERR_EN_W<'_, ECC_CTRL_SPEC> {
        ECC_CONTINUE_RECORD_ERR_EN_W::new(self, 24)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn ecc_err_bits(&mut self) -> ECC_ERR_BITS_W<'_, ECC_CTRL_SPEC> {
        ECC_ERR_BITS_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECC_CTRL_SPEC;
impl crate::RegisterSpec for ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_ctrl::R`](R) reader structure"]
impl crate::Readable for ECC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for ECC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECC_CTRL to value 0x0100_5000"]
impl crate::Resettable for ECC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0100_5000;
}
