#[doc = "Register `HP_L2_MEM_INT_CLR` writer"]
pub type W = crate::W<HP_L2_MEM_INT_CLR_SPEC>;
#[doc = "Field `HP_REG_L2_MEM_ECC_ERR_INT_CLR` writer - NA"]
pub type HP_REG_L2_MEM_ECC_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_L2_MEM_EXCEED_ADDR_INT_CLR` writer - NA"]
pub type HP_REG_L2_MEM_EXCEED_ADDR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_L2_MEM_ERR_RESP_INT_CLR` writer - NA"]
pub type HP_REG_L2_MEM_ERR_RESP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_L2_MEM_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_mem_ecc_err_int_clr(
        &mut self,
    ) -> HP_REG_L2_MEM_ECC_ERR_INT_CLR_W<HP_L2_MEM_INT_CLR_SPEC> {
        HP_REG_L2_MEM_ECC_ERR_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_mem_exceed_addr_int_clr(
        &mut self,
    ) -> HP_REG_L2_MEM_EXCEED_ADDR_INT_CLR_W<HP_L2_MEM_INT_CLR_SPEC> {
        HP_REG_L2_MEM_EXCEED_ADDR_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_mem_err_resp_int_clr(
        &mut self,
    ) -> HP_REG_L2_MEM_ERR_RESP_INT_CLR_W<HP_L2_MEM_INT_CLR_SPEC> {
        HP_REG_L2_MEM_ERR_RESP_INT_CLR_W::new(self, 2)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_L2_MEM_INT_CLR_SPEC;
impl crate::RegisterSpec for HP_L2_MEM_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_l2_mem_int_clr::W`](W) writer structure"]
impl crate::Writable for HP_L2_MEM_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_L2_MEM_INT_CLR to value 0"]
impl crate::Resettable for HP_L2_MEM_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
