#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `PER_END` reader - The enable bit for SPI_MEM_PER_END_INT interrupt."]
pub type PER_END_R = crate::BitReader;
#[doc = "Field `PER_END` writer - The enable bit for SPI_MEM_PER_END_INT interrupt."]
pub type PER_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_END` reader - The enable bit for SPI_MEM_PES_END_INT interrupt."]
pub type PES_END_R = crate::BitReader;
#[doc = "Field `PES_END` writer - The enable bit for SPI_MEM_PES_END_INT interrupt."]
pub type PES_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOTAL_TRANS_END` reader - The enable bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
pub type TOTAL_TRANS_END_R = crate::BitReader;
#[doc = "Field `TOTAL_TRANS_END` writer - The enable bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
pub type TOTAL_TRANS_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT` reader - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` writer - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The enable bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    pub fn per_end(&self) -> PER_END_R {
        PER_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    pub fn pes_end(&self) -> PES_END_R {
        PES_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
    #[inline(always)]
    pub fn total_trans_end(&self) -> TOTAL_TRANS_END_R {
        TOTAL_TRANS_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("per_end", &self.per_end())
            .field("pes_end", &self.pes_end())
            .field("total_trans_end", &self.total_trans_end())
            .field("brown_out", &self.brown_out())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for SPI_MEM_PER_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn per_end(&mut self) -> PER_END_W<INT_ENA_SPEC> {
        PER_END_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable bit for SPI_MEM_PES_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pes_end(&mut self) -> PES_END_W<INT_ENA_SPEC> {
        PES_END_W::new(self, 1)
    }
    #[doc = "Bit 2 - The enable bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn total_trans_end(&mut self) -> TOTAL_TRANS_END_W<INT_ENA_SPEC> {
        TOTAL_TRANS_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - The enable bit for SPI_MEM_BROWN_OUT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<INT_ENA_SPEC> {
        BROWN_OUT_W::new(self, 3)
    }
}
#[doc = "SPI1 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
