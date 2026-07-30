#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `GPIO_EXT_COMP_NEG_0` reader - analog comparator pos edge interrupt enable"]
pub type GPIO_EXT_COMP_NEG_0_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_COMP_NEG_0` writer - analog comparator pos edge interrupt enable"]
pub type GPIO_EXT_COMP_NEG_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_EXT_COMP_POS_0` reader - analog comparator neg edge interrupt enable"]
pub type GPIO_EXT_COMP_POS_0_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_COMP_POS_0` writer - analog comparator neg edge interrupt enable"]
pub type GPIO_EXT_COMP_POS_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_EXT_COMP_ALL_0` reader - analog comparator neg or pos edge interrupt enable"]
pub type GPIO_EXT_COMP_ALL_0_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_COMP_ALL_0` writer - analog comparator neg or pos edge interrupt enable"]
pub type GPIO_EXT_COMP_ALL_0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    pub fn gpio_ext_comp_neg_0(&self) -> GPIO_EXT_COMP_NEG_0_R {
        GPIO_EXT_COMP_NEG_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    pub fn gpio_ext_comp_pos_0(&self) -> GPIO_EXT_COMP_POS_0_R {
        GPIO_EXT_COMP_POS_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    pub fn gpio_ext_comp_all_0(&self) -> GPIO_EXT_COMP_ALL_0_R {
        GPIO_EXT_COMP_ALL_0_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("gpio_ext_comp_neg_0", &self.gpio_ext_comp_neg_0())
            .field("gpio_ext_comp_pos_0", &self.gpio_ext_comp_pos_0())
            .field("gpio_ext_comp_all_0", &self.gpio_ext_comp_all_0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt enable"]
    #[inline(always)]
    pub fn gpio_ext_comp_neg_0(&mut self) -> GPIO_EXT_COMP_NEG_0_W<'_, INT_ENA_SPEC> {
        GPIO_EXT_COMP_NEG_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt enable"]
    #[inline(always)]
    pub fn gpio_ext_comp_pos_0(&mut self) -> GPIO_EXT_COMP_POS_0_W<'_, INT_ENA_SPEC> {
        GPIO_EXT_COMP_POS_0_W::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt enable"]
    #[inline(always)]
    pub fn gpio_ext_comp_all_0(&mut self) -> GPIO_EXT_COMP_ALL_0_W<'_, INT_ENA_SPEC> {
        GPIO_EXT_COMP_ALL_0_W::new(self, 2)
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
