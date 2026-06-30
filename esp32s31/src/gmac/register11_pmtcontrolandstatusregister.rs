#[doc = "Register `REGISTER11_PMTCONTROLANDSTATUSREGISTER` reader"]
pub type R = crate::R<REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC>;
#[doc = "Register `REGISTER11_PMTCONTROLANDSTATUSREGISTER` writer"]
pub type W = crate::W<REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC>;
#[doc = "Field `PWRDWN` reader - Power Down When set, the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame. This bit is then self-cleared and the power-down mode is disabled. The Software can also clear this bit before the expected magic packet or remote wake-up frame is received. The frames, received by MAC after this bit is cleared, are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote Wake-Up Frame Enable bit is set high."]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `PWRDWN` writer - Power Down When set, the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame. This bit is then self-cleared and the power-down mode is disabled. The Software can also clear this bit before the expected magic packet or remote wake-up frame is received. The frames, received by MAC after this bit is cleared, are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote Wake-Up Frame Enable bit is set high."]
pub type PWRDWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPKTEN` reader - Magic Packet Enable When set, enables generation of a power management event because of magic packet reception."]
pub type MGKPKTEN_R = crate::BitReader;
#[doc = "Field `MGKPKTEN` writer - Magic Packet Enable When set, enables generation of a power management event because of magic packet reception."]
pub type MGKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPKTEN` reader - Remote Wake-Up Frame Enable When set, enables generation of a power management event because of remote wake-up frame reception."]
pub type RWKPKTEN_R = crate::BitReader;
#[doc = "Field `RWKPKTEN` writer - Remote Wake-Up Frame Enable When set, enables generation of a power management event because of remote wake-up frame reception."]
pub type RWKPKTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MGKPRCVD` reader - Magic Packet Received When set,this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by Read into this register."]
pub type MGKPRCVD_R = crate::BitReader;
#[doc = "Field `MGKPRCVD` writer - Magic Packet Received When set,this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by Read into this register."]
pub type MGKPRCVD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPRCVD` reader - Remote Wake-Up Frame Received When set, this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
pub type RWKPRCVD_R = crate::BitReader;
#[doc = "Field `RWKPRCVD` writer - Remote Wake-Up Frame Received When set, this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
pub type RWKPRCVD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLBLUCAST` reader - Global Unicast. When set, enables any unicast packet filtered by the MAX(DAF) address recognition to be a remote wake-up frame."]
pub type GLBLUCAST_R = crate::BitReader;
#[doc = "Field `GLBLUCAST` writer - Global Unicast. When set, enables any unicast packet filtered by the MAX(DAF) address recognition to be a remote wake-up frame."]
pub type GLBLUCAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKPTR` reader - Remote Wake-up FIFO Pointer. This field gives the current value(0 to 31) of the Remote Wake-up Frame filter register pointer. When the value of this pointer is erual to 7, 15, 23 or 31, the contents of the Remote Wake-up Frame Filter Register are transferred to the clk_rx_i domain when a write occurs to that register. The maximum value of the pointer is 7, 15, 23 and 31 respectively depending on the number of Remote Wakeup Filters selected during configuration."]
pub type RWKPTR_R = crate::FieldReader;
#[doc = "Field `RWKFILTRST` reader - Remote Wake-Up Frame Filter Register Pointer Reset. When this bit is set, it resets the remote wake-up frame filter register pointer to 3'b000. It is automatically cleared after 1 clock cycle."]
pub type RWKFILTRST_R = crate::BitReader;
#[doc = "Field `RWKFILTRST` writer - Remote Wake-Up Frame Filter Register Pointer Reset. When this bit is set, it resets the remote wake-up frame filter register pointer to 3'b000. It is automatically cleared after 1 clock cycle."]
pub type RWKFILTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Down When set, the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame. This bit is then self-cleared and the power-down mode is disabled. The Software can also clear this bit before the expected magic packet or remote wake-up frame is received. The frames, received by MAC after this bit is cleared, are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote Wake-Up Frame Enable bit is set high."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable When set, enables generation of a power management event because of magic packet reception."]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Remote Wake-Up Frame Enable When set, enables generation of a power management event because of remote wake-up frame reception."]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received When set,this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by Read into this register."]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Remote Wake-Up Frame Received When set, this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Unicast. When set, enables any unicast packet filtered by the MAX(DAF) address recognition to be a remote wake-up frame."]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Remote Wake-up FIFO Pointer. This field gives the current value(0 to 31) of the Remote Wake-up Frame filter register pointer. When the value of this pointer is erual to 7, 15, 23 or 31, the contents of the Remote Wake-up Frame Filter Register are transferred to the clk_rx_i domain when a write occurs to that register. The maximum value of the pointer is 7, 15, 23 and 31 respectively depending on the number of Remote Wakeup Filters selected during configuration."]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Remote Wake-Up Frame Filter Register Pointer Reset. When this bit is set, it resets the remote wake-up frame filter register pointer to 3'b000. It is automatically cleared after 1 clock cycle."]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER11_PMTCONTROLANDSTATUSREGISTER")
            .field("pwrdwn", &self.pwrdwn())
            .field("mgkpkten", &self.mgkpkten())
            .field("rwkpkten", &self.rwkpkten())
            .field("mgkprcvd", &self.mgkprcvd())
            .field("rwkprcvd", &self.rwkprcvd())
            .field("glblucast", &self.glblucast())
            .field("rwkptr", &self.rwkptr())
            .field("rwkfiltrst", &self.rwkfiltrst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Power Down When set, the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame. This bit is then self-cleared and the power-down mode is disabled. The Software can also clear this bit before the expected magic packet or remote wake-up frame is received. The frames, received by MAC after this bit is cleared, are forwarded to the application. This bit must only be set when the Magic Packet Enable, Global Unicast, or Remote Wake-Up Frame Enable bit is set high."]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<'_, REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC> {
        PWRDWN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable When set, enables generation of a power management event because of magic packet reception."]
    #[inline(always)]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<'_, REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC> {
        MGKPKTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Remote Wake-Up Frame Enable When set, enables generation of a power management event because of remote wake-up frame reception."]
    #[inline(always)]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<'_, REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC> {
        RWKPKTEN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Magic Packet Received When set,this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by Read into this register."]
    #[inline(always)]
    pub fn mgkprcvd(&mut self) -> MGKPRCVD_W<'_, REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC> {
        MGKPRCVD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Remote Wake-Up Frame Received When set, this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
    #[inline(always)]
    pub fn rwkprcvd(&mut self) -> RWKPRCVD_W<'_, REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC> {
        RWKPRCVD_W::new(self, 6)
    }
    #[doc = "Bit 9 - Global Unicast. When set, enables any unicast packet filtered by the MAX(DAF) address recognition to be a remote wake-up frame."]
    #[inline(always)]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<'_, REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC> {
        GLBLUCAST_W::new(self, 9)
    }
    #[doc = "Bit 31 - Remote Wake-Up Frame Filter Register Pointer Reset. When this bit is set, it resets the remote wake-up frame filter register pointer to 3'b000. It is automatically cleared after 1 clock cycle."]
    #[inline(always)]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<'_, REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC> {
        RWKFILTRST_W::new(self, 31)
    }
}
#[doc = "PMT Control and Status Register. This register is present only when you select the PMT module in coreConsultant.\n\nYou can [`read`](crate::Reg::read) this register and get [`register11_pmtcontrolandstatusregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register11_pmtcontrolandstatusregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register11_pmtcontrolandstatusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register11_pmtcontrolandstatusregister::W`](W) writer structure"]
impl crate::Writable for REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER11_PMTCONTROLANDSTATUSREGISTER to value 0"]
impl crate::Resettable for REGISTER11_PMTCONTROLANDSTATUSREGISTER_SPEC {}
