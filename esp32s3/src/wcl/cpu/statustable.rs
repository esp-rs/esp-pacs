#[doc = "Register `STATUSTABLE%s` reader"]
pub type R = crate::R<STATUSTABLE_SPEC>;
#[doc = "Register `STATUSTABLE%s` writer"]
pub type W = crate::W<STATUSTABLE_SPEC>;
#[doc = "Field `FROM_WORLD` reader - This bit is used to confirm world before enter entry 1"]
pub type FROM_WORLD_R = crate::BitReader;
#[doc = "Field `FROM_WORLD` writer - This bit is used to confirm world before enter entry 1"]
pub type FROM_WORLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FROM_ENTRY` reader - This filed is used to confirm in which entry before enter entry 1"]
pub type FROM_ENTRY_R = crate::FieldReader;
#[doc = "Field `FROM_ENTRY` writer - This filed is used to confirm in which entry before enter entry 1"]
pub type FROM_ENTRY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CURRENT` reader - This bit is used to confirm whether the current state is in entry 1"]
pub type CURRENT_R = crate::BitReader;
#[doc = "Field `CURRENT` writer - This bit is used to confirm whether the current state is in entry 1"]
pub type CURRENT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 1"]
    #[inline(always)]
    pub fn from_world(&self) -> FROM_WORLD_R {
        FROM_WORLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 1"]
    #[inline(always)]
    pub fn from_entry(&self) -> FROM_ENTRY_R {
        FROM_ENTRY_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 1"]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUSTABLE")
            .field("from_world", &self.from_world())
            .field("from_entry", &self.from_entry())
            .field("current", &self.current())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 1"]
    #[inline(always)]
    pub fn from_world(&mut self) -> FROM_WORLD_W<STATUSTABLE_SPEC> {
        FROM_WORLD_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 1"]
    #[inline(always)]
    pub fn from_entry(&mut self) -> FROM_ENTRY_W<STATUSTABLE_SPEC> {
        FROM_ENTRY_W::new(self, 1)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 1"]
    #[inline(always)]
    pub fn current(&mut self) -> CURRENT_W<STATUSTABLE_SPEC> {
        CURRENT_W::new(self, 5)
    }
}
#[doc = "Status register of world switch of entry %s\n\nYou can [`read`](crate::Reg::read) this register and get [`statustable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statustable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUSTABLE_SPEC;
impl crate::RegisterSpec for STATUSTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statustable::R`](R) reader structure"]
impl crate::Readable for STATUSTABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`statustable::W`](W) writer structure"]
impl crate::Writable for STATUSTABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUSTABLE%s to value 0"]
impl crate::Resettable for STATUSTABLE_SPEC {}
