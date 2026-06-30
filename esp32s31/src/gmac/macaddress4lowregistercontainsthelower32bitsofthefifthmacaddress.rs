#[doc = "Register `MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS` reader"]
pub type R = crate::R<MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS_SPEC>;
#[doc = "Register `MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS` writer"]
pub type W = crate::W<MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS_SPEC>;
#[doc = "Field `ADDRLO_4` reader - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_4_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO_4` writer - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_4(&self) -> ADDRLO_4_R {
        ADDRLO_4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS")
            .field("addrlo_4", &self.addrlo_4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_4(
        &mut self,
    ) -> ADDRLO_4_W<'_, MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS_SPEC> {
        ADDRLO_4_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS_SPEC;
impl crate::RegisterSpec for MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress::R`](R) reader structure"]
impl crate::Readable for MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macaddress4lowregistercontainsthelower32bitsofthefifthmacaddress::W`](W) writer structure"]
impl crate::Writable for MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS to value 0xffff_ffff"]
impl crate::Resettable for MACADDRESS4LOWREGISTERCONTAINSTHELOWER32BITSOFTHEFIFTHMACADDRESS_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
