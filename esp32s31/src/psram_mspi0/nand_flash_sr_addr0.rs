#[doc = "Register `NAND_FLASH_SR_ADDR0` reader"]
pub type R = crate::R<NAND_FLASH_SR_ADDR0_SPEC>;
#[doc = "Register `NAND_FLASH_SR_ADDR0` writer"]
pub type W = crate::W<NAND_FLASH_SR_ADDR0_SPEC>;
#[doc = "Field `NAND_FLASH_SR_ADDR0` reader - "]
pub type NAND_FLASH_SR_ADDR0_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_SR_ADDR0` writer - "]
pub type NAND_FLASH_SR_ADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NAND_FLASH_SR_ADDR1` reader - "]
pub type NAND_FLASH_SR_ADDR1_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_SR_ADDR1` writer - "]
pub type NAND_FLASH_SR_ADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NAND_FLASH_SR_ADDR2` reader - "]
pub type NAND_FLASH_SR_ADDR2_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_SR_ADDR2` writer - "]
pub type NAND_FLASH_SR_ADDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NAND_FLASH_SR_ADDR3` reader - "]
pub type NAND_FLASH_SR_ADDR3_R = crate::FieldReader;
#[doc = "Field `NAND_FLASH_SR_ADDR3` writer - "]
pub type NAND_FLASH_SR_ADDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn nand_flash_sr_addr0(&self) -> NAND_FLASH_SR_ADDR0_R {
        NAND_FLASH_SR_ADDR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn nand_flash_sr_addr1(&self) -> NAND_FLASH_SR_ADDR1_R {
        NAND_FLASH_SR_ADDR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn nand_flash_sr_addr2(&self) -> NAND_FLASH_SR_ADDR2_R {
        NAND_FLASH_SR_ADDR2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nand_flash_sr_addr3(&self) -> NAND_FLASH_SR_ADDR3_R {
        NAND_FLASH_SR_ADDR3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_SR_ADDR0")
            .field("nand_flash_sr_addr0", &self.nand_flash_sr_addr0())
            .field("nand_flash_sr_addr1", &self.nand_flash_sr_addr1())
            .field("nand_flash_sr_addr2", &self.nand_flash_sr_addr2())
            .field("nand_flash_sr_addr3", &self.nand_flash_sr_addr3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn nand_flash_sr_addr0(&mut self) -> NAND_FLASH_SR_ADDR0_W<'_, NAND_FLASH_SR_ADDR0_SPEC> {
        NAND_FLASH_SR_ADDR0_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn nand_flash_sr_addr1(&mut self) -> NAND_FLASH_SR_ADDR1_W<'_, NAND_FLASH_SR_ADDR0_SPEC> {
        NAND_FLASH_SR_ADDR1_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn nand_flash_sr_addr2(&mut self) -> NAND_FLASH_SR_ADDR2_W<'_, NAND_FLASH_SR_ADDR0_SPEC> {
        NAND_FLASH_SR_ADDR2_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nand_flash_sr_addr3(&mut self) -> NAND_FLASH_SR_ADDR3_W<'_, NAND_FLASH_SR_ADDR0_SPEC> {
        NAND_FLASH_SR_ADDR3_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_sr_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nand_flash_sr_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_SR_ADDR0_SPEC;
impl crate::RegisterSpec for NAND_FLASH_SR_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_sr_addr0::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_SR_ADDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nand_flash_sr_addr0::W`](W) writer structure"]
impl crate::Writable for NAND_FLASH_SR_ADDR0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NAND_FLASH_SR_ADDR0 to value 0"]
impl crate::Resettable for NAND_FLASH_SR_ADDR0_SPEC {}
