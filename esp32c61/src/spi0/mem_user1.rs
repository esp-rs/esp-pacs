#[doc = "Register `MEM_USER1` reader"]
pub type R = crate::R<MEM_USER1_SPEC>;
#[doc = "Register `MEM_USER1` writer"]
pub type W = crate::W<MEM_USER1_SPEC>;
#[doc = "Field `MEM_USR_DUMMY_CYCLELEN` reader - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
pub type MEM_USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `MEM_USR_DUMMY_CYCLELEN` writer - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
pub type MEM_USR_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MEM_USR_DBYTELEN` reader - SPI0 USR_CMD read or write data byte length -1"]
pub type MEM_USR_DBYTELEN_R = crate::FieldReader;
#[doc = "Field `MEM_USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1)."]
pub type MEM_USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `MEM_USR_ADDR_BITLEN` writer - The length in bits of address phase. The register value shall be (bit_num-1)."]
pub type MEM_USR_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn mem_usr_dummy_cyclelen(&self) -> MEM_USR_DUMMY_CYCLELEN_R {
        MEM_USR_DUMMY_CYCLELEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - SPI0 USR_CMD read or write data byte length -1"]
    #[inline(always)]
    pub fn mem_usr_dbytelen(&self) -> MEM_USR_DBYTELEN_R {
        MEM_USR_DBYTELEN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn mem_usr_addr_bitlen(&self) -> MEM_USR_ADDR_BITLEN_R {
        MEM_USR_ADDR_BITLEN_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_USER1")
            .field("mem_usr_dummy_cyclelen", &self.mem_usr_dummy_cyclelen())
            .field("mem_usr_dbytelen", &self.mem_usr_dbytelen())
            .field("mem_usr_addr_bitlen", &self.mem_usr_addr_bitlen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn mem_usr_dummy_cyclelen(&mut self) -> MEM_USR_DUMMY_CYCLELEN_W<MEM_USER1_SPEC> {
        MEM_USR_DUMMY_CYCLELEN_W::new(self, 0)
    }
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn mem_usr_addr_bitlen(&mut self) -> MEM_USR_ADDR_BITLEN_W<MEM_USER1_SPEC> {
        MEM_USR_ADDR_BITLEN_W::new(self, 26)
    }
}
#[doc = "SPI0 user1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_user1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_user1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_USER1_SPEC;
impl crate::RegisterSpec for MEM_USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_user1::R`](R) reader structure"]
impl crate::Readable for MEM_USER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_user1::W`](W) writer structure"]
impl crate::Writable for MEM_USER1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_USER1 to value 0x5c00_0047"]
impl crate::Resettable for MEM_USER1_SPEC {
    const RESET_VALUE: u32 = 0x5c00_0047;
}
