#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `SLAVE_TRANS_COMPLETE_INT_RAW` reader - Slave accepted 1 byte and address matched"]
pub type SLAVE_TRANS_COMPLETE_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLAVE_TRANS_COMPLETE_INT_RAW` writer - Slave accepted 1 byte and address matched"]
pub type SLAVE_TRANS_COMPLETE_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARBITRATION_LOST_INT_RAW` reader - Master lost arbitration"]
pub type ARBITRATION_LOST_INT_RAW_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_RAW` writer - Master lost arbitration"]
pub type ARBITRATION_LOST_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASTER_TRANS_COMPLETE_INT_RAW` reader - "]
pub type MASTER_TRANS_COMPLETE_INT_RAW_R = crate::BitReader;
#[doc = "Field `MASTER_TRANS_COMPLETE_INT_RAW` writer - "]
pub type MASTER_TRANS_COMPLETE_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRANS_COMPLETE_INT_RAW` reader - Stop condition has been detected interrupt raw status"]
pub type TRANS_COMPLETE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE_INT_RAW` writer - Stop condition has been detected interrupt raw status"]
pub type TRANS_COMPLETE_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIME_OUT_INT_RAW` reader - time out interrupt raw status"]
pub type TIME_OUT_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Slave accepted 1 byte and address matched"]
    #[inline(always)]
    pub fn slave_trans_complete_int_raw(&self) -> SLAVE_TRANS_COMPLETE_INT_RAW_R {
        SLAVE_TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master lost arbitration"]
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&self) -> ARBITRATION_LOST_INT_RAW_R {
        ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn master_trans_complete_int_raw(&self) -> MASTER_TRANS_COMPLETE_INT_RAW_R {
        MASTER_TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop condition has been detected interrupt raw status"]
    #[inline(always)]
    pub fn trans_complete_int_raw(&self) -> TRANS_COMPLETE_INT_RAW_R {
        TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - time out interrupt raw status"]
    #[inline(always)]
    pub fn time_out_int_raw(&self) -> TIME_OUT_INT_RAW_R {
        TIME_OUT_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "slave_trans_complete_int_raw",
                &format_args!("{}", self.slave_trans_complete_int_raw().bit()),
            )
            .field(
                "arbitration_lost_int_raw",
                &format_args!("{}", self.arbitration_lost_int_raw().bit()),
            )
            .field(
                "master_trans_complete_int_raw",
                &format_args!("{}", self.master_trans_complete_int_raw().bit()),
            )
            .field(
                "trans_complete_int_raw",
                &format_args!("{}", self.trans_complete_int_raw().bit()),
            )
            .field(
                "time_out_int_raw",
                &format_args!("{}", self.time_out_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - Slave accepted 1 byte and address matched"]
    #[inline(always)]
    #[must_use]
    pub fn slave_trans_complete_int_raw(
        &mut self,
    ) -> SLAVE_TRANS_COMPLETE_INT_RAW_W<INT_RAW_SPEC, 3> {
        SLAVE_TRANS_COMPLETE_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - Master lost arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_raw(&mut self) -> ARBITRATION_LOST_INT_RAW_W<INT_RAW_SPEC, 4> {
        ARBITRATION_LOST_INT_RAW_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn master_trans_complete_int_raw(
        &mut self,
    ) -> MASTER_TRANS_COMPLETE_INT_RAW_W<INT_RAW_SPEC, 5> {
        MASTER_TRANS_COMPLETE_INT_RAW_W::new(self)
    }
    #[doc = "Bit 6 - Stop condition has been detected interrupt raw status"]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete_int_raw(&mut self) -> TRANS_COMPLETE_INT_RAW_W<INT_RAW_SPEC, 6> {
        TRANS_COMPLETE_INT_RAW_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
