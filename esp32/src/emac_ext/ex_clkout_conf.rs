#[doc = "Register `EX_CLKOUT_CONF` reader"]
pub type R = crate::R<EX_CLKOUT_CONF_SPEC>;
#[doc = "Register `EX_CLKOUT_CONF` writer"]
pub type W = crate::W<EX_CLKOUT_CONF_SPEC>;
#[doc = "Field `DIV_NUM` reader - "]
pub type DIV_NUM_R = crate::FieldReader;
#[doc = "Field `DIV_NUM` writer - "]
pub type DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `H_DIV_NUM` reader - "]
pub type H_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `H_DIV_NUM` writer - "]
pub type H_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLY_NUM` reader - "]
pub type DLY_NUM_R = crate::FieldReader;
#[doc = "Field `DLY_NUM` writer - "]
pub type DLY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn div_num(&self) -> DIV_NUM_R {
        DIV_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn h_div_num(&self) -> H_DIV_NUM_R {
        H_DIV_NUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dly_num(&self) -> DLY_NUM_R {
        DLY_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EX_CLKOUT_CONF")
            .field("div_num", &self.div_num())
            .field("h_div_num", &self.h_div_num())
            .field("dly_num", &self.dly_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn div_num(&mut self) -> DIV_NUM_W<EX_CLKOUT_CONF_SPEC> {
        DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn h_div_num(&mut self) -> H_DIV_NUM_W<EX_CLKOUT_CONF_SPEC> {
        H_DIV_NUM_W::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn dly_num(&mut self) -> DLY_NUM_W<EX_CLKOUT_CONF_SPEC> {
        DLY_NUM_W::new(self, 8)
    }
}
#[doc = "RMII clock divider setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ex_clkout_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ex_clkout_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EX_CLKOUT_CONF_SPEC;
impl crate::RegisterSpec for EX_CLKOUT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ex_clkout_conf::R`](R) reader structure"]
impl crate::Readable for EX_CLKOUT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ex_clkout_conf::W`](W) writer structure"]
impl crate::Writable for EX_CLKOUT_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EX_CLKOUT_CONF to value 0"]
impl crate::Resettable for EX_CLKOUT_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
