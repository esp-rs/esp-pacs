#[doc = "Register `INT_CTRL%s` reader"]
pub type R = crate::R<INT_CTRL_SPEC>;
#[doc = "Register `INT_CTRL%s` writer"]
pub type W = crate::W<INT_CTRL_SPEC>;
#[doc = "Field `INT_IP` reader - Pending interrupt bit"]
pub type INT_IP_R = crate::BitReader;
#[doc = "Field `INT_IP` writer - Pending interrupt bit"]
pub type INT_IP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_IE` reader - Interrupt enable bit"]
pub type INT_IE_R = crate::BitReader;
#[doc = "Field `INT_IE` writer - Interrupt enable bit"]
pub type INT_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_ATTR_SHV` reader - Shadow vector attribute"]
pub type INT_ATTR_SHV_R = crate::BitReader;
#[doc = "Field `INT_ATTR_SHV` writer - Shadow vector attribute"]
pub type INT_ATTR_SHV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_ATTR_TRIG` reader - Trigger attribute for interrupt"]
pub type INT_ATTR_TRIG_R = crate::FieldReader;
#[doc = "Field `INT_ATTR_TRIG` writer - Trigger attribute for interrupt"]
pub type INT_ATTR_TRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT_ATTR_MODE` reader - Interrupt mode attribute"]
pub type INT_ATTR_MODE_R = crate::FieldReader;
#[doc = "Field `INT_ATTR_MODE` writer - Interrupt mode attribute"]
pub type INT_ATTR_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT_CTL` reader - Control bits for interrupt"]
pub type INT_CTL_R = crate::FieldReader;
#[doc = "Field `INT_CTL` writer - Control bits for interrupt"]
pub type INT_CTL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Pending interrupt bit"]
    #[inline(always)]
    pub fn int_ip(&self) -> INT_IP_R {
        INT_IP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable bit"]
    #[inline(always)]
    pub fn int_ie(&self) -> INT_IE_R {
        INT_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Shadow vector attribute"]
    #[inline(always)]
    pub fn int_attr_shv(&self) -> INT_ATTR_SHV_R {
        INT_ATTR_SHV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Trigger attribute for interrupt"]
    #[inline(always)]
    pub fn int_attr_trig(&self) -> INT_ATTR_TRIG_R {
        INT_ATTR_TRIG_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Interrupt mode attribute"]
    #[inline(always)]
    pub fn int_attr_mode(&self) -> INT_ATTR_MODE_R {
        INT_ATTR_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Control bits for interrupt"]
    #[inline(always)]
    pub fn int_ctl(&self) -> INT_CTL_R {
        INT_CTL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CTRL")
            .field("int_ctl", &self.int_ctl())
            .field("int_attr_mode", &self.int_attr_mode())
            .field("int_attr_trig", &self.int_attr_trig())
            .field("int_attr_shv", &self.int_attr_shv())
            .field("int_ie", &self.int_ie())
            .field("int_ip", &self.int_ip())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Pending interrupt bit"]
    #[inline(always)]
    pub fn int_ip(&mut self) -> INT_IP_W<INT_CTRL_SPEC> {
        INT_IP_W::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable bit"]
    #[inline(always)]
    pub fn int_ie(&mut self) -> INT_IE_W<INT_CTRL_SPEC> {
        INT_IE_W::new(self, 8)
    }
    #[doc = "Bit 16 - Shadow vector attribute"]
    #[inline(always)]
    pub fn int_attr_shv(&mut self) -> INT_ATTR_SHV_W<INT_CTRL_SPEC> {
        INT_ATTR_SHV_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Trigger attribute for interrupt"]
    #[inline(always)]
    pub fn int_attr_trig(&mut self) -> INT_ATTR_TRIG_W<INT_CTRL_SPEC> {
        INT_ATTR_TRIG_W::new(self, 17)
    }
    #[doc = "Bits 22:23 - Interrupt mode attribute"]
    #[inline(always)]
    pub fn int_attr_mode(&mut self) -> INT_ATTR_MODE_W<INT_CTRL_SPEC> {
        INT_ATTR_MODE_W::new(self, 22)
    }
    #[doc = "Bits 24:31 - Control bits for interrupt"]
    #[inline(always)]
    pub fn int_ctl(&mut self) -> INT_CTL_W<INT_CTRL_SPEC> {
        INT_CTL_W::new(self, 24)
    }
}
#[doc = "Interrupt control register for each interrupt source\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CTRL_SPEC;
impl crate::RegisterSpec for INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ctrl::R`](R) reader structure"]
impl crate::Readable for INT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ctrl::W`](W) writer structure"]
impl crate::Writable for INT_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CTRL%s to value 0"]
impl crate::Resettable for INT_CTRL_SPEC {}
