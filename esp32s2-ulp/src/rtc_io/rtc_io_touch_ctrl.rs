#[doc = "Register `RTC_IO_TOUCH_CTRL` reader"]
pub type R = crate::R<RTC_IO_TOUCH_CTRL_SPEC>;
#[doc = "Register `RTC_IO_TOUCH_CTRL` writer"]
pub type W = crate::W<RTC_IO_TOUCH_CTRL_SPEC>;
#[doc = "Field `IO_TOUCH_BUFSEL` reader - "]
pub type IO_TOUCH_BUFSEL_R = crate::FieldReader;
#[doc = "Field `IO_TOUCH_BUFSEL` writer - "]
pub type IO_TOUCH_BUFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IO_TOUCH_BUFMODE` reader - "]
pub type IO_TOUCH_BUFMODE_R = crate::BitReader;
#[doc = "Field `IO_TOUCH_BUFMODE` writer - "]
pub type IO_TOUCH_BUFMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn io_touch_bufsel(&self) -> IO_TOUCH_BUFSEL_R {
        IO_TOUCH_BUFSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn io_touch_bufmode(&self) -> IO_TOUCH_BUFMODE_R {
        IO_TOUCH_BUFMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_IO_TOUCH_CTRL")
            .field("io_touch_bufsel", &self.io_touch_bufsel())
            .field("io_touch_bufmode", &self.io_touch_bufmode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn io_touch_bufsel(&mut self) -> IO_TOUCH_BUFSEL_W<RTC_IO_TOUCH_CTRL_SPEC> {
        IO_TOUCH_BUFSEL_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn io_touch_bufmode(&mut self) -> IO_TOUCH_BUFMODE_W<RTC_IO_TOUCH_CTRL_SPEC> {
        IO_TOUCH_BUFMODE_W::new(self, 4)
    }
}
#[doc = "Touch control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_io_touch_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_io_touch_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_IO_TOUCH_CTRL_SPEC;
impl crate::RegisterSpec for RTC_IO_TOUCH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_io_touch_ctrl::R`](R) reader structure"]
impl crate::Readable for RTC_IO_TOUCH_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_io_touch_ctrl::W`](W) writer structure"]
impl crate::Writable for RTC_IO_TOUCH_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_IO_TOUCH_CTRL to value 0"]
impl crate::Resettable for RTC_IO_TOUCH_CTRL_SPEC {}
