#[doc = "Register `MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS` reader"]
pub type R = crate::R<MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC>;
#[doc = "Register `MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS` writer"]
pub type W = crate::W<MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC>;
#[doc = "Field `ADDRHI_2` reader - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_2_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI_2` writer - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC_2` reader - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type MBC_2_R = crate::FieldReader;
#[doc = "Field `MBC_2` writer - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type MBC_2_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA_2` reader - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type SA_2_R = crate::BitReader;
#[doc = "Field `SA_2` writer - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type SA_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_2` reader - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type AE_2_R = crate::BitReader;
#[doc = "Field `AE_2` writer - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
pub type AE_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_2(&self) -> ADDRHI_2_R {
        ADDRHI_2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_2(&self) -> MBC_2_R {
        MBC_2_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_2(&self) -> SA_2_R {
        SA_2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_2(&self) -> AE_2_R {
        AE_2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS")
            .field("addrhi_2", &self.addrhi_2())
            .field("mbc_2", &self.mbc_2())
            .field("sa_2", &self.sa_2())
            .field("ae_2", &self.ae_2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_2(
        &mut self,
    ) -> ADDRHI_2_W<'_, MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC>
    {
        ADDRHI_2_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_2(
        &mut self,
    ) -> MBC_2_W<'_, MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC> {
        MBC_2_W::new(self, 24)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_2(
        &mut self,
    ) -> SA_2_W<'_, MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC> {
        SA_2_W::new(self, 30)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address2 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_2(
        &mut self,
    ) -> AE_2_W<'_, MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC> {
        AE_2_W::new(self, 31)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC;
impl crate::RegisterSpec
    for MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress::R`](R) reader structure"]
impl crate::Readable for MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macaddress2highregistercontainsthehigher16bitsofthethirdmacaddress::W`](W) writer structure"]
impl crate::Writable for MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS to value 0xffff"]
impl crate::Resettable for MACADDRESS2HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHETHIRDMACADDRESS_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
