#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `COMP_NEG_0_INT_ENA` reader - analog comparator pos edge interrupt enable"]
pub type COMP_NEG_0_INT_ENA_R = crate::BitReader;
#[doc = "Field `COMP_NEG_0_INT_ENA` writer - analog comparator pos edge interrupt enable"]
pub type COMP_NEG_0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_POS_0_INT_ENA` reader - analog comparator neg edge interrupt enable"]
pub type COMP_POS_0_INT_ENA_R = crate::BitReader;
#[doc = "Field `COMP_POS_0_INT_ENA` writer - analog comparator neg edge interrupt enable"]
pub type COMP_POS_0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP_ALL_0_INT_ENA` reader - analog comparator neg or pos edge interrupt enable"]
pub type COMP_ALL_0_INT_ENA_R = crate::BitReader;
#[doc = "Field `COMP_ALL_0_INT_ENA` writer - analog comparator neg or pos edge interrupt enable"]
pub type COMP_ALL_0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp_neg_0_int_ena(&self) -> COMP_NEG_0_INT_ENA_R {
        COMP_NEG_0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    pub fn comp_pos_0_int_ena(&self) -> COMP_POS_0_INT_ENA_R {
        COMP_POS_0_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp_all_0_int_ena(&self) -> COMP_ALL_0_INT_ENA_R {
        COMP_ALL_0_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("comp_neg_0_int_ena", &self.comp_neg_0_int_ena())
            .field("comp_pos_0_int_ena", &self.comp_pos_0_int_ena())
            .field("comp_all_0_int_ena", &self.comp_all_0_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp_neg_0_int_ena(&mut self) -> COMP_NEG_0_INT_ENA_W<INT_ENA_SPEC> {
        COMP_NEG_0_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    pub fn comp_pos_0_int_ena(&mut self) -> COMP_POS_0_INT_ENA_W<INT_ENA_SPEC> {
        COMP_POS_0_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    pub fn comp_all_0_int_ena(&mut self) -> COMP_ALL_0_INT_ENA_W<INT_ENA_SPEC> {
        COMP_ALL_0_INT_ENA_W::new(self, 2)
    }
}
#[doc = "GPIO_EXT interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0x07"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
