#[doc = "Register `DIN_NUM` reader"]
pub type R = crate::R<DIN_NUM_SPEC>;
#[doc = "Register `DIN_NUM` writer"]
pub type W = crate::W<DIN_NUM_SPEC>;
#[doc = "Field `DIN0_NUM` reader - SPI_D input delay number."]
pub type DIN0_NUM_R = crate::FieldReader;
#[doc = "Field `DIN0_NUM` writer - SPI_D input delay number."]
pub type DIN0_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIN1_NUM` reader - SPI_Q input delay number."]
pub type DIN1_NUM_R = crate::FieldReader;
#[doc = "Field `DIN1_NUM` writer - SPI_Q input delay number."]
pub type DIN1_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIN2_NUM` reader - SPI_WP input delay number."]
pub type DIN2_NUM_R = crate::FieldReader;
#[doc = "Field `DIN2_NUM` writer - SPI_WP input delay number."]
pub type DIN2_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIN3_NUM` reader - SPI_HD input delay number."]
pub type DIN3_NUM_R = crate::FieldReader;
#[doc = "Field `DIN3_NUM` writer - SPI_HD input delay number."]
pub type DIN3_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIN4_NUM` reader - SPI_IO4 input delay number."]
pub type DIN4_NUM_R = crate::FieldReader;
#[doc = "Field `DIN4_NUM` writer - SPI_IO4 input delay number."]
pub type DIN4_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIN5_NUM` reader - SPI_IO5 input delay number."]
pub type DIN5_NUM_R = crate::FieldReader;
#[doc = "Field `DIN5_NUM` writer - SPI_IO5 input delay number."]
pub type DIN5_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIN6_NUM` reader - SPI_IO6 input delay number."]
pub type DIN6_NUM_R = crate::FieldReader;
#[doc = "Field `DIN6_NUM` writer - SPI_IO6 input delay number."]
pub type DIN6_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIN7_NUM` reader - SPI_IO7 input delay number."]
pub type DIN7_NUM_R = crate::FieldReader;
#[doc = "Field `DIN7_NUM` writer - SPI_IO7 input delay number."]
pub type DIN7_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DINS_NUM` reader - SPI_DQS input delay number."]
pub type DINS_NUM_R = crate::FieldReader;
#[doc = "Field `DINS_NUM` writer - SPI_DQS input delay number."]
pub type DINS_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - SPI_D input delay number."]
    #[inline(always)]
    pub fn din0_num(&self) -> DIN0_NUM_R {
        DIN0_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI_Q input delay number."]
    #[inline(always)]
    pub fn din1_num(&self) -> DIN1_NUM_R {
        DIN1_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SPI_WP input delay number."]
    #[inline(always)]
    pub fn din2_num(&self) -> DIN2_NUM_R {
        DIN2_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SPI_HD input delay number."]
    #[inline(always)]
    pub fn din3_num(&self) -> DIN3_NUM_R {
        DIN3_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SPI_IO4 input delay number."]
    #[inline(always)]
    pub fn din4_num(&self) -> DIN4_NUM_R {
        DIN4_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SPI_IO5 input delay number."]
    #[inline(always)]
    pub fn din5_num(&self) -> DIN5_NUM_R {
        DIN5_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SPI_IO6 input delay number."]
    #[inline(always)]
    pub fn din6_num(&self) -> DIN6_NUM_R {
        DIN6_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - SPI_IO7 input delay number."]
    #[inline(always)]
    pub fn din7_num(&self) -> DIN7_NUM_R {
        DIN7_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SPI_DQS input delay number."]
    #[inline(always)]
    pub fn dins_num(&self) -> DINS_NUM_R {
        DINS_NUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIN_NUM")
            .field("din0_num", &format_args!("{}", self.din0_num().bits()))
            .field("din1_num", &format_args!("{}", self.din1_num().bits()))
            .field("din2_num", &format_args!("{}", self.din2_num().bits()))
            .field("din3_num", &format_args!("{}", self.din3_num().bits()))
            .field("din4_num", &format_args!("{}", self.din4_num().bits()))
            .field("din5_num", &format_args!("{}", self.din5_num().bits()))
            .field("din6_num", &format_args!("{}", self.din6_num().bits()))
            .field("din7_num", &format_args!("{}", self.din7_num().bits()))
            .field("dins_num", &format_args!("{}", self.dins_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIN_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI_D input delay number."]
    #[inline(always)]
    #[must_use]
    pub fn din0_num(&mut self) -> DIN0_NUM_W<DIN_NUM_SPEC> {
        DIN0_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - SPI_Q input delay number."]
    #[inline(always)]
    #[must_use]
    pub fn din1_num(&mut self) -> DIN1_NUM_W<DIN_NUM_SPEC> {
        DIN1_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - SPI_WP input delay number."]
    #[inline(always)]
    #[must_use]
    pub fn din2_num(&mut self) -> DIN2_NUM_W<DIN_NUM_SPEC> {
        DIN2_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - SPI_HD input delay number."]
    #[inline(always)]
    #[must_use]
    pub fn din3_num(&mut self) -> DIN3_NUM_W<DIN_NUM_SPEC> {
        DIN3_NUM_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - SPI_IO4 input delay number."]
    #[inline(always)]
    #[must_use]
    pub fn din4_num(&mut self) -> DIN4_NUM_W<DIN_NUM_SPEC> {
        DIN4_NUM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - SPI_IO5 input delay number."]
    #[inline(always)]
    #[must_use]
    pub fn din5_num(&mut self) -> DIN5_NUM_W<DIN_NUM_SPEC> {
        DIN5_NUM_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - SPI_IO6 input delay number."]
    #[inline(always)]
    #[must_use]
    pub fn din6_num(&mut self) -> DIN6_NUM_W<DIN_NUM_SPEC> {
        DIN6_NUM_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - SPI_IO7 input delay number."]
    #[inline(always)]
    #[must_use]
    pub fn din7_num(&mut self) -> DIN7_NUM_W<DIN_NUM_SPEC> {
        DIN7_NUM_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - SPI_DQS input delay number."]
    #[inline(always)]
    #[must_use]
    pub fn dins_num(&mut self) -> DINS_NUM_W<DIN_NUM_SPEC> {
        DINS_NUM_W::new(self, 16)
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
#[doc = "MSPI input timing delay number control register when accesses to flash.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIN_NUM_SPEC;
impl crate::RegisterSpec for DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din_num::R`](R) reader structure"]
impl crate::Readable for DIN_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`din_num::W`](W) writer structure"]
impl crate::Writable for DIN_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIN_NUM to value 0"]
impl crate::Resettable for DIN_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
