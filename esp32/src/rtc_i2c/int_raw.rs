#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `SLAVE_TRANS_COMPLETE` reader - Slave accepted 1 byte and address matched"]
pub type SLAVE_TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `SLAVE_TRANS_COMPLETE` writer - Slave accepted 1 byte and address matched"]
pub type SLAVE_TRANS_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_LOST` reader - Master lost arbitration"]
pub type ARBITRATION_LOST_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` writer - Master lost arbitration"]
pub type ARBITRATION_LOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_TRANS_COMPLETE` reader - "]
pub type MASTER_TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `MASTER_TRANS_COMPLETE` writer - "]
pub type MASTER_TRANS_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMPLETE` reader - Stop condition has been detected interrupt raw status"]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` writer - Stop condition has been detected interrupt raw status"]
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_OUT` reader - time out interrupt raw status"]
pub type TIME_OUT_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Slave accepted 1 byte and address matched"]
    #[inline(always)]
    pub fn slave_trans_complete(&self) -> SLAVE_TRANS_COMPLETE_R {
        SLAVE_TRANS_COMPLETE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master lost arbitration"]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn master_trans_complete(&self) -> MASTER_TRANS_COMPLETE_R {
        MASTER_TRANS_COMPLETE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop condition has been detected interrupt raw status"]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - time out interrupt raw status"]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("slave_trans_complete", &self.slave_trans_complete())
            .field("arbitration_lost", &self.arbitration_lost())
            .field("master_trans_complete", &self.master_trans_complete())
            .field("trans_complete", &self.trans_complete())
            .field("time_out", &self.time_out())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Slave accepted 1 byte and address matched"]
    #[inline(always)]
    #[must_use]
    pub fn slave_trans_complete(&mut self) -> SLAVE_TRANS_COMPLETE_W<INT_RAW_SPEC> {
        SLAVE_TRANS_COMPLETE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Master lost arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost(&mut self) -> ARBITRATION_LOST_W<INT_RAW_SPEC> {
        ARBITRATION_LOST_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn master_trans_complete(&mut self) -> MASTER_TRANS_COMPLETE_W<INT_RAW_SPEC> {
        MASTER_TRANS_COMPLETE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stop condition has been detected interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<INT_RAW_SPEC> {
        TRANS_COMPLETE_W::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
