#[doc = "Register `MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS` reader"]
pub type R = crate::R<MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC>;
#[doc = "Register `MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS` writer"]
pub type W = crate::W<MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC>;
#[doc = "Field `ADDRHI_4` reader - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_4_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI_4` writer - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_4_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC_4` reader - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
pub type MBC_4_R = crate::FieldReader;
#[doc = "Field `MBC_4` writer - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
pub type MBC_4_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA_4` reader - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
pub type SA_4_R = crate::BitReader;
#[doc = "Field `SA_4` writer - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
pub type SA_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_4` reader - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
pub type AE_4_R = crate::BitReader;
#[doc = "Field `AE_4` writer - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
pub type AE_4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_4(&self) -> ADDRHI_4_R {
        ADDRHI_4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_4(&self) -> MBC_4_R {
        MBC_4_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_4(&self) -> SA_4_R {
        SA_4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_4(&self) -> AE_4_R {
        AE_4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS")
            .field("addrhi_4", &self.addrhi_4())
            .field("mbc_4", &self.mbc_4())
            .field("sa_4", &self.sa_4())
            .field("ae_4", &self.ae_4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_4(
        &mut self,
    ) -> ADDRHI_4_W<'_, MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC>
    {
        ADDRHI_4_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_4(
        &mut self,
    ) -> MBC_4_W<'_, MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC> {
        MBC_4_W::new(self, 24)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_4(
        &mut self,
    ) -> SA_4_W<'_, MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC> {
        SA_4_W::new(self, 30)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address4 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_4(
        &mut self,
    ) -> AE_4_W<'_, MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC> {
        AE_4_W::new(self, 31)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC;
impl crate::RegisterSpec
    for MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress::R`](R) reader structure"]
impl crate::Readable for MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macaddress4highregistercontainsthehigher16bitsofthefifthmacaddress::W`](W) writer structure"]
impl crate::Writable for MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS to value 0xffff"]
impl crate::Resettable for MACADDRESS4HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFIFTHMACADDRESS_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
