///Register `FSM` reader
pub type R = crate::R<FSM_SPEC>;
///Field `ST` reader - The status of SPI1 state machine. 0: idle state(IDLE), 1: preparation state(PREP), 2: send command state(CMD), 3: send address state(ADDR), 4: red data state(DIN), 5:write data state(DOUT), 6: wait state(DUMMY), 7: done state(DONE).
pub type ST_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - The status of SPI1 state machine. 0: idle state(IDLE), 1: preparation state(PREP), 2: send command state(CMD), 3: send address state(ADDR), 4: red data state(DIN), 5:write data state(DOUT), 6: wait state(DUMMY), 7: done state(DONE).
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
/**SPI1 state machine(FSM) status register.

You can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fsm::R`](R) reader structure
impl crate::Readable for FSM_SPEC {}
///`reset()` method sets FSM to value 0
impl crate::Resettable for FSM_SPEC {
    const RESET_VALUE: u32 = 0;
}
