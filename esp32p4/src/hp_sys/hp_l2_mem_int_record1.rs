#[doc = "Register `HP_L2_MEM_INT_RECORD1` reader"]
pub type R = crate::R<HP_L2_MEM_INT_RECORD1_SPEC>;
#[doc = "Field `HP_REG_L2_MEM_ECC_ERR_INT_ADDR` reader - NA"]
pub type HP_REG_L2_MEM_ECC_ERR_INT_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `HP_REG_L2_MEM_ECC_ONE_BIT_ERR` reader - NA"]
pub type HP_REG_L2_MEM_ECC_ONE_BIT_ERR_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_MEM_ECC_TWO_BIT_ERR` reader - NA"]
pub type HP_REG_L2_MEM_ECC_TWO_BIT_ERR_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_MEM_ECC_ERR_BIT` reader - NA"]
pub type HP_REG_L2_MEM_ECC_ERR_BIT_R = crate::FieldReader<u16>;
#[doc = "Field `HP_REG_L2_CACHE_ERR_BANK` reader - NA"]
pub type HP_REG_L2_CACHE_ERR_BANK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_mem_ecc_err_int_addr(&self) -> HP_REG_L2_MEM_ECC_ERR_INT_ADDR_R {
        HP_REG_L2_MEM_ECC_ERR_INT_ADDR_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_mem_ecc_one_bit_err(&self) -> HP_REG_L2_MEM_ECC_ONE_BIT_ERR_R {
        HP_REG_L2_MEM_ECC_ONE_BIT_ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_mem_ecc_two_bit_err(&self) -> HP_REG_L2_MEM_ECC_TWO_BIT_ERR_R {
        HP_REG_L2_MEM_ECC_TWO_BIT_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_mem_ecc_err_bit(&self) -> HP_REG_L2_MEM_ECC_ERR_BIT_R {
        HP_REG_L2_MEM_ECC_ERR_BIT_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 26 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_cache_err_bank(&self) -> HP_REG_L2_CACHE_ERR_BANK_R {
        HP_REG_L2_CACHE_ERR_BANK_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_L2_MEM_INT_RECORD1")
            .field(
                "hp_reg_l2_mem_ecc_err_int_addr",
                &format_args!("{}", self.hp_reg_l2_mem_ecc_err_int_addr().bits()),
            )
            .field(
                "hp_reg_l2_mem_ecc_one_bit_err",
                &format_args!("{}", self.hp_reg_l2_mem_ecc_one_bit_err().bit()),
            )
            .field(
                "hp_reg_l2_mem_ecc_two_bit_err",
                &format_args!("{}", self.hp_reg_l2_mem_ecc_two_bit_err().bit()),
            )
            .field(
                "hp_reg_l2_mem_ecc_err_bit",
                &format_args!("{}", self.hp_reg_l2_mem_ecc_err_bit().bits()),
            )
            .field(
                "hp_reg_l2_cache_err_bank",
                &format_args!("{}", self.hp_reg_l2_cache_err_bank().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_L2_MEM_INT_RECORD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_int_record1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_L2_MEM_INT_RECORD1_SPEC;
impl crate::RegisterSpec for HP_L2_MEM_INT_RECORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_l2_mem_int_record1::R`](R) reader structure"]
impl crate::Readable for HP_L2_MEM_INT_RECORD1_SPEC {}
#[doc = "`reset()` method sets HP_L2_MEM_INT_RECORD1 to value 0"]
impl crate::Resettable for HP_L2_MEM_INT_RECORD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
