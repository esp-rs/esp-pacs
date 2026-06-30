#[doc = "Register `MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS` reader"]
pub type R = crate::R<MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS_SPEC>;
#[doc = "Register `MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS` writer"]
pub type W = crate::W<MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS_SPEC>;
#[doc = "Field `ADDRLO_7` reader - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_7_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO_7` writer - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_7(&self) -> ADDRLO_7_R {
        ADDRLO_7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS")
            .field("addrlo_7", &self.addrlo_7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_7(
        &mut self,
    ) -> ADDRLO_7_W<'_, MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS_SPEC>
    {
        ADDRLO_7_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS_SPEC;
impl crate::RegisterSpec
    for MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS_SPEC
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress::R`](R) reader structure"]
impl crate::Readable for MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macaddress7lowregistercontainsthelower32bitsoftheeighthmacaddress::W`](W) writer structure"]
impl crate::Writable for MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS to value 0xffff_ffff"]
impl crate::Resettable for MACADDRESS7LOWREGISTERCONTAINSTHELOWER32BITSOFTHEEIGHTHMACADDRESS_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
