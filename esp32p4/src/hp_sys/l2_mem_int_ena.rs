#[doc = "Register `L2_MEM_INT_ENA` reader"]
pub type R = crate::R<L2_MEM_INT_ENA_SPEC>;
#[doc = "Register `L2_MEM_INT_ENA` writer"]
pub type W = crate::W<L2_MEM_INT_ENA_SPEC>;
#[doc = "Field `REG_L2_MEM_ECC_ERR_INT_ENA` reader - NA"]
pub type REG_L2_MEM_ECC_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_ECC_ERR_INT_ENA` writer - NA"]
pub type REG_L2_MEM_ECC_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_ENA` reader - NA"]
pub type REG_L2_MEM_EXCEED_ADDR_INT_ENA_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_ENA` writer - NA"]
pub type REG_L2_MEM_EXCEED_ADDR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_ERR_RESP_INT_ENA` reader - NA"]
pub type REG_L2_MEM_ERR_RESP_INT_ENA_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_ERR_RESP_INT_ENA` writer - NA"]
pub type REG_L2_MEM_ERR_RESP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_ena(&self) -> REG_L2_MEM_ECC_ERR_INT_ENA_R {
        REG_L2_MEM_ECC_ERR_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_ena(&self) -> REG_L2_MEM_EXCEED_ADDR_INT_ENA_R {
        REG_L2_MEM_EXCEED_ADDR_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_err_resp_int_ena(&self) -> REG_L2_MEM_ERR_RESP_INT_ENA_R {
        REG_L2_MEM_ERR_RESP_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_INT_ENA")
            .field(
                "reg_l2_mem_ecc_err_int_ena",
                &self.reg_l2_mem_ecc_err_int_ena(),
            )
            .field(
                "reg_l2_mem_exceed_addr_int_ena",
                &self.reg_l2_mem_exceed_addr_int_ena(),
            )
            .field(
                "reg_l2_mem_err_resp_int_ena",
                &self.reg_l2_mem_err_resp_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_ena(
        &mut self,
    ) -> REG_L2_MEM_ECC_ERR_INT_ENA_W<L2_MEM_INT_ENA_SPEC> {
        REG_L2_MEM_ECC_ERR_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_ena(
        &mut self,
    ) -> REG_L2_MEM_EXCEED_ADDR_INT_ENA_W<L2_MEM_INT_ENA_SPEC> {
        REG_L2_MEM_EXCEED_ADDR_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_err_resp_int_ena(
        &mut self,
    ) -> REG_L2_MEM_ERR_RESP_INT_ENA_W<L2_MEM_INT_ENA_SPEC> {
        REG_L2_MEM_ERR_RESP_INT_ENA_W::new(self, 2)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_INT_ENA_SPEC;
impl crate::RegisterSpec for L2_MEM_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_int_ena::R`](R) reader structure"]
impl crate::Readable for L2_MEM_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_int_ena::W`](W) writer structure"]
impl crate::Writable for L2_MEM_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_MEM_INT_ENA to value 0"]
impl crate::Resettable for L2_MEM_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
