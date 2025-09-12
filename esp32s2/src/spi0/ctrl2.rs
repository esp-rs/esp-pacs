#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CS_SETUP_TIME` reader - "]
pub type CS_SETUP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `CS_SETUP_TIME` writer - "]
pub type CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `CS_HOLD_TIME` reader - "]
pub type CS_HOLD_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `CS_HOLD_TIME` writer - "]
pub type CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `SYNC_RESET` reader - "]
pub type SYNC_RESET_R = crate::BitReader;
#[doc = "Field `SYNC_RESET` writer - "]
pub type SYNC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:25"]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 13) & 0x1fff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sync_reset(&self) -> SYNC_RESET_R {
        SYNC_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("sync_reset", &self.sync_reset())
            .field("cs_hold_time", &self.cs_hold_time())
            .field("cs_setup_time", &self.cs_setup_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W<'_, CTRL2_SPEC> {
        CS_SETUP_TIME_W::new(self, 0)
    }
    #[doc = "Bits 13:25"]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W<'_, CTRL2_SPEC> {
        CS_HOLD_TIME_W::new(self, 13)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sync_reset(&mut self) -> SYNC_RESET_W<'_, CTRL2_SPEC> {
        SYNC_RESET_W::new(self, 31)
    }
}
#[doc = "SPI Memory Control2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {}
