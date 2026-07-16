#[doc = "Register `REGISTER462_AVMACCONTROLREGISTER` reader"]
pub type R = crate::R<REGISTER462_AVMACCONTROLREGISTER_SPEC>;
#[doc = "Register `REGISTER462_AVMACCONTROLREGISTER` writer"]
pub type W = crate::W<REGISTER462_AVMACCONTROLREGISTER_SPEC>;
#[doc = "Field `AVT` reader - AV EtherType Value This field contains the value that is compared with the EtherType field of the incoming _tagged or untagged_ Ethernet frame to detect an AV packet"]
pub type AVT_R = crate::FieldReader<u16>;
#[doc = "Field `AVT` writer - AV EtherType Value This field contains the value that is compared with the EtherType field of the incoming _tagged or untagged_ Ethernet frame to detect an AV packet"]
pub type AVT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AVP` reader - AV Priority for Queuing The value programmed in these bits control the receive channel _0, 1, or 2_ to which an AV packet with a given priority must be queued If only Channel 1 receive path is enabled, the AV packets with priority value greater than or equal to the programmed value are queued on Channel 1 and all other packets are queued on Channel 0 If Channel 2 receive path is also enabled, the AV packets with priority value greater than or equal to the programmed value are queued on Channel 2 The AV packets with value less than the programmed value on Channel 1 and all other packets are queued on Channel 0 These bits are applicable only if at least one additional receive channel is selected in the AV mode"]
pub type AVP_R = crate::FieldReader;
#[doc = "Field `AVP` writer - AV Priority for Queuing The value programmed in these bits control the receive channel _0, 1, or 2_ to which an AV packet with a given priority must be queued If only Channel 1 receive path is enabled, the AV packets with priority value greater than or equal to the programmed value are queued on Channel 1 and all other packets are queued on Channel 0 If Channel 2 receive path is also enabled, the AV packets with priority value greater than or equal to the programmed value are queued on Channel 2 The AV packets with value less than the programmed value on Channel 1 and all other packets are queued on Channel 0 These bits are applicable only if at least one additional receive channel is selected in the AV mode"]
pub type AVP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VQE` reader - VLAN Tagged NonAV Packets Queueing Enable When this bit is set, the MAC also queues nonAV VLAN tagged packets into the available channels according to the value of the AVP bits This bit is reserved and readonly if Channel 1 and Channel 2 Receive paths are not selected during core configuration"]
pub type VQE_R = crate::BitReader;
#[doc = "Field `VQE` writer - VLAN Tagged NonAV Packets Queueing Enable When this bit is set, the MAC also queues nonAV VLAN tagged packets into the available channels according to the value of the AVP bits This bit is reserved and readonly if Channel 1 and Channel 2 Receive paths are not selected during core configuration"]
pub type VQE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVCD` reader - AV Channel Disable When this bit is set, the MAC forwards all packets to the default Channel 0 and the values programmed in the AVP, AVCH, and PTPCH fields are ignored This bit is reserved and readonly if Channel 1 or Channel 2 receive paths are not selected during core configuration"]
pub type AVCD_R = crate::BitReader;
#[doc = "Field `AVCD` writer - AV Channel Disable When this bit is set, the MAC forwards all packets to the default Channel 0 and the values programmed in the AVP, AVCH, and PTPCH fields are ignored This bit is reserved and readonly if Channel 1 or Channel 2 receive paths are not selected during core configuration"]
pub type AVCD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVCH` reader - Channel for Queuing the AV Control Packets This field specifies the channel on which the received untagged AV control packets are queued 00: Channel 0 01: Channel 1 10: Channel 2 11: Reserved These bits are reserved if the receive paths of Channel 1 or Channel 2 are not enabled"]
pub type AVCH_R = crate::FieldReader;
#[doc = "Field `AVCH` writer - Channel for Queuing the AV Control Packets This field specifies the channel on which the received untagged AV control packets are queued 00: Channel 0 01: Channel 1 10: Channel 2 11: Reserved These bits are reserved if the receive paths of Channel 1 or Channel 2 are not enabled"]
pub type AVCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PTPCH` reader - Channel for Queuing the PTP Packets This field specifies the channel on which the untagged PTP packets, sent over the Ethernet payload and not over IPv4 or IPv6, are queued 00: Channel 0 01: Channel 1 10: Channel 2 11: Reserved These bits are reserved if the receive paths of Channel 1 or Channel 2 are not enabled"]
pub type PTPCH_R = crate::FieldReader;
#[doc = "Field `PTPCH` writer - Channel for Queuing the PTP Packets This field specifies the channel on which the untagged PTP packets, sent over the Ethernet payload and not over IPv4 or IPv6, are queued 00: Channel 0 01: Channel 1 10: Channel 2 11: Reserved These bits are reserved if the receive paths of Channel 1 or Channel 2 are not enabled"]
pub type PTPCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - AV EtherType Value This field contains the value that is compared with the EtherType field of the incoming _tagged or untagged_ Ethernet frame to detect an AV packet"]
    #[inline(always)]
    pub fn avt(&self) -> AVT_R {
        AVT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - AV Priority for Queuing The value programmed in these bits control the receive channel _0, 1, or 2_ to which an AV packet with a given priority must be queued If only Channel 1 receive path is enabled, the AV packets with priority value greater than or equal to the programmed value are queued on Channel 1 and all other packets are queued on Channel 0 If Channel 2 receive path is also enabled, the AV packets with priority value greater than or equal to the programmed value are queued on Channel 2 The AV packets with value less than the programmed value on Channel 1 and all other packets are queued on Channel 0 These bits are applicable only if at least one additional receive channel is selected in the AV mode"]
    #[inline(always)]
    pub fn avp(&self) -> AVP_R {
        AVP_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - VLAN Tagged NonAV Packets Queueing Enable When this bit is set, the MAC also queues nonAV VLAN tagged packets into the available channels according to the value of the AVP bits This bit is reserved and readonly if Channel 1 and Channel 2 Receive paths are not selected during core configuration"]
    #[inline(always)]
    pub fn vqe(&self) -> VQE_R {
        VQE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AV Channel Disable When this bit is set, the MAC forwards all packets to the default Channel 0 and the values programmed in the AVP, AVCH, and PTPCH fields are ignored This bit is reserved and readonly if Channel 1 or Channel 2 receive paths are not selected during core configuration"]
    #[inline(always)]
    pub fn avcd(&self) -> AVCD_R {
        AVCD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Channel for Queuing the AV Control Packets This field specifies the channel on which the received untagged AV control packets are queued 00: Channel 0 01: Channel 1 10: Channel 2 11: Reserved These bits are reserved if the receive paths of Channel 1 or Channel 2 are not enabled"]
    #[inline(always)]
    pub fn avch(&self) -> AVCH_R {
        AVCH_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Channel for Queuing the PTP Packets This field specifies the channel on which the untagged PTP packets, sent over the Ethernet payload and not over IPv4 or IPv6, are queued 00: Channel 0 01: Channel 1 10: Channel 2 11: Reserved These bits are reserved if the receive paths of Channel 1 or Channel 2 are not enabled"]
    #[inline(always)]
    pub fn ptpch(&self) -> PTPCH_R {
        PTPCH_R::new(((self.bits >> 24) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER462_AVMACCONTROLREGISTER")
            .field("avt", &self.avt())
            .field("avp", &self.avp())
            .field("vqe", &self.vqe())
            .field("avcd", &self.avcd())
            .field("avch", &self.avch())
            .field("ptpch", &self.ptpch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - AV EtherType Value This field contains the value that is compared with the EtherType field of the incoming _tagged or untagged_ Ethernet frame to detect an AV packet"]
    #[inline(always)]
    pub fn avt(&mut self) -> AVT_W<'_, REGISTER462_AVMACCONTROLREGISTER_SPEC> {
        AVT_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - AV Priority for Queuing The value programmed in these bits control the receive channel _0, 1, or 2_ to which an AV packet with a given priority must be queued If only Channel 1 receive path is enabled, the AV packets with priority value greater than or equal to the programmed value are queued on Channel 1 and all other packets are queued on Channel 0 If Channel 2 receive path is also enabled, the AV packets with priority value greater than or equal to the programmed value are queued on Channel 2 The AV packets with value less than the programmed value on Channel 1 and all other packets are queued on Channel 0 These bits are applicable only if at least one additional receive channel is selected in the AV mode"]
    #[inline(always)]
    pub fn avp(&mut self) -> AVP_W<'_, REGISTER462_AVMACCONTROLREGISTER_SPEC> {
        AVP_W::new(self, 16)
    }
    #[doc = "Bit 19 - VLAN Tagged NonAV Packets Queueing Enable When this bit is set, the MAC also queues nonAV VLAN tagged packets into the available channels according to the value of the AVP bits This bit is reserved and readonly if Channel 1 and Channel 2 Receive paths are not selected during core configuration"]
    #[inline(always)]
    pub fn vqe(&mut self) -> VQE_W<'_, REGISTER462_AVMACCONTROLREGISTER_SPEC> {
        VQE_W::new(self, 19)
    }
    #[doc = "Bit 20 - AV Channel Disable When this bit is set, the MAC forwards all packets to the default Channel 0 and the values programmed in the AVP, AVCH, and PTPCH fields are ignored This bit is reserved and readonly if Channel 1 or Channel 2 receive paths are not selected during core configuration"]
    #[inline(always)]
    pub fn avcd(&mut self) -> AVCD_W<'_, REGISTER462_AVMACCONTROLREGISTER_SPEC> {
        AVCD_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Channel for Queuing the AV Control Packets This field specifies the channel on which the received untagged AV control packets are queued 00: Channel 0 01: Channel 1 10: Channel 2 11: Reserved These bits are reserved if the receive paths of Channel 1 or Channel 2 are not enabled"]
    #[inline(always)]
    pub fn avch(&mut self) -> AVCH_W<'_, REGISTER462_AVMACCONTROLREGISTER_SPEC> {
        AVCH_W::new(self, 21)
    }
    #[doc = "Bits 24:25 - Channel for Queuing the PTP Packets This field specifies the channel on which the untagged PTP packets, sent over the Ethernet payload and not over IPv4 or IPv6, are queued 00: Channel 0 01: Channel 1 10: Channel 2 11: Reserved These bits are reserved if the receive paths of Channel 1 or Channel 2 are not enabled"]
    #[inline(always)]
    pub fn ptpch(&mut self) -> PTPCH_W<'_, REGISTER462_AVMACCONTROLREGISTER_SPEC> {
        PTPCH_W::new(self, 24)
    }
}
#[doc = "Controls the AV traffic and queue management in the MAC Receiver This register is present only when you select the AV feature in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register462_avmaccontrolregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register462_avmaccontrolregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER462_AVMACCONTROLREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER462_AVMACCONTROLREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register462_avmaccontrolregister::R`](R) reader structure"]
impl crate::Readable for REGISTER462_AVMACCONTROLREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register462_avmaccontrolregister::W`](W) writer structure"]
impl crate::Writable for REGISTER462_AVMACCONTROLREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER462_AVMACCONTROLREGISTER to value 0"]
impl crate::Resettable for REGISTER462_AVMACCONTROLREGISTER_SPEC {}
