#[doc = "Register `SCRAMBLING_SEED2` reader"]
pub type R = crate::R<SCRAMBLING_SEED2_SPEC>;
#[doc = "Register `SCRAMBLING_SEED2` writer"]
pub type W = crate::W<SCRAMBLING_SEED2_SPEC>;
#[doc = "Field `SCRAMBLE_SEED_LANE2` reader - NA"]
pub type SCRAMBLE_SEED_LANE2_R = crate::FieldReader<u16>;
#[doc = "Field `SCRAMBLE_SEED_LANE2` writer - NA"]
pub type SCRAMBLE_SEED_LANE2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn scramble_seed_lane2(&self) -> SCRAMBLE_SEED_LANE2_R {
        SCRAMBLE_SEED_LANE2_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCRAMBLING_SEED2")
            .field("scramble_seed_lane2", &self.scramble_seed_lane2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn scramble_seed_lane2(&mut self) -> SCRAMBLE_SEED_LANE2_W<SCRAMBLING_SEED2_SPEC> {
        SCRAMBLE_SEED_LANE2_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scrambling_seed2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scrambling_seed2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRAMBLING_SEED2_SPEC;
impl crate::RegisterSpec for SCRAMBLING_SEED2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scrambling_seed2::R`](R) reader structure"]
impl crate::Readable for SCRAMBLING_SEED2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scrambling_seed2::W`](W) writer structure"]
impl crate::Writable for SCRAMBLING_SEED2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCRAMBLING_SEED2 to value 0x1188"]
impl crate::Resettable for SCRAMBLING_SEED2_SPEC {
    const RESET_VALUE: u32 = 0x1188;
}
