#[doc = "Register `MEM_ECC_CTRL` reader"]
pub type R = crate::R<MEM_ECC_CTRL_SPEC>;
#[doc = "Register `MEM_ECC_CTRL` writer"]
pub type W = crate::W<MEM_ECC_CTRL_SPEC>;
#[doc = "Field `MEM_ECC_ERR_CNT` reader - This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
pub type MEM_ECC_ERR_CNT_R = crate::FieldReader;
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
#[doc = "Field `MEM_USR_ECC_ADDR_EN` reader - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
pub type MEM_USR_ECC_ADDR_EN_R = crate::BitReader;
#[doc = "Field `MEM_USR_ECC_ADDR_EN` writer - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
pub type MEM_USR_ECC_ADDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_ECC_CONTINUE_RECORD_ERR_EN` reader - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
pub type MEM_ECC_CONTINUE_RECORD_ERR_EN_R = crate::BitReader;
#[doc = "Field `MEM_ECC_CONTINUE_RECORD_ERR_EN` writer - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
pub type MEM_ECC_CONTINUE_RECORD_ERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_ECC_ERR_BITS` reader - Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)"]
pub type MEM_ECC_ERR_BITS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 5:10 - This bits show the error times of MSPI ECC read. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
    #[inline(always)]
    pub fn mem_ecc_err_cnt(&self) -> MEM_ECC_ERR_CNT_R {
        MEM_ECC_ERR_CNT_R::new(((self.bits >> 5) & 0x3f) as u8)
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
    pub fn mem_usr_ecc_addr_en(&self) -> MEM_USR_ECC_ADDR_EN_R {
        MEM_USR_ECC_ADDR_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
    #[inline(always)]
    pub fn mem_ecc_continue_record_err_en(&self) -> MEM_ECC_CONTINUE_RECORD_ERR_EN_R {
        MEM_ECC_CONTINUE_RECORD_ERR_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Records the first ECC error bit number in the 16 bytes(From 0~127, corresponding to byte 0 bit 0 to byte 15 bit 7)"]
    #[inline(always)]
    pub fn mem_ecc_err_bits(&self) -> MEM_ECC_ERR_BITS_R {
        MEM_ECC_ERR_BITS_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_ECC_CTRL")
            .field("mem_ecc_err_cnt", &self.mem_ecc_err_cnt())
            .field("fmem_ecc_err_int_num", &self.fmem_ecc_err_int_num())
            .field("fmem_ecc_err_int_en", &self.fmem_ecc_err_int_en())
            .field("fmem_page_size", &self.fmem_page_size())
            .field("fmem_ecc_addr_en", &self.fmem_ecc_addr_en())
            .field("mem_usr_ecc_addr_en", &self.mem_usr_ecc_addr_en())
            .field(
                "mem_ecc_continue_record_err_en",
                &self.mem_ecc_continue_record_err_en(),
            )
            .field("mem_ecc_err_bits", &self.mem_ecc_err_bits())
            .finish()
    }
}
impl W {
    #[doc = "Bits 11:16 - Set the error times of MSPI ECC read to generate MSPI SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn fmem_ecc_err_int_num(&mut self) -> FMEM_ECC_ERR_INT_NUM_W<MEM_ECC_CTRL_SPEC> {
        FMEM_ECC_ERR_INT_NUM_W::new(self, 11)
    }
    #[doc = "Bit 17 - Set this bit to calculate the error times of MSPI ECC read when accesses to flash."]
    #[inline(always)]
    pub fn fmem_ecc_err_int_en(&mut self) -> FMEM_ECC_ERR_INT_EN_W<MEM_ECC_CTRL_SPEC> {
        FMEM_ECC_ERR_INT_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Set the page size of the flash accessed by MSPI. 0: 256 bytes. 1: 512 bytes. 2: 1024 bytes. 3: 2048 bytes."]
    #[inline(always)]
    pub fn fmem_page_size(&mut self) -> FMEM_PAGE_SIZE_W<MEM_ECC_CTRL_SPEC> {
        FMEM_PAGE_SIZE_W::new(self, 18)
    }
    #[doc = "Bit 21 - Set this bit to enable MSPI ECC address conversion, no matter MSPI accesses to the ECC region or non-ECC region of flash. If there is no ECC region in flash, this bit should be 0. Otherwise, this bit should be 1."]
    #[inline(always)]
    pub fn fmem_ecc_addr_en(&mut self) -> FMEM_ECC_ADDR_EN_W<MEM_ECC_CTRL_SPEC> {
        FMEM_ECC_ADDR_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to enable ECC address convert in SPI0/1 USR_CMD transfer."]
    #[inline(always)]
    pub fn mem_usr_ecc_addr_en(&mut self) -> MEM_USR_ECC_ADDR_EN_W<MEM_ECC_CTRL_SPEC> {
        MEM_USR_ECC_ADDR_EN_W::new(self, 22)
    }
    #[doc = "Bit 24 - 1: The error information in SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR is updated when there is an ECC error. 0: SPI_MEM_ECC_ERR_BITS and SPI_MEM_ECC_ERR_ADDR record the first ECC error information."]
    #[inline(always)]
    pub fn mem_ecc_continue_record_err_en(
        &mut self,
    ) -> MEM_ECC_CONTINUE_RECORD_ERR_EN_W<MEM_ECC_CTRL_SPEC> {
        MEM_ECC_CONTINUE_RECORD_ERR_EN_W::new(self, 24)
    }
}
#[doc = "MSPI ECC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ecc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ecc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_ECC_CTRL_SPEC;
impl crate::RegisterSpec for MEM_ECC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ecc_ctrl::R`](R) reader structure"]
impl crate::Readable for MEM_ECC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for MEM_ECC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_ECC_CTRL to value 0x0100_5000"]
impl crate::Resettable for MEM_ECC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0100_5000;
}
