///Register `ECC_ERR_BIT` reader
pub type R = crate::R<ECC_ERR_BIT_SPEC>;
///Field `ECC_DATA_ERR_BIT` reader - It records the first ECC data error bit number when SPI_FMEM_ECC_ERR_INT_EN/SPI_SMEM_ECC_ERR_INT_EN is set and accessed to flash/Ext_RAM. The value ranges from 0~127, corresponding to the bit number in 16 data bytes. It is cleared by SPI_MEM_ECC_ERR_INT_CLR bit.
pub type ECC_DATA_ERR_BIT_R = crate::FieldReader;
///Field `ECC_CHK_ERR_BIT` reader - When SPI_MEM_ECC_BYTE_ERR is set, these bits show the error bit number of ECC byte.
pub type ECC_CHK_ERR_BIT_R = crate::FieldReader;
///Field `ECC_BYTE_ERR` reader - It records the first ECC byte error when SPI_FMEM_ECC_ERR_INT_EN/SPI_SMEM_ECC_ERR_INT_EN is set and accessed to flash/Ext_RAM. It is cleared by SPI_MEM_ECC_ERR_INT_CLR bit.
pub type ECC_BYTE_ERR_R = crate::BitReader;
///Field `ECC_ERR_CNT` reader - This bits show the error times of MSPI ECC read, including ECC byte error and data byte error. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set.
pub type ECC_ERR_CNT_R = crate::FieldReader;
impl R {
    ///Bits 6:12 - It records the first ECC data error bit number when SPI_FMEM_ECC_ERR_INT_EN/SPI_SMEM_ECC_ERR_INT_EN is set and accessed to flash/Ext_RAM. The value ranges from 0~127, corresponding to the bit number in 16 data bytes. It is cleared by SPI_MEM_ECC_ERR_INT_CLR bit.
    #[inline(always)]
    pub fn ecc_data_err_bit(&self) -> ECC_DATA_ERR_BIT_R {
        ECC_DATA_ERR_BIT_R::new(((self.bits >> 6) & 0x7f) as u8)
    }
    ///Bits 13:15 - When SPI_MEM_ECC_BYTE_ERR is set, these bits show the error bit number of ECC byte.
    #[inline(always)]
    pub fn ecc_chk_err_bit(&self) -> ECC_CHK_ERR_BIT_R {
        ECC_CHK_ERR_BIT_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bit 16 - It records the first ECC byte error when SPI_FMEM_ECC_ERR_INT_EN/SPI_SMEM_ECC_ERR_INT_EN is set and accessed to flash/Ext_RAM. It is cleared by SPI_MEM_ECC_ERR_INT_CLR bit.
    #[inline(always)]
    pub fn ecc_byte_err(&self) -> ECC_BYTE_ERR_R {
        ECC_BYTE_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:24 - This bits show the error times of MSPI ECC read, including ECC byte error and data byte error. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set.
    #[inline(always)]
    pub fn ecc_err_cnt(&self) -> ECC_ERR_CNT_R {
        ECC_ERR_CNT_R::new(((self.bits >> 17) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_ERR_BIT")
            .field("ecc_data_err_bit", &self.ecc_data_err_bit())
            .field("ecc_chk_err_bit", &self.ecc_chk_err_bit())
            .field("ecc_byte_err", &self.ecc_byte_err())
            .field("ecc_err_cnt", &self.ecc_err_cnt())
            .finish()
    }
}
/**MSPI ECC error bits register

You can [`read`](crate::generic::Reg::read) this register and get [`ecc_err_bit::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECC_ERR_BIT_SPEC;
impl crate::RegisterSpec for ECC_ERR_BIT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ecc_err_bit::R`](R) reader structure
impl crate::Readable for ECC_ERR_BIT_SPEC {}
///`reset()` method sets ECC_ERR_BIT to value 0
impl crate::Resettable for ECC_ERR_BIT_SPEC {
    const RESET_VALUE: u32 = 0;
}
