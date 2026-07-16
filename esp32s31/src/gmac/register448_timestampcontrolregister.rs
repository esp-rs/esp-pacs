#[doc = "Register `REGISTER448_TIMESTAMPCONTROLREGISTER` reader"]
pub type R = crate::R<REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC>;
#[doc = "Register `REGISTER448_TIMESTAMPCONTROLREGISTER` writer"]
pub type W = crate::W<REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC>;
#[doc = "Field `TSENA` reader - Timestamp Enable When set, the timestamp is added for the transmit and receive frames When disabled, timestamp is not added for the transmit and receive frames and the Timestamp Generator is also suspended You need to initialize the Timestamp _system time_ after enabling this mode On the receive side, the MAC processes the 1588 frames only if this bit is set"]
pub type TSENA_R = crate::BitReader;
#[doc = "Field `TSENA` writer - Timestamp Enable When set, the timestamp is added for the transmit and receive frames When disabled, timestamp is not added for the transmit and receive frames and the Timestamp Generator is also suspended You need to initialize the Timestamp _system time_ after enabling this mode On the receive side, the MAC processes the 1588 frames only if this bit is set"]
pub type TSENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCFUPDT` reader - Timestamp Fine or Coarse Update When set, this bit indicates that the system times update should be done using the fine update method When reset, it indicates the system timestamp update should be done using the Coarse method"]
pub type TSCFUPDT_R = crate::BitReader;
#[doc = "Field `TSCFUPDT` writer - Timestamp Fine or Coarse Update When set, this bit indicates that the system times update should be done using the fine update method When reset, it indicates the system timestamp update should be done using the Coarse method"]
pub type TSCFUPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSINIT` reader - Timestamp Initialize When set, the system time is initialized _overwritten_ with the value specified in the Register 452 _System Time Seconds Update Register_ and Register 453 _System Time Nanoseconds Update Register_ This bit should be read zero before updating it This bit is reset when the initialization is complete The “Timestamp Higher Word” register _if enabled during core configuration_ can only be initialized"]
pub type TSINIT_R = crate::BitReader;
#[doc = "Field `TSINIT` writer - Timestamp Initialize When set, the system time is initialized _overwritten_ with the value specified in the Register 452 _System Time Seconds Update Register_ and Register 453 _System Time Nanoseconds Update Register_ This bit should be read zero before updating it This bit is reset when the initialization is complete The “Timestamp Higher Word” register _if enabled during core configuration_ can only be initialized"]
pub type TSINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUPDT` reader - Timestamp Update When set, the system time is updated _added or subtracted_ with the value specified in Register 452 _System Time Seconds Update Register_ and Register 453 _System Time Nanoseconds Update Register_ This bit should be read zero before updating it This bit is reset when the update is completed in hardware The “Timestamp Higher Word” register _if enabled during core configuration_ is not updated"]
pub type TSUPDT_R = crate::BitReader;
#[doc = "Field `TSUPDT` writer - Timestamp Update When set, the system time is updated _added or subtracted_ with the value specified in Register 452 _System Time Seconds Update Register_ and Register 453 _System Time Nanoseconds Update Register_ This bit should be read zero before updating it This bit is reset when the update is completed in hardware The “Timestamp Higher Word” register _if enabled during core configuration_ is not updated"]
pub type TSUPDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTRIG` reader - Timestamp Interrupt Trigger Enable When set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register This bit is reset after the generation of the Timestamp Trigger Interrupt"]
pub type TSTRIG_R = crate::BitReader;
#[doc = "Field `TSTRIG` writer - Timestamp Interrupt Trigger Enable When set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register This bit is reset after the generation of the Timestamp Trigger Interrupt"]
pub type TSTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSADDREG` reader - Addend Reg Update When set, the content of the Timestamp Addend register is updated in the PTP block for fine correction This is cleared when the update is completed This register bit should be zero before setting it"]
pub type TSADDREG_R = crate::BitReader;
#[doc = "Field `TSADDREG` writer - Addend Reg Update When set, the content of the Timestamp Addend register is updated in the PTP block for fine correction This is cleared when the update is completed This register bit should be zero before setting it"]
pub type TSADDREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENALL` reader - Enable Timestamp for All Frames When set, the timestamp snapshot is enabled for all frames received by the MAC"]
pub type TSENALL_R = crate::BitReader;
#[doc = "Field `TSENALL` writer - Enable Timestamp for All Frames When set, the timestamp snapshot is enabled for all frames received by the MAC"]
pub type TSENALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCTRLSSR` reader - Timestamp Digital or Binary Rollover Control When set, the Timestamp Low register rolls over after 0x3B9A_C9FF value _that is, 1 nanosecond accuracy_ and increments the timestamp _High_ seconds When reset, the rollover value of subsecond register is 0x7FFF_FFFF The subsecond increment has to be programmed correctly depending on the PTP reference clock frequency and the value of this bit"]
pub type TSCTRLSSR_R = crate::BitReader;
#[doc = "Field `TSCTRLSSR` writer - Timestamp Digital or Binary Rollover Control When set, the Timestamp Low register rolls over after 0x3B9A_C9FF value _that is, 1 nanosecond accuracy_ and increments the timestamp _High_ seconds When reset, the rollover value of subsecond register is 0x7FFF_FFFF The subsecond increment has to be programmed correctly depending on the PTP reference clock frequency and the value of this bit"]
pub type TSCTRLSSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVER2ENA` reader - Enable PTP packet Processing for Version 2 Format When set, the PTP packets are processed using the 1588 version 2 format Otherwise, the PTP packets are processed using the version 1 format The IEEE 1588 Version 1 and Version 2 format are described in “PTP Processing and Control” on page 155"]
pub type TSVER2ENA_R = crate::BitReader;
#[doc = "Field `TSVER2ENA` writer - Enable PTP packet Processing for Version 2 Format When set, the PTP packets are processed using the 1588 version 2 format Otherwise, the PTP packets are processed using the version 1 format The IEEE 1588 Version 1 and Version 2 format are described in “PTP Processing and Control” on page 155"]
pub type TSVER2ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPENA` reader - Enable Processing of PTP over Ethernet Frames When set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet frames When this bit is clear, the MAC ignores the PTP over Ethernet packets"]
pub type TSIPENA_R = crate::BitReader;
#[doc = "Field `TSIPENA` writer - Enable Processing of PTP over Ethernet Frames When set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet frames When this bit is clear, the MAC ignores the PTP over Ethernet packets"]
pub type TSIPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV6ENA` reader - Enable Processing of PTP Frames Sent over IPv6UDP When set, the MAC receiver processes PTP packets encapsulated in UDP over IPv6 packets When this bit is clear, the MAC ignores the PTP transported over UDPIPv6 packets"]
pub type TSIPV6ENA_R = crate::BitReader;
#[doc = "Field `TSIPV6ENA` writer - Enable Processing of PTP Frames Sent over IPv6UDP When set, the MAC receiver processes PTP packets encapsulated in UDP over IPv6 packets When this bit is clear, the MAC ignores the PTP transported over UDPIPv6 packets"]
pub type TSIPV6ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV4ENA` reader - Enable Processing of PTP Frames Sent over IPv4UDP When set, the MAC receiver processes the PTP packets encapsulated in UDP over IPv4 packets When this bit is clear, the MAC ignores the PTP transported over UDPIPv4 packets This bit is set by default"]
pub type TSIPV4ENA_R = crate::BitReader;
#[doc = "Field `TSIPV4ENA` writer - Enable Processing of PTP Frames Sent over IPv4UDP When set, the MAC receiver processes the PTP packets encapsulated in UDP over IPv4 packets When this bit is clear, the MAC ignores the PTP transported over UDPIPv4 packets This bit is set by default"]
pub type TSIPV4ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEVNTENA` reader - Enable Timestamp Snapshot for Event Messages When set, the timestamp snapshot is taken only for event messages _SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp_ When reset, the snapshot is taken for all messages except Announce, Management, and Signaling For more information about the timestamp snapshots, see Table 670 on page 462"]
pub type TSEVNTENA_R = crate::BitReader;
#[doc = "Field `TSEVNTENA` writer - Enable Timestamp Snapshot for Event Messages When set, the timestamp snapshot is taken only for event messages _SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp_ When reset, the snapshot is taken for all messages except Announce, Management, and Signaling For more information about the timestamp snapshots, see Table 670 on page 462"]
pub type TSEVNTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSMSTRENA` reader - Enable Snapshot for Messages Relevant to Master When set, the snapshot is taken only for the messages relevant to the master node Otherwise, the snapshot is taken for the messages relevant to the slave node"]
pub type TSMSTRENA_R = crate::BitReader;
#[doc = "Field `TSMSTRENA` writer - Enable Snapshot for Messages Relevant to Master When set, the snapshot is taken only for the messages relevant to the master node Otherwise, the snapshot is taken for the messages relevant to the slave node"]
pub type TSMSTRENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAPTYPSEL` reader - Select PTP packets for Taking Snapshots These bits along with Bits 15 and 14 decide the set of PTP packet types for which snapshot needs to be taken The encoding is given in Table 670 on page 462"]
pub type SNAPTYPSEL_R = crate::FieldReader;
#[doc = "Field `SNAPTYPSEL` writer - Select PTP packets for Taking Snapshots These bits along with Bits 15 and 14 decide the set of PTP packet types for which snapshot needs to be taken The encoding is given in Table 670 on page 462"]
pub type SNAPTYPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSENMACADDR` reader - Enable MAC address for PTP Frame Filtering When set, the DA MAC address _that matches any MAC Address register_ is used to filter the PTP frames when PTP is directly sent over Ethernet"]
pub type TSENMACADDR_R = crate::BitReader;
#[doc = "Field `TSENMACADDR` writer - Enable MAC address for PTP Frame Filtering When set, the DA MAC address _that matches any MAC Address register_ is used to filter the PTP frames when PTP is directly sent over Ethernet"]
pub type TSENMACADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSFC` reader - Auxiliary Snapshot FIFO Clear When set, it resets the pointers of the Auxiliary Snapshot FIFO This bit is cleared when the pointers are reset and the FIFO is empty When this bit is high, auxiliary snapshots get stored in the FIFO This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration"]
pub type ATSFC_R = crate::BitReader;
#[doc = "Field `ATSFC` writer - Auxiliary Snapshot FIFO Clear When set, it resets the pointers of the Auxiliary Snapshot FIFO This bit is cleared when the pointers are reset and the FIFO is empty When this bit is high, auxiliary snapshots get stored in the FIFO This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration"]
pub type ATSFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN0` reader - Auxiliary Snapshot 0 Enable This field controls capturing the Auxiliary Snapshot Trigger 0 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[0\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration"]
pub type ATSEN0_R = crate::BitReader;
#[doc = "Field `ATSEN0` writer - Auxiliary Snapshot 0 Enable This field controls capturing the Auxiliary Snapshot Trigger 0 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[0\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration"]
pub type ATSEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN1` reader - Auxiliary Snapshot 1 Enable This field controls capturing the Auxiliary Snapshot Trigger 1 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[1\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than two"]
pub type ATSEN1_R = crate::BitReader;
#[doc = "Field `ATSEN1` writer - Auxiliary Snapshot 1 Enable This field controls capturing the Auxiliary Snapshot Trigger 1 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[1\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than two"]
pub type ATSEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN2` reader - Auxiliary Snapshot 2 Enable This field controls capturing the Auxiliary Snapshot Trigger 2 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[2\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than three"]
pub type ATSEN2_R = crate::BitReader;
#[doc = "Field `ATSEN2` writer - Auxiliary Snapshot 2 Enable This field controls capturing the Auxiliary Snapshot Trigger 2 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[2\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than three"]
pub type ATSEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN3` reader - Auxiliary Snapshot 3 Enable This field controls capturing the Auxiliary Snapshot Trigger 3 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[3\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than four"]
pub type ATSEN3_R = crate::BitReader;
#[doc = "Field `ATSEN3` writer - Auxiliary Snapshot 3 Enable This field controls capturing the Auxiliary Snapshot Trigger 3 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[3\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than four"]
pub type ATSEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timestamp Enable When set, the timestamp is added for the transmit and receive frames When disabled, timestamp is not added for the transmit and receive frames and the Timestamp Generator is also suspended You need to initialize the Timestamp _system time_ after enabling this mode On the receive side, the MAC processes the 1588 frames only if this bit is set"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update When set, this bit indicates that the system times update should be done using the fine update method When reset, it indicates the system timestamp update should be done using the Coarse method"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timestamp Initialize When set, the system time is initialized _overwritten_ with the value specified in the Register 452 _System Time Seconds Update Register_ and Register 453 _System Time Nanoseconds Update Register_ This bit should be read zero before updating it This bit is reset when the initialization is complete The “Timestamp Higher Word” register _if enabled during core configuration_ can only be initialized"]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Update When set, the system time is updated _added or subtracted_ with the value specified in Register 452 _System Time Seconds Update Register_ and Register 453 _System Time Nanoseconds Update Register_ This bit should be read zero before updating it This bit is reset when the update is completed in hardware The “Timestamp Higher Word” register _if enabled during core configuration_ is not updated"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable When set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register This bit is reset after the generation of the Timestamp Trigger Interrupt"]
    #[inline(always)]
    pub fn tstrig(&self) -> TSTRIG_R {
        TSTRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Addend Reg Update When set, the content of the Timestamp Addend register is updated in the PTP block for fine correction This is cleared when the update is completed This register bit should be zero before setting it"]
    #[inline(always)]
    pub fn tsaddreg(&self) -> TSADDREG_R {
        TSADDREG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames When set, the timestamp snapshot is enabled for all frames received by the MAC"]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control When set, the Timestamp Low register rolls over after 0x3B9A_C9FF value _that is, 1 nanosecond accuracy_ and increments the timestamp _High_ seconds When reset, the rollover value of subsecond register is 0x7FFF_FFFF The subsecond increment has to be programmed correctly depending on the PTP reference clock frequency and the value of this bit"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format When set, the PTP packets are processed using the 1588 version 2 format Otherwise, the PTP packets are processed using the version 1 format The IEEE 1588 Version 1 and Version 2 format are described in “PTP Processing and Control” on page 155"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames When set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet frames When this bit is clear, the MAC ignores the PTP over Ethernet packets"]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent over IPv6UDP When set, the MAC receiver processes PTP packets encapsulated in UDP over IPv6 packets When this bit is clear, the MAC ignores the PTP transported over UDPIPv6 packets"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4UDP When set, the MAC receiver processes the PTP packets encapsulated in UDP over IPv4 packets When this bit is clear, the MAC ignores the PTP transported over UDPIPv4 packets This bit is set by default"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages When set, the timestamp snapshot is taken only for event messages _SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp_ When reset, the snapshot is taken for all messages except Announce, Management, and Signaling For more information about the timestamp snapshots, see Table 670 on page 462"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TSEVNTENA_R {
        TSEVNTENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When set, the snapshot is taken only for the messages relevant to the master node Otherwise, the snapshot is taken for the messages relevant to the slave node"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots These bits along with Bits 15 and 14 decide the set of PTP packet types for which snapshot needs to be taken The encoding is given in Table 670 on page 462"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering When set, the DA MAC address _that matches any MAC Address register_ is used to filter the PTP frames when PTP is directly sent over Ethernet"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Auxiliary Snapshot FIFO Clear When set, it resets the pointers of the Auxiliary Snapshot FIFO This bit is cleared when the pointers are reset and the FIFO is empty When this bit is high, auxiliary snapshots get stored in the FIFO This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration"]
    #[inline(always)]
    pub fn atsfc(&self) -> ATSFC_R {
        ATSFC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Auxiliary Snapshot 0 Enable This field controls capturing the Auxiliary Snapshot Trigger 0 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[0\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration"]
    #[inline(always)]
    pub fn atsen0(&self) -> ATSEN0_R {
        ATSEN0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Auxiliary Snapshot 1 Enable This field controls capturing the Auxiliary Snapshot Trigger 1 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[1\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than two"]
    #[inline(always)]
    pub fn atsen1(&self) -> ATSEN1_R {
        ATSEN1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Auxiliary Snapshot 2 Enable This field controls capturing the Auxiliary Snapshot Trigger 2 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[2\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than three"]
    #[inline(always)]
    pub fn atsen2(&self) -> ATSEN2_R {
        ATSEN2_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Auxiliary Snapshot 3 Enable This field controls capturing the Auxiliary Snapshot Trigger 3 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[3\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than four"]
    #[inline(always)]
    pub fn atsen3(&self) -> ATSEN3_R {
        ATSEN3_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER448_TIMESTAMPCONTROLREGISTER")
            .field("tsena", &self.tsena())
            .field("tscfupdt", &self.tscfupdt())
            .field("tsinit", &self.tsinit())
            .field("tsupdt", &self.tsupdt())
            .field("tstrig", &self.tstrig())
            .field("tsaddreg", &self.tsaddreg())
            .field("tsenall", &self.tsenall())
            .field("tsctrlssr", &self.tsctrlssr())
            .field("tsver2ena", &self.tsver2ena())
            .field("tsipena", &self.tsipena())
            .field("tsipv6ena", &self.tsipv6ena())
            .field("tsipv4ena", &self.tsipv4ena())
            .field("tsevntena", &self.tsevntena())
            .field("tsmstrena", &self.tsmstrena())
            .field("snaptypsel", &self.snaptypsel())
            .field("tsenmacaddr", &self.tsenmacaddr())
            .field("atsfc", &self.atsfc())
            .field("atsen0", &self.atsen0())
            .field("atsen1", &self.atsen1())
            .field("atsen2", &self.atsen2())
            .field("atsen3", &self.atsen3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp Enable When set, the timestamp is added for the transmit and receive frames When disabled, timestamp is not added for the transmit and receive frames and the Timestamp Generator is also suspended You need to initialize the Timestamp _system time_ after enabling this mode On the receive side, the MAC processes the 1588 frames only if this bit is set"]
    #[inline(always)]
    pub fn tsena(&mut self) -> TSENA_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update When set, this bit indicates that the system times update should be done using the fine update method When reset, it indicates the system timestamp update should be done using the Coarse method"]
    #[inline(always)]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSCFUPDT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timestamp Initialize When set, the system time is initialized _overwritten_ with the value specified in the Register 452 _System Time Seconds Update Register_ and Register 453 _System Time Nanoseconds Update Register_ This bit should be read zero before updating it This bit is reset when the initialization is complete The “Timestamp Higher Word” register _if enabled during core configuration_ can only be initialized"]
    #[inline(always)]
    pub fn tsinit(&mut self) -> TSINIT_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSINIT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timestamp Update When set, the system time is updated _added or subtracted_ with the value specified in Register 452 _System Time Seconds Update Register_ and Register 453 _System Time Nanoseconds Update Register_ This bit should be read zero before updating it This bit is reset when the update is completed in hardware The “Timestamp Higher Word” register _if enabled during core configuration_ is not updated"]
    #[inline(always)]
    pub fn tsupdt(&mut self) -> TSUPDT_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSUPDT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable When set, the timestamp interrupt is generated when the System Time becomes greater than the value written in the Target Time register This bit is reset after the generation of the Timestamp Trigger Interrupt"]
    #[inline(always)]
    pub fn tstrig(&mut self) -> TSTRIG_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSTRIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Addend Reg Update When set, the content of the Timestamp Addend register is updated in the PTP block for fine correction This is cleared when the update is completed This register bit should be zero before setting it"]
    #[inline(always)]
    pub fn tsaddreg(&mut self) -> TSADDREG_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSADDREG_W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames When set, the timestamp snapshot is enabled for all frames received by the MAC"]
    #[inline(always)]
    pub fn tsenall(&mut self) -> TSENALL_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSENALL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control When set, the Timestamp Low register rolls over after 0x3B9A_C9FF value _that is, 1 nanosecond accuracy_ and increments the timestamp _High_ seconds When reset, the rollover value of subsecond register is 0x7FFF_FFFF The subsecond increment has to be programmed correctly depending on the PTP reference clock frequency and the value of this bit"]
    #[inline(always)]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSCTRLSSR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format When set, the PTP packets are processed using the 1588 version 2 format Otherwise, the PTP packets are processed using the version 1 format The IEEE 1588 Version 1 and Version 2 format are described in “PTP Processing and Control” on page 155"]
    #[inline(always)]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSVER2ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames When set, the MAC receiver processes the PTP packets encapsulated directly in the Ethernet frames When this bit is clear, the MAC ignores the PTP over Ethernet packets"]
    #[inline(always)]
    pub fn tsipena(&mut self) -> TSIPENA_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSIPENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent over IPv6UDP When set, the MAC receiver processes PTP packets encapsulated in UDP over IPv6 packets When this bit is clear, the MAC ignores the PTP transported over UDPIPv6 packets"]
    #[inline(always)]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSIPV6ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4UDP When set, the MAC receiver processes the PTP packets encapsulated in UDP over IPv4 packets When this bit is clear, the MAC ignores the PTP transported over UDPIPv4 packets This bit is set by default"]
    #[inline(always)]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSIPV4ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages When set, the timestamp snapshot is taken only for event messages _SYNC, Delay_Req, Pdelay_Req, or Pdelay_Resp_ When reset, the snapshot is taken for all messages except Announce, Management, and Signaling For more information about the timestamp snapshots, see Table 670 on page 462"]
    #[inline(always)]
    pub fn tsevntena(&mut self) -> TSEVNTENA_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSEVNTENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master When set, the snapshot is taken only for the messages relevant to the master node Otherwise, the snapshot is taken for the messages relevant to the slave node"]
    #[inline(always)]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSMSTRENA_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots These bits along with Bits 15 and 14 decide the set of PTP packet types for which snapshot needs to be taken The encoding is given in Table 670 on page 462"]
    #[inline(always)]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        SNAPTYPSEL_W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering When set, the DA MAC address _that matches any MAC Address register_ is used to filter the PTP frames when PTP is directly sent over Ethernet"]
    #[inline(always)]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        TSENMACADDR_W::new(self, 18)
    }
    #[doc = "Bit 24 - Auxiliary Snapshot FIFO Clear When set, it resets the pointers of the Auxiliary Snapshot FIFO This bit is cleared when the pointers are reset and the FIFO is empty When this bit is high, auxiliary snapshots get stored in the FIFO This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration"]
    #[inline(always)]
    pub fn atsfc(&mut self) -> ATSFC_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        ATSFC_W::new(self, 24)
    }
    #[doc = "Bit 25 - Auxiliary Snapshot 0 Enable This field controls capturing the Auxiliary Snapshot Trigger 0 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[0\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration"]
    #[inline(always)]
    pub fn atsen0(&mut self) -> ATSEN0_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        ATSEN0_W::new(self, 25)
    }
    #[doc = "Bit 26 - Auxiliary Snapshot 1 Enable This field controls capturing the Auxiliary Snapshot Trigger 1 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[1\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than two"]
    #[inline(always)]
    pub fn atsen1(&mut self) -> ATSEN1_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        ATSEN1_W::new(self, 26)
    }
    #[doc = "Bit 27 - Auxiliary Snapshot 2 Enable This field controls capturing the Auxiliary Snapshot Trigger 2 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[2\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than three"]
    #[inline(always)]
    pub fn atsen2(&mut self) -> ATSEN2_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        ATSEN2_W::new(self, 27)
    }
    #[doc = "Bit 28 - Auxiliary Snapshot 3 Enable This field controls capturing the Auxiliary Snapshot Trigger 3 When this bit is set, the Auxiliary snapshot of event on ptp_aux_trig_i\\[3\\] input is enabled When this bit is reset, the events on this input are ignored This bit is reserved when the Add IEEE 1588 Auxiliary Snapshot option is not selected during core configuration or the selected number in the Number of IEEE 1588 Auxiliary Snapshot Inputs option is less than four"]
    #[inline(always)]
    pub fn atsen3(&mut self) -> ATSEN3_W<'_, REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC> {
        ATSEN3_W::new(self, 28)
    }
}
#[doc = "Controls the timestamp generation and update logic This register is present only when IEEE1588 timestamping is enabled during coreConsultant configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`register448_timestampcontrolregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register448_timestampcontrolregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register448_timestampcontrolregister::R`](R) reader structure"]
impl crate::Readable for REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register448_timestampcontrolregister::W`](W) writer structure"]
impl crate::Writable for REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER448_TIMESTAMPCONTROLREGISTER to value 0x2000"]
impl crate::Resettable for REGISTER448_TIMESTAMPCONTROLREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
