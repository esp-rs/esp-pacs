#[doc = "Register `FSM` reader"]
pub type R = crate::R<FSM_SPEC>;
#[doc = "Field `ST` reader - "]
pub type ST_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM").field("st", &self.st()).finish()
    }
}
#[doc = "SPI Memory FSM Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm::R`](R) reader structure"]
impl crate::Readable for FSM_SPEC {}
#[doc = "`reset()` method sets FSM to value 0"]
impl crate::Resettable for FSM_SPEC {}
