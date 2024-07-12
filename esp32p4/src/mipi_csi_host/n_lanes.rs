#[doc = "Register `N_LANES` reader"]
pub type R = crate::R<N_LANES_SPEC>;
#[doc = "Register `N_LANES` writer"]
pub type W = crate::W<N_LANES_SPEC>;
#[doc = "Field `N_LANES` reader - NA"]
pub type N_LANES_R = crate::FieldReader;
#[doc = "Field `N_LANES` writer - NA"]
pub type N_LANES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn n_lanes(&self) -> N_LANES_R {
        N_LANES_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("N_LANES")
            .field("n_lanes", &self.n_lanes())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn n_lanes(&mut self) -> N_LANES_W<N_LANES_SPEC> {
        N_LANES_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`n_lanes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`n_lanes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct N_LANES_SPEC;
impl crate::RegisterSpec for N_LANES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_lanes::R`](R) reader structure"]
impl crate::Readable for N_LANES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`n_lanes::W`](W) writer structure"]
impl crate::Writable for N_LANES_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets N_LANES to value 0x01"]
impl crate::Resettable for N_LANES_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
