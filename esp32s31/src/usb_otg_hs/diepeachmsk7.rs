#[doc = "Register `DIEPEACHMSK7` reader"]
pub type R = crate::R<DIEPEACHMSK7_SPEC>;
#[doc = "Register `DIEPEACHMSK7` writer"]
pub type W = crate::W<DIEPEACHMSK7_SPEC>;
#[doc = "Field `XFERCOMPLMSK` reader - Transfer Completed Interrupt Mask (XferComplMsk)"]
pub type XFERCOMPLMSK_R = crate::BitReader;
#[doc = "Field `XFERCOMPLMSK` writer - Transfer Completed Interrupt Mask (XferComplMsk)"]
pub type XFERCOMPLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLDMSK` reader - Endpoint Disabled Interrupt Mask (EPDisbldMsk)"]
pub type EPDISBLDMSK_R = crate::BitReader;
#[doc = "Field `EPDISBLDMSK` writer - Endpoint Disabled Interrupt Mask (EPDisbldMsk)"]
pub type EPDISBLDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRMSK` reader - AHB Error Mask (AHBErrMsk)"]
pub type AHBERRMSK_R = crate::BitReader;
#[doc = "Field `AHBERRMSK` writer - AHB Error Mask (AHBErrMsk)"]
pub type AHBERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTMSK` reader - Timeout Condition Mask (TimeOUTMsk) (Non-isochronous endpoints)"]
pub type TIMEOUTMSK_R = crate::BitReader;
#[doc = "Field `TIMEOUTMSK` writer - Timeout Condition Mask (TimeOUTMsk) (Non-isochronous endpoints)"]
pub type TIMEOUTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMPMSK` reader - IN Token Received When TxFIFO Empty Mask(INTknTXFEmpMsk)"]
pub type INTKNTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `INTKNTXFEMPMSK` writer - IN Token Received When TxFIFO Empty Mask(INTknTXFEmpMsk)"]
pub type INTKNTXFEMPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNEPMISMSK` reader - IN Token received with EP Mismatch Mask (INTknEPMisMsk)"]
pub type INTKNEPMISMSK_R = crate::BitReader;
#[doc = "Field `INTKNEPMISMSK` writer - IN Token received with EP Mismatch Mask (INTknEPMisMsk)"]
pub type INTKNEPMISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNAKEFFMSK` reader - IN Endpoint NAK Effective Mask (INEPNakEffMsk)"]
pub type INEPNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `INEPNAKEFFMSK` writer - IN Endpoint NAK Effective Mask (INEPNakEffMsk)"]
pub type INEPNAKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOUNDRNMSK` reader - Fifo Underrun Mask (TxfifoUndrnMsk)"]
pub type TXFIFOUNDRNMSK_R = crate::BitReader;
#[doc = "Field `TXFIFOUNDRNMSK` writer - Fifo Underrun Mask (TxfifoUndrnMsk)"]
pub type TXFIFOUNDRNMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAININTRMSK` reader - BNA interrupt Mask (BNAInIntrMsk)"]
pub type BNAININTRMSK_R = crate::BitReader;
#[doc = "Field `BNAININTRMSK` writer - BNA interrupt Mask (BNAInIntrMsk)"]
pub type BNAININTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - NAK interrupt Mask (NAKMsk)"]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK interrupt Mask (NAKMsk)"]
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask (XferComplMsk)"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask (EPDisbldMsk)"]
    #[inline(always)]
    pub fn epdisbldmsk(&self) -> EPDISBLDMSK_R {
        EPDISBLDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask (AHBErrMsk)"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Condition Mask (TimeOUTMsk) (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn timeoutmsk(&self) -> TIMEOUTMSK_R {
        TIMEOUTMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask(INTknTXFEmpMsk)"]
    #[inline(always)]
    pub fn intkntxfempmsk(&self) -> INTKNTXFEMPMSK_R {
        INTKNTXFEMPMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN Token received with EP Mismatch Mask (INTknEPMisMsk)"]
    #[inline(always)]
    pub fn intknepmismsk(&self) -> INTKNEPMISMSK_R {
        INTKNEPMISMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask (INEPNakEffMsk)"]
    #[inline(always)]
    pub fn inepnakeffmsk(&self) -> INEPNAKEFFMSK_R {
        INEPNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask (TxfifoUndrnMsk)"]
    #[inline(always)]
    pub fn txfifoundrnmsk(&self) -> TXFIFOUNDRNMSK_R {
        TXFIFOUNDRNMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt Mask (BNAInIntrMsk)"]
    #[inline(always)]
    pub fn bnainintrmsk(&self) -> BNAININTRMSK_R {
        BNAININTRMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt Mask (NAKMsk)"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPEACHMSK7")
            .field("xfercomplmsk", &self.xfercomplmsk())
            .field("epdisbldmsk", &self.epdisbldmsk())
            .field("ahberrmsk", &self.ahberrmsk())
            .field("timeoutmsk", &self.timeoutmsk())
            .field("intkntxfempmsk", &self.intkntxfempmsk())
            .field("intknepmismsk", &self.intknepmismsk())
            .field("inepnakeffmsk", &self.inepnakeffmsk())
            .field("txfifoundrnmsk", &self.txfifoundrnmsk())
            .field("bnainintrmsk", &self.bnainintrmsk())
            .field("nakmsk", &self.nakmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask (XferComplMsk)"]
    #[inline(always)]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W<'_, DIEPEACHMSK7_SPEC> {
        XFERCOMPLMSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask (EPDisbldMsk)"]
    #[inline(always)]
    pub fn epdisbldmsk(&mut self) -> EPDISBLDMSK_W<'_, DIEPEACHMSK7_SPEC> {
        EPDISBLDMSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error Mask (AHBErrMsk)"]
    #[inline(always)]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W<'_, DIEPEACHMSK7_SPEC> {
        AHBERRMSK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Condition Mask (TimeOUTMsk) (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn timeoutmsk(&mut self) -> TIMEOUTMSK_W<'_, DIEPEACHMSK7_SPEC> {
        TIMEOUTMSK_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN Token Received When TxFIFO Empty Mask(INTknTXFEmpMsk)"]
    #[inline(always)]
    pub fn intkntxfempmsk(&mut self) -> INTKNTXFEMPMSK_W<'_, DIEPEACHMSK7_SPEC> {
        INTKNTXFEMPMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - IN Token received with EP Mismatch Mask (INTknEPMisMsk)"]
    #[inline(always)]
    pub fn intknepmismsk(&mut self) -> INTKNEPMISMSK_W<'_, DIEPEACHMSK7_SPEC> {
        INTKNEPMISMSK_W::new(self, 5)
    }
    #[doc = "Bit 6 - IN Endpoint NAK Effective Mask (INEPNakEffMsk)"]
    #[inline(always)]
    pub fn inepnakeffmsk(&mut self) -> INEPNAKEFFMSK_W<'_, DIEPEACHMSK7_SPEC> {
        INEPNAKEFFMSK_W::new(self, 6)
    }
    #[doc = "Bit 8 - Fifo Underrun Mask (TxfifoUndrnMsk)"]
    #[inline(always)]
    pub fn txfifoundrnmsk(&mut self) -> TXFIFOUNDRNMSK_W<'_, DIEPEACHMSK7_SPEC> {
        TXFIFOUNDRNMSK_W::new(self, 8)
    }
    #[doc = "Bit 9 - BNA interrupt Mask (BNAInIntrMsk)"]
    #[inline(always)]
    pub fn bnainintrmsk(&mut self) -> BNAININTRMSK_W<'_, DIEPEACHMSK7_SPEC> {
        BNAININTRMSK_W::new(self, 9)
    }
    #[doc = "Bit 13 - NAK interrupt Mask (NAKMsk)"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NAKMSK_W<'_, DIEPEACHMSK7_SPEC> {
        NAKMSK_W::new(self, 13)
    }
}
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPEACHMSK7_SPEC;
impl crate::RegisterSpec for DIEPEACHMSK7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepeachmsk7::R`](R) reader structure"]
impl crate::Readable for DIEPEACHMSK7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepeachmsk7::W`](W) writer structure"]
impl crate::Writable for DIEPEACHMSK7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIEPEACHMSK7 to value 0"]
impl crate::Resettable for DIEPEACHMSK7_SPEC {}
