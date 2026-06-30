#[doc = "Register `HP_PERI0_1` reader"]
pub type R = crate::R<HP_PERI0_1_SPEC>;
#[doc = "Field `HP_PERI0_PMS_EXCEPTION_ADDR` reader - Represents the access address (bit27~bit0) when hp_peri0 pms has been triggered.\\\\"]
pub type HP_PERI0_PMS_EXCEPTION_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `HP_PERI0_PMS_EXCEPTION_ADDR_CONST` reader - Represents the access address (bit31~bit28) when hp_peri0 pms has been triggered.\\\\"]
pub type HP_PERI0_PMS_EXCEPTION_ADDR_CONST_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:27 - Represents the access address (bit27~bit0) when hp_peri0 pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn hp_peri0_pms_exception_addr(&self) -> HP_PERI0_PMS_EXCEPTION_ADDR_R {
        HP_PERI0_PMS_EXCEPTION_ADDR_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - Represents the access address (bit31~bit28) when hp_peri0 pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn hp_peri0_pms_exception_addr_const(&self) -> HP_PERI0_PMS_EXCEPTION_ADDR_CONST_R {
        HP_PERI0_PMS_EXCEPTION_ADDR_CONST_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_PERI0_1")
            .field(
                "hp_peri0_pms_exception_addr",
                &self.hp_peri0_pms_exception_addr(),
            )
            .field(
                "hp_peri0_pms_exception_addr_const",
                &self.hp_peri0_pms_exception_addr_const(),
            )
            .finish()
    }
}
#[doc = "HP_PERI0 PMS exception addr record register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri0_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_PERI0_1_SPEC;
impl crate::RegisterSpec for HP_PERI0_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_peri0_1::R`](R) reader structure"]
impl crate::Readable for HP_PERI0_1_SPEC {}
#[doc = "`reset()` method sets HP_PERI0_1 to value 0x2000_0000"]
impl crate::Resettable for HP_PERI0_1_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
