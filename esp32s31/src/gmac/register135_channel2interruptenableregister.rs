#[doc = "Register `REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER` reader"]
pub type R = crate::R<REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC>;
#[doc = "Register `REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER` writer"]
pub type W = crate::W<REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC>;
#[doc = "Field `CH2_TIE` reader - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Interrupt is enabled When this bit is reset, the Transmit Interrupt is disabled The sbd_intr_o interrupt is generated as shown in Figure 61 It is asserted only when the TTI, GPI, GMI, GLI, or GLPII bit of the DMA Status register is asserted, or when the NIS or AIS Status bit is asserted and the corresponding Interrupt Enable bits _NIE or AIE_ are enabled"]
pub type CH2_TIE_R = crate::BitReader;
#[doc = "Field `CH2_TIE` writer - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Interrupt is enabled When this bit is reset, the Transmit Interrupt is disabled The sbd_intr_o interrupt is generated as shown in Figure 61 It is asserted only when the TTI, GPI, GMI, GLI, or GLPII bit of the DMA Status register is asserted, or when the NIS or AIS Status bit is asserted and the corresponding Interrupt Enable bits _NIE or AIE_ are enabled"]
pub type CH2_TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TSE` reader - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmission Stopped Interrupt is enabled When this bit is reset, the Transmission Stopped Interrupt is disabled"]
pub type CH2_TSE_R = crate::BitReader;
#[doc = "Field `CH2_TSE` writer - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmission Stopped Interrupt is enabled When this bit is reset, the Transmission Stopped Interrupt is disabled"]
pub type CH2_TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TUE` reader - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Buffer Unavailable Interrupt is enabled When this bit is reset, the Transmit Buffer Unavailable Interrupt is disabled"]
pub type CH2_TUE_R = crate::BitReader;
#[doc = "Field `CH2_TUE` writer - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Buffer Unavailable Interrupt is enabled When this bit is reset, the Transmit Buffer Unavailable Interrupt is disabled"]
pub type CH2_TUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TJE` reader - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Jabber Timeout Interrupt is enabled When this bit is reset, the Transmit Jabber Timeout Interrupt is disabled"]
pub type CH2_TJE_R = crate::BitReader;
#[doc = "Field `CH2_TJE` writer - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Jabber Timeout Interrupt is enabled When this bit is reset, the Transmit Jabber Timeout Interrupt is disabled"]
pub type CH2_TJE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_OVE` reader - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Overflow Interrupt is enabled When this bit is reset, the Overflow Interrupt is disabled"]
pub type CH2_OVE_R = crate::BitReader;
#[doc = "Field `CH2_OVE` writer - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Overflow Interrupt is enabled When this bit is reset, the Overflow Interrupt is disabled"]
pub type CH2_OVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_UNE` reader - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Underflow Interrupt is enabled When this bit is reset, the Underflow Interrupt is disabled"]
pub type CH2_UNE_R = crate::BitReader;
#[doc = "Field `CH2_UNE` writer - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Underflow Interrupt is enabled When this bit is reset, the Underflow Interrupt is disabled"]
pub type CH2_UNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_RIE` reader - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Receive Interrupt is enabled When this bit is reset, the Receive Interrupt is disabled"]
pub type CH2_RIE_R = crate::BitReader;
#[doc = "Field `CH2_RIE` writer - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Receive Interrupt is enabled When this bit is reset, the Receive Interrupt is disabled"]
pub type CH2_RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_RUE` reader - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Buffer Unavailable Interrupt is enabled When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
pub type CH2_RUE_R = crate::BitReader;
#[doc = "Field `CH2_RUE` writer - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Buffer Unavailable Interrupt is enabled When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
pub type CH2_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_RSE` reader - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Stopped Interrupt is enabled When this bit is reset, the Receive Stopped Interrupt is disabled"]
pub type CH2_RSE_R = crate::BitReader;
#[doc = "Field `CH2_RSE` writer - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Stopped Interrupt is enabled When this bit is reset, the Receive Stopped Interrupt is disabled"]
pub type CH2_RSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_RWE` reader - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Watchdog Timeout Interrupt is enabled When this bit is reset, the Receive Watchdog Timeout Interrupt is disabled"]
pub type CH2_RWE_R = crate::BitReader;
#[doc = "Field `CH2_RWE` writer - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Watchdog Timeout Interrupt is enabled When this bit is reset, the Receive Watchdog Timeout Interrupt is disabled"]
pub type CH2_RWE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_ETE` reader - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable _Bit 15_, the Early Transmit Interrupt is enabled When this bit is reset, the Early Transmit Interrupt is disabled"]
pub type CH2_ETE_R = crate::BitReader;
#[doc = "Field `CH2_ETE` writer - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable _Bit 15_, the Early Transmit Interrupt is enabled When this bit is reset, the Early Transmit Interrupt is disabled"]
pub type CH2_ETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_FBE` reader - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Fatal Bus Error Interrupt is enabled When this bit is reset, the Fatal Bus Error Enable Interrupt is disabled 12:11 Reserved 00 RO"]
pub type CH2_FBE_R = crate::BitReader;
#[doc = "Field `CH2_FBE` writer - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Fatal Bus Error Interrupt is enabled When this bit is reset, the Fatal Bus Error Enable Interrupt is disabled 12:11 Reserved 00 RO"]
pub type CH2_FBE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_ERE` reader - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Early Receive Interrupt is enabled When this bit is reset, the Early Receive Interrupt is disabled"]
pub type CH2_ERE_R = crate::BitReader;
#[doc = "Field `CH2_ERE` writer - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Early Receive Interrupt is enabled When this bit is reset, the Early Receive Interrupt is disabled"]
pub type CH2_ERE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_AIE` reader - Abnormal Interrupt Summary Enable When this bit is set, abnormal interrupt summary is enabled When this bit is reset, the abnormal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error"]
pub type CH2_AIE_R = crate::BitReader;
#[doc = "Field `CH2_AIE` writer - Abnormal Interrupt Summary Enable When this bit is set, abnormal interrupt summary is enabled When this bit is reset, the abnormal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error"]
pub type CH2_AIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_NIE` reader - Normal Interrupt Summary Enable When this bit is set, normal interrupt summary is enabled When this bit is reset, normal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt"]
pub type CH2_NIE_R = crate::BitReader;
#[doc = "Field `CH2_NIE` writer - Normal Interrupt Summary Enable When this bit is set, normal interrupt summary is enabled When this bit is reset, normal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt"]
pub type CH2_NIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Interrupt is enabled When this bit is reset, the Transmit Interrupt is disabled The sbd_intr_o interrupt is generated as shown in Figure 61 It is asserted only when the TTI, GPI, GMI, GLI, or GLPII bit of the DMA Status register is asserted, or when the NIS or AIS Status bit is asserted and the corresponding Interrupt Enable bits _NIE or AIE_ are enabled"]
    #[inline(always)]
    pub fn ch2_tie(&self) -> CH2_TIE_R {
        CH2_TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmission Stopped Interrupt is enabled When this bit is reset, the Transmission Stopped Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_tse(&self) -> CH2_TSE_R {
        CH2_TSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Buffer Unavailable Interrupt is enabled When this bit is reset, the Transmit Buffer Unavailable Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_tue(&self) -> CH2_TUE_R {
        CH2_TUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Jabber Timeout Interrupt is enabled When this bit is reset, the Transmit Jabber Timeout Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_tje(&self) -> CH2_TJE_R {
        CH2_TJE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Overflow Interrupt is enabled When this bit is reset, the Overflow Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_ove(&self) -> CH2_OVE_R {
        CH2_OVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Underflow Interrupt is enabled When this bit is reset, the Underflow Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_une(&self) -> CH2_UNE_R {
        CH2_UNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Receive Interrupt is enabled When this bit is reset, the Receive Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_rie(&self) -> CH2_RIE_R {
        CH2_RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Buffer Unavailable Interrupt is enabled When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_rue(&self) -> CH2_RUE_R {
        CH2_RUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Stopped Interrupt is enabled When this bit is reset, the Receive Stopped Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_rse(&self) -> CH2_RSE_R {
        CH2_RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Watchdog Timeout Interrupt is enabled When this bit is reset, the Receive Watchdog Timeout Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_rwe(&self) -> CH2_RWE_R {
        CH2_RWE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable _Bit 15_, the Early Transmit Interrupt is enabled When this bit is reset, the Early Transmit Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_ete(&self) -> CH2_ETE_R {
        CH2_ETE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Fatal Bus Error Interrupt is enabled When this bit is reset, the Fatal Bus Error Enable Interrupt is disabled 12:11 Reserved 00 RO"]
    #[inline(always)]
    pub fn ch2_fbe(&self) -> CH2_FBE_R {
        CH2_FBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Early Receive Interrupt is enabled When this bit is reset, the Early Receive Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_ere(&self) -> CH2_ERE_R {
        CH2_ERE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable When this bit is set, abnormal interrupt summary is enabled When this bit is reset, the abnormal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error"]
    #[inline(always)]
    pub fn ch2_aie(&self) -> CH2_AIE_R {
        CH2_AIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable When this bit is set, normal interrupt summary is enabled When this bit is reset, normal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt"]
    #[inline(always)]
    pub fn ch2_nie(&self) -> CH2_NIE_R {
        CH2_NIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER")
            .field("ch2_tie", &self.ch2_tie())
            .field("ch2_tse", &self.ch2_tse())
            .field("ch2_tue", &self.ch2_tue())
            .field("ch2_tje", &self.ch2_tje())
            .field("ch2_ove", &self.ch2_ove())
            .field("ch2_une", &self.ch2_une())
            .field("ch2_rie", &self.ch2_rie())
            .field("ch2_rue", &self.ch2_rue())
            .field("ch2_rse", &self.ch2_rse())
            .field("ch2_rwe", &self.ch2_rwe())
            .field("ch2_ete", &self.ch2_ete())
            .field("ch2_fbe", &self.ch2_fbe())
            .field("ch2_ere", &self.ch2_ere())
            .field("ch2_aie", &self.ch2_aie())
            .field("ch2_nie", &self.ch2_nie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Interrupt is enabled When this bit is reset, the Transmit Interrupt is disabled The sbd_intr_o interrupt is generated as shown in Figure 61 It is asserted only when the TTI, GPI, GMI, GLI, or GLPII bit of the DMA Status register is asserted, or when the NIS or AIS Status bit is asserted and the corresponding Interrupt Enable bits _NIE or AIE_ are enabled"]
    #[inline(always)]
    pub fn ch2_tie(&mut self) -> CH2_TIE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_TIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmission Stopped Interrupt is enabled When this bit is reset, the Transmission Stopped Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_tse(&mut self) -> CH2_TSE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_TSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Transmit Buffer Unavailable Interrupt is enabled When this bit is reset, the Transmit Buffer Unavailable Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_tue(&mut self) -> CH2_TUE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_TUE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Jabber Timeout Interrupt is enabled When this bit is reset, the Transmit Jabber Timeout Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_tje(&mut self) -> CH2_TJE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_TJE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Overflow Interrupt is enabled When this bit is reset, the Overflow Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_ove(&mut self) -> CH2_OVE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_OVE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow Interrupt Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Transmit Underflow Interrupt is enabled When this bit is reset, the Underflow Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_une(&mut self) -> CH2_UNE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_UNE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Receive Interrupt is enabled When this bit is reset, the Receive Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_rie(&mut self) -> CH2_RIE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_RIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Buffer Unavailable Interrupt is enabled When this bit is reset, the Receive Buffer Unavailable Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_rue(&mut self) -> CH2_RUE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_RUE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Stopped Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Stopped Interrupt is enabled When this bit is reset, the Receive Stopped Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_rse(&mut self) -> CH2_RSE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_RSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Receive Watchdog Timeout Interrupt is enabled When this bit is reset, the Receive Watchdog Timeout Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_rwe(&mut self) -> CH2_RWE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_RWE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable When this bit is set with an Abnormal Interrupt Summary Enable _Bit 15_, the Early Transmit Interrupt is enabled When this bit is reset, the Early Transmit Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_ete(&mut self) -> CH2_ETE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_ETE_W::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal Bus Error Enable When this bit is set with Abnormal Interrupt Summary Enable _Bit 15_, the Fatal Bus Error Interrupt is enabled When this bit is reset, the Fatal Bus Error Enable Interrupt is disabled 12:11 Reserved 00 RO"]
    #[inline(always)]
    pub fn ch2_fbe(&mut self) -> CH2_FBE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_FBE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Early Receive Interrupt Enable When this bit is set with Normal Interrupt Summary Enable _Bit 16_, the Early Receive Interrupt is enabled When this bit is reset, the Early Receive Interrupt is disabled"]
    #[inline(always)]
    pub fn ch2_ere(&mut self) -> CH2_ERE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_ERE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Enable When this bit is set, abnormal interrupt summary is enabled When this bit is reset, the abnormal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[1\\]: Transmit Process Stopped Register 5\\[3\\]: Transmit Jabber Timeout Register 5\\[4\\]: Receive Overflow Register 5\\[5\\]: Transmit Underflow Register 5\\[7\\]: Receive Buffer Unavailable Register 5\\[8\\]: Receive Process Stopped Register 5\\[9\\]: Receive Watchdog Timeout Register 5\\[10\\]: Early Transmit Interrupt Register 5\\[13\\]: Fatal Bus Error"]
    #[inline(always)]
    pub fn ch2_aie(&mut self) -> CH2_AIE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_AIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Enable When this bit is set, normal interrupt summary is enabled When this bit is reset, normal interrupt summary is disabled This bit enables the following interrupts in Register 5 _Status Register_: Register 5\\[0\\]: Transmit Interrupt Register 5\\[2\\]: Transmit Buffer Unavailable Register 5\\[6\\]: Receive Interrupt Register 5\\[14\\]: Early Receive Interrupt"]
    #[inline(always)]
    pub fn ch2_nie(&mut self) -> CH2_NIE_W<'_, REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC> {
        CH2_NIE_W::new(self, 16)
    }
}
#[doc = "Enables the interrupts reported by the Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`register135_channel2interruptenableregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register135_channel2interruptenableregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register135_channel2interruptenableregister::R`](R) reader structure"]
impl crate::Readable for REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register135_channel2interruptenableregister::W`](W) writer structure"]
impl crate::Writable for REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER to value 0"]
impl crate::Resettable for REGISTER135_CHANNEL2INTERRUPTENABLEREGISTER_SPEC {}
