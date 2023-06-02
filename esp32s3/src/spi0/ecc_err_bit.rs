#[doc = "Register `ECC_ERR_BIT` reader"]
pub struct R(crate::R<ECC_ERR_BIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC_ERR_BIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC_ERR_BIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC_ERR_BIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC_DATA_ERR_BIT` reader - It records the first ECC data error bit number when SPI_FMEM_ECC_ERR_INT_EN/SPI_SMEM_ECC_ERR_INT_EN is set and accessed to flash/Ext_RAM. The value ranges from 0~127, corresponding to the bit number in 16 data bytes. It is cleared by SPI_MEM_ECC_ERR_INT_CLR bit."]
pub type ECC_DATA_ERR_BIT_R = crate::FieldReader;
#[doc = "Field `ECC_CHK_ERR_BIT` reader - When SPI_MEM_ECC_BYTE_ERR is set, these bits show the error bit number of ECC byte."]
pub type ECC_CHK_ERR_BIT_R = crate::FieldReader;
#[doc = "Field `ECC_BYTE_ERR` reader - It records the first ECC byte error when SPI_FMEM_ECC_ERR_INT_EN/SPI_SMEM_ECC_ERR_INT_EN is set and accessed to flash/Ext_RAM. It is cleared by SPI_MEM_ECC_ERR_INT_CLR bit."]
pub type ECC_BYTE_ERR_R = crate::BitReader;
#[doc = "Field `ECC_ERR_CNT` reader - This bits show the error times of MSPI ECC read, including ECC byte error and data byte error. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
pub type ECC_ERR_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 6:12 - It records the first ECC data error bit number when SPI_FMEM_ECC_ERR_INT_EN/SPI_SMEM_ECC_ERR_INT_EN is set and accessed to flash/Ext_RAM. The value ranges from 0~127, corresponding to the bit number in 16 data bytes. It is cleared by SPI_MEM_ECC_ERR_INT_CLR bit."]
    #[inline(always)]
    pub fn ecc_data_err_bit(&self) -> ECC_DATA_ERR_BIT_R {
        ECC_DATA_ERR_BIT_R::new(((self.bits >> 6) & 0x7f) as u8)
    }
    #[doc = "Bits 13:15 - When SPI_MEM_ECC_BYTE_ERR is set, these bits show the error bit number of ECC byte."]
    #[inline(always)]
    pub fn ecc_chk_err_bit(&self) -> ECC_CHK_ERR_BIT_R {
        ECC_CHK_ERR_BIT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - It records the first ECC byte error when SPI_FMEM_ECC_ERR_INT_EN/SPI_SMEM_ECC_ERR_INT_EN is set and accessed to flash/Ext_RAM. It is cleared by SPI_MEM_ECC_ERR_INT_CLR bit."]
    #[inline(always)]
    pub fn ecc_byte_err(&self) -> ECC_BYTE_ERR_R {
        ECC_BYTE_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:24 - This bits show the error times of MSPI ECC read, including ECC byte error and data byte error. It is cleared by when SPI_MEM_ECC_ERR_INT_CLR bit is set."]
    #[inline(always)]
    pub fn ecc_err_cnt(&self) -> ECC_ERR_CNT_R {
        ECC_ERR_CNT_R::new(((self.bits >> 17) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECC_ERR_BIT")
            .field(
                "ecc_data_err_bit",
                &format_args!("{}", self.ecc_data_err_bit().bits()),
            )
            .field(
                "ecc_chk_err_bit",
                &format_args!("{}", self.ecc_chk_err_bit().bits()),
            )
            .field(
                "ecc_byte_err",
                &format_args!("{}", self.ecc_byte_err().bit()),
            )
            .field(
                "ecc_err_cnt",
                &format_args!("{}", self.ecc_err_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECC_ERR_BIT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MSPI ECC error bits register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc_err_bit](index.html) module"]
pub struct ECC_ERR_BIT_SPEC;
impl crate::RegisterSpec for ECC_ERR_BIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc_err_bit::R](R) reader structure"]
impl crate::Readable for ECC_ERR_BIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC_ERR_BIT to value 0"]
impl crate::Resettable for ECC_ERR_BIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
