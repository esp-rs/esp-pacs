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
            .field(
                "scramble_seed_lane2",
                &format_args!("{}", self.scramble_seed_lane2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCRAMBLING_SEED2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn scramble_seed_lane2(&mut self) -> SCRAMBLE_SEED_LANE2_W<SCRAMBLING_SEED2_SPEC> {
        SCRAMBLE_SEED_LANE2_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scrambling_seed2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scrambling_seed2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCRAMBLING_SEED2_SPEC;
impl crate::RegisterSpec for SCRAMBLING_SEED2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scrambling_seed2::R`](R) reader structure"]
impl crate::Readable for SCRAMBLING_SEED2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scrambling_seed2::W`](W) writer structure"]
impl crate::Writable for SCRAMBLING_SEED2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCRAMBLING_SEED2 to value 0x1188"]
impl crate::Resettable for SCRAMBLING_SEED2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1188;
}
