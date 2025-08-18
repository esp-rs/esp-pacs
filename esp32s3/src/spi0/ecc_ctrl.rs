#[doc = "Register `ECC_CTRL` reader"]
pub type R = crate::R<ECC_CTRL_SPEC>;
#[doc = "Register `ECC_CTRL` writer"]
pub type W = crate::W<ECC_CTRL_SPEC>;
#[doc = "Field `ECC_ERR_INT_NUM` reader - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
pub type ECC_ERR_INT_NUM_R = crate::FieldReader;
#[doc = "Field `ECC_ERR_INT_NUM` writer - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
pub type ECC_ERR_INT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPI_FMEM_ECC_ERR_INT_EN` reader - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
pub type SPI_FMEM_ECC_ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_ECC_ERR_INT_EN` writer - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
pub type SPI_FMEM_ECC_ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err_int_num(&self) -> ECC_ERR_INT_NUM_R {
        ECC_ERR_INT_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_en(&self) -> SPI_FMEM_ECC_ERR_INT_EN_R {
        SPI_FMEM_ECC_ERR_INT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_CTRL")
            .field("ecc_err_int_num", &self.ecc_err_int_num())
            .field("spi_fmem_ecc_err_int_en", &self.spi_fmem_ecc_err_int_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err_int_num(&mut self) -> ECC_ERR_INT_NUM_W<'_, ECC_CTRL_SPEC> {
        ECC_ERR_INT_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
    #[inline(always)]
    pub fn spi_fmem_ecc_err_int_en(&mut self) -> SPI_FMEM_ECC_ERR_INT_EN_W<'_, ECC_CTRL_SPEC> {
        SPI_FMEM_ECC_ERR_INT_EN_W::new(self, 8)
    }
}
#[doc = "MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets ECC_CTRL to value 0x0a"]
impl crate::Resettable for ECC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0a;
}
