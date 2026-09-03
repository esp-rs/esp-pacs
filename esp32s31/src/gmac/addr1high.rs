#[doc = "Register `ADDR1HIGH` reader"]
pub type R = crate::R<ADDR1HIGH_SPEC>;
#[doc = "Register `ADDR1HIGH` writer"]
pub type W = crate::W<ADDR1HIGH_SPEC>;
#[doc = "Field `ADDRHI_1` reader - MAC Address1 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the second 6byte MAC address"]
pub type ADDRHI_1_R = crate::FieldReader<u16>;
#[doc = "Field `ADDRHI_1` writer - MAC Address1 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the second 6byte MAC address"]
pub type ADDRHI_1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC_1` reader - Mask Byte Control These bits are mask control bits for comparison of each of the MAC Address bytes When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers Each bit controls the masking of the bytes as follows: Bit 29: Register 18\\[15:8\\] Bit 28: Register 18\\[7:0\\] Bit 27: Register 19\\[31:24\\] Bit 24: Register 19\\[7:0\\] You can filter a group of addresses _known as group address filtering_ by masking one or more bytes of the address"]
pub type MBC_1_R = crate::FieldReader;
#[doc = "Field `MBC_1` writer - Mask Byte Control These bits are mask control bits for comparison of each of the MAC Address bytes When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers Each bit controls the masking of the bytes as follows: Bit 29: Register 18\\[15:8\\] Bit 28: Register 18\\[7:0\\] Bit 27: Register 19\\[31:24\\] Bit 24: Register 19\\[7:0\\] You can filter a group of addresses _known as group address filtering_ by masking one or more bytes of the address"]
pub type MBC_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA_1` reader - Source Address When this bit is set, the MAC Address1\\[47:0\\] is used to compare with the SA fields of the received frame When this bit is reset, the MAC Address1\\[47:0\\] is used to compare with the DA fields of the received frame"]
pub type SA_1_R = crate::BitReader;
#[doc = "Field `SA_1` writer - Source Address When this bit is set, the MAC Address1\\[47:0\\] is used to compare with the SA fields of the received frame When this bit is reset, the MAC Address1\\[47:0\\] is used to compare with the DA fields of the received frame"]
pub type SA_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_1` reader - Address Enable"]
pub type AE_1_R = crate::BitReader;
#[doc = "Field `AE_1` writer - Address Enable"]
pub type AE_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the second 6byte MAC address"]
    #[inline(always)]
    pub fn addrhi_1(&self) -> ADDRHI_1_R {
        ADDRHI_1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask Byte Control These bits are mask control bits for comparison of each of the MAC Address bytes When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers Each bit controls the masking of the bytes as follows: Bit 29: Register 18\\[15:8\\] Bit 28: Register 18\\[7:0\\] Bit 27: Register 19\\[31:24\\] Bit 24: Register 19\\[7:0\\] You can filter a group of addresses _known as group address filtering_ by masking one or more bytes of the address"]
    #[inline(always)]
    pub fn mbc_1(&self) -> MBC_1_R {
        MBC_1_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source Address When this bit is set, the MAC Address1\\[47:0\\] is used to compare with the SA fields of the received frame When this bit is reset, the MAC Address1\\[47:0\\] is used to compare with the DA fields of the received frame"]
    #[inline(always)]
    pub fn sa_1(&self) -> SA_1_R {
        SA_1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn ae_1(&self) -> AE_1_R {
        AE_1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR1HIGH")
            .field("addrhi_1", &self.addrhi_1())
            .field("mbc_1", &self.mbc_1())
            .field("sa_1", &self.sa_1())
            .field("ae_1", &self.ae_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC Address1 \\[47:32\\] This field contains the upper 16 bits _47:32_ of the second 6byte MAC address"]
    #[inline(always)]
    pub fn addrhi_1(&mut self) -> ADDRHI_1_W<'_, ADDR1HIGH_SPEC> {
        ADDRHI_1_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask Byte Control These bits are mask control bits for comparison of each of the MAC Address bytes When set high, the MAC does not compare the corresponding byte of received DA or SA with the contents of MAC Address1 registers Each bit controls the masking of the bytes as follows: Bit 29: Register 18\\[15:8\\] Bit 28: Register 18\\[7:0\\] Bit 27: Register 19\\[31:24\\] Bit 24: Register 19\\[7:0\\] You can filter a group of addresses _known as group address filtering_ by masking one or more bytes of the address"]
    #[inline(always)]
    pub fn mbc_1(&mut self) -> MBC_1_W<'_, ADDR1HIGH_SPEC> {
        MBC_1_W::new(self, 24)
    }
    #[doc = "Bit 30 - Source Address When this bit is set, the MAC Address1\\[47:0\\] is used to compare with the SA fields of the received frame When this bit is reset, the MAC Address1\\[47:0\\] is used to compare with the DA fields of the received frame"]
    #[inline(always)]
    pub fn sa_1(&mut self) -> SA_1_W<'_, ADDR1HIGH_SPEC> {
        SA_1_W::new(self, 30)
    }
    #[doc = "Bit 31 - Address Enable"]
    #[inline(always)]
    pub fn ae_1(&mut self) -> AE_1_W<'_, ADDR1HIGH_SPEC> {
        AE_1_W::new(self, 31)
    }
}
#[doc = "Upper 16 bits of the second 6-byte MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr1high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr1high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR1HIGH_SPEC;
impl crate::RegisterSpec for ADDR1HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr1high::R`](R) reader structure"]
impl crate::Readable for ADDR1HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr1high::W`](W) writer structure"]
impl crate::Writable for ADDR1HIGH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR1HIGH to value 0xffff"]
impl crate::Resettable for ADDR1HIGH_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
