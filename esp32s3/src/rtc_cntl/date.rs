#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `DATE` reader - version register"]
pub type DATE_R = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - version register"]
pub type DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `LDO_SLAVE` reader - LDO slave control"]
pub type LDO_SLAVE_R = crate::FieldReader;
#[doc = "Field `LDO_SLAVE` writer - LDO slave control"]
pub type LDO_SLAVE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:27 - version register"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 13:18 - LDO slave control"]
    #[inline(always)]
    pub fn ldo_slave(&self) -> LDO_SLAVE_R {
        LDO_SLAVE_R::new(((self.bits >> 13) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("date", &self.date())
            .field("ldo_slave", &self.ldo_slave())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - version register"]
    #[inline(always)]
    pub fn date(&mut self) -> DATE_W<'_, DATE_SPEC> {
        DATE_W::new(self, 0)
    }
    #[doc = "Bits 13:18 - LDO slave control"]
    #[inline(always)]
    pub fn ldo_slave(&mut self) -> LDO_SLAVE_W<'_, DATE_SPEC> {
        LDO_SLAVE_W::new(self, 13)
    }
}
#[doc = "version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATE to value 0x0210_1271"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0210_1271;
}
