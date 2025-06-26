#[doc = "Register `CONFIG0` reader"]
pub type R = crate::R<CONFIG0_SPEC>;
#[doc = "Register `CONFIG0` writer"]
pub type W = crate::W<CONFIG0_SPEC>;
#[doc = "Field `MAGIC_CTRL` reader - I2C RTC Magic Control"]
pub type MAGIC_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `MAGIC_CTRL` writer - I2C RTC Magic Control"]
pub type MAGIC_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 4:16 - I2C RTC Magic Control"]
    #[inline(always)]
    pub fn magic_ctrl(&self) -> MAGIC_CTRL_R {
        MAGIC_CTRL_R::new(((self.bits >> 4) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG0")
            .field("magic_ctrl", &self.magic_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:16 - I2C RTC Magic Control"]
    #[inline(always)]
    pub fn magic_ctrl(&mut self) -> MAGIC_CTRL_W<CONFIG0_SPEC> {
        MAGIC_CTRL_W::new(self, 4)
    }
}
#[doc = "I2C RTC Configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`config0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG0_SPEC;
impl crate::RegisterSpec for CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config0::R`](R) reader structure"]
impl crate::Readable for CONFIG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config0::W`](W) writer structure"]
impl crate::Writable for CONFIG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG0 to value 0"]
impl crate::Resettable for CONFIG0_SPEC {}
