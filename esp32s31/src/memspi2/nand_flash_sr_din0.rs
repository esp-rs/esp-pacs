#[doc = "Register `NAND_FLASH_SR_DIN0` reader"]
pub type R = crate::R<NAND_FLASH_SR_DIN0_SPEC>;
#[doc = "Field `SR_DIN(0-3)` reader - "]
pub type SR_DIN_R = crate::FieldReader;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SR_DIN0` field.</div>"]
    #[inline(always)]
    pub fn sr_din(&self, n: u8) -> SR_DIN_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        SR_DIN_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn sr_din_iter(&self) -> impl Iterator<Item = SR_DIN_R> + '_ {
        (0..4).map(move |n| SR_DIN_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - SR_DIN0"]
    #[inline(always)]
    pub fn sr_din0(&self) -> SR_DIN_R {
        SR_DIN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SR_DIN1"]
    #[inline(always)]
    pub fn sr_din1(&self) -> SR_DIN_R {
        SR_DIN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SR_DIN2"]
    #[inline(always)]
    pub fn sr_din2(&self) -> SR_DIN_R {
        SR_DIN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SR_DIN3"]
    #[inline(always)]
    pub fn sr_din3(&self) -> SR_DIN_R {
        SR_DIN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NAND_FLASH_SR_DIN0")
            .field("sr_din0", &self.sr_din0())
            .field("sr_din1", &self.sr_din1())
            .field("sr_din2", &self.sr_din2())
            .field("sr_din3", &self.sr_din3())
            .finish()
    }
}
#[doc = "NAND FLASH SPI SEQ control register\n\nYou can [`read`](crate::Reg::read) this register and get [`nand_flash_sr_din0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NAND_FLASH_SR_DIN0_SPEC;
impl crate::RegisterSpec for NAND_FLASH_SR_DIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nand_flash_sr_din0::R`](R) reader structure"]
impl crate::Readable for NAND_FLASH_SR_DIN0_SPEC {}
#[doc = "`reset()` method sets NAND_FLASH_SR_DIN0 to value 0"]
impl crate::Resettable for NAND_FLASH_SR_DIN0_SPEC {}
