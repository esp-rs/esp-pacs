#[doc = "Register `SMEM_AC` reader"]
pub type R = crate::R<SMEM_AC_SPEC>;
#[doc = "Register `SMEM_AC` writer"]
pub type W = crate::W<SMEM_AC_SPEC>;
#[doc = "Field `CS_SETUP` reader - "]
pub type CS_SETUP_R = crate::BitReader;
#[doc = "Field `CS_SETUP` writer - "]
pub type CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_HOLD` reader - "]
pub type CS_HOLD_R = crate::BitReader;
#[doc = "Field `CS_HOLD` writer - "]
pub type CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_SETUP_TIME` reader - "]
pub type CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `CS_SETUP_TIME` writer - "]
pub type CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CS_HOLD_TIME` reader - "]
pub type CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `CS_HOLD_TIME` writer - "]
pub type CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ECC_CS_HOLD_TIME` reader - "]
pub type ECC_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `ECC_CS_HOLD_TIME` writer - "]
pub type ECC_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ECC_SKIP_PAGE_CORNER` reader - "]
pub type ECC_SKIP_PAGE_CORNER_R = crate::BitReader;
#[doc = "Field `ECC_SKIP_PAGE_CORNER` writer - "]
pub type ECC_SKIP_PAGE_CORNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_16TO18_BYTE_EN` reader - "]
pub type ECC_16TO18_BYTE_EN_R = crate::BitReader;
#[doc = "Field `ECC_16TO18_BYTE_EN` writer - "]
pub type ECC_16TO18_BYTE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_HOLD_DELAY` reader - "]
pub type CS_HOLD_DELAY_R = crate::FieldReader;
#[doc = "Field `CS_HOLD_DELAY` writer - "]
pub type CS_HOLD_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPLIT_TRANS_EN` reader - "]
pub type SPLIT_TRANS_EN_R = crate::BitReader;
#[doc = "Field `SPLIT_TRANS_EN` writer - "]
pub type SPLIT_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cs_setup(&self) -> CS_SETUP_R {
        CS_SETUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cs_hold(&self) -> CS_HOLD_R {
        CS_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ecc_cs_hold_time(&self) -> ECC_CS_HOLD_TIME_R {
        ECC_CS_HOLD_TIME_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ecc_skip_page_corner(&self) -> ECC_SKIP_PAGE_CORNER_R {
        ECC_SKIP_PAGE_CORNER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ecc_16to18_byte_en(&self) -> ECC_16TO18_BYTE_EN_R {
        ECC_16TO18_BYTE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 25:30"]
    #[inline(always)]
    pub fn cs_hold_delay(&self) -> CS_HOLD_DELAY_R {
        CS_HOLD_DELAY_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn split_trans_en(&self) -> SPLIT_TRANS_EN_R {
        SPLIT_TRANS_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_AC")
            .field("cs_setup", &self.cs_setup())
            .field("cs_hold", &self.cs_hold())
            .field("cs_setup_time", &self.cs_setup_time())
            .field("cs_hold_time", &self.cs_hold_time())
            .field("ecc_cs_hold_time", &self.ecc_cs_hold_time())
            .field("ecc_skip_page_corner", &self.ecc_skip_page_corner())
            .field("ecc_16to18_byte_en", &self.ecc_16to18_byte_en())
            .field("cs_hold_delay", &self.cs_hold_delay())
            .field("split_trans_en", &self.split_trans_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cs_setup(&mut self) -> CS_SETUP_W<'_, SMEM_AC_SPEC> {
        CS_SETUP_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cs_hold(&mut self) -> CS_HOLD_W<'_, SMEM_AC_SPEC> {
        CS_HOLD_W::new(self, 1)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W<'_, SMEM_AC_SPEC> {
        CS_SETUP_TIME_W::new(self, 2)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W<'_, SMEM_AC_SPEC> {
        CS_HOLD_TIME_W::new(self, 7)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ecc_cs_hold_time(&mut self) -> ECC_CS_HOLD_TIME_W<'_, SMEM_AC_SPEC> {
        ECC_CS_HOLD_TIME_W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ecc_skip_page_corner(&mut self) -> ECC_SKIP_PAGE_CORNER_W<'_, SMEM_AC_SPEC> {
        ECC_SKIP_PAGE_CORNER_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ecc_16to18_byte_en(&mut self) -> ECC_16TO18_BYTE_EN_W<'_, SMEM_AC_SPEC> {
        ECC_16TO18_BYTE_EN_W::new(self, 16)
    }
    #[doc = "Bits 25:30"]
    #[inline(always)]
    pub fn cs_hold_delay(&mut self) -> CS_HOLD_DELAY_W<'_, SMEM_AC_SPEC> {
        CS_HOLD_DELAY_W::new(self, 25)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn split_trans_en(&mut self) -> SPLIT_TRANS_EN_W<'_, SMEM_AC_SPEC> {
        SPLIT_TRANS_EN_W::new(self, 31)
    }
}
#[doc = "MSPI external RAM ECC and SPI CS timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_AC_SPEC;
impl crate::RegisterSpec for SMEM_AC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_ac::R`](R) reader structure"]
impl crate::Readable for SMEM_AC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_ac::W`](W) writer structure"]
impl crate::Writable for SMEM_AC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_AC to value 0xb084"]
impl crate::Resettable for SMEM_AC_SPEC {
    const RESET_VALUE: u32 = 0xb084;
}
