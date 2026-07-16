#[doc = "Register `MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS` reader"]
pub type R = crate::R<MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS_SPEC>;
#[doc = "Register `MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS` writer"]
pub type W = crate::W<MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS_SPEC>;
#[doc = "Field `ADDRLO_2` reader - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_2_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO_2` writer - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type ADDRLO_2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_2(&self) -> ADDRLO_2_R {
        ADDRLO_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS")
            .field("addrlo_2", &self.addrlo_2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrlo_2(
        &mut self,
    ) -> ADDRLO_2_W<'_, MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS_SPEC> {
        ADDRLO_2_W::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS_SPEC;
impl crate::RegisterSpec for MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress::R`](R) reader structure"]
impl crate::Readable for MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macaddress2lowregistercontainsthelower32bitsofthethirdmacaddress::W`](W) writer structure"]
impl crate::Writable for MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS to value 0xffff_ffff"]
impl crate::Resettable for MACADDRESS2LOWREGISTERCONTAINSTHELOWER32BITSOFTHETHIRDMACADDRESS_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
