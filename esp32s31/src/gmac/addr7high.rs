#[doc = "Register `ADDR7HIGH` reader"]
pub type R = crate::R<ADDR7HIGH_SPEC>;
#[doc = "Register `ADDR7HIGH` writer"]
pub type W = crate::W<ADDR7HIGH_SPEC>;
#[doc = "Field `ADDRHI_7` reader - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_7_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI_7` writer - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_7_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC_7` reader - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
pub type MBC_7_R = crate::FieldReader;
#[doc = "Field `MBC_7` writer - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
pub type MBC_7_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA_7` reader - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
pub type SA_7_R = crate::BitReader;
#[doc = "Field `SA_7` writer - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
pub type SA_7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_7` reader - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
pub type AE_7_R = crate::BitReader;
#[doc = "Field `AE_7` writer - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
pub type AE_7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_7(&self) -> ADDRHI_7_R {
        ADDRHI_7_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_7(&self) -> MBC_7_R {
        MBC_7_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_7(&self) -> SA_7_R {
        SA_7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_7(&self) -> AE_7_R {
        AE_7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR7HIGH")
            .field("addrhi_7", &self.addrhi_7())
            .field("mbc_7", &self.mbc_7())
            .field("sa_7", &self.sa_7())
            .field("ae_7", &self.ae_7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_7(&mut self) -> ADDRHI_7_W<'_, ADDR7HIGH_SPEC> {
        ADDRHI_7_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_7(&mut self) -> MBC_7_W<'_, ADDR7HIGH_SPEC> {
        MBC_7_W::new(self, 24)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_7(&mut self) -> SA_7_W<'_, ADDR7HIGH_SPEC> {
        SA_7_W::new(self, 30)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address7 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_7(&mut self) -> AE_7_W<'_, ADDR7HIGH_SPEC> {
        AE_7_W::new(self, 31)
    }
}
#[doc = "Upper 16 bits of the eighth 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr7high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr7high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR7HIGH_SPEC;
impl crate::RegisterSpec for ADDR7HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr7high::R`](R) reader structure"]
impl crate::Readable for ADDR7HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr7high::W`](W) writer structure"]
impl crate::Writable for ADDR7HIGH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR7HIGH to value 0xffff"]
impl crate::Resettable for ADDR7HIGH_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
