#[doc = "Register `HCINTMSK2` reader"]
pub type R = crate::R<HCINTMSK2_SPEC>;
#[doc = "Register `HCINTMSK2` writer"]
pub type W = crate::W<HCINTMSK2_SPEC>;
#[doc = "Field `XFERCOMPLMSK` reader - Transfer Completed Mask (XferComplMsk)"]
pub type XFERCOMPLMSK_R = crate::BitReader;
#[doc = "Field `XFERCOMPLMSK` writer - Transfer Completed Mask (XferComplMsk)"]
pub type XFERCOMPLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTDMSK` reader - Channel Halted Mask (ChHltdMsk)"]
pub type CHHLTDMSK_R = crate::BitReader;
#[doc = "Field `CHHLTDMSK` writer - Channel Halted Mask (ChHltdMsk)"]
pub type CHHLTDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRMSK` reader - AHB Error Mask (AHBErrMsk) In scatter/gather DMA mode for host, interrupts are not generated due to the corresponding bits set in HCINTn."]
pub type AHBERRMSK_R = crate::BitReader;
#[doc = "Field `AHBERRMSK` writer - AHB Error Mask (AHBErrMsk) In scatter/gather DMA mode for host, interrupts are not generated due to the corresponding bits set in HCINTn."]
pub type AHBERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINTRMSK` reader - BNA (Buffer Not Available) Interrupt mask register (BNAIntrMsk) This bit is valid only when Scatter/Gather DMA mode is enabled."]
pub type BNAINTRMSK_R = crate::BitReader;
#[doc = "Field `BNAINTRMSK` writer - BNA (Buffer Not Available) Interrupt mask register (BNAIntrMsk) This bit is valid only when Scatter/Gather DMA mode is enabled."]
pub type BNAINTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LST_ROLLINTRMSK` reader - Descriptor List rollover interrupt Mask register(DESC_LST_ROLLIntrMsk) This bit is valid only when Scatter/Gather DMA mode is enabled."]
pub type DESC_LST_ROLLINTRMSK_R = crate::BitReader;
#[doc = "Field `DESC_LST_ROLLINTRMSK` writer - Descriptor List rollover interrupt Mask register(DESC_LST_ROLLIntrMsk) This bit is valid only when Scatter/Gather DMA mode is enabled."]
pub type DESC_LST_ROLLINTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Mask (XferComplMsk)"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted Mask (ChHltdMsk)"]
    #[inline(always)]
    pub fn chhltdmsk(&self) -> CHHLTDMSK_R {
        CHHLTDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask (AHBErrMsk) In scatter/gather DMA mode for host, interrupts are not generated due to the corresponding bits set in HCINTn."]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt mask register (BNAIntrMsk) This bit is valid only when Scatter/Gather DMA mode is enabled."]
    #[inline(always)]
    pub fn bnaintrmsk(&self) -> BNAINTRMSK_R {
        BNAINTRMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Descriptor List rollover interrupt Mask register(DESC_LST_ROLLIntrMsk) This bit is valid only when Scatter/Gather DMA mode is enabled."]
    #[inline(always)]
    pub fn desc_lst_rollintrmsk(&self) -> DESC_LST_ROLLINTRMSK_R {
        DESC_LST_ROLLINTRMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK2")
            .field("xfercomplmsk", &self.xfercomplmsk())
            .field("chhltdmsk", &self.chhltdmsk())
            .field("ahberrmsk", &self.ahberrmsk())
            .field("bnaintrmsk", &self.bnaintrmsk())
            .field("desc_lst_rollintrmsk", &self.desc_lst_rollintrmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Mask (XferComplMsk)"]
    #[inline(always)]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W<'_, HCINTMSK2_SPEC> {
        XFERCOMPLMSK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Halted Mask (ChHltdMsk)"]
    #[inline(always)]
    pub fn chhltdmsk(&mut self) -> CHHLTDMSK_W<'_, HCINTMSK2_SPEC> {
        CHHLTDMSK_W::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error Mask (AHBErrMsk) In scatter/gather DMA mode for host, interrupts are not generated due to the corresponding bits set in HCINTn."]
    #[inline(always)]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W<'_, HCINTMSK2_SPEC> {
        AHBERRMSK_W::new(self, 2)
    }
    #[doc = "Bit 11 - BNA (Buffer Not Available) Interrupt mask register (BNAIntrMsk) This bit is valid only when Scatter/Gather DMA mode is enabled."]
    #[inline(always)]
    pub fn bnaintrmsk(&mut self) -> BNAINTRMSK_W<'_, HCINTMSK2_SPEC> {
        BNAINTRMSK_W::new(self, 11)
    }
    #[doc = "Bit 13 - Descriptor List rollover interrupt Mask register(DESC_LST_ROLLIntrMsk) This bit is valid only when Scatter/Gather DMA mode is enabled."]
    #[inline(always)]
    pub fn desc_lst_rollintrmsk(&mut self) -> DESC_LST_ROLLINTRMSK_W<'_, HCINTMSK2_SPEC> {
        DESC_LST_ROLLINTRMSK_W::new(self, 13)
    }
}
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINTMSK2_SPEC;
impl crate::RegisterSpec for HCINTMSK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk2::R`](R) reader structure"]
impl crate::Readable for HCINTMSK2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk2::W`](W) writer structure"]
impl crate::Writable for HCINTMSK2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCINTMSK2 to value 0"]
impl crate::Resettable for HCINTMSK2_SPEC {}
