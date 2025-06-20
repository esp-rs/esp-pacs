#[doc = "Register `ECC_CTRL` reader"]
pub type R = crate::R<ECC_CTRL_SPEC>;
#[doc = "Register `ECC_CTRL` writer"]
pub type W = crate::W<ECC_CTRL_SPEC>;
#[doc = "Field `ECC_ERR_CNT` reader - This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
pub type ECC_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `FMEM_ECC_ERR_INT_NUM` reader - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
pub type FMEM_ECC_ERR_INT_NUM_R = crate::FieldReader;
#[doc = "Field `FMEM_ECC_ERR_INT_NUM` writer - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
pub type FMEM_ECC_ERR_INT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FMEM_ECC_ERR_INT_EN` reader - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
pub type FMEM_ECC_ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `FMEM_ECC_ERR_INT_EN` writer - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
pub type FMEM_ECC_ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMEM_PAGE_SIZE` reader - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type FMEM_PAGE_SIZE_R = crate::FieldReader;
#[doc = "Field `FMEM_PAGE_SIZE` writer - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
pub type FMEM_PAGE_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FMEM_ECC_ADDR_EN` reader - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
pub type FMEM_ECC_ADDR_EN_R = crate::BitReader;
#[doc = "Field `FMEM_ECC_ADDR_EN` writer - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
pub type FMEM_ECC_ADDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_ECC_ADDR_EN` reader - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
pub type USR_ECC_ADDR_EN_R = crate::BitReader;
#[doc = "Field `USR_ECC_ADDR_EN` writer - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
pub type USR_ECC_ADDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_CONTINUE_RECORD_ERR_EN` reader - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
pub type ECC_CONTINUE_RECORD_ERR_EN_R = crate::BitReader;
#[doc = "Field `ECC_CONTINUE_RECORD_ERR_EN` writer - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
pub type ECC_CONTINUE_RECORD_ERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR_BITS` reader - Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)"]
pub type ECC_ERR_BITS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 5:10 - This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
    #[inline(always)]
    pub fn ecc_err_cnt(&self) -> ECC_ERR_CNT_R {
        ECC_ERR_CNT_R::new(((self.bits >> 5) & 0x3f) as u8)
    }
    #[doc = "Bits 11:16 - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn fmem_ecc_err_int_num(&self) -> FMEM_ECC_ERR_INT_NUM_R {
        FMEM_ECC_ERR_INT_NUM_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
    #[inline(always)]
    pub fn fmem_ecc_err_int_en(&self) -> FMEM_ECC_ERR_INT_EN_R {
        FMEM_ECC_ERR_INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn fmem_page_size(&self) -> FMEM_PAGE_SIZE_R {
        FMEM_PAGE_SIZE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
    #[inline(always)]
    pub fn fmem_ecc_addr_en(&self) -> FMEM_ECC_ADDR_EN_R {
        FMEM_ECC_ADDR_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
    #[inline(always)]
    pub fn usr_ecc_addr_en(&self) -> USR_ECC_ADDR_EN_R {
        USR_ECC_ADDR_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
    #[inline(always)]
    pub fn ecc_continue_record_err_en(&self) -> ECC_CONTINUE_RECORD_ERR_EN_R {
        ECC_CONTINUE_RECORD_ERR_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)"]
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
            .field("fmem_ecc_err_int_num", &self.fmem_ecc_err_int_num())
            .field("fmem_ecc_err_int_en", &self.fmem_ecc_err_int_en())
            .field("fmem_page_size", &self.fmem_page_size())
            .field("fmem_ecc_addr_en", &self.fmem_ecc_addr_en())
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
    #[doc = "Bits 11:16 - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn fmem_ecc_err_int_num(&mut self) -> FMEM_ECC_ERR_INT_NUM_W<ECC_CTRL_SPEC> {
        FMEM_ECC_ERR_INT_NUM_W::new(self, 11)
    }
    #[doc = "Bit 17 - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
    #[inline(always)]
    pub fn fmem_ecc_err_int_en(&mut self) -> FMEM_ECC_ERR_INT_EN_W<ECC_CTRL_SPEC> {
        FMEM_ECC_ERR_INT_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn fmem_page_size(&mut self) -> FMEM_PAGE_SIZE_W<ECC_CTRL_SPEC> {
        FMEM_PAGE_SIZE_W::new(self, 18)
    }
    #[doc = "Bit 21 - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
    #[inline(always)]
    pub fn fmem_ecc_addr_en(&mut self) -> FMEM_ECC_ADDR_EN_W<ECC_CTRL_SPEC> {
        FMEM_ECC_ADDR_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
    #[inline(always)]
    pub fn usr_ecc_addr_en(&mut self) -> USR_ECC_ADDR_EN_W<ECC_CTRL_SPEC> {
        USR_ECC_ADDR_EN_W::new(self, 22)
    }
    #[doc = "Bit 24 - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
    #[inline(always)]
    pub fn ecc_continue_record_err_en(&mut self) -> ECC_CONTINUE_RECORD_ERR_EN_W<ECC_CTRL_SPEC> {
        ECC_CONTINUE_RECORD_ERR_EN_W::new(self, 24)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_CTRL to value 0x0100_5000"]
impl crate::Resettable for ECC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0100_5000;
}
