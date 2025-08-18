#[doc = "Register `DEBUG_STATUS` reader"]
pub type R = crate::R<DEBUG_STATUS_SPEC>;
#[doc = "Register `DEBUG_STATUS` writer"]
pub type W = crate::W<DEBUG_STATUS_SPEC>;
#[doc = "Field `ACK_VAL` reader - The value of an acknowledge signal on the bus"]
pub type ACK_VAL_R = crate::BitReader;
#[doc = "Field `ACK_VAL` writer - The value of an acknowledge signal on the bus"]
pub type ACK_VAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_RW` reader - When working as a slave, the value of R/W bit received"]
pub type SLAVE_RW_R = crate::BitReader;
#[doc = "Field `SLAVE_RW` writer - When working as a slave, the value of R/W bit received"]
pub type SLAVE_RW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMED_OUT` reader - Transfer has timed out"]
pub type TIMED_OUT_R = crate::BitReader;
#[doc = "Field `TIMED_OUT` writer - Transfer has timed out"]
pub type TIMED_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_LOST` reader - When working as a master, lost control of I2C bus"]
pub type ARB_LOST_R = crate::BitReader;
#[doc = "Field `ARB_LOST` writer - When working as a master, lost control of I2C bus"]
pub type ARB_LOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_BUSY` reader - operation is in progress"]
pub type BUS_BUSY_R = crate::BitReader;
#[doc = "Field `BUS_BUSY` writer - operation is in progress"]
pub type BUS_BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_ADDR_MATCH` reader - When working as a slave, whether address was matched"]
pub type SLAVE_ADDR_MATCH_R = crate::BitReader;
#[doc = "Field `SLAVE_ADDR_MATCH` writer - When working as a slave, whether address was matched"]
pub type SLAVE_ADDR_MATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_TRANS` reader - 8 bit transmit done"]
pub type BYTE_TRANS_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS` writer - 8 bit transmit done"]
pub type BYTE_TRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_STATE` reader - state of the main state machine"]
pub type MAIN_STATE_R = crate::FieldReader;
#[doc = "Field `MAIN_STATE` writer - state of the main state machine"]
pub type MAIN_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCL_STATE` reader - state of SCL state machine"]
pub type SCL_STATE_R = crate::FieldReader;
#[doc = "Field `SCL_STATE` writer - state of SCL state machine"]
pub type SCL_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - The value of an acknowledge signal on the bus"]
    #[inline(always)]
    pub fn ack_val(&self) -> ACK_VAL_R {
        ACK_VAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When working as a slave, the value of R/W bit received"]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer has timed out"]
    #[inline(always)]
    pub fn timed_out(&self) -> TIMED_OUT_R {
        TIMED_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When working as a master, lost control of I2C bus"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - operation is in progress"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When working as a slave, whether address was matched"]
    #[inline(always)]
    pub fn slave_addr_match(&self) -> SLAVE_ADDR_MATCH_R {
        SLAVE_ADDR_MATCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8 bit transmit done"]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 25:27 - state of the main state machine"]
    #[inline(always)]
    pub fn main_state(&self) -> MAIN_STATE_R {
        MAIN_STATE_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - state of SCL state machine"]
    #[inline(always)]
    pub fn scl_state(&self) -> SCL_STATE_R {
        SCL_STATE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_STATUS")
            .field("ack_val", &self.ack_val())
            .field("slave_rw", &self.slave_rw())
            .field("timed_out", &self.timed_out())
            .field("arb_lost", &self.arb_lost())
            .field("bus_busy", &self.bus_busy())
            .field("slave_addr_match", &self.slave_addr_match())
            .field("byte_trans", &self.byte_trans())
            .field("main_state", &self.main_state())
            .field("scl_state", &self.scl_state())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The value of an acknowledge signal on the bus"]
    #[inline(always)]
    pub fn ack_val(&mut self) -> ACK_VAL_W<'_, DEBUG_STATUS_SPEC> {
        ACK_VAL_W::new(self, 0)
    }
    #[doc = "Bit 1 - When working as a slave, the value of R/W bit received"]
    #[inline(always)]
    pub fn slave_rw(&mut self) -> SLAVE_RW_W<'_, DEBUG_STATUS_SPEC> {
        SLAVE_RW_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer has timed out"]
    #[inline(always)]
    pub fn timed_out(&mut self) -> TIMED_OUT_W<'_, DEBUG_STATUS_SPEC> {
        TIMED_OUT_W::new(self, 2)
    }
    #[doc = "Bit 3 - When working as a master, lost control of I2C bus"]
    #[inline(always)]
    pub fn arb_lost(&mut self) -> ARB_LOST_W<'_, DEBUG_STATUS_SPEC> {
        ARB_LOST_W::new(self, 3)
    }
    #[doc = "Bit 4 - operation is in progress"]
    #[inline(always)]
    pub fn bus_busy(&mut self) -> BUS_BUSY_W<'_, DEBUG_STATUS_SPEC> {
        BUS_BUSY_W::new(self, 4)
    }
    #[doc = "Bit 5 - When working as a slave, whether address was matched"]
    #[inline(always)]
    pub fn slave_addr_match(&mut self) -> SLAVE_ADDR_MATCH_W<'_, DEBUG_STATUS_SPEC> {
        SLAVE_ADDR_MATCH_W::new(self, 5)
    }
    #[doc = "Bit 6 - 8 bit transmit done"]
    #[inline(always)]
    pub fn byte_trans(&mut self) -> BYTE_TRANS_W<'_, DEBUG_STATUS_SPEC> {
        BYTE_TRANS_W::new(self, 6)
    }
    #[doc = "Bits 25:27 - state of the main state machine"]
    #[inline(always)]
    pub fn main_state(&mut self) -> MAIN_STATE_W<'_, DEBUG_STATUS_SPEC> {
        MAIN_STATE_W::new(self, 25)
    }
    #[doc = "Bits 28:30 - state of SCL state machine"]
    #[inline(always)]
    pub fn scl_state(&mut self) -> SCL_STATE_W<'_, DEBUG_STATUS_SPEC> {
        SCL_STATE_W::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_STATUS_SPEC;
impl crate::RegisterSpec for DEBUG_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status::R`](R) reader structure"]
impl crate::Readable for DEBUG_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_status::W`](W) writer structure"]
impl crate::Writable for DEBUG_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_STATUS to value 0"]
impl crate::Resettable for DEBUG_STATUS_SPEC {}
