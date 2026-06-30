#[doc = "Register `MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS` reader"]
pub type R = crate::R<MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC>;
#[doc = "Register `MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS` writer"]
pub type W = crate::W<MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC>;
#[doc = "Field `ADDRHI_3` reader - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_3_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI_3` writer - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
pub type ADDRHI_3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC_3` reader - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
pub type MBC_3_R = crate::FieldReader;
#[doc = "Field `MBC_3` writer - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
pub type MBC_3_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA_3` reader - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
pub type SA_3_R = crate::BitReader;
#[doc = "Field `SA_3` writer - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
pub type SA_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_3` reader - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
pub type AE_3_R = crate::BitReader;
#[doc = "Field `AE_3` writer - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
pub type AE_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_3(&self) -> ADDRHI_3_R {
        ADDRHI_3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_3(&self) -> MBC_3_R {
        MBC_3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_3(&self) -> SA_3_R {
        SA_3_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_3(&self) -> AE_3_R {
        AE_3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS")
            .field("addrhi_3", &self.addrhi_3())
            .field("mbc_3", &self.mbc_3())
            .field("sa_3", &self.sa_3())
            .field("ae_3", &self.ae_3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn addrhi_3(
        &mut self,
    ) -> ADDRHI_3_W<'_, MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC>
    {
        ADDRHI_3_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn mbc_3(
        &mut self,
    ) -> MBC_3_W<'_, MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC> {
        MBC_3_W::new(self, 24)
    }
    #[doc = "Bit 30 - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn sa_3(
        &mut self,
    ) -> SA_3_W<'_, MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC> {
        SA_3_W::new(self, 30)
    }
    #[doc = "Bit 31 - This register is present only when Enable MAC Address3 is selected in coreConsultant _See Table 78_"]
    #[inline(always)]
    pub fn ae_3(
        &mut self,
    ) -> AE_3_W<'_, MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC> {
        AE_3_W::new(self, 31)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC;
impl crate::RegisterSpec
    for MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress::R`](R) reader structure"]
impl crate::Readable for MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macaddress3highregistercontainsthehigher16bitsofthefourthmacaddress::W`](W) writer structure"]
impl crate::Writable for MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS to value 0xffff"]
impl crate::Resettable
    for MACADDRESS3HIGHREGISTERCONTAINSTHEHIGHER16BITSOFTHEFOURTHMACADDRESS_SPEC
{
    const RESET_VALUE: u32 = 0xffff;
}
