#[doc = "Register `REGISTER544_MACADDRESS32HIGHREGISTER` reader"]
pub type R = crate::R<REGISTER544_MACADDRESS32HIGHREGISTER_SPEC>;
#[doc = "Register `REGISTER544_MACADDRESS32HIGHREGISTER` writer"]
pub type W = crate::W<REGISTER544_MACADDRESS32HIGHREGISTER_SPEC>;
#[doc = "Field `ADDRHI` reader - MAC Address32 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the 33rd 6byte MAC address"]
pub type ADDRHI_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI` writer - MAC Address32 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the 33rd 6byte MAC address"]
pub type ADDRHI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AE` reader - Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering When reset, the address filter module ignores the address for filtering"]
pub type AE_R = crate::BitReader;
#[doc = "Field `AE` writer - Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering When reset, the address filter module ignores the address for filtering"]
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC Address32 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the 33rd 6byte MAC address"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering When reset, the address filter module ignores the address for filtering"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER544_MACADDRESS32HIGHREGISTER")
            .field("addrhi", &self.addrhi())
            .field("ae", &self.ae())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address32 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the 33rd 6byte MAC address"]
    #[inline(always)]
    pub fn addrhi(&mut self) -> ADDRHI_W<'_, REGISTER544_MACADDRESS32HIGHREGISTER_SPEC> {
        ADDRHI_W::new(self, 0)
    }
    #[doc = "Bit 31 - Address Enable When this bit is set, the Address filter module uses the 33rd MAC address for perfect filtering When reset, the address filter module ignores the address for filtering"]
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W<'_, REGISTER544_MACADDRESS32HIGHREGISTER_SPEC> {
        AE_W::new(self, 31)
    }
}
#[doc = "Contains the higher 16 bits of the 33rd MAC address This register is present only when Enable Additional 32 MAC Address Registers is selected in coreConsultant _See Table 78_\n\nYou can [`read`](crate::Reg::read) this register and get [`register544_macaddress32highregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register544_macaddress32highregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER544_MACADDRESS32HIGHREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER544_MACADDRESS32HIGHREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register544_macaddress32highregister::R`](R) reader structure"]
impl crate::Readable for REGISTER544_MACADDRESS32HIGHREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register544_macaddress32highregister::W`](W) writer structure"]
impl crate::Writable for REGISTER544_MACADDRESS32HIGHREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER544_MACADDRESS32HIGHREGISTER to value 0xffff"]
impl crate::Resettable for REGISTER544_MACADDRESS32HIGHREGISTER_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
