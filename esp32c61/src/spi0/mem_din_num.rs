#[doc = "Register `MEM_DIN_NUM` reader"]
pub type R = crate::R<MEM_DIN_NUM_SPEC>;
#[doc = "Register `MEM_DIN_NUM` writer"]
pub type W = crate::W<MEM_DIN_NUM_SPEC>;
#[doc = "Field `MEM_DIN0_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN0_NUM_R = crate::FieldReader;
#[doc = "Field `MEM_DIN0_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN0_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_DIN1_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN1_NUM_R = crate::FieldReader;
#[doc = "Field `MEM_DIN1_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN1_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_DIN2_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN2_NUM_R = crate::FieldReader;
#[doc = "Field `MEM_DIN2_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN2_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_DIN3_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN3_NUM_R = crate::FieldReader;
#[doc = "Field `MEM_DIN3_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN3_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_DIN4_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN4_NUM_R = crate::FieldReader;
#[doc = "Field `MEM_DIN4_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN4_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_DIN5_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN5_NUM_R = crate::FieldReader;
#[doc = "Field `MEM_DIN5_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN5_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_DIN6_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN6_NUM_R = crate::FieldReader;
#[doc = "Field `MEM_DIN6_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN6_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_DIN7_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN7_NUM_R = crate::FieldReader;
#[doc = "Field `MEM_DIN7_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DIN7_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_DINS_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DINS_NUM_R = crate::FieldReader;
#[doc = "Field `MEM_DINS_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type MEM_DINS_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din0_num(&self) -> MEM_DIN0_NUM_R {
        MEM_DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din1_num(&self) -> MEM_DIN1_NUM_R {
        MEM_DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din2_num(&self) -> MEM_DIN2_NUM_R {
        MEM_DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din3_num(&self) -> MEM_DIN3_NUM_R {
        MEM_DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din4_num(&self) -> MEM_DIN4_NUM_R {
        MEM_DIN4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din5_num(&self) -> MEM_DIN5_NUM_R {
        MEM_DIN5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din6_num(&self) -> MEM_DIN6_NUM_R {
        MEM_DIN6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din7_num(&self) -> MEM_DIN7_NUM_R {
        MEM_DIN7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_dins_num(&self) -> MEM_DINS_NUM_R {
        MEM_DINS_NUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_DIN_NUM")
            .field("mem_din0_num", &self.mem_din0_num())
            .field("mem_din1_num", &self.mem_din1_num())
            .field("mem_din2_num", &self.mem_din2_num())
            .field("mem_din3_num", &self.mem_din3_num())
            .field("mem_din4_num", &self.mem_din4_num())
            .field("mem_din5_num", &self.mem_din5_num())
            .field("mem_din6_num", &self.mem_din6_num())
            .field("mem_din7_num", &self.mem_din7_num())
            .field("mem_dins_num", &self.mem_dins_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din0_num(&mut self) -> MEM_DIN0_NUM_W<MEM_DIN_NUM_SPEC> {
        MEM_DIN0_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din1_num(&mut self) -> MEM_DIN1_NUM_W<MEM_DIN_NUM_SPEC> {
        MEM_DIN1_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din2_num(&mut self) -> MEM_DIN2_NUM_W<MEM_DIN_NUM_SPEC> {
        MEM_DIN2_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din3_num(&mut self) -> MEM_DIN3_NUM_W<MEM_DIN_NUM_SPEC> {
        MEM_DIN3_NUM_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din4_num(&mut self) -> MEM_DIN4_NUM_W<MEM_DIN_NUM_SPEC> {
        MEM_DIN4_NUM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din5_num(&mut self) -> MEM_DIN5_NUM_W<MEM_DIN_NUM_SPEC> {
        MEM_DIN5_NUM_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din6_num(&mut self) -> MEM_DIN6_NUM_W<MEM_DIN_NUM_SPEC> {
        MEM_DIN6_NUM_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_din7_num(&mut self) -> MEM_DIN7_NUM_W<MEM_DIN_NUM_SPEC> {
        MEM_DIN7_NUM_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn mem_dins_num(&mut self) -> MEM_DINS_NUM_W<MEM_DIN_NUM_SPEC> {
        MEM_DINS_NUM_W::new(self, 16)
    }
}
#[doc = "MSPI flash input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_din_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_din_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_DIN_NUM_SPEC;
impl crate::RegisterSpec for MEM_DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_din_num::R`](R) reader structure"]
impl crate::Readable for MEM_DIN_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_din_num::W`](W) writer structure"]
impl crate::Writable for MEM_DIN_NUM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_DIN_NUM to value 0"]
impl crate::Resettable for MEM_DIN_NUM_SPEC {}
