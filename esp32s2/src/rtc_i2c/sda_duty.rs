#[doc = "Register `SDA_DUTY` reader"]
pub type R = crate::R<SDA_DUTY_SPEC>;
#[doc = "Register `SDA_DUTY` writer"]
pub type W = crate::W<SDA_DUTY_SPEC>;
#[doc = "Field `NUM` reader - The number of clock cycles between the SDA switch and the falling edge of SCL."]
pub type NUM_R = crate::FieldReader<u32>;
#[doc = "Field `NUM` writer - The number of clock cycles between the SDA switch and the falling edge of SCL."]
pub type NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - The number of clock cycles between the SDA switch and the falling edge of SCL."]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_DUTY")
            .field("num", &self.num().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDA_DUTY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - The number of clock cycles between the SDA switch and the falling edge of SCL."]
    #[inline(always)]
    #[must_use]
    pub fn num(&mut self) -> NUM_W<SDA_DUTY_SPEC> {
        NUM_W::new(self, 0)
    }
}
#[doc = "Configure the SDA hold time after a negative SCL edge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_duty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_duty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDA_DUTY_SPEC;
impl crate::RegisterSpec for SDA_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_duty::R`](R) reader structure"]
impl crate::Readable for SDA_DUTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sda_duty::W`](W) writer structure"]
impl crate::Writable for SDA_DUTY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDA_DUTY to value 0x10"]
impl crate::Resettable for SDA_DUTY_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
