#[doc = "Register `RD_STATUS` reader"]
pub type R = crate::R<RD_STATUS_SPEC>;
#[doc = "Register `RD_STATUS` writer"]
pub type W = crate::W<RD_STATUS_SPEC>;
#[doc = "Field `STATUS` reader - "]
pub type STATUS_R = crate::FieldReader<u16>;
#[doc = "Field `STATUS` writer - "]
pub type STATUS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WB_MODE` reader - "]
pub type WB_MODE_R = crate::FieldReader<u16>;
#[doc = "Field `WB_MODE` writer - "]
pub type WB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wb_mode(&self) -> WB_MODE_R {
        WB_MODE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_STATUS")
            .field("wb_mode", &self.wb_mode())
            .field("status", &self.status())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W<RD_STATUS_SPEC> {
        STATUS_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wb_mode(&mut self) -> WB_MODE_W<RD_STATUS_SPEC> {
        WB_MODE_W::new(self, 16)
    }
}
#[doc = "SPI Memory Read Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_STATUS_SPEC;
impl crate::RegisterSpec for RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_status::R`](R) reader structure"]
impl crate::Readable for RD_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_status::W`](W) writer structure"]
impl crate::Writable for RD_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_STATUS to value 0"]
impl crate::Resettable for RD_STATUS_SPEC {}
