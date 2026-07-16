#[doc = "Register `REGISTER16_MACADDRESS0HIGHREGISTER` reader"]
pub type R = crate::R<REGISTER16_MACADDRESS0HIGHREGISTER_SPEC>;
#[doc = "Register `REGISTER16_MACADDRESS0HIGHREGISTER` writer"]
pub type W = crate::W<REGISTER16_MACADDRESS0HIGHREGISTER_SPEC>;
#[doc = "Field `ADDRHI_0` reader - MAC Address0 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the first 6byte MAC address The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
pub type ADDRHI_0_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI_0` writer - MAC Address0 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the first 6byte MAC address The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
pub type ADDRHI_0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AE_0` reader - Address Enable This bit is always set to 1"]
pub type AE_0_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the first 6byte MAC address The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
    #[inline(always)]
    pub fn addrhi_0(&self) -> ADDRHI_0_R {
        ADDRHI_0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address Enable This bit is always set to 1"]
    #[inline(always)]
    pub fn ae_0(&self) -> AE_0_R {
        AE_0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER16_MACADDRESS0HIGHREGISTER")
            .field("addrhi_0", &self.addrhi_0())
            .field("ae_0", &self.ae_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address0 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the first 6byte MAC address The MAC uses this field for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
    #[inline(always)]
    pub fn addrhi_0(&mut self) -> ADDRHI_0_W<'_, REGISTER16_MACADDRESS0HIGHREGISTER_SPEC> {
        ADDRHI_0_W::new(self, 0)
    }
}
#[doc = "Contains the higher 16 bits of the first MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`register16_macaddress0highregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register16_macaddress0highregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER16_MACADDRESS0HIGHREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER16_MACADDRESS0HIGHREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register16_macaddress0highregister::R`](R) reader structure"]
impl crate::Readable for REGISTER16_MACADDRESS0HIGHREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register16_macaddress0highregister::W`](W) writer structure"]
impl crate::Writable for REGISTER16_MACADDRESS0HIGHREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER16_MACADDRESS0HIGHREGISTER to value 0x8000_ffff"]
impl crate::Resettable for REGISTER16_MACADDRESS0HIGHREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x8000_ffff;
}
