#[doc = "Register `HP_L2_MEM_INT_ST` reader"]
pub type R = crate::R<HP_L2_MEM_INT_ST_SPEC>;
#[doc = "Field `HP_REG_L2_MEM_ECC_ERR_INT_ST` reader - NA"]
pub type HP_REG_L2_MEM_ECC_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_MEM_EXCEED_ADDR_INT_ST` reader - NA"]
pub type HP_REG_L2_MEM_EXCEED_ADDR_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_MEM_ERR_RESP_INT_ST` reader - NA"]
pub type HP_REG_L2_MEM_ERR_RESP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_mem_ecc_err_int_st(&self) -> HP_REG_L2_MEM_ECC_ERR_INT_ST_R {
        HP_REG_L2_MEM_ECC_ERR_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_mem_exceed_addr_int_st(&self) -> HP_REG_L2_MEM_EXCEED_ADDR_INT_ST_R {
        HP_REG_L2_MEM_EXCEED_ADDR_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn hp_reg_l2_mem_err_resp_int_st(&self) -> HP_REG_L2_MEM_ERR_RESP_INT_ST_R {
        HP_REG_L2_MEM_ERR_RESP_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_L2_MEM_INT_ST")
            .field(
                "hp_reg_l2_mem_ecc_err_int_st",
                &format_args!("{}", self.hp_reg_l2_mem_ecc_err_int_st().bit()),
            )
            .field(
                "hp_reg_l2_mem_exceed_addr_int_st",
                &format_args!("{}", self.hp_reg_l2_mem_exceed_addr_int_st().bit()),
            )
            .field(
                "hp_reg_l2_mem_err_resp_int_st",
                &format_args!("{}", self.hp_reg_l2_mem_err_resp_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_L2_MEM_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_L2_MEM_INT_ST_SPEC;
impl crate::RegisterSpec for HP_L2_MEM_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_l2_mem_int_st::R`](R) reader structure"]
impl crate::Readable for HP_L2_MEM_INT_ST_SPEC {}
#[doc = "`reset()` method sets HP_L2_MEM_INT_ST to value 0"]
impl crate::Resettable for HP_L2_MEM_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
