#[doc = "Register `MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS` reader"]
pub type R = crate::R<MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS_SPEC>;
#[doc = "Register `MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS` writer"]
pub type W = crate::W<MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS_SPEC>;
#[doc = "Field `ADDRLO_6` reader - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_6_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO_6` writer - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_6(&self) -> ADDRLO_6_R {
        ADDRLO_6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS")
            .field("addrlo_6", &self.addrlo_6())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_6(
        &mut self,
    ) -> ADDRLO_6_W<'_, MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS_SPEC>
    {
        ADDRLO_6_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS_SPEC;
impl crate::RegisterSpec
    for MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS_SPEC
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress::R`](R) reader structure"]
impl crate::Readable for MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macaddress6lowregistercontainsthelower32bitsoftheseventhmacaddress::W`](W) writer structure"]
impl crate::Writable for MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS to value 0xffff_ffff"]
impl crate::Resettable for MACADDRESS6LOWREGISTERCONTAINSTHELOWER32BITSOFTHESEVENTHMACADDRESS_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
