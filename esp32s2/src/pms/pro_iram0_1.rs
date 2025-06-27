#[doc = "Register `PRO_IRAM0_1` reader"]
pub type R = crate::R<PRO_IRAM0_1_SPEC>;
#[doc = "Register `PRO_IRAM0_1` writer"]
pub type W = crate::W<PRO_IRAM0_1_SPEC>;
#[doc = "Field `PRO_IRAM0_SRAM_0_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 0."]
pub type PRO_IRAM0_SRAM_0_F_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_0_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 0."]
pub type PRO_IRAM0_SRAM_0_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_0_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 0."]
pub type PRO_IRAM0_SRAM_0_R_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_0_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 0."]
pub type PRO_IRAM0_SRAM_0_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_0_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 0."]
pub type PRO_IRAM0_SRAM_0_W_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_0_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 0."]
pub type PRO_IRAM0_SRAM_0_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_1_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 1."]
pub type PRO_IRAM0_SRAM_1_F_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_1_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 1."]
pub type PRO_IRAM0_SRAM_1_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_1_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 1."]
pub type PRO_IRAM0_SRAM_1_R_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_1_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 1."]
pub type PRO_IRAM0_SRAM_1_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_1_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 1."]
pub type PRO_IRAM0_SRAM_1_W_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_1_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 1."]
pub type PRO_IRAM0_SRAM_1_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_2_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 2."]
pub type PRO_IRAM0_SRAM_2_F_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_2_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 2."]
pub type PRO_IRAM0_SRAM_2_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_2_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 2."]
pub type PRO_IRAM0_SRAM_2_R_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_2_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 2."]
pub type PRO_IRAM0_SRAM_2_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_2_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 2."]
pub type PRO_IRAM0_SRAM_2_W_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_2_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 2."]
pub type PRO_IRAM0_SRAM_2_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_3_F` reader - Setting to 1 grants IBUS permission to fetch SRAM Block 3."]
pub type PRO_IRAM0_SRAM_3_F_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_3_F` writer - Setting to 1 grants IBUS permission to fetch SRAM Block 3."]
pub type PRO_IRAM0_SRAM_3_F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_3_R` reader - Setting to 1 grants IBUS permission to read SRAM Block 3."]
pub type PRO_IRAM0_SRAM_3_R_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_3_R` writer - Setting to 1 grants IBUS permission to read SRAM Block 3."]
pub type PRO_IRAM0_SRAM_3_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRO_IRAM0_SRAM_3_W` reader - Setting to 1 grants IBUS permission to write SRAM Block 3."]
pub type PRO_IRAM0_SRAM_3_W_R = crate::BitReader;
#[doc = "Field `PRO_IRAM0_SRAM_3_W` writer - Setting to 1 grants IBUS permission to write SRAM Block 3."]
pub type PRO_IRAM0_SRAM_3_W_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setting to 1 grants IBUS permission to fetch SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_f(&self) -> PRO_IRAM0_SRAM_0_F_R {
        PRO_IRAM0_SRAM_0_F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting to 1 grants IBUS permission to read SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_r(&self) -> PRO_IRAM0_SRAM_0_R_R {
        PRO_IRAM0_SRAM_0_R_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting to 1 grants IBUS permission to write SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_w(&self) -> PRO_IRAM0_SRAM_0_W_R {
        PRO_IRAM0_SRAM_0_W_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setting to 1 grants IBUS permission to fetch SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_f(&self) -> PRO_IRAM0_SRAM_1_F_R {
        PRO_IRAM0_SRAM_1_F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting to 1 grants IBUS permission to read SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_r(&self) -> PRO_IRAM0_SRAM_1_R_R {
        PRO_IRAM0_SRAM_1_R_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setting to 1 grants IBUS permission to write SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_w(&self) -> PRO_IRAM0_SRAM_1_W_R {
        PRO_IRAM0_SRAM_1_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Setting to 1 grants IBUS permission to fetch SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_f(&self) -> PRO_IRAM0_SRAM_2_F_R {
        PRO_IRAM0_SRAM_2_F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Setting to 1 grants IBUS permission to read SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_r(&self) -> PRO_IRAM0_SRAM_2_R_R {
        PRO_IRAM0_SRAM_2_R_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Setting to 1 grants IBUS permission to write SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_w(&self) -> PRO_IRAM0_SRAM_2_W_R {
        PRO_IRAM0_SRAM_2_W_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Setting to 1 grants IBUS permission to fetch SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_f(&self) -> PRO_IRAM0_SRAM_3_F_R {
        PRO_IRAM0_SRAM_3_F_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Setting to 1 grants IBUS permission to read SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_r(&self) -> PRO_IRAM0_SRAM_3_R_R {
        PRO_IRAM0_SRAM_3_R_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Setting to 1 grants IBUS permission to write SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_w(&self) -> PRO_IRAM0_SRAM_3_W_R {
        PRO_IRAM0_SRAM_3_W_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_IRAM0_1")
            .field("pro_iram0_sram_0_f", &self.pro_iram0_sram_0_f())
            .field("pro_iram0_sram_0_r", &self.pro_iram0_sram_0_r())
            .field("pro_iram0_sram_0_w", &self.pro_iram0_sram_0_w())
            .field("pro_iram0_sram_1_f", &self.pro_iram0_sram_1_f())
            .field("pro_iram0_sram_1_r", &self.pro_iram0_sram_1_r())
            .field("pro_iram0_sram_1_w", &self.pro_iram0_sram_1_w())
            .field("pro_iram0_sram_2_f", &self.pro_iram0_sram_2_f())
            .field("pro_iram0_sram_2_r", &self.pro_iram0_sram_2_r())
            .field("pro_iram0_sram_2_w", &self.pro_iram0_sram_2_w())
            .field("pro_iram0_sram_3_f", &self.pro_iram0_sram_3_f())
            .field("pro_iram0_sram_3_r", &self.pro_iram0_sram_3_r())
            .field("pro_iram0_sram_3_w", &self.pro_iram0_sram_3_w())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 grants IBUS permission to fetch SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_f(&mut self) -> PRO_IRAM0_SRAM_0_F_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_0_F_W::new(self, 0)
    }
    #[doc = "Bit 1 - Setting to 1 grants IBUS permission to read SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_r(&mut self) -> PRO_IRAM0_SRAM_0_R_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_0_R_W::new(self, 1)
    }
    #[doc = "Bit 2 - Setting to 1 grants IBUS permission to write SRAM Block 0."]
    #[inline(always)]
    pub fn pro_iram0_sram_0_w(&mut self) -> PRO_IRAM0_SRAM_0_W_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_0_W_W::new(self, 2)
    }
    #[doc = "Bit 3 - Setting to 1 grants IBUS permission to fetch SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_f(&mut self) -> PRO_IRAM0_SRAM_1_F_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_1_F_W::new(self, 3)
    }
    #[doc = "Bit 4 - Setting to 1 grants IBUS permission to read SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_r(&mut self) -> PRO_IRAM0_SRAM_1_R_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_1_R_W::new(self, 4)
    }
    #[doc = "Bit 5 - Setting to 1 grants IBUS permission to write SRAM Block 1."]
    #[inline(always)]
    pub fn pro_iram0_sram_1_w(&mut self) -> PRO_IRAM0_SRAM_1_W_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_1_W_W::new(self, 5)
    }
    #[doc = "Bit 6 - Setting to 1 grants IBUS permission to fetch SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_f(&mut self) -> PRO_IRAM0_SRAM_2_F_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_2_F_W::new(self, 6)
    }
    #[doc = "Bit 7 - Setting to 1 grants IBUS permission to read SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_r(&mut self) -> PRO_IRAM0_SRAM_2_R_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_2_R_W::new(self, 7)
    }
    #[doc = "Bit 8 - Setting to 1 grants IBUS permission to write SRAM Block 2."]
    #[inline(always)]
    pub fn pro_iram0_sram_2_w(&mut self) -> PRO_IRAM0_SRAM_2_W_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_2_W_W::new(self, 8)
    }
    #[doc = "Bit 9 - Setting to 1 grants IBUS permission to fetch SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_f(&mut self) -> PRO_IRAM0_SRAM_3_F_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_3_F_W::new(self, 9)
    }
    #[doc = "Bit 10 - Setting to 1 grants IBUS permission to read SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_r(&mut self) -> PRO_IRAM0_SRAM_3_R_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_3_R_W::new(self, 10)
    }
    #[doc = "Bit 11 - Setting to 1 grants IBUS permission to write SRAM Block 3."]
    #[inline(always)]
    pub fn pro_iram0_sram_3_w(&mut self) -> PRO_IRAM0_SRAM_3_W_W<PRO_IRAM0_1_SPEC> {
        PRO_IRAM0_SRAM_3_W_W::new(self, 11)
    }
}
#[doc = "IBUS permission control register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_iram0_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_iram0_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_IRAM0_1_SPEC;
impl crate::RegisterSpec for PRO_IRAM0_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_iram0_1::R`](R) reader structure"]
impl crate::Readable for PRO_IRAM0_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_iram0_1::W`](W) writer structure"]
impl crate::Writable for PRO_IRAM0_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_IRAM0_1 to value 0x0fff"]
impl crate::Resettable for PRO_IRAM0_1_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
