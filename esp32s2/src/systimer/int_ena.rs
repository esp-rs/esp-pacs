#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `TARGET(0-2)` reader - Interrupt enable bit of system timer target %s."]
pub type TARGET_R = crate::BitReader;
#[doc = "Field `TARGET(0-2)` writer - Interrupt enable bit of system timer target %s."]
pub type TARGET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Interrupt enable bit of system timer target (0-2)."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TARGET0` field.</div>"]
    #[inline(always)]
    pub fn target(&self, n: u8) -> TARGET_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TARGET_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Interrupt enable bit of system timer target (0-2)."]
    #[inline(always)]
    pub fn target_iter(&self) -> impl Iterator<Item = TARGET_R> + '_ {
        (0..3).map(move |n| TARGET_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Interrupt enable bit of system timer target 0."]
    #[inline(always)]
    pub fn target0(&self) -> TARGET_R {
        TARGET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable bit of system timer target 1."]
    #[inline(always)]
    pub fn target1(&self) -> TARGET_R {
        TARGET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable bit of system timer target 2."]
    #[inline(always)]
    pub fn target2(&self) -> TARGET_R {
        TARGET_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("target0", &self.target0())
            .field("target1", &self.target1())
            .field("target2", &self.target2())
            .finish()
    }
}
impl W {
    #[doc = "Interrupt enable bit of system timer target (0-2)."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TARGET0` field.</div>"]
    #[inline(always)]
    pub fn target(&mut self, n: u8) -> TARGET_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TARGET_W::new(self, n)
    }
    #[doc = "Bit 0 - Interrupt enable bit of system timer target 0."]
    #[inline(always)]
    pub fn target0(&mut self) -> TARGET_W<INT_ENA_SPEC> {
        TARGET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable bit of system timer target 1."]
    #[inline(always)]
    pub fn target1(&mut self) -> TARGET_W<INT_ENA_SPEC> {
        TARGET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable bit of system timer target 2."]
    #[inline(always)]
    pub fn target2(&mut self) -> TARGET_W<INT_ENA_SPEC> {
        TARGET_W::new(self, 2)
    }
}
#[doc = "System timer interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
