#[doc = "Register `MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS` reader"]
pub type R = crate::R<MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC>;
#[doc = "Register `MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS` writer"]
pub type W = crate::W<MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC>;
#[doc = "Field `ADDRHI_6` reader - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_6_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI_6` writer - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_6_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC_6` reader - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
pub type MBC_6_R = crate::FieldReader;
#[doc = "Field `MBC_6` writer - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
pub type MBC_6_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA_6` reader - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
pub type SA_6_R = crate::BitReader;
#[doc = "Field `SA_6` writer - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
pub type SA_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_6` reader - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
pub type AE_6_R = crate::BitReader;
#[doc = "Field `AE_6` writer - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
pub type AE_6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_6(&self) -> ADDRHI_6_R {
        ADDRHI_6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_6(&self) -> MBC_6_R {
        MBC_6_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_6(&self) -> SA_6_R {
        SA_6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_6(&self) -> AE_6_R {
        AE_6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS")
            .field("addrhi_6", &self.addrhi_6())
            .field("mbc_6", &self.mbc_6())
            .field("sa_6", &self.sa_6())
            .field("ae_6", &self.ae_6())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_6(
        &mut self,
    ) -> ADDRHI_6_W<'_, MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC>
    {
        ADDRHI_6_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_6(
        &mut self,
    ) -> MBC_6_W<'_, MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC>
    {
        MBC_6_W::new(self, 24)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_6(
        &mut self,
    ) -> SA_6_W<'_, MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC> {
        SA_6_W::new(self, 30)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address6 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_6(
        &mut self,
    ) -> AE_6_W<'_, MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC> {
        AE_6_W::new(self, 31)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC;
impl crate::RegisterSpec
    for MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress::R`](R) reader structure"]
impl crate::Readable for MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macaddress6highregistercontainsthehigher16bitsoftheseventhmacaddress::W`](W) writer structure"]
impl crate::Writable for MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS to value 0xffff"]
impl crate::Resettable
    for MACADDRESS6HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHESEVENTHMACADDRESS_SPEC
{
    const RESET_VALUE: u32 = 0xffff;
}
