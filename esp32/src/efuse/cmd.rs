#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `READ_CMD` reader - "]
pub type READ_CMD_R = crate::BitReader;
#[doc = "Field `READ_CMD` writer - "]
pub type READ_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGM_CMD` reader - "]
pub type PGM_CMD_R = crate::BitReader;
#[doc = "Field `PGM_CMD` writer - "]
pub type PGM_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_cmd(&self) -> READ_CMD_R {
        READ_CMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pgm_cmd(&self) -> PGM_CMD_R {
        PGM_CMD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("read_cmd", &self.read_cmd())
            .field("pgm_cmd", &self.pgm_cmd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_cmd(&mut self) -> READ_CMD_W<CMD_SPEC> {
        READ_CMD_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pgm_cmd(&mut self) -> PGM_CMD_W<CMD_SPEC> {
        PGM_CMD_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {}
