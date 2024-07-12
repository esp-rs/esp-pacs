#[doc = "Register `L2_MEM_INT_ST` reader"]
pub type R = crate::R<L2_MEM_INT_ST_SPEC>;
#[doc = "Field `REG_L2_MEM_ECC_ERR_INT_ST` reader - NA"]
pub type REG_L2_MEM_ECC_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_ST` reader - NA"]
pub type REG_L2_MEM_EXCEED_ADDR_INT_ST_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_ERR_RESP_INT_ST` reader - NA"]
pub type REG_L2_MEM_ERR_RESP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_st(&self) -> REG_L2_MEM_ECC_ERR_INT_ST_R {
        REG_L2_MEM_ECC_ERR_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_st(&self) -> REG_L2_MEM_EXCEED_ADDR_INT_ST_R {
        REG_L2_MEM_EXCEED_ADDR_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_err_resp_int_st(&self) -> REG_L2_MEM_ERR_RESP_INT_ST_R {
        REG_L2_MEM_ERR_RESP_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_INT_ST")
            .field(
                "reg_l2_mem_ecc_err_int_st",
                &self.reg_l2_mem_ecc_err_int_st(),
            )
            .field(
                "reg_l2_mem_exceed_addr_int_st",
                &self.reg_l2_mem_exceed_addr_int_st(),
            )
            .field(
                "reg_l2_mem_err_resp_int_st",
                &self.reg_l2_mem_err_resp_int_st(),
            )
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_INT_ST_SPEC;
impl crate::RegisterSpec for L2_MEM_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_int_st::R`](R) reader structure"]
impl crate::Readable for L2_MEM_INT_ST_SPEC {}
#[doc = "`reset()` method sets L2_MEM_INT_ST to value 0"]
impl crate::Resettable for L2_MEM_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
