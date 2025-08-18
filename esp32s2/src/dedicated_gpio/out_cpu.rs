#[doc = "Register `OUT_CPU` reader"]
pub type R = crate::R<OUT_CPU_SPEC>;
#[doc = "Register `OUT_CPU` writer"]
pub type W = crate::W<OUT_CPU_SPEC>;
#[doc = "Field `SEL(0-7)` reader - Select GPIO out value configured by registers or CPU instructions for channel %s. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL_R = crate::BitReader;
#[doc = "Field `SEL(0-7)` writer - Select GPIO out value configured by registers or CPU instructions for channel %s. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Select GPIO out value configured by registers or CPU instructions for channel (0-7). 0: Configured by registers. 1: configured by CPU instructions."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SEL0` field.</div>"]
    #[inline(always)]
    pub fn sel(&self, n: u8) -> SEL_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SEL_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Select GPIO out value configured by registers or CPU instructions for channel (0-7). 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel_iter(&self) -> impl Iterator<Item = SEL_R> + '_ {
        (0..8).map(move |n| SEL_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Select GPIO out value configured by registers or CPU instructions for channel 0. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel0(&self) -> SEL_R {
        SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select GPIO out value configured by registers or CPU instructions for channel 1. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel1(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select GPIO out value configured by registers or CPU instructions for channel 2. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel2(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select GPIO out value configured by registers or CPU instructions for channel 3. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel3(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select GPIO out value configured by registers or CPU instructions for channel 4. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel4(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select GPIO out value configured by registers or CPU instructions for channel 5. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel5(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select GPIO out value configured by registers or CPU instructions for channel 6. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel6(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select GPIO out value configured by registers or CPU instructions for channel 7. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel7(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CPU")
            .field("sel0", &self.sel0())
            .field("sel1", &self.sel1())
            .field("sel2", &self.sel2())
            .field("sel3", &self.sel3())
            .field("sel4", &self.sel4())
            .field("sel5", &self.sel5())
            .field("sel6", &self.sel6())
            .field("sel7", &self.sel7())
            .finish()
    }
}
impl W {
    #[doc = "Select GPIO out value configured by registers or CPU instructions for channel (0-7). 0: Configured by registers. 1: configured by CPU instructions."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SEL0` field.</div>"]
    #[inline(always)]
    pub fn sel(&mut self, n: u8) -> SEL_W<'_, OUT_CPU_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SEL_W::new(self, n)
    }
    #[doc = "Bit 0 - Select GPIO out value configured by registers or CPU instructions for channel 0. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel0(&mut self) -> SEL_W<'_, OUT_CPU_SPEC> {
        SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select GPIO out value configured by registers or CPU instructions for channel 1. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel1(&mut self) -> SEL_W<'_, OUT_CPU_SPEC> {
        SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Select GPIO out value configured by registers or CPU instructions for channel 2. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel2(&mut self) -> SEL_W<'_, OUT_CPU_SPEC> {
        SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Select GPIO out value configured by registers or CPU instructions for channel 3. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel3(&mut self) -> SEL_W<'_, OUT_CPU_SPEC> {
        SEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Select GPIO out value configured by registers or CPU instructions for channel 4. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel4(&mut self) -> SEL_W<'_, OUT_CPU_SPEC> {
        SEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Select GPIO out value configured by registers or CPU instructions for channel 5. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel5(&mut self) -> SEL_W<'_, OUT_CPU_SPEC> {
        SEL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Select GPIO out value configured by registers or CPU instructions for channel 6. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel6(&mut self) -> SEL_W<'_, OUT_CPU_SPEC> {
        SEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Select GPIO out value configured by registers or CPU instructions for channel 7. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel7(&mut self) -> SEL_W<'_, OUT_CPU_SPEC> {
        SEL_W::new(self, 7)
    }
}
#[doc = "Dedicated GPIO output mode selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_cpu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_cpu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CPU_SPEC;
impl crate::RegisterSpec for OUT_CPU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_cpu::R`](R) reader structure"]
impl crate::Readable for OUT_CPU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_cpu::W`](W) writer structure"]
impl crate::Writable for OUT_CPU_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CPU to value 0"]
impl crate::Resettable for OUT_CPU_SPEC {}
