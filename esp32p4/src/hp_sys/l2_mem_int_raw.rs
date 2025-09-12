#[doc = "Register `L2_MEM_INT_RAW` reader"]
pub type R = crate::R<L2_MEM_INT_RAW_SPEC>;
#[doc = "Register `L2_MEM_INT_RAW` writer"]
pub type W = crate::W<L2_MEM_INT_RAW_SPEC>;
#[doc = "Field `REG_L2_MEM_ECC_ERR_INT_RAW` reader - intr triggered when two bit error detected and corrected from ecc"]
pub type REG_L2_MEM_ECC_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_ECC_ERR_INT_RAW` writer - intr triggered when two bit error detected and corrected from ecc"]
pub type REG_L2_MEM_ECC_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_RAW` reader - intr triggered when access addr exceeds 0xff9ffff at bypass mode or exceeds 0xff80000 at l2cache 128kb mode or exceeds 0xff60000 at l2cache 256kb mode"]
pub type REG_L2_MEM_EXCEED_ADDR_INT_RAW_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_RAW` writer - intr triggered when access addr exceeds 0xff9ffff at bypass mode or exceeds 0xff80000 at l2cache 128kb mode or exceeds 0xff60000 at l2cache 256kb mode"]
pub type REG_L2_MEM_EXCEED_ADDR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L2_MEM_ERR_RESP_INT_RAW` reader - intr triggered when err response occurs"]
pub type REG_L2_MEM_ERR_RESP_INT_RAW_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_ERR_RESP_INT_RAW` writer - intr triggered when err response occurs"]
pub type REG_L2_MEM_ERR_RESP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - intr triggered when two bit error detected and corrected from ecc"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_raw(&self) -> REG_L2_MEM_ECC_ERR_INT_RAW_R {
        REG_L2_MEM_ECC_ERR_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - intr triggered when access addr exceeds 0xff9ffff at bypass mode or exceeds 0xff80000 at l2cache 128kb mode or exceeds 0xff60000 at l2cache 256kb mode"]
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_raw(&self) -> REG_L2_MEM_EXCEED_ADDR_INT_RAW_R {
        REG_L2_MEM_EXCEED_ADDR_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - intr triggered when err response occurs"]
    #[inline(always)]
    pub fn reg_l2_mem_err_resp_int_raw(&self) -> REG_L2_MEM_ERR_RESP_INT_RAW_R {
        REG_L2_MEM_ERR_RESP_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_INT_RAW")
            .field(
                "reg_l2_mem_ecc_err_int_raw",
                &self.reg_l2_mem_ecc_err_int_raw(),
            )
            .field(
                "reg_l2_mem_exceed_addr_int_raw",
                &self.reg_l2_mem_exceed_addr_int_raw(),
            )
            .field(
                "reg_l2_mem_err_resp_int_raw",
                &self.reg_l2_mem_err_resp_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - intr triggered when two bit error detected and corrected from ecc"]
    #[inline(always)]
    pub fn reg_l2_mem_ecc_err_int_raw(
        &mut self,
    ) -> REG_L2_MEM_ECC_ERR_INT_RAW_W<'_, L2_MEM_INT_RAW_SPEC> {
        REG_L2_MEM_ECC_ERR_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - intr triggered when access addr exceeds 0xff9ffff at bypass mode or exceeds 0xff80000 at l2cache 128kb mode or exceeds 0xff60000 at l2cache 256kb mode"]
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_raw(
        &mut self,
    ) -> REG_L2_MEM_EXCEED_ADDR_INT_RAW_W<'_, L2_MEM_INT_RAW_SPEC> {
        REG_L2_MEM_EXCEED_ADDR_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - intr triggered when err response occurs"]
    #[inline(always)]
    pub fn reg_l2_mem_err_resp_int_raw(
        &mut self,
    ) -> REG_L2_MEM_ERR_RESP_INT_RAW_W<'_, L2_MEM_INT_RAW_SPEC> {
        REG_L2_MEM_ERR_RESP_INT_RAW_W::new(self, 2)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_mem_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_mem_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_INT_RAW_SPEC;
impl crate::RegisterSpec for L2_MEM_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_int_raw::R`](R) reader structure"]
impl crate::Readable for L2_MEM_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_int_raw::W`](W) writer structure"]
impl crate::Writable for L2_MEM_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_MEM_INT_RAW to value 0"]
impl crate::Resettable for L2_MEM_INT_RAW_SPEC {}
