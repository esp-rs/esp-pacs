#[doc = "Register `LP_PERI1_1` reader"]
pub type R = crate::R<LP_PERI1_1_SPEC>;
#[doc = "Field `LP_PERI1_PMS_EXCEPTION_ADDR` reader - Represents the access address (bit27~bit0) when lp_peri1 pms has been triggered.\\\\"]
pub type LP_PERI1_PMS_EXCEPTION_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `LP_PERI1_PMS_EXCEPTION_ADDR_CONST` reader - Represents the access address (bit31~bit28) when lp_peri1 pms has been triggered.\\\\"]
pub type LP_PERI1_PMS_EXCEPTION_ADDR_CONST_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - Represents the access address (bit27~bit0) when lp_peri1 pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn lp_peri1_pms_exception_addr(&self) -> LP_PERI1_PMS_EXCEPTION_ADDR_R {
        LP_PERI1_PMS_EXCEPTION_ADDR_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Represents the access address (bit31~bit28) when lp_peri1 pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn lp_peri1_pms_exception_addr_const(&self) -> LP_PERI1_PMS_EXCEPTION_ADDR_CONST_R {
        LP_PERI1_PMS_EXCEPTION_ADDR_CONST_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PERI1_1")
            .field(
                "lp_peri1_pms_exception_addr",
                &self.lp_peri1_pms_exception_addr(),
            )
            .field(
                "lp_peri1_pms_exception_addr_const",
                &self.lp_peri1_pms_exception_addr_const(),
            )
            .finish()
    }
}
#[doc = "LP_PERI1 PMS exception addr record register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri1_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_PERI1_1_SPEC;
impl crate::RegisterSpec for LP_PERI1_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_peri1_1::R`](R) reader structure"]
impl crate::Readable for LP_PERI1_1_SPEC {}
#[doc = "`reset()` method sets LP_PERI1_1 to value 0x2000_0000"]
impl crate::Resettable for LP_PERI1_1_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
