#[doc = "Register `ADDR0HIGH` reader"]
pub type R = crate::R<ADDR0HIGH_SPEC>;
#[doc = "Register `ADDR0HIGH` writer"]
pub type W = crate::W<ADDR0HIGH_SPEC>;
#[doc = "Field `ADDRESS0_HI` reader - MAC Address0 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the first 6byte MAC address The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
pub type ADDRESS0_HI_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRESS0_HI` writer - MAC Address0 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the first 6byte MAC address The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
pub type ADDRESS0_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRESS_ENABLE0` reader - Address Enable This bit is always set to 1"]
pub type ADDRESS_ENABLE0_R = crate::BitReader;
#[doc = "Field `ADDRESS_ENABLE0` writer - Address Enable This bit is always set to 1"]
pub type ADDRESS_ENABLE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the first 6byte MAC address The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
    #[inline(always)]
    pub fn address0_hi(&self) -> ADDRESS0_HI_R {
        ADDRESS0_HI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address Enable This bit is always set to 1"]
    #[inline(always)]
    pub fn address_enable0(&self) -> ADDRESS_ENABLE0_R {
        ADDRESS_ENABLE0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR0HIGH")
            .field("address0_hi", &self.address0_hi())
            .field("address_enable0", &self.address_enable0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the first 6byte MAC address The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
    #[inline(always)]
    pub fn address0_hi(&mut self) -> ADDRESS0_HI_W<'_, ADDR0HIGH_SPEC> {
        ADDRESS0_HI_W::new(self, 0)
    }
    #[doc = "Bit 31 - Address Enable This bit is always set to 1"]
    #[inline(always)]
    pub fn address_enable0(&mut self) -> ADDRESS_ENABLE0_W<'_, ADDR0HIGH_SPEC> {
        ADDRESS_ENABLE0_W::new(self, 31)
    }
}
#[doc = "Upper 16 bits of the first 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr0high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr0high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR0HIGH_SPEC;
impl crate::RegisterSpec for ADDR0HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr0high::R`](R) reader structure"]
impl crate::Readable for ADDR0HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr0high::W`](W) writer structure"]
impl crate::Writable for ADDR0HIGH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR0HIGH to value 0x8000_ffff"]
impl crate::Resettable for ADDR0HIGH_SPEC {
    const RESET_VALUE: u32 = 0x8000_ffff;
}
