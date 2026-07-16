#[doc = "Register `REGISTER7_INTERRUPTENABLEREGISTER` reader"]
pub type R = crate::R<REGISTER7_INTERRUPTENABLEREGISTER_SPEC>;
#[doc = "Register `REGISTER7_INTERRUPTENABLEREGISTER` writer"]
pub type W = crate::W<REGISTER7_INTERRUPTENABLEREGISTER_SPEC>;
#[doc = "Field `TIE` reader - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Interrupt is enabled When this bit is reset, the Transmit Interrupt is disabled The sbd_intr_o interrupt is generated as shown in Figure 61 It is asserted only when the TTI, GPI, GMI, GLI, or GLPII bit of the DMA Status register is asserted, or when the NIS or AIS Status bit is asserted and the corresponding Interrupt Enable bits _NIE or AIE_ are enabled"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Interrupt is enabled When this bit is reset, the Transmit Interrupt is disabled The sbd_intr_o interrupt is generated as shown in Figure 61 It is asserted only when the TTI, GPI, GMI, GLI, or GLPII bit of the DMA Status register is asserted, or when the NIS or AIS Status bit is asserted and the corresponding Interrupt Enable bits _NIE or AIE_ are enabled"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmission Stopped Interrupt is enabled When this bit is reset, the Transmission Stopped Interrupt is disabled"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmission Stopped Interrupt is enabled When this bit is reset, the Transmission Stopped Interrupt is disabled"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUE` reader - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Buffer Unavailable Interrupt is enabled When this bit is reset, the Transmit Buffer Unavailable Interrupt is disabled"]
pub type TUE_R = crate::BitReader;
#[doc = "Field `TUE` writer - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Buffer Unavailable Interrupt is enabled When this bit is reset, the Transmit Buffer Unavailable Interrupt is disabled"]
pub type TUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJE` reader - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Jabber Timeout Interrupt is enabled When this bit is reset, the Transmit Jabber Timeout Interrupt is disabled"]
pub type TJE_R = crate::BitReader;
#[doc = "Field `TJE` writer - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Jabber Timeout Interrupt is enabled When this bit is reset, the Transmit Jabber Timeout Interrupt is disabled"]
pub type TJE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVE` reader - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Overflow Interrupt is enabled When this bit is reset, the Overflow Interrupt is disabled"]
pub type OVE_R = crate::BitReader;
#[doc = "Field `OVE` writer - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Overflow Interrupt is enabled When this bit is reset, the Overflow Interrupt is disabled"]
pub type OVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNE` reader - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Underflow Interrupt is enabled When this bit is reset, the Underflow Interrupt is disabled"]
pub type UNE_R = crate::BitReader;
#[doc = "Field `UNE` writer - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Underflow Interrupt is enabled When this bit is reset, the Underflow Interrupt is disabled"]
pub type UNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Receive Interrupt is enabled When this bit is reset, the Receive Interrupt is disabled"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Receive Interrupt is enabled When this bit is reset, the Receive Interrupt is disabled"]
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Buffer Unavailable Interrupt is enabled When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Buffer Unavailable Interrupt is enabled When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
pub type RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSE` reader - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Stopped Interrupt is enabled When this bit is reset, the Receive Stopped Interrupt is disabled"]
pub type RSE_R = crate::BitReader;
#[doc = "Field `RSE` writer - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Stopped Interrupt is enabled When this bit is reset, the Receive Stopped Interrupt is disabled"]
pub type RSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWE` reader - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Watchdog Timeout Interrupt is enabled When this bit is reset, the Receive Watchdog Timeout Interrupt is disabled"]
pub type RWE_R = crate::BitReader;
#[doc = "Field `RWE` writer - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Watchdog Timeout Interrupt is enabled When this bit is reset, the Receive Watchdog Timeout Interrupt is disabled"]
pub type RWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETE` reader - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable _Bit 15_, the Early Transmit Interrupt is enabled When this bit is reset, the Early Transmit Interrupt is disabled"]
pub type ETE_R = crate::BitReader;
#[doc = "Field `ETE` writer - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable _Bit 15_, the Early Transmit Interrupt is enabled When this bit is reset, the Early Transmit Interrupt is disabled"]
pub type ETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Fatal Bus Error Interrupt is enabled When this bit is reset, the Fatal Bus Error Enable Interrupt is disabled 12:11 Reserved 00 RO"]
pub type FBE_R = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Fatal Bus Error Interrupt is enabled When this bit is reset, the Fatal Bus Error Enable Interrupt is disabled 12:11 Reserved 00 RO"]
pub type FBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERE` reader - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Early Receive Interrupt is enabled When this bit is reset, the Early Receive Interrupt is disabled"]
pub type ERE_R = crate::BitReader;
#[doc = "Field `ERE` writer - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Early Receive Interrupt is enabled When this bit is reset, the Early Receive Interrupt is disabled"]
pub type ERE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable When this bit is set, abnormal interrupt summary is enabled When this bit is reset, the abnormal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error"]
pub type AIE_R = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable When this bit is set, abnormal interrupt summary is enabled When this bit is reset, the abnormal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error"]
pub type AIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable When this bit is set, normal interrupt summary is enabled When this bit is reset, normal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt"]
pub type NIE_R = crate::BitReader;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable When this bit is set, normal interrupt summary is enabled When this bit is reset, normal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt"]
pub type NIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Interrupt is enabled When this bit is reset, the Transmit Interrupt is disabled The sbd_intr_o interrupt is generated as shown in Figure 61 It is asserted only when the TTI, GPI, GMI, GLI, or GLPII bit of the DMA Status register is asserted, or when the NIS or AIS Status bit is asserted and the corresponding Interrupt Enable bits _NIE or AIE_ are enabled"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmission Stopped Interrupt is enabled When this bit is reset, the Transmission Stopped Interrupt is disabled"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Buffer Unavailable Interrupt is enabled When this bit is reset, the Transmit Buffer Unavailable Interrupt is disabled"]
    #[inline(always)]
    pub fn tue(&self) -> TUE_R {
        TUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Jabber Timeout Interrupt is enabled When this bit is reset, the Transmit Jabber Timeout Interrupt is disabled"]
    #[inline(always)]
    pub fn tje(&self) -> TJE_R {
        TJE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Overflow Interrupt is enabled When this bit is reset, the Overflow Interrupt is disabled"]
    #[inline(always)]
    pub fn ove(&self) -> OVE_R {
        OVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Underflow Interrupt is enabled When this bit is reset, the Underflow Interrupt is disabled"]
    #[inline(always)]
    pub fn une(&self) -> UNE_R {
        UNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Receive Interrupt is enabled When this bit is reset, the Receive Interrupt is disabled"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Buffer Unavailable Interrupt is enabled When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Stopped Interrupt is enabled When this bit is reset, the Receive Stopped Interrupt is disabled"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Watchdog Timeout Interrupt is enabled When this bit is reset, the Receive Watchdog Timeout Interrupt is disabled"]
    #[inline(always)]
    pub fn rwe(&self) -> RWE_R {
        RWE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable _Bit 15_, the Early Transmit Interrupt is enabled When this bit is reset, the Early Transmit Interrupt is disabled"]
    #[inline(always)]
    pub fn ete(&self) -> ETE_R {
        ETE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Fatal Bus Error Interrupt is enabled When this bit is reset, the Fatal Bus Error Enable Interrupt is disabled 12:11 Reserved 00 RO"]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Early Receive Interrupt is enabled When this bit is reset, the Early Receive Interrupt is disabled"]
    #[inline(always)]
    pub fn ere(&self) -> ERE_R {
        ERE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable When this bit is set, abnormal interrupt summary is enabled When this bit is reset, the abnormal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable When this bit is set, normal interrupt summary is enabled When this bit is reset, normal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt"]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER7_INTERRUPTENABLEREGISTER")
            .field("tie", &self.tie())
            .field("tse", &self.tse())
            .field("tue", &self.tue())
            .field("tje", &self.tje())
            .field("ove", &self.ove())
            .field("une", &self.une())
            .field("rie", &self.rie())
            .field("rue", &self.rue())
            .field("rse", &self.rse())
            .field("rwe", &self.rwe())
            .field("ete", &self.ete())
            .field("fbe", &self.fbe())
            .field("ere", &self.ere())
            .field("aie", &self.aie())
            .field("nie", &self.nie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Interrupt is enabled When this bit is reset, the Transmit Interrupt is disabled The sbd_intr_o interrupt is generated as shown in Figure 61 It is asserted only when the TTI, GPI, GMI, GLI, or GLPII bit of the DMA Status register is asserted, or when the NIS or AIS Status bit is asserted and the corresponding Interrupt Enable bits _NIE or AIE_ are enabled"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        TIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmission Stopped Interrupt is enabled When this bit is reset, the Transmission Stopped Interrupt is disabled"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        TSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Buffer Unavailable Interrupt is enabled When this bit is reset, the Transmit Buffer Unavailable Interrupt is disabled"]
    #[inline(always)]
    pub fn tue(&mut self) -> TUE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        TUE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Jabber Timeout Interrupt is enabled When this bit is reset, the Transmit Jabber Timeout Interrupt is disabled"]
    #[inline(always)]
    pub fn tje(&mut self) -> TJE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        TJE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Overflow Interrupt is enabled When this bit is reset, the Overflow Interrupt is disabled"]
    #[inline(always)]
    pub fn ove(&mut self) -> OVE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        OVE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Underflow Interrupt is enabled When this bit is reset, the Underflow Interrupt is disabled"]
    #[inline(always)]
    pub fn une(&mut self) -> UNE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        UNE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Receive Interrupt is enabled When this bit is reset, the Receive Interrupt is disabled"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        RIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Buffer Unavailable Interrupt is enabled When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        RUE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Stopped Interrupt is enabled When this bit is reset, the Receive Stopped Interrupt is disabled"]
    #[inline(always)]
    pub fn rse(&mut self) -> RSE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        RSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Watchdog Timeout Interrupt is enabled When this bit is reset, the Receive Watchdog Timeout Interrupt is disabled"]
    #[inline(always)]
    pub fn rwe(&mut self) -> RWE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        RWE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable _Bit 15_, the Early Transmit Interrupt is enabled When this bit is reset, the Early Transmit Interrupt is disabled"]
    #[inline(always)]
    pub fn ete(&mut self) -> ETE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        ETE_W::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Fatal Bus Error Interrupt is enabled When this bit is reset, the Fatal Bus Error Enable Interrupt is disabled 12:11 Reserved 00 RO"]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        FBE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Early Receive Interrupt is enabled When this bit is reset, the Early Receive Interrupt is disabled"]
    #[inline(always)]
    pub fn ere(&mut self) -> ERE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        ERE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable When this bit is set, abnormal interrupt summary is enabled When this bit is reset, the abnormal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error"]
    #[inline(always)]
    pub fn aie(&mut self) -> AIE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        AIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable When this bit is set, normal interrupt summary is enabled When this bit is reset, normal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt"]
    #[inline(always)]
    pub fn nie(&mut self) -> NIE_W<'_, REGISTER7_INTERRUPTENABLEREGISTER_SPEC> {
        NIE_W::new(self, 16)
    }
}
#[doc = "Enables the interrupts reported by the Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register7_interruptenableregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register7_interruptenableregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER7_INTERRUPTENABLEREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER7_INTERRUPTENABLEREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register7_interruptenableregister::R`](R) reader structure"]
impl crate::Readable for REGISTER7_INTERRUPTENABLEREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register7_interruptenableregister::W`](W) writer structure"]
impl crate::Writable for REGISTER7_INTERRUPTENABLEREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER7_INTERRUPTENABLEREGISTER to value 0"]
impl crate::Resettable for REGISTER7_INTERRUPTENABLEREGISTER_SPEC {}
