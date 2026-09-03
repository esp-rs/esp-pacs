#[doc = "Register `INTMASK` reader"]
pub type R = crate::R<INTMASK_SPEC>;
#[doc = "Register `INTMASK` writer"]
pub type W = crate::W<INTMASK_SPEC>;
#[doc = "Field `PMTINTMASK` reader - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
pub type PMTINTMASK_R = crate::BitReader;
#[doc = "Field `PMTINTMASK` writer - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
pub type PMTINTMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPIINTMASK` reader - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
pub type LPIINTMASK_R = crate::BitReader;
#[doc = "Field `LPIINTMASK` writer - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
pub type LPIINTMASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pmtintmask(&self) -> PMTINTMASK_R {
        PMTINTMASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    pub fn lpiintmask(&self) -> LPIINTMASK_R {
        LPIINTMASK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTMASK")
            .field("pmtintmask", &self.pmtintmask())
            .field("lpiintmask", &self.lpiintmask())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - When set this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pmtintmask(&mut self) -> PMTINTMASK_W<'_, INTMASK_SPEC> {
        PMTINTMASK_W::new(self, 3)
    }
    #[doc = "Bit 10 - When set this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register (Interrupt Status Register)."]
    #[inline(always)]
    pub fn lpiintmask(&mut self) -> LPIINTMASK_W<'_, INTMASK_SPEC> {
        LPIINTMASK_W::new(self, 10)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMASK_SPEC;
impl crate::RegisterSpec for INTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmask::R`](R) reader structure"]
impl crate::Readable for INTMASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intmask::W`](W) writer structure"]
impl crate::Writable for INTMASK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for INTMASK_SPEC {}
