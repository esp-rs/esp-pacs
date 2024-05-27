#[doc = "Register `EMACADDR0HIGH` reader"]
pub type R = crate::R<EMACADDR0HIGH_SPEC>;
#[doc = "Register `EMACADDR0HIGH` writer"]
pub type W = crate::W<EMACADDR0HIGH_SPEC>;
#[doc = "Field `ADDRESS0_HI` reader - This field contains the upper 16 bits (47:32) of the first 6-byte MAC address.The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
pub type ADDRESS0_HI_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRESS0_HI` writer - This field contains the upper 16 bits (47:32) of the first 6-byte MAC address.The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
pub type ADDRESS0_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRESS_ENABLE0` reader - This bit is always set to 1."]
pub type ADDRESS_ENABLE0_R = crate::BitReader;
#[doc = "Field `ADDRESS_ENABLE0` writer - This bit is always set to 1."]
pub type ADDRESS_ENABLE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits (47:32) of the first 6-byte MAC address.The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
    #[inline(always)]
    pub fn address0_hi(&self) -> ADDRESS0_HI_R {
        ADDRESS0_HI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - This bit is always set to 1."]
    #[inline(always)]
    pub fn address_enable0(&self) -> ADDRESS_ENABLE0_R {
        ADDRESS_ENABLE0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACADDR0HIGH")
            .field("address0_hi", &self.address0_hi())
            .field("address_enable0", &self.address_enable0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This field contains the upper 16 bits (47:32) of the first 6-byte MAC address.The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames."]
    #[inline(always)]
    #[must_use]
    pub fn address0_hi(&mut self) -> ADDRESS0_HI_W<EMACADDR0HIGH_SPEC> {
        ADDRESS0_HI_W::new(self, 0)
    }
    #[doc = "Bit 31 - This bit is always set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn address_enable0(&mut self) -> ADDRESS_ENABLE0_W<EMACADDR0HIGH_SPEC> {
        ADDRESS_ENABLE0_W::new(self, 31)
    }
}
#[doc = "Upper 16 bits of the first 6-byte MAC address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr0high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr0high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACADDR0HIGH_SPEC;
impl crate::RegisterSpec for EMACADDR0HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacaddr0high::R`](R) reader structure"]
impl crate::Readable for EMACADDR0HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacaddr0high::W`](W) writer structure"]
impl crate::Writable for EMACADDR0HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMACADDR0HIGH to value 0"]
impl crate::Resettable for EMACADDR0HIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
