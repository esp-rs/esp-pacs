#[doc = "Register `SMEM_DIN_NUM` reader"]
pub type R = crate::R<SMEM_DIN_NUM_SPEC>;
#[doc = "Register `SMEM_DIN_NUM` writer"]
pub type W = crate::W<SMEM_DIN_NUM_SPEC>;
#[doc = "Field `DIN_NUM(0-7)` reader - "]
pub type DIN_NUM_R = crate::FieldReader;
#[doc = "Field `DIN_NUM(0-7)` writer - "]
pub type DIN_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DINS_NUM` reader - "]
pub type DINS_NUM_R = crate::FieldReader;
#[doc = "Field `DINS_NUM` writer - "]
pub type DINS_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DIN0_NUM` field.</div>"]
    #[inline(always)]
    pub fn din_num(&self, n: u8) -> DIN_NUM_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DIN_NUM_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn din_num_iter(&self) -> impl Iterator<Item = DIN_NUM_R> + '_ {
        (0..8).map(move |n| DIN_NUM_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - DIN0_NUM"]
    #[inline(always)]
    pub fn din0_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DIN1_NUM"]
    #[inline(always)]
    pub fn din1_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DIN2_NUM"]
    #[inline(always)]
    pub fn din2_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DIN3_NUM"]
    #[inline(always)]
    pub fn din3_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DIN4_NUM"]
    #[inline(always)]
    pub fn din4_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DIN5_NUM"]
    #[inline(always)]
    pub fn din5_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - DIN6_NUM"]
    #[inline(always)]
    pub fn din6_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DIN7_NUM"]
    #[inline(always)]
    pub fn din7_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dins_num(&self) -> DINS_NUM_R {
        DINS_NUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_DIN_NUM")
            .field("din0_num", &self.din0_num())
            .field("din1_num", &self.din1_num())
            .field("din2_num", &self.din2_num())
            .field("din3_num", &self.din3_num())
            .field("din4_num", &self.din4_num())
            .field("din5_num", &self.din5_num())
            .field("din6_num", &self.din6_num())
            .field("din7_num", &self.din7_num())
            .field("dins_num", &self.dins_num())
            .finish()
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DIN0_NUM` field.</div>"]
    #[inline(always)]
    pub fn din_num(&mut self, n: u8) -> DIN_NUM_W<'_, SMEM_DIN_NUM_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DIN_NUM_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - DIN0_NUM"]
    #[inline(always)]
    pub fn din0_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_NUM_SPEC> {
        DIN_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - DIN1_NUM"]
    #[inline(always)]
    pub fn din1_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_NUM_SPEC> {
        DIN_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - DIN2_NUM"]
    #[inline(always)]
    pub fn din2_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_NUM_SPEC> {
        DIN_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - DIN3_NUM"]
    #[inline(always)]
    pub fn din3_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_NUM_SPEC> {
        DIN_NUM_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - DIN4_NUM"]
    #[inline(always)]
    pub fn din4_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_NUM_SPEC> {
        DIN_NUM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - DIN5_NUM"]
    #[inline(always)]
    pub fn din5_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_NUM_SPEC> {
        DIN_NUM_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - DIN6_NUM"]
    #[inline(always)]
    pub fn din6_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_NUM_SPEC> {
        DIN_NUM_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - DIN7_NUM"]
    #[inline(always)]
    pub fn din7_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_NUM_SPEC> {
        DIN_NUM_W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dins_num(&mut self) -> DINS_NUM_W<'_, SMEM_DIN_NUM_SPEC> {
        DINS_NUM_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_din_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_DIN_NUM_SPEC;
impl crate::RegisterSpec for SMEM_DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_din_num::R`](R) reader structure"]
impl crate::Readable for SMEM_DIN_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_din_num::W`](W) writer structure"]
impl crate::Writable for SMEM_DIN_NUM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_DIN_NUM to value 0"]
impl crate::Resettable for SMEM_DIN_NUM_SPEC {}
