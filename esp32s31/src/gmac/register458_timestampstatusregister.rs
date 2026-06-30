#[doc = "Register `REGISTER458_TIMESTAMPSTATUSREGISTER` reader"]
pub type R = crate::R<REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC>;
#[doc = "Register `REGISTER458_TIMESTAMPSTATUSREGISTER` writer"]
pub type W = crate::W<REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC>;
#[doc = "Field `TSSOVF` reader - Timestamp Seconds Overflow When set, this bit indicates that the seconds value of the timestamp _when supporting version 2 format_ has overflowed beyond 32’hFFFF_FFFF"]
pub type TSSOVF_R = crate::BitReader;
#[doc = "Field `TSSOVF` writer - Timestamp Seconds Overflow When set, this bit indicates that the seconds value of the timestamp _when supporting version 2 format_ has overflowed beyond 32’hFFFF_FFFF"]
pub type TSSOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTARGT` reader - Timestamp Target Time Reached When set, this bit indicates that the value of system time is greater than or equal to the value specified in the Register 455 _Target Time Seconds Register_ and Register 456 _Target Time Nanoseconds Register_"]
pub type TSTARGT_R = crate::BitReader;
#[doc = "Field `TSTARGT` writer - Timestamp Target Time Reached When set, this bit indicates that the value of system time is greater than or equal to the value specified in the Register 455 _Target Time Seconds Register_ and Register 456 _Target Time Nanoseconds Register_"]
pub type TSTARGT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXTSTRIG` reader - Auxiliary Timestamp Trigger Snapshot This bit is set high when the auxiliary snapshot is written to the FIFO This bit is valid only if the Enable IEEE 1588 Auxiliary Snapshot feature is selected"]
pub type AUXTSTRIG_R = crate::BitReader;
#[doc = "Field `AUXTSTRIG` writer - Auxiliary Timestamp Trigger Snapshot This bit is set high when the auxiliary snapshot is written to the FIFO This bit is valid only if the Enable IEEE 1588 Auxiliary Snapshot feature is selected"]
pub type AUXTSTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTRGTERR` reader - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 455 and Register 456, is already elapsed This bit is cleared when read by the application"]
pub type TSTRGTERR_R = crate::BitReader;
#[doc = "Field `TSTRGTERR` writer - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 455 and Register 456, is already elapsed This bit is cleared when read by the application"]
pub type TSTRGTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTARGT1` reader - Timestamp Target Time Reached for Target Time PPS1 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 480 _PPS1 Target Time High Register_ and Register 481 _PPS1 Target Time Low Register_"]
pub type TSTARGT1_R = crate::BitReader;
#[doc = "Field `TSTARGT1` writer - Timestamp Target Time Reached for Target Time PPS1 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 480 _PPS1 Target Time High Register_ and Register 481 _PPS1 Target Time Low Register_"]
pub type TSTARGT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTRGTERR1` reader - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 480 and Register 481, is already elapsed This bit is cleared when read by the application"]
pub type TSTRGTERR1_R = crate::BitReader;
#[doc = "Field `TSTRGTERR1` writer - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 480 and Register 481, is already elapsed This bit is cleared when read by the application"]
pub type TSTRGTERR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTARGT2` reader - Timestamp Target Time Reached for Target Time PPS2 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 488 _PPS2 Target Time High Register_ and Register 489 _PPS2 Target Time Low Register_"]
pub type TSTARGT2_R = crate::BitReader;
#[doc = "Field `TSTARGT2` writer - Timestamp Target Time Reached for Target Time PPS2 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 488 _PPS2 Target Time High Register_ and Register 489 _PPS2 Target Time Low Register_"]
pub type TSTARGT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTRGTERR2` reader - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 488 and Register 489, is already elapsed This bit is cleared when read by the application"]
pub type TSTRGTERR2_R = crate::BitReader;
#[doc = "Field `TSTRGTERR2` writer - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 488 and Register 489, is already elapsed This bit is cleared when read by the application"]
pub type TSTRGTERR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTARGT3` reader - Timestamp Target Time Reached for Target Time PPS3 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 496 _PPS3 Target Time High Register_ and Register 497 _PPS3 Target Time Low Register_"]
pub type TSTARGT3_R = crate::BitReader;
#[doc = "Field `TSTARGT3` writer - Timestamp Target Time Reached for Target Time PPS3 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 496 _PPS3 Target Time High Register_ and Register 497 _PPS3 Target Time Low Register_"]
pub type TSTARGT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTRGTERR3` reader - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 496 and Register 497, is already elapsed This bit is cleared when read by the application"]
pub type TSTRGTERR3_R = crate::BitReader;
#[doc = "Field `TSTRGTERR3` writer - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 496 and Register 497, is already elapsed This bit is cleared when read by the application"]
pub type TSTRGTERR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSSTN` reader - Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock These bits are applicable only if the number of Auxiliary snapshots is more than one One bit is assigned for each trigger as shown in the following list: Bit 16: Auxiliary trigger 0 Bit 17: Auxiliary trigger 1 Bit 18: Auxiliary trigger 2 Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken"]
pub type ATSSTN_R = crate::FieldReader;
#[doc = "Field `ATSSTN` writer - Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock These bits are applicable only if the number of Auxiliary snapshots is more than one One bit is assigned for each trigger as shown in the following list: Bit 16: Auxiliary trigger 0 Bit 17: Auxiliary trigger 1 Bit 18: Auxiliary trigger 2 Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken"]
pub type ATSSTN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ATSSTM` reader - Auxiliary Timestamp Snapshot Trigger Missed"]
pub type ATSSTM_R = crate::BitReader;
#[doc = "Field `ATSNS` reader - Number of Auxiliary Timestamp Snapshots This field indicates the number of Snapshots available in the FIFO A value equal to the selected depth of FIFO _4, 8, or 16_ indicates that the Auxiliary Snapshot FIFO is full These bits are cleared _to 00000_ when the Auxiliary snapshot FIFO clear bit is set This bit is valid only if the Add IEEE 1588 Auxiliary Snapshot option is selected during core configuration"]
pub type ATSNS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Timestamp Seconds Overflow When set, this bit indicates that the seconds value of the timestamp _when supporting version 2 format_ has overflowed beyond 32’hFFFF_FFFF"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached When set, this bit indicates that the value of system time is greater than or equal to the value specified in the Register 455 _Target Time Seconds Register_ and Register 456 _Target Time Nanoseconds Register_"]
    #[inline(always)]
    pub fn tstargt(&self) -> TSTARGT_R {
        TSTARGT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auxiliary Timestamp Trigger Snapshot This bit is set high when the auxiliary snapshot is written to the FIFO This bit is valid only if the Enable IEEE 1588 Auxiliary Snapshot feature is selected"]
    #[inline(always)]
    pub fn auxtstrig(&self) -> AUXTSTRIG_R {
        AUXTSTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 455 and Register 456, is already elapsed This bit is cleared when read by the application"]
    #[inline(always)]
    pub fn tstrgterr(&self) -> TSTRGTERR_R {
        TSTRGTERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp Target Time Reached for Target Time PPS1 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 480 _PPS1 Target Time High Register_ and Register 481 _PPS1 Target Time Low Register_"]
    #[inline(always)]
    pub fn tstargt1(&self) -> TSTARGT1_R {
        TSTARGT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 480 and Register 481, is already elapsed This bit is cleared when read by the application"]
    #[inline(always)]
    pub fn tstrgterr1(&self) -> TSTRGTERR1_R {
        TSTRGTERR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timestamp Target Time Reached for Target Time PPS2 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 488 _PPS2 Target Time High Register_ and Register 489 _PPS2 Target Time Low Register_"]
    #[inline(always)]
    pub fn tstargt2(&self) -> TSTARGT2_R {
        TSTARGT2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 488 and Register 489, is already elapsed This bit is cleared when read by the application"]
    #[inline(always)]
    pub fn tstrgterr2(&self) -> TSTRGTERR2_R {
        TSTRGTERR2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timestamp Target Time Reached for Target Time PPS3 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 496 _PPS3 Target Time High Register_ and Register 497 _PPS3 Target Time Low Register_"]
    #[inline(always)]
    pub fn tstargt3(&self) -> TSTARGT3_R {
        TSTARGT3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 496 and Register 497, is already elapsed This bit is cleared when read by the application"]
    #[inline(always)]
    pub fn tstrgterr3(&self) -> TSTRGTERR3_R {
        TSTRGTERR3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock These bits are applicable only if the number of Auxiliary snapshots is more than one One bit is assigned for each trigger as shown in the following list: Bit 16: Auxiliary trigger 0 Bit 17: Auxiliary trigger 1 Bit 18: Auxiliary trigger 2 Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken"]
    #[inline(always)]
    pub fn atsstn(&self) -> ATSSTN_R {
        ATSSTN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Auxiliary Timestamp Snapshot Trigger Missed"]
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Number of Auxiliary Timestamp Snapshots This field indicates the number of Snapshots available in the FIFO A value equal to the selected depth of FIFO _4, 8, or 16_ indicates that the Auxiliary Snapshot FIFO is full These bits are cleared _to 00000_ when the Auxiliary snapshot FIFO clear bit is set This bit is valid only if the Add IEEE 1588 Auxiliary Snapshot option is selected during core configuration"]
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER458_TIMESTAMPSTATUSREGISTER")
            .field("tssovf", &self.tssovf())
            .field("tstargt", &self.tstargt())
            .field("auxtstrig", &self.auxtstrig())
            .field("tstrgterr", &self.tstrgterr())
            .field("tstargt1", &self.tstargt1())
            .field("tstrgterr1", &self.tstrgterr1())
            .field("tstargt2", &self.tstargt2())
            .field("tstrgterr2", &self.tstrgterr2())
            .field("tstargt3", &self.tstargt3())
            .field("tstrgterr3", &self.tstrgterr3())
            .field("atsstn", &self.atsstn())
            .field("atsstm", &self.atsstm())
            .field("atsns", &self.atsns())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Seconds Overflow When set, this bit indicates that the seconds value of the timestamp _when supporting version 2 format_ has overflowed beyond 32’hFFFF_FFFF"]
    #[inline(always)]
    pub fn tssovf(&mut self) -> TSSOVF_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        TSSOVF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached When set, this bit indicates that the value of system time is greater than or equal to the value specified in the Register 455 _Target Time Seconds Register_ and Register 456 _Target Time Nanoseconds Register_"]
    #[inline(always)]
    pub fn tstargt(&mut self) -> TSTARGT_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        TSTARGT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Auxiliary Timestamp Trigger Snapshot This bit is set high when the auxiliary snapshot is written to the FIFO This bit is valid only if the Enable IEEE 1588 Auxiliary Snapshot feature is selected"]
    #[inline(always)]
    pub fn auxtstrig(&mut self) -> AUXTSTRIG_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        AUXTSTRIG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 455 and Register 456, is already elapsed This bit is cleared when read by the application"]
    #[inline(always)]
    pub fn tstrgterr(&mut self) -> TSTRGTERR_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        TSTRGTERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timestamp Target Time Reached for Target Time PPS1 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 480 _PPS1 Target Time High Register_ and Register 481 _PPS1 Target Time Low Register_"]
    #[inline(always)]
    pub fn tstargt1(&mut self) -> TSTARGT1_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        TSTARGT1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 480 and Register 481, is already elapsed This bit is cleared when read by the application"]
    #[inline(always)]
    pub fn tstrgterr1(&mut self) -> TSTRGTERR1_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        TSTRGTERR1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Timestamp Target Time Reached for Target Time PPS2 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 488 _PPS2 Target Time High Register_ and Register 489 _PPS2 Target Time Low Register_"]
    #[inline(always)]
    pub fn tstargt2(&mut self) -> TSTARGT2_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        TSTARGT2_W::new(self, 6)
    }
    #[doc = "Bit 7 - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 488 and Register 489, is already elapsed This bit is cleared when read by the application"]
    #[inline(always)]
    pub fn tstrgterr2(&mut self) -> TSTRGTERR2_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        TSTRGTERR2_W::new(self, 7)
    }
    #[doc = "Bit 8 - Timestamp Target Time Reached for Target Time PPS3 When set, this bit indicates that the value of system time is greater than or equal to the value specified in Register 496 _PPS3 Target Time High Register_ and Register 497 _PPS3 Target Time Low Register_"]
    #[inline(always)]
    pub fn tstargt3(&mut self) -> TSTARGT3_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        TSTARGT3_W::new(self, 8)
    }
    #[doc = "Bit 9 - Timestamp Target Time Error This bit is set when the target time, being programmed in Register 496 and Register 497, is already elapsed This bit is cleared when read by the application"]
    #[inline(always)]
    pub fn tstrgterr3(&mut self) -> TSTRGTERR3_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        TSTRGTERR3_W::new(self, 9)
    }
    #[doc = "Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier These bits identify the Auxiliary trigger inputs for which the timestamp available in the Auxiliary Snapshot Register is applicable When more than one bit is set at the same time, it means that corresponding auxiliary triggers were sampled at the same clock These bits are applicable only if the number of Auxiliary snapshots is more than one One bit is assigned for each trigger as shown in the following list: Bit 16: Auxiliary trigger 0 Bit 17: Auxiliary trigger 1 Bit 18: Auxiliary trigger 2 Bit 19: Auxiliary trigger 3 The software can read this register to find the triggers that are set when the timestamp is taken"]
    #[inline(always)]
    pub fn atsstn(&mut self) -> ATSSTN_W<'_, REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC> {
        ATSSTN_W::new(self, 16)
    }
}
#[doc = "Contains the PTP status This register is available only when the advanced IEEE 1588 timestamp feature is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register458_timestampstatusregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register458_timestampstatusregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register458_timestampstatusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register458_timestampstatusregister::W`](W) writer structure"]
impl crate::Writable for REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER458_TIMESTAMPSTATUSREGISTER to value 0"]
impl crate::Resettable for REGISTER458_TIMESTAMPSTATUSREGISTER_SPEC {}
