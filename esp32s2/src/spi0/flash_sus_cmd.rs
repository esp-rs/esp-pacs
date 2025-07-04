#[doc = "Register `FLASH_SUS_CMD` reader"]
pub type R = crate::R<FLASH_SUS_CMD_SPEC>;
#[doc = "Register `FLASH_SUS_CMD` writer"]
pub type W = crate::W<FLASH_SUS_CMD_SPEC>;
#[doc = "Field `FLASH_PER` reader - "]
pub type FLASH_PER_R = crate::BitReader;
#[doc = "Field `FLASH_PER` writer - "]
pub type FLASH_PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES` reader - "]
pub type FLASH_PES_R = crate::BitReader;
#[doc = "Field `FLASH_PES` writer - "]
pub type FLASH_PES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_per(&self) -> FLASH_PER_R {
        FLASH_PER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flash_pes(&self) -> FLASH_PES_R {
        FLASH_PES_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_SUS_CMD")
            .field("flash_pes", &self.flash_pes())
            .field("flash_per", &self.flash_per())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_per(&mut self) -> FLASH_PER_W<FLASH_SUS_CMD_SPEC> {
        FLASH_PER_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flash_pes(&mut self) -> FLASH_PES_W<FLASH_SUS_CMD_SPEC> {
        FLASH_PES_W::new(self, 1)
    }
}
#[doc = "SPI Memory Flash Suspend Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_SUS_CMD_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_sus_cmd::R`](R) reader structure"]
impl crate::Readable for FLASH_SUS_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_sus_cmd::W`](W) writer structure"]
impl crate::Writable for FLASH_SUS_CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_SUS_CMD to value 0"]
impl crate::Resettable for FLASH_SUS_CMD_SPEC {}
