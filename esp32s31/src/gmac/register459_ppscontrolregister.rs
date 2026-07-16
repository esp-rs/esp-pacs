#[doc = "Register `REGISTER459_PPSCONTROLREGISTER` reader"]
pub type R = crate::R<REGISTER459_PPSCONTROLREGISTER_SPEC>;
#[doc = "Register `REGISTER459_PPSCONTROLREGISTER` writer"]
pub type W = crate::W<REGISTER459_PPSCONTROLREGISTER_SPEC>;
#[doc = "Field `PPSCTRL0` reader - PPSCTRL0: PPS0 Output Frequency Control This field controls the frequency of the PPS0 output _ptp_pps_o\\[0\\]_ signal The default value of PPSCTRL is 0000, and the PPS output is 1 pulse _of width clk_ptp_i_ every second For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: 0001: The binary rollover is 2 Hz, and the digital rollover is 1 Hz 0010: The binary rollover is 4 Hz, and the digital rollover is 2 Hz 0011: The binary rollover is 8 Hz, and the digital rollover is 4 Hz 0100: The binary rollover is 16 Hz, and the digital rollover is 8 Hz 1111: The binary rollover is 32768 KHz, and the digital rollover is 16384 KHz Note: In the binary rollover mode, the PPS output _ptp_pps_o_ has a duty cycle of 50 percent with these frequencies In the digital rollover mode, the PPS output frequency is an average number The actual clock is of different frequency that gets synchronized every second For example: When PPSCTRL = 0001, the PPS _1 Hz_ has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS _2 Hz_ is a sequence of: One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period _268 ms low and 195 ms high_ When PPSCTRL = 0011, the PPS _4 Hz_ is a sequence of: Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period _134 ms low and 61 ms high_ This behavior is because of the nonlinear toggling of bits in the digital rollover mode in Register 451 _System Time Nanoseconds Register_ / PPSCMD0: Flexible PPS0 Output _ptp_pps_o\\[0\\]_ Control Programming these bits with a nonzero value instructs the MAC to initiate an event When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically The Software should ensure that these bits are programmed only when they are “allzero” The following list describes the values of PPSCMD0: 0000: No Command 0001: START Single Pulse This command generates single pulse rising at the start point defined in Target Time Registers _register 455 and 456_ and of a duration defined in the PPS0 Width Register 0010: START Pulse Train This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS0 Width Register and repeated at interval defined in the PPS Interval Register By default, the PPS pulse train is freerunning unless stopped by 'STOP Pulse train at time' or 'STOP Pulse Train immediately' commands 0011: Cancel START This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time 0100: STOP Pulse train at time This command stops the train of pulses initiated by the START Pulse Train command _PPSCMD = 0010_ after the time programmed in the Target Time registers elapses 0101: STOP Pulse Train immediately This command immediately stops the train of pulses initiated by the START Pulse Train command _PPSCMD = 0010_ 0110: Cancel STOP Pulse train This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed The PPS pulse train becomes freerunning on the successful execution of this command 01111111: Reserved"]
pub type PPSCTRL0_R = crate::FieldReader;
#[doc = "Field `PPSCTRL0` writer - PPSCTRL0: PPS0 Output Frequency Control This field controls the frequency of the PPS0 output _ptp_pps_o\\[0\\]_ signal The default value of PPSCTRL is 0000, and the PPS output is 1 pulse _of width clk_ptp_i_ every second For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: 0001: The binary rollover is 2 Hz, and the digital rollover is 1 Hz 0010: The binary rollover is 4 Hz, and the digital rollover is 2 Hz 0011: The binary rollover is 8 Hz, and the digital rollover is 4 Hz 0100: The binary rollover is 16 Hz, and the digital rollover is 8 Hz 1111: The binary rollover is 32768 KHz, and the digital rollover is 16384 KHz Note: In the binary rollover mode, the PPS output _ptp_pps_o_ has a duty cycle of 50 percent with these frequencies In the digital rollover mode, the PPS output frequency is an average number The actual clock is of different frequency that gets synchronized every second For example: When PPSCTRL = 0001, the PPS _1 Hz_ has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS _2 Hz_ is a sequence of: One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period _268 ms low and 195 ms high_ When PPSCTRL = 0011, the PPS _4 Hz_ is a sequence of: Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period _134 ms low and 61 ms high_ This behavior is because of the nonlinear toggling of bits in the digital rollover mode in Register 451 _System Time Nanoseconds Register_ / PPSCMD0: Flexible PPS0 Output _ptp_pps_o\\[0\\]_ Control Programming these bits with a nonzero value instructs the MAC to initiate an event When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically The Software should ensure that these bits are programmed only when they are “allzero” The following list describes the values of PPSCMD0: 0000: No Command 0001: START Single Pulse This command generates single pulse rising at the start point defined in Target Time Registers _register 455 and 456_ and of a duration defined in the PPS0 Width Register 0010: START Pulse Train This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS0 Width Register and repeated at interval defined in the PPS Interval Register By default, the PPS pulse train is freerunning unless stopped by 'STOP Pulse train at time' or 'STOP Pulse Train immediately' commands 0011: Cancel START This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time 0100: STOP Pulse train at time This command stops the train of pulses initiated by the START Pulse Train command _PPSCMD = 0010_ after the time programmed in the Target Time registers elapses 0101: STOP Pulse Train immediately This command immediately stops the train of pulses initiated by the START Pulse Train command _PPSCMD = 0010_ 0110: Cancel STOP Pulse train This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed The PPS pulse train becomes freerunning on the successful execution of this command 01111111: Reserved"]
pub type PPSCTRL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPSEN0` reader - Flexible PPS Output Mode Enable When set low, Bits \\[3:0\\] function as PPSCTRL _backward compatible_ When set high, Bits\\[3:0\\] function as PPSCMD"]
pub type PPSEN0_R = crate::BitReader;
#[doc = "Field `PPSEN0` writer - Flexible PPS Output Mode Enable When set low, Bits \\[3:0\\] function as PPSCTRL _backward compatible_ When set high, Bits\\[3:0\\] function as PPSCMD"]
pub type PPSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS0 Output This field indicates the Target Time registers _register 455 and 456_ mode for PPS0 output signal: 00: Indicates that the Target Time registers are programmed only for generating the interrupt event 01: Reserved 10: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the PPS0 output signal 11: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the PPS0 output signal No interrupt is asserted"]
pub type TRGTMODSEL0_R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL0` writer - Target Time Register Mode for PPS0 Output This field indicates the Target Time registers _register 455 and 456_ mode for PPS0 output signal: 00: Indicates that the Target Time registers are programmed only for generating the interrupt event 01: Reserved 10: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the PPS0 output signal 11: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the PPS0 output signal No interrupt is asserted"]
pub type TRGTMODSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PPSCMD1` reader - Flexible PPS1 Output Control This field controls the flexible PPS1 output _ptp_pps_o\\[1\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
pub type PPSCMD1_R = crate::FieldReader;
#[doc = "Field `PPSCMD1` writer - Flexible PPS1 Output Control This field controls the flexible PPS1 output _ptp_pps_o\\[1\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
pub type PPSCMD1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRGTMODSEL1` reader - Target Time Register Mode for PPS1 Output This field indicates the Target Time registers _register 480 and 481_ mode for PPS1 output signal This field is similar to the TRGTMODSEL0 field"]
pub type TRGTMODSEL1_R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL1` writer - Target Time Register Mode for PPS1 Output This field indicates the Target Time registers _register 480 and 481_ mode for PPS1 output signal This field is similar to the TRGTMODSEL0 field"]
pub type TRGTMODSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PPSCMD2` reader - Flexible PPS2 Output Control This field controls the flexible PPS2 output _ptp_pps_o\\[2\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
pub type PPSCMD2_R = crate::FieldReader;
#[doc = "Field `PPSCMD2` writer - Flexible PPS2 Output Control This field controls the flexible PPS2 output _ptp_pps_o\\[2\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
pub type PPSCMD2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRGTMODSEL2` reader - Target Time Register Mode for PPS2 Output This field indicates the Target Time registers _register 488 and 489_ mode for PPS2 output signal This field is similar to the TRGTMODSEL0 field"]
pub type TRGTMODSEL2_R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL2` writer - Target Time Register Mode for PPS2 Output This field indicates the Target Time registers _register 488 and 489_ mode for PPS2 output signal This field is similar to the TRGTMODSEL0 field"]
pub type TRGTMODSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PPSCMD3` reader - Flexible PPS3 Output Control This field controls the flexible PPS3 output _ptp_pps_o\\[3\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
pub type PPSCMD3_R = crate::FieldReader;
#[doc = "Field `PPSCMD3` writer - Flexible PPS3 Output Control This field controls the flexible PPS3 output _ptp_pps_o\\[3\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
pub type PPSCMD3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRGTMODSEL3` reader - Target Time Register Mode for PPS3 Output This field indicates the Target Time registers _register 496 and 497_ mode for PPS3 output signal This field is similar to the TRGTMODSEL0 field"]
pub type TRGTMODSEL3_R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL3` writer - Target Time Register Mode for PPS3 Output This field indicates the Target Time registers _register 496 and 497_ mode for PPS3 output signal This field is similar to the TRGTMODSEL0 field"]
pub type TRGTMODSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - PPSCTRL0: PPS0 Output Frequency Control This field controls the frequency of the PPS0 output _ptp_pps_o\\[0\\]_ signal The default value of PPSCTRL is 0000, and the PPS output is 1 pulse _of width clk_ptp_i_ every second For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: 0001: The binary rollover is 2 Hz, and the digital rollover is 1 Hz 0010: The binary rollover is 4 Hz, and the digital rollover is 2 Hz 0011: The binary rollover is 8 Hz, and the digital rollover is 4 Hz 0100: The binary rollover is 16 Hz, and the digital rollover is 8 Hz 1111: The binary rollover is 32768 KHz, and the digital rollover is 16384 KHz Note: In the binary rollover mode, the PPS output _ptp_pps_o_ has a duty cycle of 50 percent with these frequencies In the digital rollover mode, the PPS output frequency is an average number The actual clock is of different frequency that gets synchronized every second For example: When PPSCTRL = 0001, the PPS _1 Hz_ has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS _2 Hz_ is a sequence of: One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period _268 ms low and 195 ms high_ When PPSCTRL = 0011, the PPS _4 Hz_ is a sequence of: Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period _134 ms low and 61 ms high_ This behavior is because of the nonlinear toggling of bits in the digital rollover mode in Register 451 _System Time Nanoseconds Register_ / PPSCMD0: Flexible PPS0 Output _ptp_pps_o\\[0\\]_ Control Programming these bits with a nonzero value instructs the MAC to initiate an event When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically The Software should ensure that these bits are programmed only when they are “allzero” The following list describes the values of PPSCMD0: 0000: No Command 0001: START Single Pulse This command generates single pulse rising at the start point defined in Target Time Registers _register 455 and 456_ and of a duration defined in the PPS0 Width Register 0010: START Pulse Train This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS0 Width Register and repeated at interval defined in the PPS Interval Register By default, the PPS pulse train is freerunning unless stopped by 'STOP Pulse train at time' or 'STOP Pulse Train immediately' commands 0011: Cancel START This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time 0100: STOP Pulse train at time This command stops the train of pulses initiated by the START Pulse Train command _PPSCMD = 0010_ after the time programmed in the Target Time registers elapses 0101: STOP Pulse Train immediately This command immediately stops the train of pulses initiated by the START Pulse Train command _PPSCMD = 0010_ 0110: Cancel STOP Pulse train This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed The PPS pulse train becomes freerunning on the successful execution of this command 01111111: Reserved"]
    #[inline(always)]
    pub fn ppsctrl0(&self) -> PPSCTRL0_R {
        PPSCTRL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable When set low, Bits \\[3:0\\] function as PPSCTRL _backward compatible_ When set high, Bits\\[3:0\\] function as PPSCMD"]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output This field indicates the Target Time registers _register 455 and 456_ mode for PPS0 output signal: 00: Indicates that the Target Time registers are programmed only for generating the interrupt event 01: Reserved 10: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the PPS0 output signal 11: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the PPS0 output signal No interrupt is asserted"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Flexible PPS1 Output Control This field controls the flexible PPS1 output _ptp_pps_o\\[1\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
    #[inline(always)]
    pub fn ppscmd1(&self) -> PPSCMD1_R {
        PPSCMD1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 13:14 - Target Time Register Mode for PPS1 Output This field indicates the Target Time registers _register 480 and 481_ mode for PPS1 output signal This field is similar to the TRGTMODSEL0 field"]
    #[inline(always)]
    pub fn trgtmodsel1(&self) -> TRGTMODSEL1_R {
        TRGTMODSEL1_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Flexible PPS2 Output Control This field controls the flexible PPS2 output _ptp_pps_o\\[2\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
    #[inline(always)]
    pub fn ppscmd2(&self) -> PPSCMD2_R {
        PPSCMD2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Target Time Register Mode for PPS2 Output This field indicates the Target Time registers _register 488 and 489_ mode for PPS2 output signal This field is similar to the TRGTMODSEL0 field"]
    #[inline(always)]
    pub fn trgtmodsel2(&self) -> TRGTMODSEL2_R {
        TRGTMODSEL2_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Flexible PPS3 Output Control This field controls the flexible PPS3 output _ptp_pps_o\\[3\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
    #[inline(always)]
    pub fn ppscmd3(&self) -> PPSCMD3_R {
        PPSCMD3_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 29:30 - Target Time Register Mode for PPS3 Output This field indicates the Target Time registers _register 496 and 497_ mode for PPS3 output signal This field is similar to the TRGTMODSEL0 field"]
    #[inline(always)]
    pub fn trgtmodsel3(&self) -> TRGTMODSEL3_R {
        TRGTMODSEL3_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER459_PPSCONTROLREGISTER")
            .field("ppsctrl0", &self.ppsctrl0())
            .field("ppsen0", &self.ppsen0())
            .field("trgtmodsel0", &self.trgtmodsel0())
            .field("ppscmd1", &self.ppscmd1())
            .field("trgtmodsel1", &self.trgtmodsel1())
            .field("ppscmd2", &self.ppscmd2())
            .field("trgtmodsel2", &self.trgtmodsel2())
            .field("ppscmd3", &self.ppscmd3())
            .field("trgtmodsel3", &self.trgtmodsel3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - PPSCTRL0: PPS0 Output Frequency Control This field controls the frequency of the PPS0 output _ptp_pps_o\\[0\\]_ signal The default value of PPSCTRL is 0000, and the PPS output is 1 pulse _of width clk_ptp_i_ every second For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: 0001: The binary rollover is 2 Hz, and the digital rollover is 1 Hz 0010: The binary rollover is 4 Hz, and the digital rollover is 2 Hz 0011: The binary rollover is 8 Hz, and the digital rollover is 4 Hz 0100: The binary rollover is 16 Hz, and the digital rollover is 8 Hz 1111: The binary rollover is 32768 KHz, and the digital rollover is 16384 KHz Note: In the binary rollover mode, the PPS output _ptp_pps_o_ has a duty cycle of 50 percent with these frequencies In the digital rollover mode, the PPS output frequency is an average number The actual clock is of different frequency that gets synchronized every second For example: When PPSCTRL = 0001, the PPS _1 Hz_ has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS _2 Hz_ is a sequence of: One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period _268 ms low and 195 ms high_ When PPSCTRL = 0011, the PPS _4 Hz_ is a sequence of: Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period _134 ms low and 61 ms high_ This behavior is because of the nonlinear toggling of bits in the digital rollover mode in Register 451 _System Time Nanoseconds Register_ / PPSCMD0: Flexible PPS0 Output _ptp_pps_o\\[0\\]_ Control Programming these bits with a nonzero value instructs the MAC to initiate an event When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically The Software should ensure that these bits are programmed only when they are “allzero” The following list describes the values of PPSCMD0: 0000: No Command 0001: START Single Pulse This command generates single pulse rising at the start point defined in Target Time Registers _register 455 and 456_ and of a duration defined in the PPS0 Width Register 0010: START Pulse Train This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS0 Width Register and repeated at interval defined in the PPS Interval Register By default, the PPS pulse train is freerunning unless stopped by 'STOP Pulse train at time' or 'STOP Pulse Train immediately' commands 0011: Cancel START This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time 0100: STOP Pulse train at time This command stops the train of pulses initiated by the START Pulse Train command _PPSCMD = 0010_ after the time programmed in the Target Time registers elapses 0101: STOP Pulse Train immediately This command immediately stops the train of pulses initiated by the START Pulse Train command _PPSCMD = 0010_ 0110: Cancel STOP Pulse train This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed The PPS pulse train becomes freerunning on the successful execution of this command 01111111: Reserved"]
    #[inline(always)]
    pub fn ppsctrl0(&mut self) -> PPSCTRL0_W<'_, REGISTER459_PPSCONTROLREGISTER_SPEC> {
        PPSCTRL0_W::new(self, 0)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable When set low, Bits \\[3:0\\] function as PPSCTRL _backward compatible_ When set high, Bits\\[3:0\\] function as PPSCMD"]
    #[inline(always)]
    pub fn ppsen0(&mut self) -> PPSEN0_W<'_, REGISTER459_PPSCONTROLREGISTER_SPEC> {
        PPSEN0_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output This field indicates the Target Time registers _register 455 and 456_ mode for PPS0 output signal: 00: Indicates that the Target Time registers are programmed only for generating the interrupt event 01: Reserved 10: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the PPS0 output signal 11: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the PPS0 output signal No interrupt is asserted"]
    #[inline(always)]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<'_, REGISTER459_PPSCONTROLREGISTER_SPEC> {
        TRGTMODSEL0_W::new(self, 5)
    }
    #[doc = "Bits 8:10 - Flexible PPS1 Output Control This field controls the flexible PPS1 output _ptp_pps_o\\[1\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
    #[inline(always)]
    pub fn ppscmd1(&mut self) -> PPSCMD1_W<'_, REGISTER459_PPSCONTROLREGISTER_SPEC> {
        PPSCMD1_W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Target Time Register Mode for PPS1 Output This field indicates the Target Time registers _register 480 and 481_ mode for PPS1 output signal This field is similar to the TRGTMODSEL0 field"]
    #[inline(always)]
    pub fn trgtmodsel1(&mut self) -> TRGTMODSEL1_W<'_, REGISTER459_PPSCONTROLREGISTER_SPEC> {
        TRGTMODSEL1_W::new(self, 13)
    }
    #[doc = "Bits 16:18 - Flexible PPS2 Output Control This field controls the flexible PPS2 output _ptp_pps_o\\[2\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
    #[inline(always)]
    pub fn ppscmd2(&mut self) -> PPSCMD2_W<'_, REGISTER459_PPSCONTROLREGISTER_SPEC> {
        PPSCMD2_W::new(self, 16)
    }
    #[doc = "Bits 21:22 - Target Time Register Mode for PPS2 Output This field indicates the Target Time registers _register 488 and 489_ mode for PPS2 output signal This field is similar to the TRGTMODSEL0 field"]
    #[inline(always)]
    pub fn trgtmodsel2(&mut self) -> TRGTMODSEL2_W<'_, REGISTER459_PPSCONTROLREGISTER_SPEC> {
        TRGTMODSEL2_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Flexible PPS3 Output Control This field controls the flexible PPS3 output _ptp_pps_o\\[3\\]_ signal This field is similar to PPSCMD0\\[2:0\\] in functionality"]
    #[inline(always)]
    pub fn ppscmd3(&mut self) -> PPSCMD3_W<'_, REGISTER459_PPSCONTROLREGISTER_SPEC> {
        PPSCMD3_W::new(self, 24)
    }
    #[doc = "Bits 29:30 - Target Time Register Mode for PPS3 Output This field indicates the Target Time registers _register 496 and 497_ mode for PPS3 output signal This field is similar to the TRGTMODSEL0 field"]
    #[inline(always)]
    pub fn trgtmodsel3(&mut self) -> TRGTMODSEL3_W<'_, REGISTER459_PPSCONTROLREGISTER_SPEC> {
        TRGTMODSEL3_W::new(self, 29)
    }
}
#[doc = "This register is used to control the interval of the PPS signal output This register is available only when the advanced IEEE 1588 timestamp feature is selected\n\nYou can [`read`](crate::Reg::read) this register and get [`register459_ppscontrolregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register459_ppscontrolregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER459_PPSCONTROLREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER459_PPSCONTROLREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register459_ppscontrolregister::R`](R) reader structure"]
impl crate::Readable for REGISTER459_PPSCONTROLREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register459_ppscontrolregister::W`](W) writer structure"]
impl crate::Writable for REGISTER459_PPSCONTROLREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER459_PPSCONTROLREGISTER to value 0"]
impl crate::Resettable for REGISTER459_PPSCONTROLREGISTER_SPEC {}
