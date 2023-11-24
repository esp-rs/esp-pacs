#[doc = "Register `PLL` reader"]
pub type R = crate::R<PLL_SPEC>;
#[doc = "Register `PLL` writer"]
pub type W = crate::W<PLL_SPEC>;
#[doc = "Field `BLOCK` reader - Block"]
pub type BLOCK_R = crate::FieldReader;
#[doc = "Field `BLOCK` writer - Block"]
pub type BLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDR` reader - Address"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA` reader - Data"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - Data"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRITE` reader - Write"]
pub type WRITE_R = crate::BitReader;
#[doc = "Field `WRITE` writer - Write"]
pub type WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Ready"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - Ready"]
pub type BUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Block"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Write"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Ready"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL")
            .field("block", &format_args!("{}", self.block().bits()))
            .field("addr", &format_args!("{}", self.addr().bits()))
            .field("data", &format_args!("{}", self.data().bits()))
            .field("write", &format_args!("{}", self.write().bit()))
            .field("busy", &format_args!("{}", self.busy().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PLL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Block"]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<PLL_SPEC> {
        BLOCK_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<PLL_SPEC> {
        ADDR_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<PLL_SPEC> {
        DATA_W::new(self, 16)
    }
    #[doc = "Bit 24 - Write"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<PLL_SPEC> {
        WRITE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Ready"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<PLL_SPEC> {
        BUSY_W::new(self, 25)
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
#[doc = "PLL I2C Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_SPEC;
impl crate::RegisterSpec for PLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll::R`](R) reader structure"]
impl crate::Readable for PLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll::W`](W) writer structure"]
impl crate::Writable for PLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL to value 0"]
impl crate::Resettable for PLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
