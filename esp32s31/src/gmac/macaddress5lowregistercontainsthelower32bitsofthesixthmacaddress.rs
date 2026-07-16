#[doc = "Register `MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS` reader"]
pub type R = crate::R<MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS_SPEC>;
#[doc = "Register `MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS` writer"]
pub type W = crate::W<MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS_SPEC>;
#[doc = "Field `ADDRLO_5` reader - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_5_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO_5` writer - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_5(&self) -> ADDRLO_5_R {
        ADDRLO_5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS")
            .field("addrlo_5", &self.addrlo_5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_5(
        &mut self,
    ) -> ADDRLO_5_W<'_, MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS_SPEC> {
        ADDRLO_5_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS_SPEC;
impl crate::RegisterSpec for MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress::R`](R) reader structure"]
impl crate::Readable for MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macaddress5lowregistercontainsthelower32bitsofthesixthmacaddress::W`](W) writer structure"]
impl crate::Writable for MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS to value 0xffff_ffff"]
impl crate::Resettable for MACADDRESS5LOWREGISTERCONTAINSTHELOWER32BITSOFTHESIXTHMACADDRESS_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
