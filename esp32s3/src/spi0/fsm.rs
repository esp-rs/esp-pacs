#[doc = "Register `FSM` reader"]
pub struct R(crate::R<FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ST` reader - The status of SPI0 state machine. 0: idle state(IDLE), 1: preparation state(PREP), 2: send command state(CMD), 3: send address state(ADDR), 4: red data state(DIN), 5:write data state(DOUT), 6: wait state(DUMMY), 7: done state(DONE)."]
pub type ST_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The status of SPI0 state machine. 0: idle state(IDLE), 1: preparation state(PREP), 2: send command state(CMD), 3: send address state(ADDR), 4: red data state(DIN), 5:write data state(DOUT), 6: wait state(DUMMY), 7: done state(DONE)."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM")
            .field("st", &format_args!("{}", self.st().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FSM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SPI0 state machine(FSM) status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm](index.html) module"]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm::R](R) reader structure"]
impl crate::Readable for FSM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSM to value 0"]
impl crate::Resettable for FSM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
