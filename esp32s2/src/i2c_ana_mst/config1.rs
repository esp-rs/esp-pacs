#[doc = "Register `CONFIG1` reader"]
pub type R = crate::R<CONFIG1_SPEC>;
#[doc = "Register `CONFIG1` writer"]
pub type W = crate::W<CONFIG1_SPEC>;
#[doc = "Field `MAGIC_CTRL` reader - I2C RTC Magic Control"]
pub type MAGIC_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `MAGIC_CTRL` writer - I2C RTC Magic Control"]
pub type MAGIC_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `ALL_MASK` reader - I2C RTC All Mask"]
pub type ALL_MASK_R = crate::FieldReader<u16>;
#[doc = "Field `ALL_MASK` writer - I2C RTC All Mask"]
pub type ALL_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `APLL` reader - I2C RTC APLL Mask"]
pub type APLL_R = crate::BitReader;
#[doc = "Field `APLL` writer - I2C RTC APLL Mask"]
pub type APLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBPLL` reader - I2C RTC BBPLL Mask"]
pub type BBPLL_R = crate::BitReader;
#[doc = "Field `BBPLL` writer - I2C RTC BBPLL Mask"]
pub type BBPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR` reader - I2C RTC SAR Mask"]
pub type SAR_R = crate::BitReader;
#[doc = "Field `SAR` writer - I2C RTC SAR Mask"]
pub type SAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD` reader - I2C RTC BOD Mask"]
pub type BOD_R = crate::BitReader;
#[doc = "Field `BOD` writer - I2C RTC BOD Mask"]
pub type BOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:16 - I2C RTC Magic Control"]
    #[inline(always)]
    pub fn magic_ctrl(&self) -> MAGIC_CTRL_R {
        MAGIC_CTRL_R::new(((self.bits >> 4) & 0x1fff) as u16)
    }
    #[doc = "Bits 8:22 - I2C RTC All Mask"]
    #[inline(always)]
    pub fn all_mask(&self) -> ALL_MASK_R {
        ALL_MASK_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bit 14 - I2C RTC APLL Mask"]
    #[inline(always)]
    pub fn apll(&self) -> APLL_R {
        APLL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - I2C RTC BBPLL Mask"]
    #[inline(always)]
    pub fn bbpll(&self) -> BBPLL_R {
        BBPLL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C RTC SAR Mask"]
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C RTC BOD Mask"]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG1")
            .field("magic_ctrl", &self.magic_ctrl())
            .field("all_mask", &self.all_mask())
            .field("apll", &self.apll())
            .field("bbpll", &self.bbpll())
            .field("sar", &self.sar())
            .field("bod", &self.bod())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:16 - I2C RTC Magic Control"]
    #[inline(always)]
    pub fn magic_ctrl(&mut self) -> MAGIC_CTRL_W<CONFIG1_SPEC> {
        MAGIC_CTRL_W::new(self, 4)
    }
    #[doc = "Bits 8:22 - I2C RTC All Mask"]
    #[inline(always)]
    pub fn all_mask(&mut self) -> ALL_MASK_W<CONFIG1_SPEC> {
        ALL_MASK_W::new(self, 8)
    }
    #[doc = "Bit 14 - I2C RTC APLL Mask"]
    #[inline(always)]
    pub fn apll(&mut self) -> APLL_W<CONFIG1_SPEC> {
        APLL_W::new(self, 14)
    }
    #[doc = "Bit 17 - I2C RTC BBPLL Mask"]
    #[inline(always)]
    pub fn bbpll(&mut self) -> BBPLL_W<CONFIG1_SPEC> {
        BBPLL_W::new(self, 17)
    }
    #[doc = "Bit 18 - I2C RTC SAR Mask"]
    #[inline(always)]
    pub fn sar(&mut self) -> SAR_W<CONFIG1_SPEC> {
        SAR_W::new(self, 18)
    }
    #[doc = "Bit 22 - I2C RTC BOD Mask"]
    #[inline(always)]
    pub fn bod(&mut self) -> BOD_W<CONFIG1_SPEC> {
        BOD_W::new(self, 22)
    }
}
#[doc = "I2C RTC Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG1_SPEC;
impl crate::RegisterSpec for CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config1::R`](R) reader structure"]
impl crate::Readable for CONFIG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config1::W`](W) writer structure"]
impl crate::Writable for CONFIG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG1 to value 0"]
impl crate::Resettable for CONFIG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
