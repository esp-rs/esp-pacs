#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `TARGET(0-2)` reader - interupt%s raw"]
pub type TARGET_R = crate::BitReader;
#[doc = "Field `TARGET(0-2)` writer - interupt%s raw"]
pub type TARGET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "interupt(0-2) raw"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `TARGET0` field"]
    #[inline(always)]
    pub fn target(&self, n: u8) -> TARGET_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TARGET_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "interupt(0-2) raw"]
    #[inline(always)]
    pub fn target_iter(&self) -> impl Iterator<Item = TARGET_R> + '_ {
        (0..3).map(move |n| TARGET_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - interupt0 raw"]
    #[inline(always)]
    pub fn target0(&self) -> TARGET_R {
        TARGET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - interupt1 raw"]
    #[inline(always)]
    pub fn target1(&self) -> TARGET_R {
        TARGET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - interupt2 raw"]
    #[inline(always)]
    pub fn target2(&self) -> TARGET_R {
        TARGET_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("target0", &self.target0())
            .field("target1", &self.target1())
            .field("target2", &self.target2())
            .finish()
    }
}
impl W {
    #[doc = "interupt(0-2) raw"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `TARGET0` field"]
    #[inline(always)]
    #[must_use]
    pub fn target(&mut self, n: u8) -> TARGET_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TARGET_W::new(self, n)
    }
    #[doc = "Bit 0 - interupt0 raw"]
    #[inline(always)]
    #[must_use]
    pub fn target0(&mut self) -> TARGET_W<INT_RAW_SPEC> {
        TARGET_W::new(self, 0)
    }
    #[doc = "Bit 1 - interupt1 raw"]
    #[inline(always)]
    #[must_use]
    pub fn target1(&mut self) -> TARGET_W<INT_RAW_SPEC> {
        TARGET_W::new(self, 1)
    }
    #[doc = "Bit 2 - interupt2 raw"]
    #[inline(always)]
    #[must_use]
    pub fn target2(&mut self) -> TARGET_W<INT_RAW_SPEC> {
        TARGET_W::new(self, 2)
    }
}
#[doc = "systimer interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
