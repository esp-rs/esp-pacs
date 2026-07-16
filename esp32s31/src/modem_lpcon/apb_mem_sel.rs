#[doc = "Register `APB_MEM_SEL` reader"]
pub type R = crate::R<APB_MEM_SEL_SPEC>;
#[doc = "Register `APB_MEM_SEL` writer"]
pub type W = crate::W<APB_MEM_SEL_SPEC>;
#[doc = "Field `CHAN_FREQ_MEM_EN` reader - "]
pub type CHAN_FREQ_MEM_EN_R = crate::BitReader;
#[doc = "Field `CHAN_FREQ_MEM_EN` writer - "]
pub type CHAN_FREQ_MEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBUS_MEM_EN` reader - "]
pub type PBUS_MEM_EN_R = crate::BitReader;
#[doc = "Field `PBUS_MEM_EN` writer - "]
pub type PBUS_MEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGC_MEM_EN` reader - "]
pub type AGC_MEM_EN_R = crate::BitReader;
#[doc = "Field `AGC_MEM_EN` writer - "]
pub type AGC_MEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFR_MEM_EN` reader - "]
pub type CFR_MEM_EN_R = crate::BitReader;
#[doc = "Field `CFR_MEM_EN` writer - "]
pub type CFR_MEM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWR_MEM_BASE` reader - "]
pub type PWR_MEM_BASE_R = crate::FieldReader<u16>;
#[doc = "Field `PWR_MEM_BASE` writer - "]
pub type PWR_MEM_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn chan_freq_mem_en(&self) -> CHAN_FREQ_MEM_EN_R {
        CHAN_FREQ_MEM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pbus_mem_en(&self) -> PBUS_MEM_EN_R {
        PBUS_MEM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn agc_mem_en(&self) -> AGC_MEM_EN_R {
        AGC_MEM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cfr_mem_en(&self) -> CFR_MEM_EN_R {
        CFR_MEM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn pwr_mem_base(&self) -> PWR_MEM_BASE_R {
        PWR_MEM_BASE_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_MEM_SEL")
            .field("chan_freq_mem_en", &self.chan_freq_mem_en())
            .field("pbus_mem_en", &self.pbus_mem_en())
            .field("agc_mem_en", &self.agc_mem_en())
            .field("cfr_mem_en", &self.cfr_mem_en())
            .field("pwr_mem_base", &self.pwr_mem_base())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn chan_freq_mem_en(&mut self) -> CHAN_FREQ_MEM_EN_W<'_, APB_MEM_SEL_SPEC> {
        CHAN_FREQ_MEM_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pbus_mem_en(&mut self) -> PBUS_MEM_EN_W<'_, APB_MEM_SEL_SPEC> {
        PBUS_MEM_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn agc_mem_en(&mut self) -> AGC_MEM_EN_W<'_, APB_MEM_SEL_SPEC> {
        AGC_MEM_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cfr_mem_en(&mut self) -> CFR_MEM_EN_W<'_, APB_MEM_SEL_SPEC> {
        CFR_MEM_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn pwr_mem_base(&mut self) -> PWR_MEM_BASE_W<'_, APB_MEM_SEL_SPEC> {
        PWR_MEM_BASE_W::new(self, 4)
    }
}
#[doc = "APB_MEM_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_mem_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_mem_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_MEM_SEL_SPEC;
impl crate::RegisterSpec for APB_MEM_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_mem_sel::R`](R) reader structure"]
impl crate::Readable for APB_MEM_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_mem_sel::W`](W) writer structure"]
impl crate::Writable for APB_MEM_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB_MEM_SEL to value 0"]
impl crate::Resettable for APB_MEM_SEL_SPEC {}
