#[doc = "Register `L2_MEM_INT_RECORD0` reader"]
pub type R = crate::R<L2_MEM_INT_RECORD0_SPEC>;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_ADDR` reader - NA"]
pub type REG_L2_MEM_EXCEED_ADDR_INT_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_WE` reader - NA"]
pub type REG_L2_MEM_EXCEED_ADDR_INT_WE_R = crate::BitReader;
#[doc = "Field `REG_L2_MEM_EXCEED_ADDR_INT_MASTER` reader - NA"]
pub type REG_L2_MEM_EXCEED_ADDR_INT_MASTER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:20 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_addr(&self) -> REG_L2_MEM_EXCEED_ADDR_INT_ADDR_R {
        REG_L2_MEM_EXCEED_ADDR_INT_ADDR_R::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bit 21 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_we(&self) -> REG_L2_MEM_EXCEED_ADDR_INT_WE_R {
        REG_L2_MEM_EXCEED_ADDR_INT_WE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:24 - NA"]
    #[inline(always)]
    pub fn reg_l2_mem_exceed_addr_int_master(&self) -> REG_L2_MEM_EXCEED_ADDR_INT_MASTER_R {
        REG_L2_MEM_EXCEED_ADDR_INT_MASTER_R::new(((self.bits >> 22) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_INT_RECORD0")
            .field(
                "reg_l2_mem_exceed_addr_int_addr",
                &format_args!("{}", self.reg_l2_mem_exceed_addr_int_addr().bits()),
            )
            .field(
                "reg_l2_mem_exceed_addr_int_we",
                &format_args!("{}", self.reg_l2_mem_exceed_addr_int_we().bit()),
            )
            .field(
                "reg_l2_mem_exceed_addr_int_master",
                &format_args!("{}", self.reg_l2_mem_exceed_addr_int_master().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_MEM_INT_RECORD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_int_record0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_INT_RECORD0_SPEC;
impl crate::RegisterSpec for L2_MEM_INT_RECORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_int_record0::R`](R) reader structure"]
impl crate::Readable for L2_MEM_INT_RECORD0_SPEC {}
#[doc = "`reset()` method sets L2_MEM_INT_RECORD0 to value 0"]
impl crate::Resettable for L2_MEM_INT_RECORD0_SPEC {
    const RESET_VALUE: u32 = 0;
}
