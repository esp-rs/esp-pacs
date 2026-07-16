#[doc = "Register `KEY_MANAGER_CTRL0` reader"]
pub type R = crate::R<KEY_MANAGER_CTRL0_SPEC>;
#[doc = "Register `KEY_MANAGER_CTRL0` writer"]
pub type W = crate::W<KEY_MANAGER_CTRL0_SPEC>;
#[doc = "Field `KEY_MANAGER_SYS_CLK_EN` reader - need_des"]
pub type KEY_MANAGER_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `KEY_MANAGER_SYS_CLK_EN` writer - need_des"]
pub type KEY_MANAGER_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_KM_RST_EN` reader - need_des"]
pub type CRYPTO_KM_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_KM_RST_EN` writer - need_des"]
pub type CRYPTO_KM_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_KM_CLK_EN` reader - need_des"]
pub type CRYPTO_KM_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_KM_CLK_EN` writer - need_des"]
pub type CRYPTO_KM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn key_manager_sys_clk_en(&self) -> KEY_MANAGER_SYS_CLK_EN_R {
        KEY_MANAGER_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn crypto_km_rst_en(&self) -> CRYPTO_KM_RST_EN_R {
        CRYPTO_KM_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn crypto_km_clk_en(&self) -> CRYPTO_KM_CLK_EN_R {
        CRYPTO_KM_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_MANAGER_CTRL0")
            .field("key_manager_sys_clk_en", &self.key_manager_sys_clk_en())
            .field("crypto_km_rst_en", &self.crypto_km_rst_en())
            .field("crypto_km_clk_en", &self.crypto_km_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn key_manager_sys_clk_en(
        &mut self,
    ) -> KEY_MANAGER_SYS_CLK_EN_W<'_, KEY_MANAGER_CTRL0_SPEC> {
        KEY_MANAGER_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn crypto_km_rst_en(&mut self) -> CRYPTO_KM_RST_EN_W<'_, KEY_MANAGER_CTRL0_SPEC> {
        CRYPTO_KM_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn crypto_km_clk_en(&mut self) -> CRYPTO_KM_CLK_EN_W<'_, KEY_MANAGER_CTRL0_SPEC> {
        CRYPTO_KM_CLK_EN_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`key_manager_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_manager_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_MANAGER_CTRL0_SPEC;
impl crate::RegisterSpec for KEY_MANAGER_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_manager_ctrl0::R`](R) reader structure"]
impl crate::Readable for KEY_MANAGER_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_manager_ctrl0::W`](W) writer structure"]
impl crate::Writable for KEY_MANAGER_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_MANAGER_CTRL0 to value 0x05"]
impl crate::Resettable for KEY_MANAGER_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
