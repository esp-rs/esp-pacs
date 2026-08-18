#[doc = "Register `SMEM_DIN_MODE` reader"]
pub type R = crate::R<SMEM_DIN_MODE_SPEC>;
#[doc = "Register `SMEM_DIN_MODE` writer"]
pub type W = crate::W<SMEM_DIN_MODE_SPEC>;
#[doc = "Field `DIN_MODE(0-7)` reader - "]
pub type DIN_MODE_R = crate::FieldReader;
#[doc = "Field `DIN_MODE(0-7)` writer - "]
pub type DIN_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DINS_MODE` reader - "]
pub type DINS_MODE_R = crate::FieldReader;
#[doc = "Field `DINS_MODE` writer - "]
pub type DINS_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DIN0_MODE` field.</div>"]
    #[inline(always)]
    pub fn din_mode(&self, n: u8) -> DIN_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DIN_MODE_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn din_mode_iter(&self) -> impl Iterator<Item = DIN_MODE_R> + '_ {
        (0..8).map(move |n| DIN_MODE_R::new(((self.bits >> (n * 3)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - DIN0_MODE"]
    #[inline(always)]
    pub fn din0_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - DIN1_MODE"]
    #[inline(always)]
    pub fn din1_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - DIN2_MODE"]
    #[inline(always)]
    pub fn din2_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - DIN3_MODE"]
    #[inline(always)]
    pub fn din3_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - DIN4_MODE"]
    #[inline(always)]
    pub fn din4_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - DIN5_MODE"]
    #[inline(always)]
    pub fn din5_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - DIN6_MODE"]
    #[inline(always)]
    pub fn din6_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - DIN7_MODE"]
    #[inline(always)]
    pub fn din7_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn dins_mode(&self) -> DINS_MODE_R {
        DINS_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_DIN_MODE")
            .field("din0_mode", &self.din0_mode())
            .field("din1_mode", &self.din1_mode())
            .field("din2_mode", &self.din2_mode())
            .field("din3_mode", &self.din3_mode())
            .field("din4_mode", &self.din4_mode())
            .field("din5_mode", &self.din5_mode())
            .field("din6_mode", &self.din6_mode())
            .field("din7_mode", &self.din7_mode())
            .field("dins_mode", &self.dins_mode())
            .finish()
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DIN0_MODE` field.</div>"]
    #[inline(always)]
    pub fn din_mode(&mut self, n: u8) -> DIN_MODE_W<'_, SMEM_DIN_MODE_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DIN_MODE_W::new(self, n * 3)
    }
    #[doc = "Bits 0:2 - DIN0_MODE"]
    #[inline(always)]
    pub fn din0_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_MODE_SPEC> {
        DIN_MODE_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - DIN1_MODE"]
    #[inline(always)]
    pub fn din1_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_MODE_SPEC> {
        DIN_MODE_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - DIN2_MODE"]
    #[inline(always)]
    pub fn din2_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_MODE_SPEC> {
        DIN_MODE_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - DIN3_MODE"]
    #[inline(always)]
    pub fn din3_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_MODE_SPEC> {
        DIN_MODE_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - DIN4_MODE"]
    #[inline(always)]
    pub fn din4_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_MODE_SPEC> {
        DIN_MODE_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - DIN5_MODE"]
    #[inline(always)]
    pub fn din5_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_MODE_SPEC> {
        DIN_MODE_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - DIN6_MODE"]
    #[inline(always)]
    pub fn din6_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_MODE_SPEC> {
        DIN_MODE_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - DIN7_MODE"]
    #[inline(always)]
    pub fn din7_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_MODE_SPEC> {
        DIN_MODE_W::new(self, 21)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn dins_mode(&mut self) -> DINS_MODE_W<'_, SMEM_DIN_MODE_SPEC> {
        DINS_MODE_W::new(self, 24)
    }
}
#[doc = "MSPI external RAM input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_din_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_DIN_MODE_SPEC;
impl crate::RegisterSpec for SMEM_DIN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_din_mode::R`](R) reader structure"]
impl crate::Readable for SMEM_DIN_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_din_mode::W`](W) writer structure"]
impl crate::Writable for SMEM_DIN_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_DIN_MODE to value 0"]
impl crate::Resettable for SMEM_DIN_MODE_SPEC {}
