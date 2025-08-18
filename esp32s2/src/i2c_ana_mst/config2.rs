#[doc = "Register `CONFIG2` reader"]
pub type R = crate::R<CONFIG2_SPEC>;
#[doc = "Register `CONFIG2` writer"]
pub type W = crate::W<CONFIG2_SPEC>;
#[doc = "Field `SLAVE_ID` reader - I2C RTC Slave ID"]
pub type SLAVE_ID_R = crate::FieldReader;
#[doc = "Field `SLAVE_ID` writer - I2C RTC Slave ID"]
pub type SLAVE_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR` reader - I2C RTC Address"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - I2C RTC Address"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA` reader - I2C RTC Data"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - I2C RTC Data"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR_MASK` reader - I2C RTC SAR Mask"]
pub type SAR_MASK_R = crate::BitReader;
#[doc = "Field `SAR_MASK` writer - I2C RTC SAR Mask"]
pub type SAR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_MASK` reader - I2C RTC BOD Mask"]
pub type BOD_MASK_R = crate::BitReader;
#[doc = "Field `BOD_MASK` writer - I2C RTC BOD Mask"]
pub type BOD_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_CNTL` reader - I2C RTC Write Control"]
pub type WR_CNTL_R = crate::BitReader;
#[doc = "Field `WR_CNTL` writer - I2C RTC Write Control"]
pub type WR_CNTL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - I2C RTC Busy"]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - I2C RTC Slave ID"]
    #[inline(always)]
    pub fn slave_id(&self) -> SLAVE_ID_R {
        SLAVE_ID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2C RTC Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C RTC Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 18 - I2C RTC SAR Mask"]
    #[inline(always)]
    pub fn sar_mask(&self) -> SAR_MASK_R {
        SAR_MASK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C RTC BOD Mask"]
    #[inline(always)]
    pub fn bod_mask(&self) -> BOD_MASK_R {
        BOD_MASK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C RTC Write Control"]
    #[inline(always)]
    pub fn wr_cntl(&self) -> WR_CNTL_R {
        WR_CNTL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2C RTC Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG2")
            .field("slave_id", &self.slave_id())
            .field("addr", &self.addr())
            .field("data", &self.data())
            .field("wr_cntl", &self.wr_cntl())
            .field("busy", &self.busy())
            .field("sar_mask", &self.sar_mask())
            .field("bod_mask", &self.bod_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C RTC Slave ID"]
    #[inline(always)]
    pub fn slave_id(&mut self) -> SLAVE_ID_W<'_, CONFIG2_SPEC> {
        SLAVE_ID_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - I2C RTC Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<'_, CONFIG2_SPEC> {
        ADDR_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - I2C RTC Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<'_, CONFIG2_SPEC> {
        DATA_W::new(self, 16)
    }
    #[doc = "Bit 18 - I2C RTC SAR Mask"]
    #[inline(always)]
    pub fn sar_mask(&mut self) -> SAR_MASK_W<'_, CONFIG2_SPEC> {
        SAR_MASK_W::new(self, 18)
    }
    #[doc = "Bit 22 - I2C RTC BOD Mask"]
    #[inline(always)]
    pub fn bod_mask(&mut self) -> BOD_MASK_W<'_, CONFIG2_SPEC> {
        BOD_MASK_W::new(self, 22)
    }
    #[doc = "Bit 24 - I2C RTC Write Control"]
    #[inline(always)]
    pub fn wr_cntl(&mut self) -> WR_CNTL_W<'_, CONFIG2_SPEC> {
        WR_CNTL_W::new(self, 24)
    }
}
#[doc = "I2C RTC Configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`config2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG2_SPEC;
impl crate::RegisterSpec for CONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config2::R`](R) reader structure"]
impl crate::Readable for CONFIG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config2::W`](W) writer structure"]
impl crate::Writable for CONFIG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG2 to value 0"]
impl crate::Resettable for CONFIG2_SPEC {}
