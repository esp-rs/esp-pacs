#[doc = "Register `L2_MEM_INT_CLR` writer"]
pub type W = crate::W<L2_MEM_INT_CLR_SPEC>;
#[doc = "Field `REG_L2_MEM_ECC_ERR_INT_CLR` writer - NA"]
pub type REG_L2_MEM_ECC_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_CLR` writer - NA"]
pub type REG_L2_MEM_EXCEED_ADDR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_ERR_RESP_INT_CLR` writer - NA"]
pub type REG_L2_MEM_ERR_RESP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_MEM_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_ecc_err_int_clr(
        &mut self,
    ) -> REG_L2_MEM_ECC_ERR_INT_CLR_W<L2_MEM_INT_CLR_SPEC> {
        REG_L2_MEM_ECC_ERR_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_exceed_addr_int_clr(
        &mut self,
    ) -> REG_L2_MEM_EXCEED_ADDR_INT_CLR_W<L2_MEM_INT_CLR_SPEC> {
        REG_L2_MEM_EXCEED_ADDR_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_err_resp_int_clr(
        &mut self,
    ) -> REG_L2_MEM_ERR_RESP_INT_CLR_W<L2_MEM_INT_CLR_SPEC> {
        REG_L2_MEM_ERR_RESP_INT_CLR_W::new(self, 2)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_INT_CLR_SPEC;
impl crate::RegisterSpec for L2_MEM_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l2_mem_int_clr::W`](W) writer structure"]
impl crate::Writable for L2_MEM_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_MEM_INT_CLR to value 0"]
impl crate::Resettable for L2_MEM_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
