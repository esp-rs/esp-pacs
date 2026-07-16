#[doc = "Register `MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS` reader"]
pub type R = crate::R<MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC>;
#[doc = "Register `MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS` writer"]
pub type W = crate::W<MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC>;
#[doc = "Field `ADDRHI_5` reader - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_5_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI_5` writer - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_5_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC_5` reader - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
pub type MBC_5_R = crate::FieldReader;
#[doc = "Field `MBC_5` writer - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
pub type MBC_5_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA_5` reader - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
pub type SA_5_R = crate::BitReader;
#[doc = "Field `SA_5` writer - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
pub type SA_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_5` reader - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
pub type AE_5_R = crate::BitReader;
#[doc = "Field `AE_5` writer - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
pub type AE_5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_5(&self) -> ADDRHI_5_R {
        ADDRHI_5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_5(&self) -> MBC_5_R {
        MBC_5_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_5(&self) -> SA_5_R {
        SA_5_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_5(&self) -> AE_5_R {
        AE_5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS")
            .field("addrhi_5", &self.addrhi_5())
            .field("mbc_5", &self.mbc_5())
            .field("sa_5", &self.sa_5())
            .field("ae_5", &self.ae_5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_5(
        &mut self,
    ) -> ADDRHI_5_W<'_, MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC>
    {
        ADDRHI_5_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_5(
        &mut self,
    ) -> MBC_5_W<'_, MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC> {
        MBC_5_W::new(self, 24)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_5(
        &mut self,
    ) -> SA_5_W<'_, MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC> {
        SA_5_W::new(self, 30)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address5 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_5(
        &mut self,
    ) -> AE_5_W<'_, MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC> {
        AE_5_W::new(self, 31)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC;
impl crate::RegisterSpec
    for MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress::R`](R) reader structure"]
impl crate::Readable for MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macaddress5highregistercontainsthehigher16bitsofthesixthmacaddress::W`](W) writer structure"]
impl crate::Writable for MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS to value 0xffff"]
impl crate::Resettable for MACADDRESS5HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESIXTHMACADDRESS_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
