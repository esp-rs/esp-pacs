#[doc = "Register `HCSPLT5` reader"]
pub type R = crate::R<HCSPLT5_SPEC>;
#[doc = "Register `HCSPLT5` writer"]
pub type W = crate::W<HCSPLT5_SPEC>;
#[doc = "Field `PRTADDR` reader - Port Address (PrtAddr) This field is the port number of the recipient transaction translator."]
pub type PRTADDR_R = crate::FieldReader;
#[doc = "Field `PRTADDR` writer - Port Address (PrtAddr) This field is the port number of the recipient transaction translator."]
pub type PRTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HUBADDR` reader - Hub Address (HubAddr) This field holds the device address of the transaction translator's hub."]
pub type HUBADDR_R = crate::FieldReader;
#[doc = "Field `HUBADDR` writer - Hub Address (HubAddr) This field holds the device address of the transaction translator's hub."]
pub type HUBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XACTPOS` reader - Transaction Position (XactPos) This field is used to determine whether to send all, first, middle, or last payloads with each OUT transaction. - 2'b11: All. This is the entire data payload is of this transaction (which is less than or equal to 188 bytes). - 2'b10: Begin. This is the first data payload of this transaction (which is larger than 188 bytes). - 2'b00: Mid. This is the middle payload of this transaction (which is larger than 188 bytes). - 2'b01: End. This is the last payload of this transaction (which is larger than 188 bytes)."]
pub type XACTPOS_R = crate::FieldReader;
#[doc = "Field `XACTPOS` writer - Transaction Position (XactPos) This field is used to determine whether to send all, first, middle, or last payloads with each OUT transaction. - 2'b11: All. This is the entire data payload is of this transaction (which is less than or equal to 188 bytes). - 2'b10: Begin. This is the first data payload of this transaction (which is larger than 188 bytes). - 2'b00: Mid. This is the middle payload of this transaction (which is larger than 188 bytes). - 2'b01: End. This is the last payload of this transaction (which is larger than 188 bytes)."]
pub type XACTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPSPLT` reader - Do Complete Split (CompSplt) The application sets this field to request the OTG host to perform a complete split transaction."]
pub type COMPSPLT_R = crate::BitReader;
#[doc = "Field `COMPSPLT` writer - Do Complete Split (CompSplt) The application sets this field to request the OTG host to perform a complete split transaction."]
pub type COMPSPLT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLTENA` reader - Split Enable (SpltEna) The application sets this field to indicate that this channel is enabled to perform split transactions."]
pub type SPLTENA_R = crate::BitReader;
#[doc = "Field `SPLTENA` writer - Split Enable (SpltEna) The application sets this field to indicate that this channel is enabled to perform split transactions."]
pub type SPLTENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Port Address (PrtAddr) This field is the port number of the recipient transaction translator."]
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub Address (HubAddr) This field holds the device address of the transaction translator's hub."]
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - Transaction Position (XactPos) This field is used to determine whether to send all, first, middle, or last payloads with each OUT transaction. - 2'b11: All. This is the entire data payload is of this transaction (which is less than or equal to 188 bytes). - 2'b10: Begin. This is the first data payload of this transaction (which is larger than 188 bytes). - 2'b00: Mid. This is the middle payload of this transaction (which is larger than 188 bytes). - 2'b01: End. This is the last payload of this transaction (which is larger than 188 bytes)."]
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Do Complete Split (CompSplt) The application sets this field to request the OTG host to perform a complete split transaction."]
    #[inline(always)]
    pub fn compsplt(&self) -> COMPSPLT_R {
        COMPSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Split Enable (SpltEna) The application sets this field to indicate that this channel is enabled to perform split transactions."]
    #[inline(always)]
    pub fn spltena(&self) -> SPLTENA_R {
        SPLTENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCSPLT5")
            .field("prtaddr", &self.prtaddr())
            .field("hubaddr", &self.hubaddr())
            .field("xactpos", &self.xactpos())
            .field("compsplt", &self.compsplt())
            .field("spltena", &self.spltena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Port Address (PrtAddr) This field is the port number of the recipient transaction translator."]
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PRTADDR_W<'_, HCSPLT5_SPEC> {
        PRTADDR_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Hub Address (HubAddr) This field holds the device address of the transaction translator's hub."]
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HUBADDR_W<'_, HCSPLT5_SPEC> {
        HUBADDR_W::new(self, 7)
    }
    #[doc = "Bits 14:15 - Transaction Position (XactPos) This field is used to determine whether to send all, first, middle, or last payloads with each OUT transaction. - 2'b11: All. This is the entire data payload is of this transaction (which is less than or equal to 188 bytes). - 2'b10: Begin. This is the first data payload of this transaction (which is larger than 188 bytes). - 2'b00: Mid. This is the middle payload of this transaction (which is larger than 188 bytes). - 2'b01: End. This is the last payload of this transaction (which is larger than 188 bytes)."]
    #[inline(always)]
    pub fn xactpos(&mut self) -> XACTPOS_W<'_, HCSPLT5_SPEC> {
        XACTPOS_W::new(self, 14)
    }
    #[doc = "Bit 16 - Do Complete Split (CompSplt) The application sets this field to request the OTG host to perform a complete split transaction."]
    #[inline(always)]
    pub fn compsplt(&mut self) -> COMPSPLT_W<'_, HCSPLT5_SPEC> {
        COMPSPLT_W::new(self, 16)
    }
    #[doc = "Bit 31 - Split Enable (SpltEna) The application sets this field to indicate that this channel is enabled to perform split transactions."]
    #[inline(always)]
    pub fn spltena(&mut self) -> SPLTENA_W<'_, HCSPLT5_SPEC> {
        SPLTENA_W::new(self, 31)
    }
}
#[doc = "This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCSPLT5_SPEC;
impl crate::RegisterSpec for HCSPLT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcsplt5::R`](R) reader structure"]
impl crate::Readable for HCSPLT5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcsplt5::W`](W) writer structure"]
impl crate::Writable for HCSPLT5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCSPLT5 to value 0"]
impl crate::Resettable for HCSPLT5_SPEC {}
